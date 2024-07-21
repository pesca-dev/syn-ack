use std::env;

use anyhow::Result;
use hmac::{digest::KeyInit, Hmac};
use jwt::{SignWithKey, VerifyWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Refreshtoken {
    pub sub: String,
    pub iss: String,
}

fn key() -> Result<Hmac<Sha256>> {
    let key = env::var("REFRESH_JWT_KEY").expect("JWT key should be given");

    Ok(Hmac::new_from_slice(key.as_bytes())?)
}

impl Refreshtoken {
    pub fn new(sub: impl ToString) -> Refreshtoken {
        Refreshtoken {
            sub: sub.to_string(),
            iss: "syn-ack".into(),
        }
    }

    pub fn sign(self) -> Result<String> {
        let key = key()?;

        Ok(self.sign_with_key(&key)?)
    }

    pub fn extract(token: String) -> Result<Refreshtoken> {
        let key = key()?;

        Ok(token.verify_with_key(&key)?)
    }
}
