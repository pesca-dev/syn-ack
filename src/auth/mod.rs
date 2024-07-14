use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Hash, Serialize, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn find_username(username: impl ToString) -> Option<User> {
        // TODO: use some kind of database for this
        Some(User {
            id: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            email: "mail@example.com".into(),
            password: "SECRET".into(),
        })
    }
}
