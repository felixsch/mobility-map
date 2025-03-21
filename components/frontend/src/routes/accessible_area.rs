use crate::prelude::*;
use common::prelude::*;

use geojson::{Feature, FeatureCollection, GeoJson, Geometry};

#[derive(Deserialize)]
pub struct AccessibleAreaParams {
    pub bbox: String,
    pub distance: i64,
}

pub async fn accessible_area(
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

    let all_geometry: std::result::Result<Vec<Geometry>, geojson::Error> = rows
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
