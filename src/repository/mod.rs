use sqlx::{postgres::PgPoolOptions, Database, Pool, Postgres};

trait AppState<D: Database> {
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
    pub async fn new(max_connections: u32, db_connection_string: &str) -> Result<PostgresAppState, sqlx::Error> {
        match create_postgres_pool(max_connections, db_connection_string).await {
            Ok(pool) => Ok(PostgresAppState { db: pool }),
            Err(e) => Err(e)
        }
    }
}

// Private impl
async fn create_postgres_pool(
    max_connections: u32,
    db_connection_string: &str,
) -> Result<Pool<Postgres>, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(max_connections)
        .connect(db_connection_string)
        .await
}
