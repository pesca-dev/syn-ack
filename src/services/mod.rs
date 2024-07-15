//! Module for holding the business logic of our application. Here, all the integrity and
//! validation are performed. Furthermore, we alter certain requests or payloads in a way that fits
//! our database scheme.
mod user;

use rocket::{Build, Rocket};

pub use self::user::*;

pub async fn mount(instance: Rocket<Build>) -> Rocket<Build> {
    instance.manage(UserService::new().await)
}
