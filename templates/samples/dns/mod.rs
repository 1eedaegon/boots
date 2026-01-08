//! DNS and multi-region health check module for {{project_name}}
//!
//! Provides DNS-based health checking and region routing capabilities.
//!
//! # Example
//!
//! ```rust,ignore
//! use {{project_name_snake}}_core::dns::{HealthChecker, Region};
//!
//! let checker = HealthChecker::new(vec![
//!     Region::new("us-east-1", "https://api-us.example.com"),
//!     Region::new("eu-west-1", "https://api-eu.example.com"),
//! ]);
//!
//! let healthy_regions = checker.check_all().await;
//! ```

pub mod healthcheck;

pub use healthcheck::{HealthChecker, HealthStatus, Region};
