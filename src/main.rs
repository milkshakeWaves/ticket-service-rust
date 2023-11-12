use std::sync::{Arc, Mutex};

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use ticket_service_rust::dao::Database;
use ticket_service_rust::{service, AppState};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let address = std::env::var("ADDRESS").expect("ADDRESS must be set");
    let port_string = std::env::var("PORT").expect("PORT must be set");
    let port = port_string
        .parse::<u16>()
        .expect("PORT variable is not an int");

    let db_context = Database::new(&db_connection_string).await;

    let app_state = web::Data::new(AppState {
        connections: Mutex::new(0),
        context: Arc::new(db_context),
    });

    let app = HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .configure(service::init_user_service)
    })
    .bind((address, port))?;

    println!("Listening on: {}", port);

    app.run().await
}
