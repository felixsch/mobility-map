use std::io::{Read, Seek};
use std::time::Instant;

use csv;
use log::{debug, info};
use serde::de::DeserializeOwned;
use zip::read::ZipArchive;

use common::database::Pool;
use common::Result;

use crate::{Stop, StopTime};

pub trait Importable: Sized {
    #[allow(async_fn_in_trait)]
    async fn create_or_update(self, conn: &Pool) -> Result<Self>;
}

#[derive(Debug, Clone)]
pub struct ZipImporter<R: Read + Seek> {
    archive: ZipArchive<R>,
    stops: i64,
    stop_times: i64,
    records_processed: usize,
    timer: Instant,
}

pub fn from_reader<R: Read + Seek>(reader: R) -> Result<ZipImporter<R>> {
    let timer = Instant::now();
    let archive = ZipArchive::new(reader)?;

    let gtfs = ZipImporter {
        archive: archive,
        stops: 0,
        stop_times: 0,
        records_processed: 0,
        timer: timer,
    };
    Ok(gtfs)
}

impl<R: Read + Seek> ZipImporter<R> {
    pub fn print_stats(&self) {
        let total = self.timer.elapsed().as_secs();
        let hours = total / 3600;
        let minutes = (total % 3600) / 60;
        let seconds = total % 60;

        info!("update/import statistics:");
        info!("  => stops: {}", self.stops);
        info!("  => stop_times: {}", self.stop_times);
        info!("  => records processed: {}", self.records_processed);
        info!("Total duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
    }

    async fn import<T: Importable + DeserializeOwned>(
        mut self,
        file: &str,
        conn: &Pool,
    ) -> Result<Self> {
        debug!("  => reading {}", file);
        let file = self.archive.by_name(file)?;

        {
            let mut reader = csv::Reader::from_reader(file);

            for result in reader.deserialize() {
                let record: T = result?;
                record.create_or_update(conn).await?;
                self.records_processed += 1;
            }
        }
        Ok(self)
    }
}

impl<R: Read + Seek> Importable for ZipImporter<R> {
    async fn create_or_update(mut self, conn: &Pool) -> Result<Self> {
        info!("running update/import.. This may take a while");
        self = self
            .import::<StopTime>("stop_times.txt", conn)
            .await?
            .import::<Stop>("stops.txt", conn)
            .await?;

        let stops: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM stops")
            .fetch_one(conn)
            .await?;

        let stop_times: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM stop_times")
            .fetch_one(conn)
            .await?;

        self.stops = stops;
        self.stop_times = stop_times;

        Ok(self)
    }
}
