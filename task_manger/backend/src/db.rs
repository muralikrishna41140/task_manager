use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;

pub async fn connect() -> PgPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to DB")
}