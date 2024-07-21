use std::sync::Arc;

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

use crate::{
    jwt,
    repositories::{CreateUserPayload, User},
    services::{AuthService, TokenPair, UserService},
};

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
        .map_err(|e| {
            eprintln!("{e}");
        })?;

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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(auth_service) = request.rocket().state::<AuthService>() else {
            println!("Could not find UserService");
            return Outcome::Error((Status::InternalServerError, ()));
        };

        // first check, if we have a valid JWT attached
        let Authorization(token) = match Authorization::from_request(request).await {
            rocket::outcome::Outcome::Success(auth) => auth,
            rocket::outcome::Outcome::Error(e) => return Outcome::Error(e),
            rocket::outcome::Outcome::Forward(e) => return Outcome::Forward(e),
        };

        // then try to find the user
        let Some(user) = auth_service.fetch_user_for_session(&token.sub).await else {
            // TODO: log error to console
            return Outcome::Error((Status::InternalServerError, ()));
        };

        Outcome::Success(user)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Responder)]
enum LoginResponse {
    #[response(status = 202)]
    Success(Json<TokenPair>),

    #[response(content_type = "json")]
    Error(Status),
}

#[post("/login", data = "<auth>")]
async fn login(
    auth: Json<LoginRequest>,
    auth_service: &State<AuthService>,
) -> Result<LoginResponse, Status> {
    let LoginRequest { username, password } = auth.into_inner();

    let token = match auth_service.login(username, password).await {
        Ok(token) => token,
        Err(_) => {
            return Ok(LoginResponse::Error(Status::Unauthorized));
        }
    };

    Ok(LoginResponse::Success(Json(token)))
}

#[post("/register", data = "<payload>")]
async fn register(payload: Json<CreateUserPayload>, service: &State<UserService>) {
    let result = service.create_user(payload.into_inner()).await;

    println!("{result:?}");
}

#[get("/authorized")]
async fn authorized(user: User, g: &State<MyGuard>) -> String {
    let mut counter = g.counter.lock().await;
    *counter += 1;
    format!("Hey {}, called {counter}", user.username)
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login, register, authorized]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_from_bearer_success() {
        std::env::set_var("ACCESS_JWT_KEY", "some-key");

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
