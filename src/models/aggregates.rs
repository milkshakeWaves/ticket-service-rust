use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::FromRow;

#[derive(FromRow, Serialize)]
pub struct SoldTicketPerUser {
    user_id: i32,
    email: String,
    event_id: i32,
    description: String,
    place: String,
    date: DateTime<Utc>
}