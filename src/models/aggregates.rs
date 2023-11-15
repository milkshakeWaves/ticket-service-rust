use chrono::{DateTime, Utc};

pub struct SoldTicketPerUser {
    user_id: i32,
    email: String,
    event_id: i32,
    description: String,
    place: String,
    date: DateTime<Utc>
}