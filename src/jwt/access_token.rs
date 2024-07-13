use std::{env, error::Error};

use chrono::Duration;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Accesstoken {
    pub sub: String,
    pub iss: String,
    pub exp: i64,
    pub scopes: Vec<String>,
}

fn key() -> Result<Hmac<Sha256>, Box<dyn Error>> {
    let key = env::var("ACCESS_JWT_KEY").expect("JWT key should be given");

    Ok(Hmac::new_from_slice(key.as_bytes())?)
}

impl Accesstoken {
    pub fn new(sub: impl ToString) -> Accesstoken {
        Accesstoken {
            sub: sub.to_string(),
            exp: (chrono::Utc::now().naive_local() + Duration::minutes(15))
                .and_utc()
                .timestamp(),
            iss: "syn-ack".into(),
            scopes: vec![],
        }
    }

    pub fn new_with_scopes(sub: impl ToString, scopes: Vec<impl ToString>) -> Accesstoken {
        Accesstoken {
            sub: sub.to_string(),
            exp: (chrono::Utc::now().naive_local() + Duration::minutes(15))
                .and_utc()
                .timestamp(),
            iss: "syn-ack".into(),
            scopes: scopes.iter().map(ToString::to_string).collect(),
        }
    }

    pub fn sign(self) -> Result<String, Box<dyn Error>> {
        let key = key()?;

        Ok(self.sign_with_key(&key)?)
    }

    pub fn extract(token: String) -> Result<Accesstoken, Box<dyn Error>> {
        let key = key()?;

        Ok(token.verify_with_key(&key)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::env;

    #[test]
    fn test_jwt_sign() {
        env::set_var("ACCESS_JWT_KEY", "some-key");

        let claims = Accesstoken {
            sub: "some_user".to_string(),
            exp: 0,
            iss: "".to_string(),
            scopes: vec![],
        };
        assert!(claims.sign().is_ok())
    }

    #[test]
    fn test_jwt_extract() {
        env::set_var("ACCESS_JWT_KEY", "some-key");

        let claims = Accesstoken {
            sub: "some_user".to_string(),
            exp: 0,
            iss: "".to_string(),
            scopes: vec![],
        };

        let token = claims.sign().unwrap();
        let claims = Accesstoken::extract(token).unwrap();

        assert_eq!(claims.sub, "some_user".to_string());
    }
}
