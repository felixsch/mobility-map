use crate::Result;
use apalis_sql::postgres::*;
use log::LevelFilter;
use sqlx::postgres;
use sqlx::ConnectOptions;
use std::time::Duration;
use tracing;

pub use sqlx;
pub type Pool = sqlx::PgPool;

#[tracing::instrument]
pub async fn connect(url: &str) -> Result<Pool> {
    let mut options: postgres::PgConnectOptions = url.parse()?;
    options = options.log_slow_statements(LevelFilter::Info, Duration::from_secs(5));

    let pool = postgres::PgPoolOptions::new()
        .max_connections(3)
        .connect_with(options)
        .await?;
    Ok(pool)
}

#[tracing::instrument]
pub async fn migrate_job_queue(conn: &Pool) -> Result<()> {
    let mut migrator = PostgresStorage::migrations();

    // Currently sqlx lacks support for multiple migrations
    // but this is neede here so we ignore everything for now
    // until https://github.com/geofmureithi/apalis/issues/439
    // is actually addressded
    // FIXME: Check if there is a better solution now!
    migrator.set_ignore_missing(true).run(conn).await?;
    Ok(())
}

#[tracing::instrument]
pub async fn migrate_tables(conn: &Pool) -> Result<()> {
    // Same as above. There should be an table name option and
    // not have it hardcoded
    // See: https://github.com/launchbadge/sqlx/issues/1698
    sqlx::migrate!("../../migrations/")
        .set_ignore_missing(true)
        .run(conn)
        .await?;
    Ok(())
}
