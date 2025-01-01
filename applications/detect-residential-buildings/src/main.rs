use common::batches::batches;
use common::database;
use common::logging;
use common::Result;

use log::{error, info};
use sqlx;
use std::env;
use std::process;
use std::time::Instant;

#[tokio::main]
async fn main() {
    logging::init();

    let batch_size = env::var("BATCH_SIZE")
        .ok()
        .and_then(|size| size.parse().ok())
        .unwrap_or(80_000);

    let timer = Instant::now();

    info!("Detecting residental buildings");

    let result: Result<usize> = async {
        let pool = database::connect().await?;

        let found = detect_residential_buildings(&pool, batch_size).await?;

        Ok(found)
    }
    .await;

    let total = timer.elapsed().as_secs();
    let hours = total / 3600;
    let minutes = (total % 3600) / 60;
    let seconds = total % 60;

    info!("Total duration: {:02}:{:02}:{:02}", hours, minutes, seconds);

    match result {
        Ok(count) => info!("Found {} new residential buildings", count),
        Err(err) => {
            error!("detection failed: {}", err);
            process::exit(1)
        }
    }
}

async fn detect_residential_buildings(pool: &database::Pool, batch_size: i32) -> Result<usize> {
    let mut count: u64 = 0;

    let (min, max): (i32, i32) = sqlx::query_as("SELECT MIN(id), MAX(id) FROM osm_buildings")
        .fetch_one(pool)
        .await?;

    for step in batches(min, max, batch_size) {
        info!(
            "  scanning buildings with id from {} to {}..",
            step.start, step.end
        );
        let result = sqlx::query(
            "UPDATE osm_buildings b
            SET residential = TRUE
            FROM osm_residential_areas r
               WHERE b.id BETWEEN $1 AND $2
               AND b.residential = FALSE
               AND ST_Within(b.center, r.geom)",
        )
        .bind(step.start)
        .bind(step.end)
        .execute(pool)
        .await?;

        count += result.rows_affected();
    }

    Ok(count as usize)
}
