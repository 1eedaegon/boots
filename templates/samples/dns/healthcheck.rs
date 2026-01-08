//! Multi-region DNS health checking.

use std::time::Duration;

/// Region configuration
#[derive(Debug, Clone)]
pub struct Region {
    /// Region identifier (e.g., "us-east-1")
    pub id: String,
    /// Health check endpoint URL
    pub endpoint: String,
    /// Weight for load balancing (higher = more traffic)
    pub weight: u32,
}

impl Region {
    /// Create a new region with default weight
    pub fn new(id: impl Into<String>, endpoint: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            weight: 100,
        }
    }

    /// Create a new region with custom weight
    pub fn with_weight(id: impl Into<String>, endpoint: impl Into<String>, weight: u32) -> Self {
        Self {
            id: id.into(),
            endpoint: endpoint.into(),
            weight,
        }
    }
}

/// Health check result for a region
#[derive(Debug, Clone)]
pub struct HealthStatus {
    /// Region identifier
    pub region_id: String,
    /// Whether the region is healthy
    pub healthy: bool,
    /// Response latency
    pub latency: Duration,
    /// Last check timestamp
    pub last_check: std::time::SystemTime,
    /// Error message if unhealthy
    pub error: Option<String>,
}

impl HealthStatus {
    fn healthy(region_id: String, latency: Duration) -> Self {
        Self {
            region_id,
            healthy: true,
            latency,
            last_check: std::time::SystemTime::now(),
            error: None,
        }
    }

    fn unhealthy(region_id: String, error: String) -> Self {
        Self {
            region_id,
            healthy: false,
            latency: Duration::ZERO,
            last_check: std::time::SystemTime::now(),
            error: Some(error),
        }
    }
}

/// Health checker for multiple regions
pub struct HealthChecker {
    regions: Vec<Region>,
    timeout: Duration,
}

impl HealthChecker {
    /// Create a new health checker
    pub fn new(regions: Vec<Region>) -> Self {
        Self {
            regions,
            timeout: Duration::from_secs(5),
        }
    }

    /// Set the health check timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Check health of a single region
    pub async fn check(&self, region: &Region) -> HealthStatus {
        let start = std::time::Instant::now();

        // TODO: Implement actual HTTP health check with reqwest
        // For now, return a placeholder implementation
        //
        // let client = reqwest::Client::builder()
        //     .timeout(self.timeout)
        //     .build()
        //     .unwrap();
        //
        // match client.get(&region.endpoint).send().await {
        //     Ok(resp) if resp.status().is_success() => {
        //         HealthStatus::healthy(region.id.clone(), start.elapsed())
        //     }
        //     Ok(resp) => {
        //         HealthStatus::unhealthy(region.id.clone(), format!("Status: {}", resp.status()))
        //     }
        //     Err(e) => {
        //         HealthStatus::unhealthy(region.id.clone(), e.to_string())
        //     }
        // }

        // Placeholder: simulate successful check
        HealthStatus::healthy(region.id.clone(), start.elapsed())
    }

    /// Check health of all regions
    pub async fn check_all(&self) -> Vec<HealthStatus> {
        let mut results = Vec::new();
        for region in &self.regions {
            results.push(self.check(region).await);
        }
        results
    }

    /// Get all healthy regions
    pub async fn healthy_regions(&self) -> Vec<&Region> {
        let statuses = self.check_all().await;
        self.regions
            .iter()
            .zip(statuses.iter())
            .filter_map(|(region, status)| {
                if status.healthy {
                    Some(region)
                } else {
                    None
                }
            })
            .collect()
    }

    /// Select the best region based on health and latency
    pub async fn select_best(&self) -> Option<&Region> {
        let statuses = self.check_all().await;

        self.regions
            .iter()
            .zip(statuses.iter())
            .filter(|(_, status)| status.healthy)
            .min_by_key(|(_, status)| status.latency)
            .map(|(region, _)| region)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_region_creation() {
        let region = Region::new("us-east-1", "https://api.example.com/health");
        assert_eq!(region.id, "us-east-1");
        assert_eq!(region.weight, 100);
    }

    #[test]
    fn test_region_with_weight() {
        let region = Region::with_weight("eu-west-1", "https://api-eu.example.com/health", 50);
        assert_eq!(region.weight, 50);
    }

    #[tokio::test]
    async fn test_health_checker() {
        let checker = HealthChecker::new(vec![
            Region::new("region-1", "http://localhost:8080/health"),
        ]);

        let statuses = checker.check_all().await;
        assert_eq!(statuses.len(), 1);
    }
}
