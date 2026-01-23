use axum::Json;
use serde_json::{json, Value};

pub async fn health() -> Json<Value> {
    Json(json!({ "healthy": true }))
}

pub async fn metrics() -> &'static str {
    "# HELP up Server is up\nup 1"
}
