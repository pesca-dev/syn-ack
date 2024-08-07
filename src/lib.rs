pub mod api;
pub mod jwt;
pub mod repositories;
pub mod services;
pub mod utils;

pub use rocket;

use rocket::{Build, Rocket};
use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

macro_rules! get_env {
    ($name:expr) => {
        std::env::var($name).expect(&format!("{} should be given", $name))
    };
    ($name:expr, $default:expr) => {
        std::env::var($name).unwrap_or($default.into())
    };
}

pub async fn use_db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>(get_env!("DB_HOST"))
        .await
        .expect("Could not establish connection to db");

    db.signin(Root {
        username: &get_env!("DB_USER"),
        password: &get_env!("DB_PASSWORD"),
    })
    .await
    .expect("Could not login to database");

    db.use_ns(get_env!("DB_NS", "syn_ack"))
        .use_db(get_env!("DB_DB", "syn_ack"))
        .await
        .expect("could not select namespace");
    db
}

#[macro_export]
macro_rules! define_repository {
    ($s:tt, $($t:tt)*) => {
        #[derive(Default, Clone, Debug, Hash, Serialize, Deserialize, Repository)]
        pub struct $s {
            pub id: Option<surrealdb::sql::Thing>,
            $($t)*
        }
    };
}

pub async fn start() -> Rocket<Build> {
    let r = rocket::build();

    let r = api::mount(r).await;

    services::mount(r).await
}
