use common::database;
use common::database::Pool;
use common::Result;

use analyze;

use clap::{Parser, Subcommand};
use std::env;
use std::process;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "run-county-analyzer")]
#[command(about = "Analyze a single county or run a worker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Analyze { ags: String },
}

// COMMON
use apalis::prelude::*;
use apalis_sql::postgres::PostgresStorage;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

//---

#[derive(Clone, Serialize, Deserialize)]
struct AnalyzeCounty {
    ags: String,
}

async fn analyze_single(pool: &Pool, ags: String) -> Result<()> {
    info!("analyzing county `{}`..", ags);

    let stops: Vec<i64> = analyze::county::fetch_stops_within_county(pool, &ags).await?;

    stops.into_iter().for_each(|id| {
        info!("enqueued stop {}..", id);
    });

    Ok(())
}

async fn run_worker(pool: Pool) -> Result<()> {
    info!("starting worker: county-analyzer...");

    PostgresStorage::setup(&pool).await.unwrap();

    let pg: PostgresStorage<AnalyzeCounty> = PostgresStorage::new(pool.clone());

    async fn analyze(job: AnalyzeCounty, pool: Data<Pool>) -> Result<(), Error> {
        analyze_single(&pool, job.ags)
            .await
            .map_err(|e| apalis::prelude::Error::Failed(Arc::new(e.into())))
    }

    Monitor::new()
        .register({
            WorkerBuilder::new(&format!("analyze-county"))
                .data(pool.clone())
                .backend(pg)
                .build_fn(analyze)
        })
        .run()
        .await?;
    Ok(())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    let url = env::var("DATABASE_URL").expect("no database connection URL specified");

    let result: Result<()> = async {
        let pool = database::connect(&url).await?;

        return match &cli.command {
            Some(Commands::Analyze { ags }) => analyze_single(&pool, ags.clone()).await,
            None => run_worker(pool).await,
        };
    }
    .await;

    match result {
        Ok(_) => info!("done!"),
        Err(err) => {
            error!("county analyzer failed: {}", err);
            process::exit(1)
        }
    }
}
