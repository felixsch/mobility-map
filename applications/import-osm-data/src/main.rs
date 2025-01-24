use common::database;
use common::{Result, Timer};

use analyze;
use import;

use std::env;
use std::process;
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    let url = env::var("DATABASE_URL").expect("no database connection URL specified");
    let extract_file_path = env::var("EXTRACT_FILE").expect("no OSM extract file specified.");
    let batch_size = env::var("BATCH_SIZE")
        .ok()
        .and_then(|size| size.parse().ok())
        .unwrap_or(80_000);

    info!("Import OSM data from extract file");

    let result: Result<()> = async {
        let mut timer = Timer::new();
        timer.start_ticking();

        let pool = database::connect(&url).await?;

        import::import_osm_data(&url, &extract_file_path).await?;
        analyze::detect_residential_buildings(&pool, batch_size).await?;

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
