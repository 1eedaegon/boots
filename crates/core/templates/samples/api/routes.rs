use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::handlers;

pub fn create_router() -> Router {
    let api_routes = Router::new()
        .route("/health", get(handlers::health))
        // Posts
        .route("/posts", get(handlers::list_posts).post(handlers::create_post))
        .route("/posts/:id", get(handlers::get_post))
        .route("/posts/:id", put(handlers::update_post))
        .route("/posts/:id", delete(handlers::delete_post))
        // Comments
        .route(
            "/posts/:id/comments",
            get(handlers::list_comments).post(handlers::create_comment),
        )
        .route("/comments/:id", put(handlers::update_comment))
        .route("/comments/:id", delete(handlers::delete_comment))
        // Auth
        .route("/auth/login", post(handlers::login))
        .route("/auth/logout", post(handlers::logout))
        .route("/auth/me", get(handlers::me));

    Router::new()
        .route("/health", get(handlers::health))
        .route("/metrics", get(handlers::metrics))
        .nest("/api", api_routes)
}
