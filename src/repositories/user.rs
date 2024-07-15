use anyhow::Result;
use serde::{Deserialize, Serialize};
use surreal_derive::Repository;
use surrealdb::{engine::remote::ws::Client, Surreal};

use crate::{define_repository, use_db};

define_repository!(User,
    pub username: String,
    pub email: String,
    pub password: String,
);

pub struct UserRepository {
    db: Surreal<Client>,
}

impl UserRepository {
    pub async fn new() -> UserRepository {
        UserRepository { db: use_db().await }
    }

    pub async fn create(
        &self,
        CreateUserPayload {
            username,
            email,
            password,
        }: CreateUserPayload,
    ) -> Result<Option<User>> {
        let result: Option<User> = self
            .db
            .create("user")
            .content(User {
                username,
                password,
                email,
                ..Default::default()
            })
            .await?
            .first()
            .cloned();

        Ok(result)
    }
}
