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


async fn analyze_single(pool: &Pool, ifopt: String) -> Result<()> {
    info!("analyzing stop `{}`..", ifopt);

    analyze::stop::calculate_cycle(pool, &ifopt).await?;
    analyze::stop::calculate_stats_by_distances(pool, &ifopt, common::DISTANCES.into()).await?;

    Ok(())
}

async fn run_worker(_pool: &Pool) -> Result<()> {
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
            Some(Commands::Analyze { ifopt }) => analyze_single(&pool, ifopt.clone()).await,
            None => run_worker(&pool).await,
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
