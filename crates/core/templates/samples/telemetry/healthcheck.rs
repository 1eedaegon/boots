//! Health check endpoints for Kubernetes probes.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

/// Health probe types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Probe {
    /// Liveness probe - is the application running?
    Liveness,
    /// Readiness probe - is the application ready to serve traffic?
    Readiness,
    /// Startup probe - has the application started?
    Startup,
}

/// Health check state manager
#[derive(Clone)]
pub struct HealthCheck {
    alive: Arc<AtomicBool>,
    ready: Arc<AtomicBool>,
    started: Arc<AtomicBool>,
}

impl Default for HealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthCheck {
    /// Create a new health check instance
    pub fn new() -> Self {
        Self {
            alive: Arc::new(AtomicBool::new(true)),
            ready: Arc::new(AtomicBool::new(false)),
            started: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Check if the application is alive
    pub fn is_alive(&self) -> bool {
        self.alive.load(Ordering::SeqCst)
    }

    /// Check if the application is ready
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::SeqCst)
    }

    /// Check if the application has started
    pub fn is_started(&self) -> bool {
        self.started.load(Ordering::SeqCst)
    }

    /// Set the application as ready
    pub fn set_ready(&self, ready: bool) {
        self.ready.store(ready, Ordering::SeqCst);
    }

    /// Set the application as started
    pub fn set_started(&self, started: bool) {
        self.started.store(started, Ordering::SeqCst);
    }

    /// Set the application as not alive (triggers restart)
    pub fn set_unhealthy(&self) {
        self.alive.store(false, Ordering::SeqCst);
    }

    /// Check a specific probe
    pub fn check(&self, probe: Probe) -> bool {
        match probe {
            Probe::Liveness => self.is_alive(),
            Probe::Readiness => self.is_ready(),
            Probe::Startup => self.is_started(),
        }
    }

    /// Get JSON response for health endpoint
    pub fn to_json(&self, probe: Probe) -> String {
        let healthy = self.check(probe);
        format!(r#"{{"healthy": {}, "probe": "{:?}"}}"#, healthy, probe)
    }
}

/// Axum handler for /health endpoint
///
/// ```rust,ignore
/// use axum::{routing::get, Router};
/// use {{project_name_snake}}_core::telemetry::healthcheck;
///
/// let health = HealthCheck::new();
/// let app = Router::new()
///     .route("/health", get(healthcheck::liveness_handler))
///     .route("/ready", get(healthcheck::readiness_handler))
///     .with_state(health);
/// ```
pub mod handlers {
    use super::*;

    /// Liveness probe response
    pub fn liveness(health: &HealthCheck) -> (u16, String) {
        if health.is_alive() {
            (200, health.to_json(Probe::Liveness))
        } else {
            (503, health.to_json(Probe::Liveness))
        }
    }

    /// Readiness probe response
    pub fn readiness(health: &HealthCheck) -> (u16, String) {
        if health.is_ready() {
            (200, health.to_json(Probe::Readiness))
        } else {
            (503, health.to_json(Probe::Readiness))
        }
    }

    /// Startup probe response
    pub fn startup(health: &HealthCheck) -> (u16, String) {
        if health.is_started() {
            (200, health.to_json(Probe::Startup))
        } else {
            (503, health.to_json(Probe::Startup))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_health_check_default() {
        let health = HealthCheck::new();
        assert!(health.is_alive());
        assert!(!health.is_ready());
        assert!(!health.is_started());
    }

    #[test]
    fn test_set_ready() {
        let health = HealthCheck::new();
        health.set_ready(true);
        assert!(health.is_ready());
    }

    #[test]
    fn test_set_unhealthy() {
        let health = HealthCheck::new();
        health.set_unhealthy();
        assert!(!health.is_alive());
    }

    #[test]
    fn test_handlers() {
        let health = HealthCheck::new();
        health.set_ready(true);

        let (status, _) = handlers::liveness(&health);
        assert_eq!(status, 200);

        let (status, _) = handlers::readiness(&health);
        assert_eq!(status, 200);
    }
}
