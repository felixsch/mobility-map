use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use log::info;
use tower_http::services::ServeDir;

use common::logging;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let index = IndexTemplate;
    Html(index.render().unwrap())
}

#[tokio::main]
async fn main() {
    logging::init();

    info!("mounting assets..");
    // Serve static files from the "assets" directory
    let serve_dir = ServeDir::new("applications/run-frontend/static");

    info!("mounting app..");
    // Build our application with some routes
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", serve_dir);

    info!("fetching port..");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
