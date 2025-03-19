use crate::stop::import_stops;
use crate::stop_time::import_stop_times;

use common::prelude::*;
use common::Timer;

use zip::read::ZipArchive;

pub async fn import_gtfs_data<R: Read + Seek>(
    pool: &Pool,
    archive_hdl: R,
    batch_size: usize,
) -> Result<Timer, BoxDynError> {
    let mut timer = Timer::new();
    timer.start_ticking();

    let mut archive = ZipArchive::new(archive_hdl)?;

    {
        info!("importing all stop information");
        let stops = archive.by_name("stops.txt")?;

        let total_stops = import_stops(pool, stops, batch_size).await?;
        timer.push_info("stops proccessed", total_stops);
    }

    {
        info!("importing all stop time information");
        let stop_times = archive.by_name("stop_times.txt")?;

        let total_stop_times = import_stop_times(pool, stop_times, batch_size).await?;
        timer.push_info("stop times proccessed", total_stop_times);
    }

    Ok(timer)
}
