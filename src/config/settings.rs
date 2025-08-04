use dotenvy::dotenv;
use std::env;

pub struct Settings {
    pub api_address: String,
    pub backend: String,
}

impl Settings {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenv().ok();

        let api_address = env::var("API_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
        let backend = env::var("MEMORY_BACKEND").unwrap_or_else(|_| "inmemory".to_string());

        Ok(Self {
            api_address,
            backend,
        })
    }
}
