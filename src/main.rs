mod services;
mod repository;

use std::sync::Arc;
use std::time::Duration;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use services::status;
use repository::PostgresAppState;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let address = std::env::var("ADDRESS").expect("ADDRESS must be set");
    let port_string = std::env::var("PORT").expect("PORT must be set");
    let port = port_string
        .parse::<u16>()
        .expect("PORT variable is not an int");

    let app_state = PostgresAppState::new(
        5, &db_connection_string, Duration::from_secs(10)
    ).await.expect("Cannot create postgres pool");
    let arc_app_state = Arc::new(app_state);

    println!("Starting the server...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(arc_app_state.clone()))
            .service(status)
    })
    .bind((address, port))?
    .run()
    .await
}
