mod db;
mod routes;
mod handlers;

use actix_web::{web, App, HttpServer};
use crate::routes::user_router::user_router;
use crate::db::init_db;
use dotenv::dotenv;
use std::process;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    println!("Initializing database connection...");
    let pool = match init_db().await {
        Ok(pool) => {
            println!("Database connection established.");
            pool
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            process::exit(1);
        }
    };

    let server_address = "127.0.0.1:8081";
    println!("Server starting at http://{}", server_address);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(user_router)
    })
    .bind(server_address)?
    .run()
    .await
}
