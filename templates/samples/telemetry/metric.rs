//! Prometheus metrics integration.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

/// Metrics registry
pub struct Metrics {
    counters: RwLock<HashMap<String, AtomicU64>>,
    gauges: RwLock<HashMap<String, AtomicU64>>,
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Metrics {
    /// Create a new metrics registry
    pub fn new() -> Self {
        Self {
            counters: RwLock::new(HashMap::new()),
            gauges: RwLock::new(HashMap::new()),
        }
    }

    /// Increment a counter
    pub fn increment_counter(&self, name: &str, labels: &[(&str, &str)]) {
        let key = Self::make_key(name, labels);
        let counters = self.counters.read().unwrap();
        if let Some(counter) = counters.get(&key) {
            counter.fetch_add(1, Ordering::Relaxed);
            return;
        }
        drop(counters);

        let mut counters = self.counters.write().unwrap();
        counters
            .entry(key)
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(1, Ordering::Relaxed);
    }

    /// Add to a counter
    pub fn add_counter(&self, name: &str, value: u64, labels: &[(&str, &str)]) {
        let key = Self::make_key(name, labels);
        let mut counters = self.counters.write().unwrap();
        counters
            .entry(key)
            .or_insert_with(|| AtomicU64::new(0))
            .fetch_add(value, Ordering::Relaxed);
    }

    /// Set a gauge value
    pub fn set_gauge(&self, name: &str, value: u64, labels: &[(&str, &str)]) {
        let key = Self::make_key(name, labels);
        let mut gauges = self.gauges.write().unwrap();
        gauges
            .entry(key)
            .or_insert_with(|| AtomicU64::new(0))
            .store(value, Ordering::Relaxed);
    }

    /// Get a counter value
    pub fn get_counter(&self, name: &str, labels: &[(&str, &str)]) -> u64 {
        let key = Self::make_key(name, labels);
        self.counters
            .read()
            .unwrap()
            .get(&key)
            .map(|c| c.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Get a gauge value
    pub fn get_gauge(&self, name: &str, labels: &[(&str, &str)]) -> u64 {
        let key = Self::make_key(name, labels);
        self.gauges
            .read()
            .unwrap()
            .get(&key)
            .map(|g| g.load(Ordering::Relaxed))
            .unwrap_or(0)
    }

    /// Export metrics in Prometheus format
    pub fn export(&self) -> String {
        let mut output = String::new();

        // Export counters
        for (key, value) in self.counters.read().unwrap().iter() {
            output.push_str(&format!(
                "{} {}\n",
                key,
                value.load(Ordering::Relaxed)
            ));
        }

        // Export gauges
        for (key, value) in self.gauges.read().unwrap().iter() {
            output.push_str(&format!(
                "{} {}\n",
                key,
                value.load(Ordering::Relaxed)
            ));
        }

        output
    }

    fn make_key(name: &str, labels: &[(&str, &str)]) -> String {
        if labels.is_empty() {
            name.to_string()
        } else {
            let label_str: Vec<String> = labels
                .iter()
                .map(|(k, v)| format!("{}=\"{}\"", k, v))
                .collect();
            format!("{}{{{}}} ", name, label_str.join(","))
        }
    }
}

/// Global metrics instance
static METRICS: std::sync::OnceLock<Metrics> = std::sync::OnceLock::new();

/// Get the global metrics instance
pub fn global() -> &'static Metrics {
    METRICS.get_or_init(Metrics::new)
}

/// Convenience function to increment a counter
pub fn increment_counter(name: &str, labels: &[(&str, &str)]) {
    global().increment_counter(name, labels);
}

/// Convenience function to set a gauge
pub fn set_gauge(name: &str, value: u64, labels: &[(&str, &str)]) {
    global().set_gauge(name, value, labels);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_counter() {
        let metrics = Metrics::new();
        metrics.increment_counter("requests_total", &[]);
        metrics.increment_counter("requests_total", &[]);
        assert_eq!(metrics.get_counter("requests_total", &[]), 2);
    }

    #[test]
    fn test_counter_with_labels() {
        let metrics = Metrics::new();
        metrics.increment_counter("requests_total", &[("method", "GET")]);
        metrics.increment_counter("requests_total", &[("method", "POST")]);
        assert_eq!(metrics.get_counter("requests_total", &[("method", "GET")]), 1);
        assert_eq!(metrics.get_counter("requests_total", &[("method", "POST")]), 1);
    }

    #[test]
    fn test_gauge() {
        let metrics = Metrics::new();
        metrics.set_gauge("active_connections", 42, &[]);
        assert_eq!(metrics.get_gauge("active_connections", &[]), 42);
    }

    #[test]
    fn test_export() {
        let metrics = Metrics::new();
        metrics.increment_counter("test_counter", &[]);
        metrics.set_gauge("test_gauge", 100, &[]);

        let output = metrics.export();
        assert!(output.contains("test_counter"));
        assert!(output.contains("test_gauge"));
    }
}
