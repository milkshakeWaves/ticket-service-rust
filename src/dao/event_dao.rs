use crate::models::CreateEventBody;

use super::{Event, Table};

impl<'c> Table<'c, Event> {
    pub async fn get_all_events(&self) -> Result<Vec<Event>, sqlx::Error> {
        let query_string =
            "SELECT code, description, place, available_seats, price, date FROM event";
        sqlx::query_as(query_string).fetch_all(&*self.pool).await
    }

    pub async fn get_event_by_code(&self, code: &str) -> Result<Option<Event>, sqlx::Error> {
        let query_string =
            "SELECT code, description, place, available_seats, price, date FROM event WHERE code = $1";
        sqlx::query_as(query_string)
            .bind(code)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn get_event_by_place(
        &self,
        event_place: &str,
    ) -> Result<Option<Event>, sqlx::Error> {
        let query_string =
            "SELECT code, description, place, available_seats, price, date FROM event WHERE place = $1";
        sqlx::query_as(query_string)
            .bind(event_place)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn add_event(&self, event: &CreateEventBody) -> Result<u64, sqlx::Error> {
        let query_string =
            "INSERT INTO event (code, description, place, available_seats, price, date) VALUES ($1, $2, $3, $4, $5, $6)";
        sqlx::query(query_string)
            .bind(&event.code)
            .bind(&event.description)
            .bind(&event.place)
            .bind(&event.available_seats)
            .bind(&event.price)
            .bind(&event.date)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn delete_event(&self, code: &str) -> Result<u64, sqlx::Error> {
        let query_string = "DELETE FROM event WHERE code = $1";
        sqlx::query(query_string)
            .bind(code)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }
}
