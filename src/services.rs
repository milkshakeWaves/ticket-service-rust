use crate::crypto::hash_password;
use crate::models::{CreateUserBody, User};
use crate::repository::{AppState, PostgresAppState};
use actix_web::post;
use actix_web::{
    get,
    web::{Data, Json},
    HttpResponse, Responder,
};

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().body("{\"status\": \"UP\"}")
}

#[get("/users")]
pub async fn get_all_user(state: Data<PostgresAppState>) -> impl Responder {
    let query_string = r#"
        SELECT id, username, email
        FROM users
    "#;
    match sqlx::query_as::<_, User>(query_string)
        .fetch_all(&state.db())
        .await
    {
        Ok(users) => HttpResponse::Ok().json(users),
        Err(e) => HttpResponse::NotFound().json(format!("No user found: {}", e)),
    }
}

#[post("/users")]
pub async fn create_user(
    state: Data<PostgresAppState>,
    body: Json<CreateUserBody>,
) -> impl Responder {
    let query_string = "INSERT INTO users (username, password, email) VALUES ($1, $2, $3) RETURNING id, username, password, email";
    let password_hash = match hash_password(body.password.to_string()) {
        Ok(hash) => hash,
        Err(_) => return HttpResponse::InternalServerError().json("Failed to hash password"),
    };
    
    match sqlx::query_as::<_, User>(query_string)
        .bind(body.username.to_string())
        .bind(password_hash)
        .bind(body.email.to_string())
        .fetch_one(&state.db())
        .await
    {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => HttpResponse::InternalServerError().json(format!("Failed to create user: {}", e)),
    }
}
