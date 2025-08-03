use crate::core::memory_store::MemoryBackend;
use crate::core::memory_types::{MemoryToken, RecallQuery};
use std::sync::Arc;
use tracing::info;

pub struct MemorySystem {
    backend: Arc<dyn MemoryBackend>,
}

impl MemorySystem {
    pub fn new(backend: Arc<dyn MemoryBackend>) -> Self {
        Self { backend }
    }

    pub async fn store_token(&self, token: MemoryToken) {
        info!("Received token: {:?}", token);
        self.backend.store_token(token).await;
    }

    pub async fn recall(&self, query: RecallQuery) -> Vec<String> {
    let all_tokens = self.backend.get_all_tokens().await;
    all_tokens
        .into_iter()
        .filter(|t| t.subject == query.query)
        .map(|t| format!("{} {} {}", t.subject, t.relation, t.object))
        .collect()
}
}