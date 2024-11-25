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

#[derive(Debug)]
pub struct ZipImporter<R: Read + Seek> {
    archive: ZipArchive<R>,
    stops: usize,
    stop_times: usize,
    changed_stops: usize,
    changed_stop_times: usize,
    timer: Instant,
}

pub fn from_reader<R: Read + Seek>(reader: R) -> Result<ZipImporter<R>> {
    let timer = Instant::now();
    let archive = ZipArchive::new(reader)?;

    let gtfs = ZipImporter {
        archive: archive,
        stops: 0,
        stop_times: 0,
        changed_stops: 0,
        changed_stop_times: 0,
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
        info!("  => processed stops: {}", self.stops);
        info!("  => processed stop_times: {}", self.stop_times);
        info!("  => changed stops: {}", self.changed_stops);
        info!("  => changed stop_times: {}", self.changed_stop_times);
        info!("Total duration: {:02}:{:02}:{:02}", hours, minutes, seconds);
    }

    async fn import<T: Importable + DeserializeOwned>(
        &mut self,
        file: &str,
        conn: &Pool,
    ) -> Result<&mut Self> {
        debug!("  => reading {}", file);
        let file = self.archive.by_name(file)?;

        {
            let mut reader = csv::Reader::from_reader(file);

            for result in reader.deserialize() {
                let stop: T = result?;
                stop.create_or_update(conn).await?;
            }
        }
        Ok(self)
    }
}

impl<R: Read + Seek> Importable for ZipImporter<R> {
    async fn create_or_update(mut self, conn: &Pool) -> Result<Self> {
        info!("running update/import.. This may take a while");
        self.import::<Stop>("stops.txt", conn)
            .await?
            .import::<StopTime>("stop_times.txt", conn)
            .await?;

        Ok(self)
    }
}
