use serde::Deserialize;
use sqlx::FromRow;

#[derive(FromRow)]
pub struct Ticket {
}

#[derive(Deserialize)]
pub struct CreateTicketBody {
    pub user_email: String,
    pub event_code: String
}