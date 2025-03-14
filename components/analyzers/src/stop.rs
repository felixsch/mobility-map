use common::database::Pool;
use common::Result;

use futures::future::{try_join_all, BoxFuture, FutureExt};
use sqlx;
use sqlx::Execute;
use tracing::debug;

#[derive(Clone)]
struct StopStats {
    id: String,
    hull: String,
    distance: usize,
    houses: i64,
    flats: i64,
    residents: i64,
}

pub async fn calculate_cycle(pool: &Pool, ifopt: &str) -> Result<()> {
    debug!("calculating average cycle time for {}", ifopt);

    sqlx::query(
        "UPDATE stops
           SET avg_cycle = average_cycle_time($1)
         WHERE id = $1",
    )
    .bind(ifopt)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn calculate_stats_by_distances(
    pool: &Pool,
    ifopt: &str,
    distances: Vec<usize>,
) -> Result<()> {
    debug!(
        "calculating residents stats for {} (distances: {:?})",
        ifopt, distances
    );

    let futures = distances.into_iter().map(|distance| -> BoxFuture<Result<StopStats>> {
        async move {
            let (id, hull, houses, flats, residents): (String, String, i64, i64, i64) = sqlx::query_as(
                " WITH stop AS (
                    SELECT id, location FROM stops
                        WHERE id = $1
                    ) SELECT id, ST_AsText(residential_buildings_hull), total_houses, total_flats, total_residents
                  FROM stop, residential_buildings_hull(location, $2), residential_buildings_stats(location, $2)"
            )
             .bind(ifopt).bind(distance as i32).fetch_one(pool).await?;

            debug!(
                "{}/{} {} houses, {} flats, {} residents",
                id, distance, houses, flats, residents
            );

            Ok(StopStats { id, hull, distance, houses, flats, residents })
        }.boxed()
    });

    let mut builder: sqlx::QueryBuilder<sqlx::Postgres> = sqlx::QueryBuilder::new(
        "INSERT INTO stop_stats (stop_id, geom, distance, total_residents, total_houses, total_flats, last_updated_at)"
    );

    let stats: Vec<StopStats> = try_join_all(futures).await?;

    builder.push_values(stats, |mut row, stats| {
        row.push_bind(stats.id)
            .push("ST_GeomFromText(")
            .push_bind_unseparated(stats.hull)
            .push_unseparated(", 3857)")
            .push_bind(stats.distance as i64)
            .push_bind(stats.residents)
            .push_bind(stats.houses)
            .push_bind(stats.flats)
            .push("NOW()");
    });

    builder.push(
        " ON CONFLICT (stop_id, distance) DO UPDATE SET total_residents = excluded.total_residents,
                                                        geom = excluded.geom,
                                                        total_houses = excluded.total_houses,
                                                        total_flats = excluded.total_flats,
                                                        last_updated_at = excluded.last_updated_at",
    );

    builder.build().execute(pool).await?;
    Ok(())
}
