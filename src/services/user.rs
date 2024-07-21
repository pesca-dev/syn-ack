use anyhow::Result;

use crate::{
    repositories::{CreateUserPayload, User, UserRepository},
    utils::hash_password,
};

#[derive(Debug, Clone)]
pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub async fn new() -> UserService {
        UserService {
            repo: UserRepository::new().await,
        }
    }

    pub async fn create_user(&self, mut payload: CreateUserPayload) -> Result<Option<User>> {
        // TODO: check if credentials are unique
        payload.password = hash_password(payload.password)?;
        self.repo.create(payload).await
    }

    pub async fn find_user_by_username(&self, username: impl ToString) -> Option<User> {
        match self.repo.find_by_username(username).await {
            Ok(maybe_user) => maybe_user,
            Err(e) => {
                eprintln!("Error while finding user: {e}");
                None
            }
        }
    }

    pub async fn find_user_by_id(&self, id: impl ToString) -> Option<User> {
        match self.repo.find_by_id(id.to_string()).await {
            Ok(maybe_user) => maybe_user,
            Err(e) => {
                eprintln!("Error while finding user: {e}");
                None
            }
        }
    }
}