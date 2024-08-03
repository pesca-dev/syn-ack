use anyhow::{Error, Result};
use serde::Serialize;
use tracing::debug;

use crate::jwt::Refreshtoken;
use crate::{
    jwt,
    repositories::{AuthRepository, Session, User},
    utils::verify_password,
};

use super::UserService;

pub struct AuthService {
    user_service: UserService,
    session_repository: AuthRepository,
}

impl AuthService {
    pub async fn new(user_service: UserService) -> AuthService {
        AuthService {
            user_service,
            session_repository: AuthRepository::new().await,
        }
    }

    pub async fn login(
        &self,
        username: impl ToString,
        password: impl ToString,
    ) -> Result<TokenPair> {
        let Some(User {
            id: Some(id),
            password: user_password,
            ..
        }) = self.user_service.find_user_by_username(username).await
        else {
            return Err(Error::msg("User not found!"));
        };

        let Ok(true) = verify_password(&password.to_string(), &user_password) else {
            return Err(Error::msg("credentials do not match"));
        };

        let Ok(Some(Session {
            id: Some(id),
            last_refresh,
            ..
        })) = self.session_repository.create_session(id).await
        else {
            return Err(Error::msg("failed to create session"));
        };

        TokenPair::new(id, last_refresh)
    }

    pub async fn fetch_session_by_id(&self, session_id: impl ToString) -> Option<Session> {
        self.session_repository
            .find_session_by_id(session_id.to_string())
            .await
            .ok()
            .flatten()
    }

    pub async fn fetch_user_for_session(&self, session: &Session) -> Option<User> {
        let Some(session_id) = &session.id else {
            return None;
        };

        let Ok(Some(Session { user_id, .. })) = self
            .session_repository
            .find_session_by_id(session_id.to_string())
            .await
        else {
            return None;
        };

        self.user_service.find_user_by_id(user_id).await
    }

    pub async fn find_session_by_last_refresh(
        &self,
        last_refresh: impl ToString,
    ) -> Option<Session> {
        self.session_repository
            .find_session_by_last_refresh(last_refresh)
            .await
            .unwrap_or_else(|e| {
                debug!("Error while finding session: {e}");
                None
            })
    }

    pub async fn refresh(&self, token: Refreshtoken) -> Result<TokenPair> {
        let Some(mut session) = self.find_session_by_last_refresh(token.sub).await else {
            return Err(anyhow::Error::msg("Invalid last refresh"));
        };

        let session_id = session.id.clone();
        let last_refresh = uuid::Uuid::new_v4().to_string();
        session.last_refresh.clone_from(&last_refresh);

        if let Err(e) = self.session_repository.update(session).await {
            return Err(anyhow::Error::msg(format!("Error refreshing session: {e}")));
        }

        TokenPair::new(
            session_id.expect("tried to refresh invalid session"),
            last_refresh,
        )
    }

    pub async fn logout(&self, session: Session) -> Result<()> {
        self.session_repository.delete(session).await
    }
}

#[derive(Serialize)]
pub struct TokenPair {
    access_token: String,
    refresh_token: String,
}

impl TokenPair {
    pub fn new(sub: impl ToString, refresh: impl ToString) -> Result<Self> {
        let access_token = jwt::Accesstoken::new(sub).sign()?;
        let refresh_token = jwt::Refreshtoken::new(refresh).sign()?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
