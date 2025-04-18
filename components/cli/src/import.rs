use common::prelude::*;
use common::{database, Timer};

pub async fn run(kind: &str) -> NoResult {
    let url = env::var("DATABASE_URL")?;
    let pool = database::connect(&url).await?;

    let mut timer = Timer::new();
    timer.start_ticking();

    match kind {
        "osm" => run_osm_import(&url, &pool).await,
        "gtfs" => run_gtfs_import(&mut timer, &pool).await,
        _ => Err(format!("unknown importer selected: {}", kind).into()),
    }?;

    timer.show_duration();
    Ok(())
}

async fn run_osm_import(url: &str, pool: &Pool) -> NoResult {
    let extract_file_path = env::var("EXTRACT_FILE").expect("no OSM extract file specified.");
    let batch_size = env::var("BATCH_SIZE")
        .ok()
        .and_then(|size| size.parse().ok())
        .unwrap_or(50_000);

    importers::import_osm_data(&url, &extract_file_path).await?;
    analyzers::detect_residential_buildings(&pool, batch_size).await?;

    Ok(())
}

async fn run_gtfs_import(timer: &mut Timer, pool: &Pool) -> NoResult {
    let gtfs_file_path = env::var("GTFS_FILE").expect("no OSM extract file specified.");
    let batch_size = env::var("BATCH_SIZE")
        .ok()
        .and_then(|size| size.parse().ok())
        .unwrap_or(4000);

    let archive = File::open(gtfs_file_path)?;

    importers::import_gtfs_data(timer, pool, archive, batch_size).await?;

    Ok(())
}
