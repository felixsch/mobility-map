use common::prelude::*;

use clap::{Parser, Subcommand};
use std::process;
use tracing_subscriber::EnvFilter;

#[derive(Parser)]
#[command(name = "mobility-map")]
#[command(about = "mobility data aggregator", long_about = None)]
struct CliParameters {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run different blocking workloads such as frontend worker
    /// or stop-analyzer
    #[command(alias("r"))]
    Run {
        action: String,
        arguments: Vec<String>,
    },
    /// Import data into the database from various sources

    #[command(alias("i"))]
    Import {
        import: String,
        arguments: Vec<String>,
    },
    /// Migrate the database to the latest date and update
    /// database functions
    Migrate {},
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let params = CliParameters::parse();

    let result: NoResult = {
        match &params.command {
            Commands::Run { action, arguments } => cli::run_action(&action, &arguments).await,
            Commands::Import { import, arguments } => cli::run_import(&import, &arguments).await,
            Commands::Migrate {} => cli::run_migrations().await,
        }
    };

    match result {
        Ok(_) => info!("bye!"),
        Err(err) => {
            error!("error: {}", err);
            process::exit(1)
        }
    }
}
