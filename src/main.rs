use actix_web::{App, HttpServer};
use dotenvy::dotenv;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(_) => println!(".env loaded"),
        Err(e) => {
            eprintln!("Failed to load .env: {}", e);
            std::process::exit(1);
        }
    }

    let port: u16 = match std::env::var("PORT") {
        Ok(val) => match val.parse::<u16>() {
            Ok(parsed) => parsed,
            Err(_) => {
                eprintln!("Port must be a valid integer");
                std::process::exit(1);
            }
        },
        Err(_) => 8080,
    };

    HttpServer::new(|| App::new())
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
