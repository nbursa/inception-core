use axum::{
    routing::{get, post},
    Router, Json, extract::State
};
use std::sync::Arc;
use crate::core::memory::{MemorySystem, RecallQuery};
use crate::core::memory_types::MemoryToken;
use tracing::info;

pub fn create_http_router(memory: Arc<MemorySystem>) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .route("/memory/store", post(store_token))
        .route("/memory/recall", post(recall_memory))
        .with_state(memory)
}

async fn health_check() -> &'static str {
    "🧠 Inception is alive."
}

async fn store_token(
    State(memory): State<Arc<MemorySystem>>,
    Json(token): Json<MemoryToken>
) -> &'static str {
    memory.store_token(token).await;
    "✅ Stored"
}

async fn recall_memory(
    State(memory): State<Arc<MemorySystem>>,
    Json(query): Json<RecallQuery>
) -> Json<Vec<String>> {
    let results = memory.recall(query).await;
    Json(results)
}