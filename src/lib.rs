pub mod api;
pub mod jwt;
pub mod repositories;
pub mod services;
pub mod utils;

use surrealdb::{
    engine::remote::ws::{Client, Ws},
    opt::auth::Root,
    Surreal,
};

pub async fn use_db() -> Surreal<Client> {
    let db = Surreal::new::<Ws>("127.0.0.1:8080")
        .await
        .expect("Could not establish connection to db");

    // TODO: replace with some reasonable values (e.g., from ENV)
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await
    .expect("Could not login to database");

    db.use_ns("syn_ack")
        .use_db("syn_ack")
        .await
        .expect("could not select namespace");
    db
}

#[macro_export]
macro_rules! define_repository {
    ($s:tt, $($t:tt)*) => {
        #[derive(Default, Clone, Debug, Hash, Serialize, Deserialize, Repository)]
        pub struct $s {
            id: Option<surrealdb::sql::Thing>,
            $($t)*
        }
    };
}
