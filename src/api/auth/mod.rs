use rocket::{get, http::Status, post, serde::json::Json, State};
use tracing::debug;

use crate::{
    repositories::{CreateUserPayload, Session, User},
    services::{AuthService, UserService},
};

pub use self::types::*;

mod types;

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
async fn register(payload: Json<CreateUserPayload>, service: &State<UserService>) -> Status {
    match service.create_user(payload.into_inner()).await {
        Ok(Some(_)) => Status::Accepted,
        Ok(None) => Status::BadRequest,
        Err(e) => {
            debug!("Registration error: {e}");
            Status::InternalServerError
        }
    }
}

#[post("/refresh")]
async fn refresh(refresh_request: RefreshRequest, service: &State<AuthService>) -> RefreshResponse {
    match service.refresh(refresh_request.0).await {
        Ok(token) => RefreshResponse::Success(Json(token)),
        Err(e) => {
            debug!("Refresh error: {e}");
            RefreshResponse::Error(Status::Unauthorized)
        }
    }
}

#[get("/logout")]
async fn logout(session: Session, service: &State<AuthService>) {
    if let Err(e) = service.logout(session).await {
        debug!("Logout error: {e}");
    }
}

#[get("/authorized")]
async fn authorized(_user: User) -> Status {
    Status::Ok
}

pub fn routes() -> Vec<rocket::Route> {
    rocket::routes![login, register, refresh, logout, authorized]
}
