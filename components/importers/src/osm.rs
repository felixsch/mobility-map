use std::process::{ExitStatus, Stdio};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

use common::boxed_error;
use common::prelude::*;

pub async fn import_osm_data(
    database_url: &str,
    extract_file_path: &str,
) -> Result<ExitStatus, BoxDynError> {
    info!("Running osm2psql import..");
    debug!(extract = extract_file_path);
    let mut process = Command::new("osm2pgsql")
        .arg(format!("--database={}", database_url))
        .arg("--create")
        .arg("--output=flex")
        .arg("--style=scripts/mobility-flex.lua")
        .arg(extract_file_path)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let output = process
        .stderr
        .take()
        .expect("could not grab stdout handle of osm2pgsql");
    let mut reader = BufReader::new(output).lines();

    tokio::spawn(async move {
        while let Some(line) = reader.next_line().await.unwrap_or(None) {
            debug!(target = "osm2pgsql", line)
        }
    });

    let status = process.wait().await?;

    if !status.success() {
        return Err(boxed_error!(
            "osm2pgsql failed with status code: {}",
            status.code().unwrap_or(-1)
        ));
    }

    Ok(status)
}
