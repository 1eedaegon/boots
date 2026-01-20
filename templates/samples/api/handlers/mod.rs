use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Health & Metrics
pub async fn health() -> impl IntoResponse {
    Json(json!({ "status": "healthy" }))
}

pub async fn metrics() -> &'static str {
    "# HELP up Server is up\nup 1"
}

// Posts
#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreatePost {
    pub title: String,
    pub content: String,
}

pub async fn list_posts() -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({ "posts": [], "total": 0 }))
}

pub async fn get_post(Path(id): Path<i64>) -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({ "id": id, "title": "Sample Post", "content": "Content" }))
}

pub async fn create_post(Json(payload): Json<CreatePost>) -> impl IntoResponse {
    // TODO: Implement with persistence
    (StatusCode::CREATED, Json(json!({ "id": 1, "title": payload.title })))
}

pub async fn update_post(Path(id): Path<i64>, Json(payload): Json<CreatePost>) -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({ "id": id, "title": payload.title }))
}

pub async fn delete_post(Path(_id): Path<i64>) -> impl IntoResponse {
    // TODO: Implement with persistence
    StatusCode::NO_CONTENT
}

// Comments
#[derive(Debug, Serialize, Deserialize)]
pub struct Comment {
    pub id: i64,
    pub post_id: i64,
    pub content: String,
    pub author_id: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateComment {
    pub content: String,
}

pub async fn list_comments(Path(post_id): Path<i64>) -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({ "comments": [], "post_id": post_id }))
}

pub async fn create_comment(
    Path(post_id): Path<i64>,
    Json(payload): Json<CreateComment>,
) -> impl IntoResponse {
    // TODO: Implement with persistence
    (StatusCode::CREATED, Json(json!({ "id": 1, "post_id": post_id, "content": payload.content })))
}

pub async fn update_comment(
    Path(id): Path<i64>,
    Json(payload): Json<CreateComment>,
) -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({ "id": id, "content": payload.content }))
}

pub async fn delete_comment(Path(_id): Path<i64>) -> impl IntoResponse {
    // TODO: Implement with persistence
    StatusCode::NO_CONTENT
}

// Auth
#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub user: UserInfo,
}

#[derive(Debug, Serialize)]
pub struct UserInfo {
    pub id: i64,
    pub email: String,
    pub role: String,
}

pub async fn login(Json(payload): Json<LoginRequest>) -> impl IntoResponse {
    // TODO: Implement with persistence
    Json(json!({
        "token": "sample-token",
        "user": {
            "id": 1,
            "email": payload.email,
            "role": "admin"
        }
    }))
}

pub async fn logout() -> impl IntoResponse {
    Json(json!({ "message": "Logged out" }))
}

pub async fn me() -> impl IntoResponse {
    // TODO: Implement with auth middleware
    Json(json!({
        "id": 1,
        "email": "admin@example.com",
        "role": "admin"
    }))
}
