use common::database;
use common::logging;
use common::Result;

use gtfs::importer;
use gtfs::importer::Importable;

use log::{error, info};
use std::env;
use std::fs::File;
use std::process;

#[tokio::main]
async fn main() {
    logging::init();

    let source = env::var("GTFS_FILE").expect("ENV variable GTFS_FILE missing. Can not proceed!");

    info!("updating/importing gtfs data..");

    let result: Result<()> = async {
        info!("  => establish database connection..");
        let pool = database::connect().await?;

        info!("  => reading sources from {}..", source);
        let archive = File::open(source)?;
        let importer = importer::from_reader(archive)?;

        importer.create_or_update(&pool).await?.print_stats();

        Ok(())
    }
    .await;

    match result {
        Ok(_) => info!("update/import complete!"),
        Err(err) => {
            error!("update/import failed: {}", err);
            process::exit(1)
        }
    }
}
