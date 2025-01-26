use common::database;
use common::Result;

use import;

use std::env;
use std::fs::File;
use std::process;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let url = env::var("DATABASE_URL").expect("no database connection URL specified");
    let gtfs_file_path = env::var("GTFS_FILE").expect("no OSM extract file specified.");
    let batch_size = env::var("BATCH_SIZE")
        .ok()
        .and_then(|size| size.parse().ok())
        .unwrap_or(4000);

    info!("Import GTFS data from archive");

    let result: Result<()> = async {
        let pool = database::connect(&url).await?;
        let archive = File::open(gtfs_file_path)?;

        let timer = import::import_gtfs_data(&pool, archive, batch_size).await?;

        timer.show_duration();

        Ok(())
    }
    .await;

    match result {
        Ok(_) => info!("import OSM data complete!"),
        Err(err) => {
            error!("Import failed: {}", err);
            process::exit(1)
        }
    }
}
