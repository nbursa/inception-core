//! 🧠 Inception: Core memory module for autonomous cognitive systems
//! Provides STM/LTM management, query execution, and memory storage backend abstraction.
//!
//! Designed for integration with EgoAI and Sentience DSL.

pub mod api;
pub mod backends;
pub mod config;
pub mod core;
pub mod rpc_client;
pub mod utils;

pub use core::memory::{MemoryResult, MemorySystem, RecallQuery};
pub use core::memory_store::MemoryBackend;
pub use core::memory_types::{MemoryEpisode, MemoryFrame, MemoryToken};
