use askama::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

use tower_http::services::ServeDir;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let index = IndexTemplate;
    Html(index.render().unwrap())
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    info!("mounting assets..");
    // Serve static files from the "assets" directory
    let serve_dir = ServeDir::new("applications/run-frontend/assets");

    info!("mounting app..");
    // Build our application with some routes
    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", serve_dir);

    info!("fetching port..");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    info!("setup signals..");
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    info!("listening on {:?}", listener.local_addr().unwrap());
    axum::serve(listener, app)
        .with_graceful_shutdown(ctrl_c)
        .await
        .unwrap();
}
