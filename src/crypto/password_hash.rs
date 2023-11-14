use argon2::{
    password_hash::{rand_core::OsRng, Error, PasswordHasher, SaltString},
    Argon2, PasswordHash, PasswordVerifier,
};

pub fn hash_password(password_to_hash: &str) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let result = argon2.hash_password(password_to_hash.as_bytes(), &salt)?;
    Ok(result.to_string())
}

pub fn verify_hashed_password(password: &str, stored_hashed_password: &str) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(stored_hashed_password)?;
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .map(|()| true)
        .or_else(|e| match e {
            Error::Password => Ok(false),
            _ => Err(e),
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn verifying_the_correct_hash_returns_true() -> Result<(), Error> {
        let plain_txt_pass = "test-password!123";
        let hashed_pass = hash_password(plain_txt_pass)?;
        assert!(verify_hashed_password(plain_txt_pass, &hashed_pass)?);

        Ok(())
    }

    #[test]
    fn verifying_a_wrong_hash_returns_false() -> Result<(), Error> {
        let plain_txt_pass = "test-password!123";
        let wrong_hashed_pass = hash_password(&plain_txt_pass[0..4])?;
        assert!(verify_hashed_password(plain_txt_pass, &wrong_hashed_pass).is_ok_and(|v| !v));

        Ok(())
    }

    #[test]
    fn verifying_a_not_well_formed_hash_returns_error() -> Result<(), Error> {
        let plain_txt_pass = "test-password!123";
        assert!(verify_hashed_password(plain_txt_pass, "not_well_formed_hash").is_err());

        Ok(())
    }
}
