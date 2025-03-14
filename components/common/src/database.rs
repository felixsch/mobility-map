use crate::{BoxDynError, Result, StdResult};
use anyhow::anyhow;
use apalis_sql::postgres::*;
use futures::future::BoxFuture;
use log::LevelFilter;
use sqlx::migrate::{Migration, MigrationSource, Migrator};
use sqlx::postgres;
use sqlx::ConnectOptions;
use std::path::Path;
use std::time::Duration;
use tracing::info;

pub use sqlx;
pub type Pool = sqlx::PgPool;

#[derive(Debug, Clone)]
struct MergedMigrations {
    sources: Vec<Vec<Migration>>,
}

impl MergedMigrations {
    fn new() -> Self {
        MergedMigrations {
            sources: Vec::new(),
        }
    }

    fn add_source(mut self, source: Vec<Migration>) -> Self {
        self.sources.push(source);
        self
    }
}

impl<'s> MigrationSource<'s> for MergedMigrations {
    fn resolve(self) -> BoxFuture<'s, StdResult<Vec<Migration>, BoxDynError>> {
        Box::pin(async move {
            let mut all: Vec<Migration> = self
                .sources
                .into_iter()
                .flatten()
                .filter(|m| m.migration_type.is_up_migration())
                .collect();
            all.sort_by_key(|m| m.version);

            Ok(all)
        })
    }
}

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
pub async fn migrate(pool: &Pool) -> Result<()> {
    let apalis: Vec<Migration> = PostgresStorage::migrations().migrations.into_owned();

    let mobility: Vec<Migration> = Path::new("migrations")
        .resolve()
        .await
        .map_err(|e| anyhow!("loading migrations: {}", e))?;

    let migrations: MergedMigrations = MergedMigrations::new()
        .add_source(mobility)
        .add_source(apalis);

    migrations
        .clone()
        .resolve()
        .await
        .map_err(|e| anyhow!("loading migrations: {}", e))?
        .iter()
        .for_each(|m| info!("  {} {}", m.version, m.description));

    let migrator = Migrator::new(migrations).await?;

    migrator
        .run(pool)
        .await
        .map_err(|e| anyhow!("migrator: {}", e))
}
