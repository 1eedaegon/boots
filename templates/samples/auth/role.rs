//! Role-based access control.

/// User roles for authorization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// Full access - can read, write, and manage
    Admin,
    /// Can read and write
    Writer,
    /// Read-only access
    Reader,
}

impl Role {
    /// Check if role has admin privileges
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin)
    }

    /// Check if role can write (Admin or Writer)
    pub fn can_write(&self) -> bool {
        matches!(self, Role::Admin | Role::Writer)
    }

    /// Check if role can read (all roles)
    pub fn can_read(&self) -> bool {
        true
    }
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

impl std::str::FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "admin" => Ok(Role::Admin),
            "writer" => Ok(Role::Writer),
            "reader" => Ok(Role::Reader),
            _ => Err(format!("Unknown role: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_admin_permissions() {
        let role = Role::Admin;
        assert!(role.is_admin());
        assert!(role.can_write());
        assert!(role.can_read());
    }

    #[test]
    fn test_writer_permissions() {
        let role = Role::Writer;
        assert!(!role.is_admin());
        assert!(role.can_write());
        assert!(role.can_read());
    }

    #[test]
    fn test_reader_permissions() {
        let role = Role::Reader;
        assert!(!role.is_admin());
        assert!(!role.can_write());
        assert!(role.can_read());
    }

    #[test]
    fn test_role_from_str() {
        assert_eq!("admin".parse::<Role>().unwrap(), Role::Admin);
        assert_eq!("WRITER".parse::<Role>().unwrap(), Role::Writer);
        assert_eq!("Reader".parse::<Role>().unwrap(), Role::Reader);
    }
}
