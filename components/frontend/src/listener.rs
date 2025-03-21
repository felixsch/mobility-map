use common::prelude::*;

pub use tokio::net::TcpListener;

pub async fn new(config: String) -> Result<TcpListener, BoxDynError> {
    TcpListener::bind(config).await.map_err(|e| e.into())
}
