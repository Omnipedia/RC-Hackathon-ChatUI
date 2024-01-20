# Chat UI for Omnipedia

## Mock API Service
This repo contains a Mock API Service under `chat-api-mock`. You will need rust 1.75. While it is running at [http://0.0.0.0:5001](http://0.0.0.0:5001), you will find the api ReDoc documentation and example api calls at [http://0.0.0.0:5001/api-doc](http://0.0.0.0:5001/api-doc). An error will trigger under the following conditions:
1. The last user message is simply 'error'
2. There are no messages in the conversation.
3. The last message belongs to the Assistant

The API Accepts a POST request containing the conversation history, and return a SSE message stream of partial messages. The partial message can be:
1. a `content` fragment, which should be appended to the message on screen and rendered. This is a possibly null `string`.
2. A `source`, which should be appended to the list of sources. This is a possibly null `string`.
3. the DONE signal, at which point the client must close the connection. This will be `null` or `"DONE"`.
4. 1 _and_ 2.

The message has the following format:
```js
event: message
data: {"content":"Lorem","source":null,"finished":null}
```

### Valid API Call
```bash
curl --location 'http://0.0.0.0:5001/streaming_conversation' \
--header 'Content-Type: application/json' \
--header 'Connection: keep-alive' \
--header 'Cache-Control: no-cache' \
--data '[
  {
    "User": "String"
  }
]'
```
### Trigger Mock API Error
```bash
curl --location 'http://0.0.0.0:5001/streaming_conversation' \
--header 'Content-Type: application/json' \
--header 'Connection: keep-alive' \
--header 'Cache-Control: no-cache' \
--data '[
  {
    "User": "error"
  }
]'
```
### Trigger Real API Error With Empty Input
```bash
curl --location 'http://0.0.0.0:5001/streaming_conversation' \
--header 'Content-Type: application/json' \
--header 'Connection: keep-alive' \
--header 'Cache-Control: no-cache' \
--data '[]'
```
### Trigger Real API Error With Trailing Agent Message
```bash
curl --location 'http://0.0.0.0:5001/streaming_conversation' \
--header 'Content-Type: application/json' \
--header 'Connection: keep-alive' \
--header 'Cache-Control: no-cache' \
--data '[
  {
    "User": "String"
  },
  {
    "Assistant": [
      "String",
      [
        {
          "citation": "Bogonam-Foulbé. 2023, December 1. In Wikipedia. Retrieved December 1, 2023, from https://en.wikipedia.org/wiki/Bogonam-Foulbé",
          "index": 987087,
          "ordinal": 0,
          "origin_text": "Bogonam-Foulbé is a village in the Kongoussi Department of Bam Province in northern Burkina Faso. It has a population of 205.",
          "url": "https://en.wikipedia.org/wiki/Bogonam-Foulbé"
        },
        {
          "citation": "Bogonam-Foulbé. 2023, December 1. In Wikipedia. Retrieved December 1, 2023, from https://en.wikipedia.org/wiki/Bogonam-Foulbé",
          "index": 987087,
          "ordinal": 0,
          "origin_text": "Bogonam-Foulbé is a village in the Kongoussi Department of Bam Province in northern Burkina Faso. It has a population of 205.",
          "url": "https://en.wikipedia.org/wiki/Bogonam-Foulbé"
        },
        {
          "citation": "Bogonam-Foulbé. 2023, December 1. In Wikipedia. Retrieved December 1, 2023, from https://en.wikipedia.org/wiki/Bogonam-Foulbé",
          "index": 987087,
          "ordinal": 0,
          "origin_text": "Bogonam-Foulbé is a village in the Kongoussi Department of Bam Province in northern Burkina Faso. It has a population of 205.",
          "url": "https://en.wikipedia.org/wiki/Bogonam-Foulbé"
        },
        {
          "citation": "Bogonam-Foulbé. 2023, December 1. In Wikipedia. Retrieved December 1, 2023, from https://en.wikipedia.org/wiki/Bogonam-Foulbé",
          "index": 987087,
          "ordinal": 0,
          "origin_text": "Bogonam-Foulbé is a village in the Kongoussi Department of Bam Province in northern Burkina Faso. It has a population of 205.",
          "url": "https://en.wikipedia.org/wiki/Bogonam-Foulbé"
        }
      ]
    ]
  }
]'
```


## Acceptance Criteria
- Given I enter a query, when the response is returned, then it contains the citation, and a reference to the source text appears on the right.
- Given the reference is printed, when I click the expand button next to it, then I see the original source passage.
- Given the reference is printed, when I click the reference, then I am taken to the original source.
- Given the response is returned, when I look at the source text, it is a passage or table that stands on it's own.
- Given the response is returned, when the response prints a cited piece of text, the reference and source appear in a column on the right.
- Given the response is returned, when the citations are printed, the references follow the preferred citation format.
- Given the response is returned, when the citations are printed, the citations are are numbers in square brackets, and the matching reference has the same number.
- Given the response is returned, when the citations and references are printed, the citations and references are numbered contiguously starting from 1.
- Given the product demo returns a response with sources, when the user clicks on the citation, then they are navigated to the source text.
- Given the machine returns a response, when it produces the citation in the text, only reference associate with the citations are shown in the sidebar.
