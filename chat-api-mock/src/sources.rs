use crate::Source;

pub(crate) fn get_sources_list() -> [Source; 4] {
    [
        Source {
            ordinal: 0usize,
            index: 98346i64,
            citation: String::from("Jackson, Michael. “Beat It.” Thriller, produced by Quincy Jones, Google Play Exclusive Edition, Epic, 1982, play.google.com/store/music/album/Thriller?id=Bzs3hkvcyvinz5tkilucmmoqjhi&hl=en_US."),
            url: String::from("https://answers.quantarchive.com/"),
            origin_text: String::from("consectetur adipiscing elit"),
        },
        Source {
            ordinal: 1usize,
            index: i64::MAX,
            citation: String::from("Jackson, Michael. “Beat It.” Thriller, produced by Quincy Jones, Google Play Exclusive Edition, Epic, 1982, play.google.com/store/music/album/Thriller?id=Bzs3hkvcyvinz5tkilucmmoqjhi&hl=en_US."),
            url: String::from("https://answers.quantarchive.com/"),
            origin_text: String::from("Commodo elit at imperdiet dui accumsan"),
        },
        Source {
            ordinal: 2usize,
            index: i64::MIN,
            citation: String::from("Jackson, Michael. “Beat It.” Thriller, produced by Quincy Jones, Google Play Exclusive Edition, Epic, 1982, play.google.com/store/music/album/Thriller?id=Bzs3hkvcyvinz5tkilucmmoqjhi&hl=en_US."),
            url: String::from("https://answers.quantarchive.com/"),
            origin_text: String::from("Sem fringilla ut morbi tincidunt augue"),
        },
        Source {
            ordinal: 3usize,
            index: 0i64,
            citation: String::from("Jackson, Michael. “Beat It.” Thriller, produced by Quincy Jones, Google Play Exclusive Edition, Epic, 1982, play.google.com/store/music/album/Thriller?id=Bzs3hkvcyvinz5tkilucmmoqjhi&hl=en_US."),
            url: String::from("https://answers.quantarchive.com/"),
            origin_text: String::from("Egestas tellus rutrum tellus"),
        },
    ]
}
