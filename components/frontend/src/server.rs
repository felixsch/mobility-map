use common::prelude::*;

use crate::{listener, router};

use axum;
use tokio;

pub async fn serve(listen: listener::TcpListener, route: router::Router) -> NoResult {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    info!("listening on {:?}", listen.local_addr().unwrap());

    axum::serve(listen, route)
        .with_graceful_shutdown(ctrl_c)
        .await
        .map_err(|e| e.into())
}
