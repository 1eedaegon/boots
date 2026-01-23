use anyhow::Result;
use reqwest::Client as HttpClient;

pub struct Client {
    http: HttpClient,
    base_url: String,
}

impl Client {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: HttpClient::new(),
            base_url: base_url.to_string(),
        }
    }

    pub async fn health(&self) -> Result<bool> {
        let resp = self
            .http
            .get(format!("{}/health", self.base_url))
            .send()
            .await?;

        Ok(resp.status().is_success())
    }
}
