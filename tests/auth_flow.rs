use std::error::Error;

use rocket::http::{ContentType, Header, Status};
use syn_ack::services::TokenPair;
use test_utils::{client, env};

#[tokio::test]
async fn auth_flow() -> Result<(), Box<dyn Error>> {
    env()
        .setup()
        .set("DB_NS", "syn_ack_testing")
        .set("DB_DB", "auth_flow");

    let client = client().await?;

    // create user

    let response = client
        .post("/api/v1/auth/register")
        .header(ContentType::JSON)
        .body(
            r#"{
                "username": "m.mustermann",
                "email": "mustermann@example.com",
                "password": "somePassword"
            }"#,
        )
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Accepted);
    assert_eq!(response.into_string().await, None);

    // login with credentials

    let response = client
        .post("/api/v1/auth/login")
        .header(ContentType::JSON)
        .body(
            r#"{
                "username": "m.mustermann",
                "password": "somePassword"
            }"#,
        )
        .dispatch()
        .await;

    // fetch tokens from login

    assert_eq!(response.status(), Status::Accepted);
    let Some(TokenPair {
        access_token,
        refresh_token,
    }) = response.into_json::<TokenPair>().await
    else {
        panic!("login response did not match");
    };

    // try to fetch authorized route

    let response = client
        .get("/api/v1/auth/authorized")
        .header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("Hey m.mustermann, called 1".into())
    );

    // refresh current session

    let response = client
        .post("/api/v1/auth/refresh")
        .header(Header::new(
            "Authorization",
            format!("Bearer {refresh_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Accepted);
    let Some(new_tokens) = response.into_json::<TokenPair>().await else {
        panic!("refresh response did not match");
    };

    // fetch the authorized route with old access token

    let response = client
        .get("/api/v1/auth/authorized")
        .header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("Hey m.mustermann, called 2".into())
    );

    let TokenPair { access_token, .. } = new_tokens;

    // fetch the authorized route with new access token

    let response = client
        .get("/api/v1/auth/authorized")
        .header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await,
        Some("Hey m.mustermann, called 3".into())
    );

    // logout

    let response = client
        .get("/api/v1/auth/logout")
        .header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Ok);

    // try to access while being unauthorized

    let response = client
        .get("/api/v1/auth/authorized")
        .header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ))
        .dispatch()
        .await;

    assert_eq!(response.status(), Status::Unauthorized);

    Ok(())
}
