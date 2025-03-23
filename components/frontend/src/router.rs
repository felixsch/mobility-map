use crate::routes;
use axum::routing::get;
use common::prelude::*;
use tower_http::services;

pub use axum::Router;

pub fn new(pool: Pool) -> axum::Router {
    let assets = services::ServeDir::new("components/frontend/assets");

    Router::new()
        .route("/", get(routes::index))
        .route("/api/accessible_area", get(routes::accessible_area))
        .nest_service("/assets", assets)
        .with_state(pool)
}
