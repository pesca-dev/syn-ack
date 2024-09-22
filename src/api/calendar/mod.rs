use ics::{url::Url, Calendar, Components, Eventc};
use rocket::{get, serde::json::Json};

use crate::repositories::User;

#[get("/")]
pub fn get_calendars(_user: User) -> Json<Vec<()>> {
    Json(vec![])
}

#[get("/<_id>")]
pub fn get_calendar(_user: Option<User>, _id: &str) -> String {
    println!("User: {_user:#?}");
    let evt = Eventc::new()
        .with_description("Some Desc")
        .with_summary("Some Summary")
        .with_dtstart(chrono::Utc::now())
        .with_location("OHP 17\\nRaum 16")
        .with_url(Url::parse("https://example.com").unwrap());

    Calendar::default()
        .with_components(vec![Components::Eventc(evt)])
        .to_string()
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![get_calendars, get_calendar]
}
