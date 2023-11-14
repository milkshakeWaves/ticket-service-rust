use crate::models::CreateEventBody;

use super::{Event, Table};

impl<'c> Table<'c, Event> {
    pub async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error> {
        let query_string = "SELECT id, description, place, available_seats, price, date FROM event";
        sqlx::query_as(query_string).fetch_all(&*self.pool).await
    }

    pub async fn get_event_by_id(&self, event_id: i32) -> Result<Option<Event>, sqlx::Error> {
        let query_string =
            "SELECT id, description, place, available_seats, price, date FROM event WHERE id = $1";
        sqlx::query_as(query_string)
            .bind(event_id)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn get_event_by_place(&self, event_place: &str) -> Result<Option<Event>, sqlx::Error> {
        let query_string =
            "SELECT id, description, place, available_seats, price, date FROM event WHERE place = $1";
        sqlx::query_as(query_string)
            .bind(event_place)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn add_event(&self, event: &CreateEventBody) -> Result<u64, sqlx::Error> {
        let query_string =
            "INSERT INTO event (description, place, available_seats, price, date) VALUES ($1, $2, $3, $4, $5) RETURNING id, description, place, available_seats, price, date";
        sqlx::query(query_string)
            .bind(&event.description)
            .bind(&event.place)
            .bind(&event.available_seats)
            .bind(&event.price)
            .bind(&event.date)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn delete_event(&self, event_id: i32) -> Result<u64, sqlx::Error> {
        let query_string = "DELETE FROM event WHERE id = $1";
        sqlx::query(query_string)
            .bind(event_id)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }
}
