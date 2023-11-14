use ticket_service_rust::dao::Database;
use dotenv::dotenv;

async fn init_db_context() -> Database<'static> {
    dotenv().ok();
    let db_connection_string = std::env::var("TEST_DATABASE_URL").expect("DATABASE_URL must be set");

    Database::new(&db_connection_string).await
}

#[cfg(test)]
mod dao_test;