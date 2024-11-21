use common::database;
use common::database::sqlx;
use common::logging;

use log::{error, info};

#[tokio::main]
async fn main() {
    logging::init();

    let result: Result<(), sqlx::Error> = async {
        let pool = database::connect().await?;

        database::migrate_job_queue(&pool).await?;
        database::migrate_tables(&pool).await?;

        Ok(())
    }
    .await;

    match result {
        Ok(_) => info!("migration complete"),
        Err(err) => error!("migration failed: {}", err),
    }
}
