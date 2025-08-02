use async_trait::async_trait;
use crate::core::memory_types::{MemoryToken, MemoryFrame};

#[async_trait]
pub trait MemoryBackend: Send + Sync {
    async fn store_token(&self, token: MemoryToken);
    async fn get_all_tokens(&self) -> Vec<MemoryToken>;
    async fn get_tokens_by_subject(&self, subject: &str) -> Vec<MemoryToken>;
    async fn get_latest_frame(&self) -> Option<MemoryFrame>;
}