use chrono::naive::NaiveTime;
use serde::Deserialize;

use common::database::Pool;
use common::Result;

use crate::importer::Importable;
use crate::time_parser;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct StopTime {
    #[serde(rename = "stop_id")]
    pub id: String,

    #[serde(rename = "arrival_time", with = "time_parser::messy_time")]
    pub arrival: NaiveTime,

    #[serde(rename = "departure_time", with = "time_parser::messy_time")]
    pub departure: NaiveTime,
}

impl Importable for StopTime {
    async fn create_or_update(self, conn: &Pool) -> Result<Self> {
        use sqlx;

        sqlx::query!("INSERT INTO stop_times (id, arrival, departure)
                        VALUES ($1, $2, $3)
                      ON CONFLICT (id) DO UPDATE SET arrival = excluded.arrival, departure = excluded.departure",
                self.id,
                self.arrival,
                self.departure)
            .execute(conn)
            .await?;
        Ok(self)
    }
}
