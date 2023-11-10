use sqlx::{postgres::PgPoolOptions, Database, Pool, Postgres};
use std::time::Duration;

pub trait AppState<D: Database> {
    fn db(&self) -> Pool<D>;
}

#[derive(Clone)]
pub struct PostgresAppState {
    db: Pool<Postgres>,
}

impl AppState<Postgres> for PostgresAppState {
    fn db(&self) -> Pool<Postgres> {
        self.db.clone()
    }
}

// Public impl
impl PostgresAppState {
    pub async fn new(
        max_connections: u32,
        db_connection_string: &str,
        timeout: Duration,
    ) -> Result<PostgresAppState, sqlx::Error> {
        let pool = create_postgres_pool(max_connections, db_connection_string, timeout).await?;
        Ok(PostgresAppState { db: pool })
    }
}

// Private impl
async fn create_postgres_pool(
    max_connections: u32,
    db_connection_string: &str,
    timeout: Duration,
) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .acquire_timeout(timeout)
        .connect(db_connection_string)
        .await
}
