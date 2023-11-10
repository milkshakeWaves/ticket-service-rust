use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
pub fn hash_password(password_to_hash: String) -> String {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let result = argon2
        .hash_password(password_to_hash.as_bytes(), &salt)
        .expect("Cannot hash the password");
    result.to_string()
}
