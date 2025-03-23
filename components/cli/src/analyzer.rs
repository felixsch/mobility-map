use common::database;
use common::prelude::*;

use jobs::county::AnalyzeCountyJob;
use jobs::job::Job;
use jobs::stop::AnalyzeStopJob;

pub async fn run(kind: &str, args: &Vec<String>) -> NoResult {
    let url = env::var("DATABASE_URL")?;
    let pool = database::connect(&url).await?;

    match kind {
        "stop" => match args.first() {
            Some(ifopt) => AnalyzeStopJob::from(ifopt.clone()).perform_job(&pool).await,
            None => AnalyzeStopJob::spawn_worker(pool).await,
        },
        "county" => match args.first() {
            Some(ifopt) => {
                AnalyzeCountyJob::from(ifopt.clone())
                    .perform_job(&pool)
                    .await
            }
            None => AnalyzeCountyJob::spawn_worker(pool).await,
        },
        _ => Err(format!("unknown analyzer selected: {}", kind).into()),
    }
}
