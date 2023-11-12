use super::Table;
use super::{User, InternalUser};

impl<'c> Table<'c, User> {
    pub async fn get_all_users(&self) -> Result<Vec<User>, sqlx::Error> {
        let query_string = "SELECT id, email, password FROM users";
        sqlx::query_as(query_string)
            .fetch_all(&*self.pool)
            .await
    }

    pub async fn get_user_by_email(&self, user_email: &str) -> Result<Option<InternalUser>, sqlx::Error> {
        let query_string = "SELECT id, email, password FROM users WHERE email = $1";
        sqlx::query_as(query_string)
            .bind(user_email)
            .fetch_optional(&*self.pool)
            .await
    }

    pub async fn add_user(&self, email: &str, password: &str) -> Result<u64, sqlx::Error> {
        let query_string =
            "INSERT INTO users (email, password) VALUES ($1, $2) RETURNING id, email";
        sqlx::query(query_string)
            .bind(email)
            .bind(password)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }

    pub async fn delete_user(&self, email: &str) -> Result<u64, sqlx::Error> {
        let query_string = "DELETE FROM users WHERE email = $1";
        sqlx::query(query_string)
            .bind(email)
            .execute(&*self.pool)
            .await
            .map(|x| x.rows_affected())
    }
}
