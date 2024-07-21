//! Module for holding the business logic of our application. Here, all the integrity and
//! validation are performed. Furthermore, we alter certain requests or payloads in a way that fits
//! our database scheme.
mod auth;
mod user;

use rocket::{Build, Rocket};

pub use self::auth::*;
pub use self::user::*;

pub async fn mount(instance: Rocket<Build>) -> Rocket<Build> {
    let user_service = UserService::new().await;
    let auth_service = AuthService::new(user_service.clone()).await;
    instance.manage(user_service).manage(auth_service)
}
