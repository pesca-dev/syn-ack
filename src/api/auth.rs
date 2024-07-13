use std::{error::Error, sync::Arc};

use rocket::{
    get,
    http::Status,
    post,
    request::{FromRequest, Outcome},
    serde::json::Json,
    tokio::sync::Mutex,
    Responder, State,
};
use serde::{Deserialize, Serialize};

use crate::jwt;

#[derive(Clone, Debug, Default)]
pub struct MyGuard {
    pub counter: Arc<Mutex<i64>>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Authorization(jwt::Accesstoken);

impl Authorization {
    const BEARER: &'static str = "Bearer ";

    pub fn from_bearer(token: impl ToString) -> Result<Authorization, ()> {
        let token = token.to_string();

        if !token.starts_with(Self::BEARER) {
            return Err(());
        }

        let token = jwt::Accesstoken::extract(
            token
                .strip_prefix(Self::BEARER)
                .expect("Unreachable")
                .into(),
        )
        .map_err(|_| ())?;

        Ok(Authorization(token))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(
        request: &'r rocket::Request<'_>,
    ) -> rocket::request::Outcome<Self, Self::Error> {
        // save current timestamp to check expiration of JWT
        let now = chrono::Utc::now().timestamp();

        match request.headers().get_one("authorization") {
            Some(token) => match Self::from_bearer(token) {
                // check, if token is not expired
                Ok(token) if token.0.exp - now > 0 => Outcome::Success(token),
                // token is expired
                Ok(_) => Outcome::Error((Status::Unauthorized, ())),
                // upsi, not a valid token in general
                Err(_) => Outcome::Error((Status::BadRequest, ())),
            },
            // we do not have the header
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
struct TokenPair {
    access_token: String,
    refresh_token: String,
}

impl TokenPair {
    pub fn new(username: impl ToString) -> Result<Self, Box<dyn Error>> {
        let username = username.to_string();
        let access_token = jwt::Accesstoken::new(&username).sign()?;
        let refresh_token = jwt::Refreshtoken::new(&username).sign()?;

        Ok(TokenPair {
            access_token,
            refresh_token,
        })
    }
}

#[derive(Responder)]
enum LoginResponse {
    #[response(status = 202)]
    Success(Json<TokenPair>),

    #[response(content_type = "json")]
    Error(Status),
}

#[post("/login", data = "<auth>")]
fn login(auth: Json<LoginRequest>) -> Result<LoginResponse, Status> {
    let LoginRequest {
        username,
        password: _password,
    } = auth.into_inner();

    Ok(LoginResponse::Success(Json(
        TokenPair::new(username).map_err(|_| Status::InternalServerError)?,
    )))
}

#[post("/register")]
fn register() {}

#[get("/authorized")]
async fn authorized(auth: Authorization, g: &State<MyGuard>) -> String {
    let Authorization(token) = auth;

    let mut counter = g.counter.lock().await;
    *counter += 1;
    format!("Hey {}, called {counter}", token.sub)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login, register, authorized]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_from_bearer_success() {
        std::env::set_var("ACCESS_JWT_KEY", "ACCESS_JWT_KEY");

        let jwt_token = jwt::Accesstoken::new("foo");

        let access_token =
            Authorization::from_bearer(format!("Bearer {}", jwt_token.clone().sign().unwrap()));

        assert_eq!(access_token, Ok(Authorization(jwt_token)))
    }

    #[test]
    fn test_authorization_from_bearer_error() {
        assert_eq!(Authorization::from_bearer("Bear foo"), Err(()))
    }
}
