use ics::{Calendar, Components, Eventc};
use url::Url;

fn main() {
    let evt = Eventc::default()
        .with_description("Some Desc")
        .with_summary("Some Summary")
        .with_dtstart(chrono::Utc::now())
        .with_location("OHP 17\\nRaum 16")
        .with_url(Url::parse("https://example.com").unwrap());

    let cal = Calendar::default().with_components(vec![Components::Eventc(evt)]);

    println!("{cal}")
}
