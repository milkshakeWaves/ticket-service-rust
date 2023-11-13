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
