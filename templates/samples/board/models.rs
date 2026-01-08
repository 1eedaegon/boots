//! Board data models

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role for RBAC
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    Writer,
    Reader,
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::Writer => write!(f, "writer"),
            Role::Reader => write!(f, "reader"),
        }
    }
}

/// User model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub name: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
}

/// Post model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub author_id: Uuid,
    pub author_name: String,
    pub files: Vec<FileAttachment>,
    pub comment_count: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePostRequest {
    pub title: String,
    pub content: String,
    #[serde(default)]
    pub file_keys: Vec<String>,
}

/// Request to update a post
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdatePostRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    #[serde(default)]
    pub add_file_keys: Vec<String>,
    #[serde(default)]
    pub remove_file_keys: Vec<String>,
}

/// Comment model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub content: String,
    pub author_id: Uuid,
    pub author_name: String,
    pub files: Vec<FileAttachment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Request to create a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCommentRequest {
    pub content: String,
    #[serde(default)]
    pub file_keys: Vec<String>,
}

/// Request to update a comment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCommentRequest {
    pub content: String,
}

/// File attachment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileAttachment {
    pub id: Uuid,
    pub key: String,
    pub filename: String,
    pub content_type: String,
    pub size: u64,
    pub uploader_id: Uuid,
    pub is_image: bool,
    pub created_at: DateTime<Utc>,
}

impl FileAttachment {
    /// Check if file is an image based on content type
    pub fn check_is_image(content_type: &str) -> bool {
        content_type.starts_with("image/")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_role_display() {
        assert_eq!(Role::Admin.to_string(), "admin");
        assert_eq!(Role::Writer.to_string(), "writer");
        assert_eq!(Role::Reader.to_string(), "reader");
    }

    #[test]
    fn test_is_image() {
        assert!(FileAttachment::check_is_image("image/png"));
        assert!(FileAttachment::check_is_image("image/jpeg"));
        assert!(!FileAttachment::check_is_image("application/pdf"));
    }
}
