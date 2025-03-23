use crate::job::Job;
use common::prelude::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct AnalyzeStopJob {
    ifopt: String,
}

impl From<String> for AnalyzeStopJob {
    fn from(ifopt: String) -> Self {
        AnalyzeStopJob { ifopt }
    }
}

impl Job for AnalyzeStopJob {
    const NAME: &'static str = "analyze-stop-job";

    fn enqueue(self, pool: &Pool) -> BoxFuture<Result<String, BoxDynError>> {
        async move {
            let id: String = sqlx::query_scalar(
                "SELECT id FROM apalis.push_job('jobs::stop::AnalyzeStopJob', json_build_object('ifopt', $1));",
            )
            .bind(&self.ifopt)
            .fetch_one(pool)
            .await?;

            debug!("{}: enqueued analyze stop job (ifopt = {})", id, self.ifopt);
            Ok(id)
        }
        .boxed()
    }

    fn perform_job(self, pool: &Pool) -> BoxFuture<NoResult> {
        async move {
            info!("analyzing stop `{}`..", self.ifopt);

            analyzers::stop::calculate_cycle(pool, &self.ifopt).await?;
            analyzers::stop::calculate_stats_by_distances(
                pool,
                &self.ifopt,
                common::DISTANCES.into(),
            )
            .await?;

            Ok(())
        }
        .boxed()
    }
}
