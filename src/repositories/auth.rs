use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_derive::Repository;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::{define_repository, use_db};

define_repository!(Session,
    pub user_id: String,
    pub created_at: chrono::DateTime<Utc>
);

#[derive(Debug, Clone)]
pub struct AuthRepository {
    db: Surreal<Client>,
}

impl AuthRepository {
    const TABLE: &'static str = "session";

    pub async fn new() -> AuthRepository {
        AuthRepository { db: use_db().await }
    }

    pub async fn create_session(&self, user_id: impl ToString) -> Result<Option<Session>> {
        let result: Option<Session> = self
            .db
            .create(Self::TABLE)
            .content(Session {
                user_id: user_id.to_string(),
                created_at: chrono::Utc::now(),
                ..Default::default()
            })
            .await?
            .first()
            .cloned();

        Ok(result)
    }

    pub async fn find_session_by_id(&self, id: impl TryInto<Thing>) -> Result<Option<Session>> {
        let Ok(thing) = TryInto::<Thing>::try_into(id) else {
            return Err(anyhow::Error::msg("invalid id given"));
        };

        let result: Option<Session> = self.db.select(thing).await?;
        Ok(result)
    }
}
