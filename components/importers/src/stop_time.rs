use common::database::Pool;
use common::Result;

use itertools::Itertools;
use serde::Deserialize;
use sqlx::types::chrono::NaiveTime;
use sqlx::{Execute, Postgres, QueryBuilder};
use std::io::Read;
use tracing::debug;

use crate::time_parser;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct StopTime {
    #[serde(rename = "trip_id")]
    pub trip_id: String,

    #[serde(rename = "stop_id")]
    pub stop_id: String,

    #[serde(rename = "arrival_time", with = "time_parser::messy_time")]
    pub arrival: NaiveTime,

    #[serde(rename = "departure_time", with = "time_parser::messy_time")]
    pub departure: NaiveTime,
}

impl StopTime {
    async fn update_or_create_batch(pool: &Pool, batch: &Vec<StopTime>) -> Result<()> {
        let mut builder: QueryBuilder<Postgres> =
            QueryBuilder::new("INSERT INTO stop_times (trip_id, stop_id, arrival, departure)");

        builder.push_values(batch, |mut b, st| {
            b.push_bind(&st.trip_id)
                .push_bind(&st.stop_id)
                .push_bind(&st.arrival)
                .push_bind(&st.departure);
        });

        builder.push("ON CONFLICT ON CONSTRAINT unique_stop_times DO NOTHING");

        let query = builder.build();
        debug!("SQL: {}", query.sql());

        query.execute(pool).await?;

        Ok(())
    }
}

pub async fn import_stop_times<R: Read>(
    pool: &Pool,
    reader: R,
    batch_size: usize,
) -> Result<usize> {
    let mut file = csv::Reader::from_reader(reader);
    let mut total = 0;

    let stop_times = file.deserialize();

    for chunk in &stop_times.chunks(batch_size) {
        let batch = chunk.collect::<Result<Vec<StopTime>, _>>()?;

        StopTime::update_or_create_batch(pool, &batch).await?;
        total += batch.len();
    }

    Ok(total)
}
