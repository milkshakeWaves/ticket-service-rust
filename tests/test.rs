use ticket_service_rust::dao::Database;
use dotenv::dotenv;

async fn init_db_context() -> Database<'static> {
    dotenv().ok();
    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    Database::new(&format!("{}_test", db_connection_string)).await
}

#[cfg(test)]
mod dao_test;