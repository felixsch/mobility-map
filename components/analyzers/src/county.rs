use common::database::Pool;
use common::Result;

pub async fn fetch_stops_within_county(pool: &Pool, ags: &str) -> Result<Vec<i64>> {
    let stops: Vec<String> = sqlx::query_as(
        " WITH county AS (
          SELECT ags, geom FROM osm_counties
            WHERE ags = $1
        ) SELECT id FROM counties_within(county.geom) ",
    )
    .bind(ags)
    .fetch_all(pool)
    .await?;

    let mut stop_ids = Vec::new();
    Ok(stop_ids)
}
