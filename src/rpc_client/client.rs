use crate::rpc_client::types::{MemoryToken, RecallQuery};
use reqwest::Client;

pub struct InceptionClient {
    http: Client,
    base_url: String,
}

impl InceptionClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            http: Client::new(),
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn store_token(&self, token: &MemoryToken) -> Result<(), reqwest::Error> {
        let url = format!("{}/memory/store", self.base_url);
        self.http.post(&url).json(token).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn recall(&self, query: &RecallQuery) -> Result<Vec<String>, reqwest::Error> {
        let url = format!("{}/memory/recall", self.base_url);
        let response = self.http.post(&url).json(query).send().await?.error_for_status()?;
        let results: Vec<String> = response.json().await?;
        Ok(results)
    }
}