mod auth;
mod calendar;

use rocket::{get, routes, Build, Rocket};

const BASE: &str = "/api/v1/";

#[get("/version")]
fn get_version() -> &'static str {
    "v1"
}

pub async fn mount(instance: Rocket<Build>) -> Rocket<Build> {
    instance
        .mount(BASE, routes![get_version])
        .mount([BASE, "auth"].concat(), auth::routes())
        .mount([BASE, "cal"].concat(), calendar::routes())
}
