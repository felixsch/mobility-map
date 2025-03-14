use crate::job::Job;
use common::database::Pool;
use common::Result;

use futures::future::{BoxFuture, FutureExt};
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

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

    fn enqueue(self, pool: &Pool) -> BoxFuture<Result<String>> {
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

    fn perform_job(self, pool: &Pool) -> BoxFuture<Result<()>> {
        async move {
            info!("analyzing stop `{}`..", self.ifopt);

            analyze::stop::calculate_cycle(pool, &self.ifopt).await?;
            analyze::stop::calculate_stats_by_distances(
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
