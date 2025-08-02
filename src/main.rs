use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::{info, error};
use tracing_subscriber;

mod api;
mod backends;
mod config;
mod core;
mod rpc_client;
mod utils;

use crate::core::memory::MemorySystem;
use crate::backends::in_memory::InMemoryBackend;
use crate::config::settings::Settings;
use crate::api::http::create_http_router;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🧠 Starting Inception Memory Server...");

    // Load settings from environment or config file
    let settings = Settings::load().expect("Failed to load configuration");

    // Initialize memory backend (can be swapped for RocksDB or file store)
    let backend = Arc::new(InMemoryBackend::new()) as Arc<dyn core::memory_store::MemoryBackend>;

    // Initialize core memory system (STM + LTM)
    let memory_system = Arc::new(MemorySystem::new(backend.clone()));

    // Spawn background memory maintenance tasks
    {
        let mem_clone = memory_system.clone();
        tokio::spawn(async move {
            mem_clone.run_maintenance_tasks().await;
        });
    }

    // Create REST API router
    let app = create_http_router(memory_system.clone());

    // Bind address
    let addr: SocketAddr = settings.api_address.parse().expect("Invalid address");

    info!("🚀 HTTP server listening on http://{}", addr);

    // Start server with graceful shutdown
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    info!("🛑 Inception shut down.");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C signal handler");
    println!("\n🔌 Received shutdown signal. Cleaning up...");
}