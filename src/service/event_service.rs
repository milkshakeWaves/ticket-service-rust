use super::{AppState, CreateEventBody};
use actix_web::{get, post, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_events);
    cfg.service(create_event);
    cfg.service(get_event_by_id);
}

#[get("/event")]
pub async fn get_all_events(app_state: web::Data<AppState<'_>>) -> impl Responder {
    let events_res = app_state.context.event.get_all_events().await;

    match events_res {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(events) => {
            if events.is_empty() {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().json(events)
            }
        }
    }
}

#[get("/event/{id}")]
pub async fn get_event_by_id(
    event_id: web::Path<i32>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let event_result = app_state.context.event.get_event_by_id(*event_id).await;

    match event_result {
        Ok(event_option) => {
            if event_option.is_some() {
                HttpResponse::Ok().json(event_option.unwrap())
            } else {
                HttpResponse::NotFound().finish()
            }
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to retrieve user: {}", e))
        }
    }
}

#[post("/event")]
pub async fn create_event(
    event_body: web::Json<CreateEventBody>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let query_res = app_state.context.event.add_event(&event_body).await;

    match query_res {
        Ok(_) => HttpResponse::Ok().json(format!("Event {} created", event_body.description)),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to hash password: {}", e))
        }
    }
}
