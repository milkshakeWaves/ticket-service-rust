use crate::models::CreateTicketBody;

use super::SoldTicketPerUser;
use super::Table;
use super::Ticket;

impl<'c> Table<'c, Ticket> {
    pub async fn get_all_tickets(&self) -> Result<Vec<SoldTicketPerUser>, sqlx::Error> {
        let query_string = "
            SELECT u.id AS user_id, u.email, e.id AS event_id, e.description, e.place, e.date 
            FROM users u JOIN ticket t ON u.id = t.user_id
                JOIN event e ON t.event_id = e.id
        ";
        sqlx::query_as(query_string).fetch_all(&*self.pool).await
    }

    pub async fn get_tickets_by_user(
        &self,
        user_email: &str,
    ) -> Result<Vec<SoldTicketPerUser>, sqlx::Error> {
        let query_string = "
            SELECT u.id AS user_id, u.email, e.id AS event_id, e.description, e.place, e.date 
            FROM users u JOIN ticket t ON u.id = t.user_id
                JOIN event e ON t.event_id = e.id
            WHERE u.email = $1
        ";
        sqlx::query_as(query_string)
            .bind(user_email)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn get_tickets_by_event(
        &self,
        event_code: &str
    ) -> Result<Vec<SoldTicketPerUser>, sqlx::Error> {
        let query_string = "
            SELECT u.id AS user_id, u.email, e.id AS event_id, e.description, e.place, e.date 
            FROM users u JOIN ticket t ON u.id = t.user_id
                JOIN event e ON t.event_id = e.id
            WHERE e.code = $1
        ";
        sqlx::query_as(query_string)
            .bind(event_code)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn delete_user_ticket(&self, email: &str) -> Result<u64, sqlx::Error> {
        let query_string = "
            DELETE FROM ticket
            WHERE user_id = (SELECT id FROM users WHERE email = $1);
        ";
        sqlx::query(query_string)
            .bind(email)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn create_user_ticket(
        &self,
        create_user_ticket_body: &CreateTicketBody
    ) -> Result<u64, sqlx::Error> {
        let query_string = "
            INSERT INTO ticket (user_id, event_id)
            VALUES (
                (SELECT id FROM users WHERE email = $1),
                (SELECT id FROM event WHERE code = $2)
            )
        ";
        sqlx::query(query_string)
            .bind(&create_user_ticket_body.user_email)
            .bind(&create_user_ticket_body.event_code)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }
}
