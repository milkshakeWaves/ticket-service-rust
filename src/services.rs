use actix_web::{get, Responder, HttpResponse};
use serde_json::Value;

#[get("/status")]
pub async fn status() -> impl Responder {
    let data = r#"
        {
            "status": "UP"
        }"#;

    // Parse the string of data into serde_json::Value.
    let v: Value = serde_json::from_str(data).unwrap();
    HttpResponse::Ok().json(v)
}