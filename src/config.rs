use std::env;
use sqlx::{PgPool};
use sqlx::postgres::PgPoolOptions;
use dotenv::dotenv;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub log_level: String
}
impl Config {
    pub fn new() -> Config {
        dotenv().ok();
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env"),
            log_level: env::var("LOG_LEVEL").expect("LOG_LEVEL must be set in .env"),
        }
    }
}
pub async fn establish_connection() -> PgPool {
    dotenv().ok();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(Config::new().database_url.as_str())
        .await
        .expect("Failed to connect to the database")
}
pub fn init_logging() {
    env_logger::Builder::from_env(env_logger::Env::new().default_filter_or(Config::new().log_level))
        .init();
}
