use crate::Result;
use apalis::postgres::PostgresStorage;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub use sqlx;

pub async fn connect() -> Result<PgPool> {
    let url = env::var("DATABASE_URL").expect("no database connection URL specified");

    let pool = PgPoolOptions::new()
        .max_connections(3)
        .connect(&url)
        .await?;
    Ok(pool)
}

pub async fn migrate_job_queue(conn: &PgPool) -> Result<()> {
    let mut migrator = PostgresStorage::migrations();

    // Currently sqlx lacks support for multiple migrations
    // but this is neede here so we ignore everything for now
    // until https://github.com/geofmureithi/apalis/issues/439
    // is actually addressded
    // FIXME: Check if there is a better solution now!
    migrator.set_ignore_missing(true).run(conn).await?;
    Ok(())
}

pub async fn migrate_tables(conn: &PgPool) -> Result<()> {
    // Same as above. There should be an table name option and
    // not have it hardcoded
    // See: https://github.com/launchbadge/sqlx/issues/1698
    sqlx::migrate!("../../migrations/")
        .set_ignore_missing(true)
        .run(conn)
        .await?;
    Ok(())
}
