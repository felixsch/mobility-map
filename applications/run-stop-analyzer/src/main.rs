use common::database;
use common::database::Pool;
use common::Result;

use jobs::job::Job;
use jobs::stop::AnalyzeStopJob;

use clap::{Parser, Subcommand};
use std::env;
use std::process;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "run-stop-analyzer")]
#[command(about = "Analyze a single stop or run a worker", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Analyze { ifopt: String },
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
            Some(Commands::Analyze { ifopt }) => {
                AnalyzeStopJob::from(ifopt.clone()).perform_job(&pool).await
            }
            None => AnalyzeStopJob::spawn_worker(pool).await,
        };
    }
    .await;

    match result {
        Ok(_) => info!("done!"),
        Err(err) => {
            error!("stop analyzer failed: {}", err);
            process::exit(1)
        }
    }
}
