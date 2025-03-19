use common::prelude::BoxDynError;
use common::prelude::*;

use apalis::prelude::*;
use apalis_sql::postgres::PostgresStorage;
use tokio::signal::ctrl_c;

pub trait Job {
    const NAME: &'static str;

    fn perform_job(self, pool: &Pool) -> BoxFuture<NoResult>;

    fn enqueue(self, pool: &Pool) -> BoxFuture<Result<String, BoxDynError>>;

    fn spawn_worker(pool: Pool) -> BoxFuture<'static, NoResult>
    where
        Self: Sized + Sync + Serialize + Unpin + Send + 'static,
        for<'de> Self: Deserialize<'de>,
    {
        async move {
            let pg: PostgresStorage<Self> = PostgresStorage::new(pool.clone());

            let execute_job = |job: Self, pool: Data<Pool>| async move {
                job.perform_job(&pool)
                    .await
                    .map_err(|e| apalis::prelude::Error::Failed(Arc::new(e.into())))
            };

            Monitor::new()
                .register({
                    WorkerBuilder::new(&Self::NAME)
                        .catch_panic()
                        .data(pool.clone())
                        .backend(pg)
                        .build_fn(execute_job)
                })
                .run_with_signal(ctrl_c())
                .await?;

            Ok(())
        }
        .boxed()
    }
}
