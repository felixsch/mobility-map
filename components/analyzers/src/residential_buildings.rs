use common::batches::batches;
use common::prelude::*;

use sqlx;

pub async fn detect_residential_buildings(
    pool: &Pool,
    batch_size: i32,
) -> Result<i32, BoxDynError> {
    let mut count: u64 = 0;

    let total: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM osm_buildings")
        .fetch_one(pool)
        .await?;

    info!("checking a total of {} buildings..", total);

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

    Ok(count as i32)
}
