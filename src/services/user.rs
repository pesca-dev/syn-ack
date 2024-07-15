use anyhow::Result;

use crate::{
    repositories::{CreateUserPayload, User, UserRepository},
    utils::hash_password,
};

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
}
