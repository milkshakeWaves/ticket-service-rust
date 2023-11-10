use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString, Error},
    Argon2,
};
pub fn hash_password(password_to_hash: String) -> Result<String, Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let result = argon2.hash_password(password_to_hash.as_bytes(), &salt)?;
    Ok(result.to_string())
}
