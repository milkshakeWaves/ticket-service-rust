// use super::SoldTicketPerUser;
// use super::Table;
// use super::Ticket;

// impl<'c> Table<'c, Ticket> {
//     pub async fn get_all_tickets(&self) -> Result<Vec<SoldTicketPerUser>, sqlx::Error> {
//         let query_string = "
//             SELECT u.id, u.email, e.id, e.description, e.place, e.date 
//             FROM users u JOIN ticket t ON u.id = t.user_id
//                 JOIN evet e ON t.event_id = e.id
//         ";
//         sqlx::query_as(query_string).fetch_all(&*self.pool).await
//     }

//     pub async fn get_tickets_by_user(
//         &self,
//         user_email: &str,
//     ) -> Result<Vec<SoldTicketPerUser>, sqlx::Error> {
//         let query_string = "
//             SELECT u.id, u.email, e.id, e.description, e.place, e.date 
//             FROM users u JOIN ticket t ON u.id = t.user_id
//                 JOIN evet e ON t.event_id = e.id
//             WHERE u.email = $1
//         ";
//         sqlx::query_as(query_string)
//             .bind(user_email)
//             .fetch_optional(&*self.pool)
//             .await
//     }

//     pub async fn get_tickets_by_events(
//         &self,
//         email: &str,
//         password: &str,
//     ) -> Result<u64, sqlx::Error> {
//         let query_string =
//             "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email";
//         sqlx::query(query_string)
//             .bind(email)
//             .bind(password)
//             .execute(&*self.pool)
//             .await
//             .map(|x| x.rows_affected())
//     }

//     pub async fn delete_user_ticket(&self, email: &str) -> Result<u64, sqlx::Error> {
//         let query_string = "DELETE FROM users WHERE email = $1";
//         sqlx::query(query_string)
//             .bind(email)
//             .execute(&*self.pool)
//             .await
//             .map(|x| x.rows_affected())
//     }

//     pub async fn create_user_ticket(&self) -> Result<(), sqlx::Error> {}
// }
