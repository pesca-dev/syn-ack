use anyhow::{Error, Result};
use serde::Serialize;

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

    pub async fn fetch_user_for_session(&self, session_id: impl ToString) -> Option<User> {
        let Ok(Some(Session { user_id, .. })) = self
            .session_repository
            .find_session_by_id(session_id.to_string())
            .await
        else {
            return None;
        };

        self.user_service.find_user_by_id(user_id).await
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
