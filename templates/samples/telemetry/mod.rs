//! Telemetry module for {{project_name}}
//!
//! Provides observability features including:
//! - Health checks (liveness/readiness probes)
//! - Prometheus metrics
//! - OpenTelemetry tracing
//! - Structured logging
//!
//! # Example
//!
//! ```rust,ignore
//! use {{project_name_snake}}_core::telemetry;
//!
//! // Initialize telemetry
//! telemetry::init()?;
//!
//! // Record a metric
//! telemetry::metrics::increment_counter("requests_total", &[("endpoint", "/api/users")]);
//!
//! // Create a span
//! telemetry::tracing::span("process_request", || {
//!     // Your code here
//! });
//! ```

pub mod healthcheck;
pub mod log;
pub mod metric;
pub mod trace;

pub use healthcheck::{HealthCheck, Probe};
pub use log::init_logging;
pub use metric::Metrics;
pub use trace::init_tracing;

/// Initialize all telemetry subsystems
pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    init_logging()?;
    init_tracing()?;
    Ok(())
}
