mod api;
mod jwt;

use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();
    let mut instance = rocket::build().mount("/", routes![index]);

    instance = api::mount(instance);

    instance
}
