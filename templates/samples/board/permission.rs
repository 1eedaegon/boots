//! Permission checks for board operations

use super::models::Role;
use uuid::Uuid;

/// Check if user can create a post
/// - Admin: yes
/// - Writer: yes
/// - Reader: no
pub fn can_create_post(role: &Role) -> bool {
    matches!(role, Role::Admin | Role::Writer)
}

/// Check if user can modify (edit/delete) a post
/// - Admin: always
/// - Writer: only their own posts
/// - Reader: never
pub fn can_modify_post(role: &Role, user_id: Uuid, post_author_id: Uuid) -> bool {
    match role {
        Role::Admin => true,
        Role::Writer => user_id == post_author_id,
        Role::Reader => false,
    }
}

/// Check if user can create a comment
/// - Admin: yes
/// - Writer: yes
/// - Reader: yes
pub fn can_create_comment(_role: &Role) -> bool {
    true
}

/// Check if user can modify (edit/delete) a comment
/// - Admin: always
/// - Writer: never (cannot edit even their own comments)
/// - Reader: only their own comments
pub fn can_modify_comment(role: &Role, user_id: Uuid, comment_author_id: Uuid) -> bool {
    match role {
        Role::Admin => true,
        Role::Writer => false, // Writer cannot edit comments
        Role::Reader => user_id == comment_author_id,
    }
}

/// Check if user can delete a file
/// - Admin: always
/// - Writer/Reader: only their own uploads
pub fn can_delete_file(role: &Role, user_id: Uuid, uploader_id: Uuid) -> bool {
    match role {
        Role::Admin => true,
        _ => user_id == uploader_id,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_create_post() {
        assert!(can_create_post(&Role::Admin));
        assert!(can_create_post(&Role::Writer));
        assert!(!can_create_post(&Role::Reader));
    }

    #[test]
    fn test_can_modify_post() {
        let user_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();

        // Admin can modify any post
        assert!(can_modify_post(&Role::Admin, user_id, other_id));

        // Writer can only modify own posts
        assert!(can_modify_post(&Role::Writer, user_id, user_id));
        assert!(!can_modify_post(&Role::Writer, user_id, other_id));

        // Reader cannot modify posts
        assert!(!can_modify_post(&Role::Reader, user_id, user_id));
    }

    #[test]
    fn test_can_modify_comment() {
        let user_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();

        // Admin can modify any comment
        assert!(can_modify_comment(&Role::Admin, user_id, other_id));

        // Writer cannot modify comments (even their own)
        assert!(!can_modify_comment(&Role::Writer, user_id, user_id));

        // Reader can only modify own comments
        assert!(can_modify_comment(&Role::Reader, user_id, user_id));
        assert!(!can_modify_comment(&Role::Reader, user_id, other_id));
    }

    #[test]
    fn test_can_delete_file() {
        let user_id = Uuid::new_v4();
        let other_id = Uuid::new_v4();

        // Admin can delete any file
        assert!(can_delete_file(&Role::Admin, user_id, other_id));

        // Others can only delete own uploads
        assert!(can_delete_file(&Role::Writer, user_id, user_id));
        assert!(!can_delete_file(&Role::Writer, user_id, other_id));
    }
}
