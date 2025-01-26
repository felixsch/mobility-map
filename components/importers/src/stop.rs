use common::database::Pool;
use common::Result;

use itertools::Itertools;
use serde::Deserialize;
use sqlx::{Execute, Postgres, QueryBuilder};
use std::io::Read;
use tracing::debug;

#[derive(Debug, Deserialize, Clone)]
pub struct Stop {
    #[serde(rename = "stop_id")]
    pub id: String,

    #[serde(rename = "stop_name")]
    pub name: String,

    #[serde(rename = "stop_lat")]
    pub lat: f64,

    #[serde(rename = "stop_lon")]
    pub lon: f64,
}

impl Stop {
    async fn update_or_create_batch(pool: &Pool, batch: &Vec<Stop>) -> Result<()> {
        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO stops (id, name, location)");

        builder.push_values(batch, |mut b, stop| {
            b.push_bind(&stop.id)
                .push_bind(&stop.name)
                .push("ST_Transform(ST_SetSRID(ST_MakePoint(")
                .push_bind_unseparated(stop.lon)
                .push_bind(stop.lat)
                .push_unseparated("), 4326), 3857)");
        });
        builder.push(
            " ON CONFLICT (id) DO UPDATE SET name = excluded.name, location = excluded.location",
        );

        let query = builder.build();
        debug!("SQL: {}", query.sql());

        query.execute(pool).await?;

        Ok(())
    }
}

pub async fn import_stops<R: Read>(pool: &Pool, reader: R, batch_size: usize) -> Result<usize> {
    let mut file = csv::Reader::from_reader(reader);
    let mut total = 0;

    let stops = file.deserialize();

    for chunk in &stops.chunks(batch_size) {
        let records = chunk.collect::<Result<Vec<Stop>, _>>()?;

        Stop::update_or_create_batch(pool, &records).await?;
        total += records.len();
    }

    Ok(total)
}
