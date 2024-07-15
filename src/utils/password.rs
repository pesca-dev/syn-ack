use argon2::{
    password_hash::{
        rand_core::OsRng, Error, PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

pub fn hash_password(password: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);

    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();

    Ok(password_hash)
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(hash)?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_password() {
        assert!(hash_password("some_password".to_string()).is_ok())
    }

    #[test]
    fn test_verify_passwort_correct() {
        let password = "some_password".to_string();
        let hash = hash_password(password.clone()).unwrap();

        assert_eq!(verify_password(&password, &hash), Ok(true));
    }

    #[test]
    fn test_verify_passwort_wrong() {
        let password = "some_password".to_string();
        let hash = hash_password(password.clone()).unwrap();

        let password = "other_password".to_string();
        assert_eq!(verify_password(&password, &hash), Ok(false));
    }
}
