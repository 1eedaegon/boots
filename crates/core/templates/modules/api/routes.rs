use axum::{routing::get, Router};

use crate::handlers;

pub fn create_router() -> Router {
    Router::new()
        .route("/health", get(handlers::health))
        .route("/metrics", get(handlers::metrics))
}
