//! Authentication module for {{project_name}}
//!
//! Provides JWT token-based authentication and role-based access control.
//!
//! # Example
//!
//! ```rust,ignore
//! use {{project_name_snake}}_core::auth::{Token, Role, Claims};
//!
//! // Validate a token
//! let claims = Token::validate(&token, &secret)?;
//!
//! // Check role permissions
//! if claims.role.can_write() {
//!     // Perform write operation
//! }
//! ```

pub mod role;
pub mod token;

pub use role::Role;
pub use token::{Claims, Token, TokenError};
