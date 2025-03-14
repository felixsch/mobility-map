use common::database::Pool;
use common::Result;

pub async fn fetch_stops_within_county(pool: &Pool, ags: &str) -> Result<Vec<String>> {
    let stops: Vec<String> = sqlx::query_as::<_, (String,)>(
        " WITH county AS (
          SELECT ags, geom FROM osm_counties
            WHERE ags = $1
        ) SELECT id FROM stops_within((SELECT geom FROM county)::geometry)",
    )
    .bind(ags)
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|rec| rec.0)
    .collect();

    Ok(stops)
}
