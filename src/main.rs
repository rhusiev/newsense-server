use std::fs;

use feed_rs::parser;

#[tokio::main]
async fn main() {
    // let example_rss = reqwest::get("https://www.liga.net/newsua/all/rss.xml")
    //     .await
    //     .unwrap()
    //     .text()
    //     .await
    //     .unwrap();
    // fs::write("example.rss", example_rss.clone()).unwrap();

    let example_rss = fs::read_to_string("example/example.rss").unwrap();

    let parser = parser::Builder::new().sanitize_content(true).build();
    let feed = match parser.parse(example_rss.as_bytes()) {
        Ok(feed) => feed,
        Err(err) => panic!("{}", err),
    };

    println!("{:#?}", feed);
}
