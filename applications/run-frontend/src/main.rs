use askama::Template;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{Html, IntoResponse, Json},
    routing::get,
    Router,
};
use common::database::{connect, Pool};
use common::Result;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry, Value};

use serde::Deserialize;
use sqlx::Row;
use std::env;
use std::process;

use tower_http::services::ServeDir;
use tracing::{error, info};
use tracing_subscriber::EnvFilter;

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

async fn index() -> impl IntoResponse {
    let index = IndexTemplate;
    Html(index.render().unwrap())
}

#[derive(Deserialize)]
struct AccessibleAreaParams {
    bbox: String,
    distance: i64,
}

async fn accessible_area(
    State(pool): State<Pool>,
    Query(params): Query<AccessibleAreaParams>,
) -> Result<Json<GeoJson>, StatusCode> {
    let bbox: Vec<f64> = params
        .bbox
        .split(',')
        .filter_map(|s| s.parse().ok())
        .collect();

    let rows = sqlx::query(
        "
        SELECT ST_AsGeoJSON(ST_Transform(ST_Union(stats.geom), 4326)) AS geojson FROM stop_stats stats
          WHERE ST_Intersects(
            stats.geom, 
            ST_Transform(ST_MakeEnvelope($1, $2, $3, $4, 4326), 3857)
          )
          AND stats.distance = $5",
    )
    .bind(bbox[0])
    .bind(bbox[1])
    .bind(bbox[2])
    .bind(bbox[3])
    .bind(params.distance)
    .fetch_all(&pool)
    .await
    .map_err(|e| {
        error!("error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let all_geometry: Result<Vec<Geometry>, geojson::Error> = rows
        .into_iter()
        .map(|row| {
            let data: String = row.get("geojson");
            let geojson: GeoJson = data.parse::<GeoJson>()?;

            Geometry::try_from(geojson)
        })
        .collect::<Vec<_>>()
        .into_iter()
        .collect();

    let geometry: Vec<Geometry> = all_geometry.map_err(|e| {
        error!("error: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let features: Vec<Feature> = geometry
        .into_iter()
        .map(|geometry| Feature {
            id: None,
            bbox: None,
            geometry: Some(geometry),
            properties: None,
            foreign_members: None,
        })
        .collect();

    let collection = FeatureCollection {
        features,
        bbox: None,
        foreign_members: None,
    };
    let geojson = GeoJson::FeatureCollection(collection);

    Ok(Json(geojson))
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let serve_dir = ServeDir::new("applications/run-frontend/assets");
    let url = env::var("DATABASE_URL").expect("no database connection URL specified");

    let result: Result<()> = async {
        let pool: Pool = connect(&url).await?;
        let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;

        info!("mounting app..");
        let app = Router::new()
            .route("/", get(index))
            .route("/api/accessible_area", get(accessible_area))
            .nest_service("/assets", serve_dir)
            .with_state(pool);

        let ctrl_c = async {
            tokio::signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        info!("listening on {:?}", listener.local_addr().unwrap());
        axum::serve(listener, app)
            .with_graceful_shutdown(ctrl_c)
            .await
            .map_err(|e| e.into())
    }
    .await;

    match result {
        Ok(_) => info!("bye!"),
        Err(err) => {
            error!("FATAL: {}", err);
            process::exit(1)
        }
    }
}
