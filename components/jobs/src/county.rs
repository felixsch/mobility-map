use crate::job::Job;
use crate::stop::AnalyzeStopJob;

use common::prelude::*;
use futures::future::try_join_all;

#[derive(Clone, Serialize, Deserialize)]
pub struct AnalyzeCountyJob {
    ags: String,
}

impl From<String> for AnalyzeCountyJob {
    fn from(ags: String) -> Self {
        AnalyzeCountyJob { ags }
    }
}

impl Job for AnalyzeCountyJob {
    const NAME: &'static str = "analyze-county-job";

    fn enqueue(self, pool: &Pool) -> BoxFuture<Result<String, BoxDynError>> {
        async move {
            let id: String = sqlx::query_scalar(
                "SELECT id FROM apalis.push_job('jobs::county::AnalyzeCountyob', json_build_object('ags', $1))",
            )
            .bind(&self.ags)
            .fetch_one(pool)
            .await?;

            debug!("{}: enqueued analyze county job (ags = {})", id, self.ags);
            Ok(id)
        }
        .boxed()
    }

    fn perform_job(self, pool: &Pool) -> BoxFuture<NoResult> {
        async move {
            info!("analyzing county `{}`..", self.ags);

            let stops: Vec<String> =
                analyze::county::fetch_stops_within_county(pool, &self.ags).await?;

            let jobs = stops
                .into_iter()
                .map(|ifopt| AnalyzeStopJob::from(ifopt).enqueue(pool));

            let _ids: Vec<String> = try_join_all(jobs).await?;
            Ok(())
        }
        .boxed()
    }
}
