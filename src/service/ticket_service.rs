use super::{AppState, CreateTicketBody};
use actix_web::{delete, get, post, web, HttpResponse, Responder};

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(get_all_tickets);
    cfg.service(get_tickets_by_user_email);
    cfg.service(create_ticket);
    cfg.service(get_tickets_by_event_code);
    cfg.service(delete_ticket);
}

#[get("/tickets")]
pub async fn get_all_tickets(app_state: web::Data<AppState<'_>>) -> impl Responder {
    let tickets_res = app_state.context.tickets.get_all_tickets().await;

    match tickets_res {
        Err(_) => HttpResponse::InternalServerError().finish(),
        Ok(sold_tickets) => {
            if sold_tickets.is_empty() {
                HttpResponse::NotFound().finish()
            } else {
                HttpResponse::Ok().json(sold_tickets)
            }
        }
    }
}

#[get("/tickets/{user_email}")]
pub async fn get_tickets_by_user_email(
    user_email: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let tickets_result = app_state
        .context
        .tickets
        .get_tickets_by_user(&user_email)
        .await;

    match tickets_result {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to retrieve ticket: {}", e))
        }
    }
}

#[get("/tickets/{event_code}")]
pub async fn get_tickets_by_event_code(
    event_code: web::Path<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let tickets_result = app_state
        .context
        .tickets
        .get_tickets_by_event(&event_code)
        .await;

    match tickets_result {
        Ok(tickets) => HttpResponse::Ok().json(tickets),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to retrieve ticket: {}", e))
        }
    }
}

#[post("/tickets")]
pub async fn create_ticket(
    ticket_body: web::Json<CreateTicketBody>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let query_res = app_state
        .context
        .tickets
        .create_user_ticket(&ticket_body)
        .await;

    match query_res {
        Ok(_) => HttpResponse::Ok().json(format!(
            "{} correctly bought ticket for {} event",
            ticket_body.user_email, ticket_body.event_code
        )),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to create ticket: {}", e))
        }
    }
}

#[delete("/tickets")]
pub async fn delete_ticket(
    user_email: web::Json<String>,
    app_state: web::Data<AppState<'_>>,
) -> impl Responder {
    let query_res = app_state.context.tickets.delete_user_ticket(&user_email).await;

    match query_res {
        Ok(_) => HttpResponse::Ok().json("Ticket deleted"),
        Err(e) => {
            HttpResponse::InternalServerError().json(format!("Failed to delete ticket: {}", e))
        }
    }
}
