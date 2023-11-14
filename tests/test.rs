use ticket_service_rust::dao::Database;
use dotenv::dotenv;

async fn init_db_context() -> Database<'static> {
    dotenv().ok();
    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let test_db_conn = if !db_connection_string.contains("test") {
        format!("{}_test", db_connection_string)
    } else {
        db_connection_string
    };
    Database::new(&test_db_conn).await
}

#[cfg(test)]
mod dao_test;