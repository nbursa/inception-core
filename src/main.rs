use std::net::SocketAddr;
use std::sync::Arc;
use tokio::signal;
use tracing::{error};
use tracing_subscriber;
use tracing::info;

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
    tracing_subscriber::fmt::init();

    info!("Starting Inception Memory Server...");

    let settings = Settings::load().expect("Failed to load configuration");

    let backend = Arc::new(InMemoryBackend::new()) as Arc<dyn core::memory_store::MemoryBackend>;

    let memory_system = Arc::new(MemorySystem::new(backend.clone()));

    let app = create_http_router(memory_system.clone());

    let addr: SocketAddr = settings.api_address.parse().expect("Invalid address");

    info!("HTTP server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.expect("Failed to bind address");
    let server = axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    if let Err(e) = server.await {
        error!("Server error: {}", e);
    }

    info!("Inception shut down.");
}

async fn shutdown_signal() {
    signal::ctrl_c()
        .await
        .expect("Failed to install Ctrl+C signal handler");
    println!("\n Received shutdown signal. Cleaning up...");
}