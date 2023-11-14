use serde::{Serialize, Deserialize};
use sqlx::FromRow;
use chrono::{DateTime, Utc};

#[derive(Serialize, FromRow, Debug)]
pub struct Event {
    pub id: i32,
    pub description: String,
    pub place: String,
    pub available_seats: i32,
    pub price: i32,
    pub date: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct CreateEventBody {
    pub description: String,
    pub place: String,
    pub available_seats: i32,
    pub price: i32,
    pub date: DateTime<Utc>
}