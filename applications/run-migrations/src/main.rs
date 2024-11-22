use common::database;
use common::logging;
use common::Result;

use log::{error, info};
use std::process;

#[tokio::main]
async fn main() {
    logging::init();

    info!("Running migrations!");

    let result: Result<()> = async {
        let pool = database::connect().await?;

        database::migrate_job_queue(&pool).await?;
        database::migrate_tables(&pool).await?;

        Ok(())
    }
    .await;

    match result {
        Ok(_) => info!("migration complete"),
        Err(err) => {
            error!("migration failed: {}", err);
            process::exit(1)
        }
    }
}
