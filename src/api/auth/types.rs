use std::str::FromStr;

use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    serde::json::Json,
    Request, Responder,
};
use serde::{Deserialize, Serialize};

use crate::{
    jwt,
    repositories::{Session, User},
    services::{AuthService, TokenPair},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bearer(pub String);

const BEARER: &str = "Bearer ";

impl FromStr for Bearer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with(BEARER) {
            return Err(anyhow::Error::msg("Token does not start with 'Bearer '"));
        }

        return Ok(Bearer(s.strip_prefix(BEARER).expect("unreachable").into()));
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Authorization(pub jwt::Accesstoken);

impl Authorization {
    pub fn from_bearer(token: impl ToString) -> anyhow::Result<Authorization> {
        let Bearer(token) = Bearer::from_str(&token.to_string())?;

        let token = jwt::Accesstoken::extract(token).map_err(|e| {
            tracing::debug!("{e}");
            anyhow::Error::msg("Could not create AccessToken")
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
impl<'r> FromRequest<'r> for Session {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(auth_service) = request.rocket().state::<AuthService>() else {
            tracing::error!("Could not find AuthService");
            return Outcome::Error((Status::InternalServerError, ()));
        };

        // first check, if we have a valid JWT attached
        let Authorization(token) = match Authorization::from_request(request).await {
            rocket::outcome::Outcome::Success(auth) => auth,
            rocket::outcome::Outcome::Error(e) => return Outcome::Error(e),
            rocket::outcome::Outcome::Forward(e) => return Outcome::Forward(e),
        };

        let Some(session) = auth_service.fetch_session_by_id(token.sub).await else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        Outcome::Success(session)
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for User {
    type Error = ();

    async fn from_request(request: &'r rocket::Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(auth_service) = request.rocket().state::<AuthService>() else {
            tracing::error!("Could not find AuthService");
            return Outcome::Error((Status::InternalServerError, ()));
        };

        // first try to fetch session
        let Outcome::Success(session) = Session::from_request(request).await else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        // then try to find the user
        let Some(user) = auth_service.fetch_user_for_session(&session).await else {
            return Outcome::Error((Status::Unauthorized, ()));
        };

        Outcome::Success(user)
    }
}

#[derive(Clone, Debug)]
pub struct RefreshRequest(pub jwt::Refreshtoken);

impl RefreshRequest {
    pub fn from_bearer(token: impl ToString) -> anyhow::Result<RefreshRequest> {
        let Bearer(token) = Bearer::from_str(&token.to_string())?;

        let token = jwt::Refreshtoken::extract(token).map_err(|e| {
            tracing::trace!("RefreshToken error: {e}");
            anyhow::Error::msg("Could not create RefreshToken")
        })?;

        Ok(RefreshRequest(token))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RefreshRequest {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("authorization") {
            Some(token) => match Self::from_bearer(token) {
                Ok(token) => Outcome::Success(token),
                // upsi, not a valid token in general
                Err(_) => Outcome::Error((Status::BadRequest, ())),
            },
            // we do not have the header
            None => Outcome::Error((Status::Unauthorized, ())),
        }
    }
}

#[derive(Responder)]
pub enum RefreshResponse {
    #[response(status = 202)]
    Success(Json<TokenPair>),

    #[response(content_type = "json")]
    Error(Status),
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Responder)]
pub enum LoginResponse {
    #[response(status = 202)]
    Success(Json<TokenPair>),

    #[response(content_type = "json")]
    Error(Status),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_authorization_from_bearer_success() {
        unsafe {
            std::env::set_var("ACCESS_JWT_KEY", "some-key");
        }

        let jwt_token = jwt::Accesstoken::new("foo");

        let access_token =
            Authorization::from_bearer(format!("Bearer {}", jwt_token.clone().sign().unwrap()));

        assert_eq!(access_token.unwrap(), Authorization(jwt_token))
    }

    #[test]
    fn test_authorization_from_bearer_error() {
        assert!(Authorization::from_bearer("Bear foo").is_err())
    }
}
