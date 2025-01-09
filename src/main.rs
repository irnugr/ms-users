mod handlers;

use actix_web::{App, HttpServer};
use dotenv::from_filename;
use std::env;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Loading environment variables...");
    from_filename("dev.env").ok();
    let enable_cors = env::var("ENABLE_CORS").unwrap_or_else(|_| "false".to_string()) == "true";

    println!("Starting server on 127.0.0.1:8010");

    HttpServer::new(move || {
        println!("Configuring CORS...");
        let cors = if enable_cors {
            Cors::default()
                .allow_any_origin()
                .allow_any_method()
                .allow_any_header()
        } else {
            Cors::default()
        };

        println!("Setting up App...");
        App::new()
            .wrap(cors)
            .service(handlers::getuserlist::get_user_list)
            // Add other handlers here
    })
    .bind("127.0.0.1:8010")?
    .run()
    .await
}