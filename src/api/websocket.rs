use axum::{
    extract::ws::{WebSocketUpgrade, WebSocket, Message},
    extract::State,
    response::IntoResponse
};
use std::sync::Arc;
use crate::core::memory::MemorySystem;
use tracing::{info, error};

pub async fn handle_ws(
    ws: WebSocketUpgrade,
    State(_memory): State<Arc<MemorySystem>>,
) -> impl IntoResponse {
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(mut socket: WebSocket) {
    info!("WebSocket connection opened.");

    while let Some(Ok(msg)) = socket.recv().await {
        if let Message::Text(txt) = msg {
            if let Err(e) = socket.send(Message::Text(format!("Echo: {}", txt))).await {
                error!("WebSocket send error: {}", e);
                break;
            }
        }
    }

    info!("WebSocket connection closed.");
}