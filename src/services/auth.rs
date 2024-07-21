use anyhow::{Error, Result};
use serde::Serialize;

use crate::{jwt, repositories::User, utils::verify_password};

use super::UserService;

pub struct AuthService {
    user_service: UserService,
}

impl AuthService {
    pub async fn new(user_service: UserService) -> AuthService {
        AuthService { user_service }
    }

    pub async fn login(
        &self,
        username: impl ToString,
        password: impl ToString,
    ) -> Result<TokenPair> {
        println!("Uuuuuhm...");
        let Some(User {
            id: Some(id),
            password: user_password,
            ..
        }) = self.user_service.find_user_by_username(username).await
        else {
            return Err(Error::msg("User not found!"));
        };

        if let Ok(true) = verify_password(&password.to_string(), &user_password) {
            return TokenPair::new(id);
        };

        Err(Error::msg("credentials do not match"))
    }
}

#[derive(Serialize)]
pub struct TokenPair {
    access_token: String,
    refresh_token: String,
}

impl TokenPair {
    pub fn new(username: impl ToString) -> Result<Self> {
        let username = username.to_string();
        let access_token = jwt::Accesstoken::new(&username).sign()?;
        let refresh_token = jwt::Refreshtoken::new(&username).sign()?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}
