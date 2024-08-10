use std::error::Error;

use rocket::http::{Header, Status};
use syn_ack::services::TokenPair;
use test_utils::{env, TestFramework};

#[tokio::test]
async fn auth_flow() -> Result<(), Box<dyn Error>> {
    env()
        .setup()
        .set("DB_NS", "syn_ack_testing")
        .set("DB_DB", "auth_flow");

    let mut framework = TestFramework::new().await?;

    // create user
    {
        let response = framework
            .post(
                "/api/v1/auth/register",
                Some(
                    r#"{
                            "username": "m.mustermann",
                            "email": "mustermann@example.com",
                            "password": "somePassword"
                        }"#,
                ),
            )
            .dispatch()
            .await;

        assert_eq!(response.status(), Status::Accepted);
    }

    // login with credentials
    framework.login("m.mustermann", "somePassword").await;

    // try to fetch authorized route
    {
        let response = framework.get("/api/v1/auth/authorized").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }

    let Some(TokenPair { access_token, .. }) = framework.tokens() else {
        unreachable!()
    };

    // refresh current session
    framework.refresh().await;

    // fetch the authorized route with old access token
    {
        let mut req = framework.get("/api/v1/auth/authorized");
        req.replace_header(Header::new(
            "Authorization",
            format!("Bearer {access_token}"),
        ));
        let response = req.dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }

    // fetch the authorized route with new access token
    {
        let response = framework.get("/api/v1/auth/authorized").dispatch().await;
        assert_eq!(response.status(), Status::Ok);
    }

    // logout
    framework.logout().await;

    // try to access while being unauthorized

    let response = framework.get("/api/v1/auth/authorized").dispatch().await;
    assert_eq!(response.status(), Status::Unauthorized);

    Ok(())
}
