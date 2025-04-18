use common::database;
use common::prelude::*;

pub async fn run_migrations() -> NoResult {
    let url = env::var("DATABASE_URL")?;
    let pool = database::connect(&url).await?;

    database::migrate(&pool).await
}
