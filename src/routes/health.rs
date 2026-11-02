use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json
};
use serde_json::{json, Value};
use crate::models::health::Health;

// Option 1: If you want to return the actual Health struct (assuming it implements serde::Serialize)
pub async fn get_health() -> Result<(StatusCode, Json<Health>), StatusCode> {
    let response = Health {
        message: "Api is healthy".to_string(),
        status: 200,
    };
    
    Ok((StatusCode::OK, Json(response)))
}

// Option 2: If you want to return arbitrary/ad-hoc JSON data using the json! macro instead
pub async fn get_health_generic() -> Result<(StatusCode, Json<Value>), StatusCode> {
    Ok((StatusCode::OK, Json(json!({
        "message": ""
    }))))
}
