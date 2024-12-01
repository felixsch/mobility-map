use serde::Deserialize;

use common::database::Pool;
use common::Result;

use crate::importer::Importable;

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

impl Importable for Stop {
    async fn create_or_update(self, conn: &Pool) -> Result<Self> {
        use sqlx;

        {
            sqlx::query(
                "INSERT INTO stops (id, name, location)
                        VALUES ($1, $2, ST_SetSRID(ST_MakePoint($3, $4), 4326))
                     ON CONFLICT (id) DO UPDATE SET name = excluded.name,
                                                    location = excluded.location",
            )
            .bind(&self.id)
            .bind(&self.name)
            .bind(&self.lat)
            .bind(&self.lon)
            .execute(conn)
            .await?;
        }
        Ok(self)
    }
}
