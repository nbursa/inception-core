use std::sync::RwLock;
use crate::core::memory_store::MemoryBackend;
use crate::core::memory_types::{MemoryToken, MemoryFrame};
use async_trait::async_trait;

#[derive(Default)]
pub struct InMemoryBackend {
    tokens: RwLock<Vec<MemoryToken>>,
}

impl InMemoryBackend {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl MemoryBackend for InMemoryBackend {
    async fn store_token(&self, token: MemoryToken) {
        self.tokens.write().unwrap().push(token);
    }

    async fn get_all_tokens(&self) -> Vec<MemoryToken> {
        self.tokens.read().unwrap().clone()
    }

    async fn get_tokens_by_subject(&self, subject: &str) -> Vec<MemoryToken> {
        self.tokens.read().unwrap()
            .iter()
            .filter(|t| t.subject == subject)
            .cloned()
            .collect()
    }

    async fn get_latest_frame(&self) -> Option<MemoryFrame> {
        None // Not implemented for in-memory
    }
}