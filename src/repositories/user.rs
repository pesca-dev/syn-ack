use anyhow::Result;
use serde::{Deserialize, Serialize};
use surreal_derive::repository;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::use_db;

#[repository]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Clone)]
pub struct UserRepository {
    db: Surreal<Client>,
}

impl UserRepository {
    const TABLE: &'static str = "user";

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
            .create((Self::TABLE, uuid::Uuid::new_v4().to_string()))
            .content(User {
                username,
                password,
                email,
                ..Default::default()
            })
            .await?;

        Ok(result)
    }

    pub async fn find_by_id(&self, id: impl TryInto<Thing>) -> Result<Option<User>> {
        let Ok(thing) = TryInto::<Thing>::try_into(id) else {
            return Err(anyhow::Error::msg("invalid id given"));
        };

        let result: Option<User> = self.db.select(thing).await?;
        Ok(result)
    }

    pub async fn find_by_username(&self, username: impl ToString) -> Result<Option<User>> {
        let mut result = self
            .db
            .query("SELECT * FROM type::table($table) where username = $username;")
            .bind(("table", Self::TABLE))
            .bind(("username", username.to_string()))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn find_by_email(&self, email: impl ToString) -> Result<Option<User>> {
        let mut result = self
            .db
            .query("SELECT * FROM type::table($table) where email = $email;")
            .bind(("table", Self::TABLE))
            .bind(("email", email.to_string()))
            .await?;

        Ok(result.take(0)?)
    }
}
