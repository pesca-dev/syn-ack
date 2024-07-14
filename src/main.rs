mod api;
mod auth;
mod jwt;

use rocket::{get, launch, routes};

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

trait Registerable {
    fn mount_fn(self, func: &dyn Fn(Self) -> Self) -> Self;
}

impl Registerable for rocket::Rocket<rocket::Build> {
    fn mount_fn(self, func: &dyn Fn(Self) -> Self) -> Self {
        func(self)
    }
}

#[launch]
fn rocket() -> _ {
    dotenv::dotenv().ok();

    rocket::build()
        .mount("/", routes![index])
        .mount_fn(&api::mount)
}
