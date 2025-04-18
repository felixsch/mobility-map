use common::prelude::*;

mod analyzer;
mod frontend;
mod import;
mod migrate;

pub use migrate::run_migrations;

// mobility-map run stop-analyzer

pub async fn run_action(action: &String, args: &Vec<String>) -> NoResult {
    match action.as_ref() {
        "frontend" => frontend::run(args).await,
        "stop-analyzer" => analyzer::run("stop", args).await,
        "county-analyzer" => analyzer::run("county", args).await,
        _ => Err(format!("Unknown run action: {}", action).into()),
    }
}

pub async fn run_import(import: &String, _args: &Vec<String>) -> NoResult {
    match import.as_ref() {
        "osm-data" => import::run("osm").await,
        "gtfs-data" => import::run("gtfs").await,
        _ => Err(format!("Unknown import type: {}", import).into()),
    }
}
