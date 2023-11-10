use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, Error},
    Argon2, PasswordVerifier, PasswordHash,
};

pub fn hash_password(password_to_hash: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let result = argon2.hash_password(password_to_hash.as_bytes(), &salt)?;
    Ok(result.to_string())
}

pub fn verify_hashed_password(password: String, stored_hashed_password: String) -> Result<bool, Error> {
    let parsed_hash = PasswordHash::new(&stored_hashed_password)?;
    match Argon2::default().verify_password(password.as_bytes(), &parsed_hash) {
        Ok(()) => Ok(true),
        Err(_) => Ok(false)
    }
}