use common::prelude::*;

use clap::{Parser, Subcommand};
//use tracing::{error, info};
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
    Run {
        action: String,
        arguments: Vec<String>,
    },
    Import {
        action: String,
        arguments: Vec<String>,
    },
    Migrate {},
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let params = CliParameters::parse();

    let _result: NoResult = {
        match &params.command {
            Commands::Run { action, arguments } => cli::run_action(action, arguments),
            Commands::Import { action, arguments } => todo!(),
            Commands::Migrate {} => todo!(),
        }
    };
}
