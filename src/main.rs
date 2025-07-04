use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use sqlx::PgPool;

mod config;
use config::db::init_pg_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match dotenv() {
        Ok(_) => println!(".env loaded"),
        Err(e) => {
            eprintln!("Failed to load .env: {}", e);
            std::process::exit(1);
        }
    }

    let db_url: String = match std::env::var("DATABASE_URL") {
        Ok(val) => val,
        Err(_) => {
            eprintln!("Error loading database URL from .env");
            std::process::exit(1);
        }
    };

    let db_pool: PgPool = match init_pg_pool(&db_url).await {
        Ok(pool) => {
            println!("DB pool initialised");
            pool
        }
        Err(e) => {
            eprintln!("Failed to initialise DB: {}", e);
            std::process::exit(1);
        }
    };

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

    // .app_data(...) injects shared state into the App, making the data accessible to route handlers
    // web::Data::new wraps Arc<T>::new(), which creates cloneable, thread-safe instances that all reference the same instance (db pool in this case)
    HttpServer::new(move || App::new().app_data(web::Data::new(db_pool.clone())))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
