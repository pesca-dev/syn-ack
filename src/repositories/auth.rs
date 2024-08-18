use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use surreal_derive::repository;
use surrealdb::{engine::remote::ws::Client, sql::Thing, Surreal};

use crate::use_db;

#[repository]
pub struct Session {
    pub user_id: String,
    pub last_refresh: String,
    pub created_at: chrono::DateTime<Utc>,
}

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
            .create((Self::TABLE, uuid::Uuid::new_v4().to_string()))
            .content(Session {
                user_id: user_id.to_string(),
                created_at: chrono::Utc::now(),
                last_refresh: uuid::Uuid::new_v4().to_string(),
                ..Default::default()
            })
            .await?;

        Ok(result)
    }

    pub async fn update(&self, session: Session) -> Result<()> {
        assert!(session.id.is_some());

        let _: Option<Session> = self
            .db
            .update(session.id.as_ref().expect("unreachable"))
            .content(session)
            .await?;

        Ok(())
    }

    pub async fn find_session_by_id(&self, id: impl TryInto<Thing>) -> Result<Option<Session>> {
        let Ok(thing) = TryInto::<Thing>::try_into(id) else {
            return Err(anyhow::Error::msg("invalid id given"));
        };

        let result: Option<Session> = self.db.select(thing).await?;
        Ok(result)
    }

    pub async fn find_session_by_last_refresh(
        &self,
        last_refresh: impl ToString,
    ) -> Result<Option<Session>> {
        let mut result = self
            .db
            .query("SELECT * FROM type::table($table) where last_refresh = $last_refresh;")
            .bind(("table", Self::TABLE))
            .bind(("last_refresh", last_refresh.to_string()))
            .await?;

        Ok(result.take(0)?)
    }

    pub async fn delete(&self, session: Session) -> Result<()> {
        assert!(session.id.is_some());
        let _: Option<Session> = self
            .db
            .delete(session.id.expect("Something went wrong"))
            .await?;

        Ok(())
    }
}
