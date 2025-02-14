use sqlx::PgPool;
use std::env;

pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env");

    println!("Attempting to connect to the database...");

    match PgPool::connect(&database_url).await {
        Ok(pool) => {
            println!("Successfully connected to the database.");
            Ok(pool)
        }
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            Err(err)
        }
    }
}
