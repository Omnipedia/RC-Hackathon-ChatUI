use actix_cors::Cors;
use actix_web::{
    middleware, post,
    web::{Data, Json},
    App, HttpResponse, HttpServer, Responder,
};
use bytes::Bytes;
use futures::Stream;
use message::MESSAGE_CONTENT;
use serde::{Deserialize, Serialize};
use sources::get_sources_list;
use std::{
    fmt::{Display, Formatter},
    pin::Pin,
    string,
    sync::Arc,
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use utoipa::{OpenApi, ToSchema};
use utoipa_redoc::{Redoc, Servable};
mod message;
mod sources;

#[derive(OpenApi)]
#[openapi(
    paths(streaming_conversation),
    components(
        schemas(Message),
        schemas(Source),
        schemas(PartialMessage),
        schemas(Conversation),
    )
)]
pub(crate) struct ApiDoc;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = assistant_partial_message_schema_example)]
pub(crate) struct PartialMessage {
    pub(crate) content: Option<String>,
    pub(crate) source: Option<Source>,
    pub(crate) finished: Option<String>,
}
impl PartialMessage {
    pub(crate) fn done() -> Self {
        Self {
            content: None,
            source: None,
            finished: Some(String::from("DONE")),
        }
    }

    pub(crate) fn source(source: Source) -> Self {
        Self {
            content: None,
            source: Some(source),
            finished: None,
        }
    }

    pub(crate) fn content(content: String) -> Self {
        Self {
            content: Some(content),
            source: None,
            finished: None,
        }
    }

    pub(crate) fn message(self) -> Bytes {
        let message_string = &serde_json::to_string(&self).unwrap();

        Bytes::from(["event: message\ndata: ", message_string, "\n\n"].concat())
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = assistant_message_schema_example)]
pub(crate) struct Source {
    pub(crate) ordinal: usize,
    pub(crate) index: i64,
    pub(crate) citation: String,
    pub(crate) url: String,
    pub(crate) origin_text: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = assistant_message_schema_example)]
pub(crate) enum Message {
    User(String),
    Assistant(String, Vec<Source>),
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[schema(example = conversation_schema_example)]
pub(crate) struct Conversation(pub(crate) Vec<Message>);

#[utoipa::path(
    request_body(content = Conversation, content_type = "application/json"),
    responses(
        (status = 200, description = "AI Response", body = PartialMessage, content_type = "application/json"),
        (status = 204, description = "No user input"),
        (status = 400, description = "Empty Request")
    )
)]
#[post("/streaming_conversation")]
async fn streaming_conversation(
    Json(conversation): Json<Conversation>,
    query_engine: Data<Arc<Engine>>,
) -> impl Responder {
    log::info!("Received \n{:#?}", conversation);

    let (client, sender) = Client::new();

    match query_engine.streaming_conversation_validator(&conversation) {
        Err(e) => HttpResponse::BadRequest().body(format!("{}", e)).into(),
        Ok(()) => {
            actix_web::rt::spawn(async move {
                let _ = query_engine
                    .streaming_conversation(conversation, sender)
                    .await
                    .map_err(|e| log::error!("{e}"));
            });

            HttpResponse::Ok()
                .append_header(("content-type", "text/event-stream"))
                .append_header(("connection", "keep-alive"))
                .append_header(("cache-control", "no-cache"))
                .streaming(client)
        }
    }
}

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://0.0.0.0:5001");

    let openapi = ApiDoc::openapi();
    let engine = Arc::new(Engine);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(Cors::permissive())
            .app_data(Data::new(engine.clone()))
            .service(streaming_conversation)
            .service(Redoc::with_url("/api-doc", openapi.clone()))
    })
    .bind(("0.0.0.0", 5001))?
    .run()
    .await
}

fn conversation_schema_example() -> Conversation {
    Conversation(vec![
        user_message_schema_example(),
        assistant_message_schema_example(),
    ])
}

fn assistant_message_schema_example() -> Message {
    Message::Assistant(
        String::from("String"),
        vec![
            source_schema_example(),
            source_schema_example(),
            source_schema_example(),
            source_schema_example(),
        ],
    )
}
fn user_message_schema_example() -> Message {
    Message::User(String::from("String"))
}
fn source_schema_example() -> Source {
    Source { ordinal: 0, index: 987087, citation: "Bogonam-Foulbé. 2023, December 1. In Wikipedia. Retrieved December 1, 2023, from https://en.wikipedia.org/wiki/Bogonam-Foulbé".to_string(), url: "https://en.wikipedia.org/wiki/Bogonam-Foulbé".to_string(), origin_text: "Bogonam-Foulbé is a village in the Kongoussi Department of Bam Province in northern Burkina Faso. It has a population of 205.".to_string() }
}

fn assistant_partial_message_schema_example() -> PartialMessage {
    PartialMessage {
        content: Some(String::from(" fragment")),
        source: Some(source_schema_example()),
        finished: Some(String::new()),
    }
}

pub struct Client(UnboundedReceiver<Bytes>);

impl Client {
    pub(crate) fn new() -> (Self, UnboundedSender<Bytes>) {
        let (tx, rx) = unbounded_channel();
        (Self(rx), tx)
    }
}

impl Stream for Client {
    type Item = Result<Bytes, actix_web::http::Error>;
    /// This does NOT work without self.0 being a tokio receiver of some kind
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match Pin::new(&mut self.0).poll_recv(cx) {
            Poll::Ready(Some(v)) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Pending => Poll::Pending,
        }
    }
}

#[derive(Debug)]
pub(crate) enum QueryEngineError {
    MockError,
    NoMessage,
    AgentMessage,
}

impl std::error::Error for QueryEngineError {}

impl Display for QueryEngineError {
    fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
        match self {
            QueryEngineError::MockError => {
                write!(f, "Error: Something catastrophic happened. You did this.")
            }
            QueryEngineError::NoMessage => {
                write!(f, "Error: There is no Message to respond to.")
            }
            QueryEngineError::AgentMessage => {
                write!(f, "Error: The last message must be from the user.")
            }
        }
    }
}

pub struct Engine;

impl Engine {
    pub(crate) async fn streaming_conversation(
        &self,
        Conversation(message_history): Conversation,
        tx: UnboundedSender<Bytes>,
    ) -> Result<(), QueryEngineError> {
        let words = MESSAGE_CONTENT.split(' ').collect::<Vec<_>>();
        let sources = get_sources_list();

        actix_web::rt::spawn(async move {
            for source in sources {
                let _ = tx.send(PartialMessage::source(source).message());
                tokio::time::sleep(Duration::from_millis(50u64)).await;
            }
            for word in words {
                let _ = tx.send(PartialMessage::content(word.to_string()).message());
                tokio::time::sleep(Duration::from_millis(50u64)).await;
            }
            tokio::time::sleep(Duration::from_millis(50u64)).await;
            let _ = tx.send(PartialMessage::done().message());
        });
        Ok(())
    }
    pub(crate) fn streaming_conversation_validator(
        &self,
        Conversation(message_history): &Conversation,
    ) -> Result<(), QueryEngineError> {
        match message_history.last() {
            None => Err(QueryEngineError::MockError),
            Some(Message::Assistant(_, _)) => Err(QueryEngineError::MockError),
            Some(Message::User(string)) => {
                if &string.to_lowercase() == &String::from("error") {
                    return Err(QueryEngineError::MockError);
                } else {
                    Ok(())
                }
            }
        }
    }
}
