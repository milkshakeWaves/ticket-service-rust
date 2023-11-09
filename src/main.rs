mod services;

use actix_web::{web::Data, App, HttpServer};
use dotenv::dotenv;
use services::status;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

pub struct AppState {
    db: Pool<Postgres>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let db_connection_string = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_connection_string)
        .await
        .expect("Error building a connection pool");

    let address = std::env::var("ADDRESS").expect("ADDRESS must be set");
    let port_string = std::env::var("PORT").expect("PORT must be set");
    let port = port_string
        .parse::<u16>()
        .expect("PORT variable is not an int");

    println!("Starting the server...");
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState { db: pool.clone() }))
            .service(status)
    })
    .bind((address, port))?
    .run()
    .await
}
