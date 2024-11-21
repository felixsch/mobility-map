use apalis::postgres::PostgresStorage;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub use sqlx;

pub async fn connect() -> Result<PgPool, sqlx::Error> {
    let url = env::var("DATABASE_URL").expect("No database connection URL specified!");

    PgPoolOptions::new().max_connections(3).connect(&url).await
}

pub async fn migrate_job_queue(conn: &PgPool) -> Result<(), sqlx::Error> {
    PostgresStorage::setup(conn).await
}

pub async fn migrate_tables(_conn: &PgPool) -> Result<(), sqlx::Error> {
    Ok(())
}
