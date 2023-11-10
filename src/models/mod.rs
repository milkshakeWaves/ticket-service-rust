use serde::{Serialize, Deserialize};
use sqlx::{Decode, FromRow};

#[derive(Serialize, FromRow)]
pub struct User {
    id: i32,
    email: String,
}

#[derive(Serialize, FromRow, Decode)]
pub struct PrivateUser {
    pub id: i32,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct CreateUserBody {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}