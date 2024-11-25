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
    async fn create_or_update(self, _conn: &Pool) -> Result<Self> {
        Ok(self)
    }
}
