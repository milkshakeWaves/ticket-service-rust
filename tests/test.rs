use ticket_service_rust::dao::Database;

async fn init_db_context() -> Database<'static> {
    let db_connection_string = "postgresql://ticket_service:ticket_service@localhost:5432/ticket_app_test";
    Database::new(db_connection_string).await
}

#[cfg(test)]
mod dao_test;