use std::error::Error;

use rocket::http::Status;
use test_utils::{client, env};

#[tokio::test]
async fn test_get_version() -> Result<(), Box<dyn Error>> {
    env().setup();

    let client = client().await?;

    let response = client.get("/api/v1/version").dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(response.into_string().await, Some("v1".into()));

    Ok(())
}
