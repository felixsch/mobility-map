use common::database;
use common::Result;

use std::env;
use std::process;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let url = env::var("DATABASE_URL").expect("no database connection URL specified");

    info!("Running migrations!");

    let result: Result<()> = async {
        let pool = database::connect(&url).await?;

        database::migrate_job_queue(&pool).await?;
        database::migrate_tables(&pool).await?;

        Ok(())
    }
    .await;

    match result {
        Ok(_) => info!("migration complete"),
        Err(err) => {
            error!("migration failed: {}", err);
            process::exit(1)
        }
    }
}
