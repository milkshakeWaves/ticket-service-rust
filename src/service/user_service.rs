use super::password_hash::{hash_password, verify_hashed_password};
use super::{AppState, CreateUserBody, LoginUser};
use actix_web::{get, post, web, web::Json, HttpResponse, Responder};
use serde_json::json;
use tokio::task;

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_users);
    cfg.service(create_user);
    cfg.service(login);
}

#[get("/status")]
pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(json!({"status": "UP"}))
}

#[get("/users")]
pub async fn get_all_users(app_state: web::Data<AppState<'_>>) -> impl Responder {
    let users = app_state.context.users.get_all_users().await;

    match users {
        Err(_) => HttpResponse::NotFound().finish(),
        Ok(users) => HttpResponse::Ok().json(users),
    }
}

#[post("/users")]
pub async fn create_user(
    user: web::Json<CreateUserBody>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let email = user.email.clone();
    let hashed_pass = task::spawn_blocking(
        move || {
            let password = &user.password;
            hash_password(password)
        }
    ).await.unwrap();

    match hashed_pass {
        Ok(password) => {
            let rows_affected = app_state
                .context
                .users
                .add_user(&email, &password)
                .await;

            match rows_affected {
                Ok(_) => HttpResponse::Ok().json(format!("User {} created", email)),
                Err(e) => HttpResponse::InternalServerError()
                    .json(format!("Failed to create user: {}", e)),
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to hash password: {}", e))
        }
    }
}

#[post("/login")]
pub async fn login(app_state: web::Data<AppState<'_>>, body: Json<LoginUser>) -> impl Responder {
    let user_option = app_state.context.users.get_user_by_email(&body.email).await;
    let wrong_credentials_msg = "User does not exist or wrong credentials";

    match user_option {
        Ok(user_option) => match user_option {
            Some(user) => match verify_hashed_password(&body.password.to_string(), &user.password) {
                Ok(true) => HttpResponse::Ok().json("Login successfull"),
                Ok(false) => {
                    HttpResponse::Unauthorized().json(wrong_credentials_msg)
                }
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .json(format!("Password verification failed: {}", e))
                }
            },
            None => HttpResponse::Unauthorized().json(wrong_credentials_msg),
        },
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
        }
    }
}
