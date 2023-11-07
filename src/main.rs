mod services;

use actix_web::{App, HttpServer};
use dotenv::dotenv;
use services::status;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let address = std::env::var("ADDRESS").expect("ADDRESS must be set");
    let port_string = std::env::var("PORT").expect("PORT must be set");
    let port = port_string
        .parse::<u16>()
        .expect("PORT variable is not an int");

    println!("Starting the server...");
    HttpServer::new(move || App::new().service(status))
        .bind((address, port))?
        .run()
        .await
}
