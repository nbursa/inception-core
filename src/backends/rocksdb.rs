#[cfg(feature = "rocksdb")]
use rocksdb::{DB, Options};
#[cfg(feature = "rocksdb")]
use crate::core::memory_store::MemoryBackend;
#[cfg(feature = "rocksdb")]
use crate::core::memory_types::{MemoryToken, MemoryFrame};
#[cfg(feature = "rocksdb")]
use async_trait::async_trait;
#[cfg(feature = "rocksdb")]
use serde_json;

#[cfg(feature = "rocksdb")]
pub struct RocksDbBackend {
    db: DB,
}

#[cfg(feature = "rocksdb")]
impl RocksDbBackend {
    pub fn new(path: &str) -> Self {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        let db = DB::open(&opts, path).expect("Failed to open RocksDB");
        Self { db }
    }
}

#[cfg(feature = "rocksdb")]
#[async_trait]
impl MemoryBackend for RocksDbBackend {
    async fn store_token(&self, token: MemoryToken) {
        let key = token.id.to_string();
        let value = serde_json::to_vec(&token).unwrap();
        self.db.put(key, value).unwrap();
    }

    async fn get_all_tokens(&self) -> Vec<MemoryToken> {
        self.db
            .iterator(rocksdb::IteratorMode::Start)
            .filter_map(|(_, v)| serde_json::from_slice(&v).ok())
            .collect()
    }

    async fn get_tokens_by_subject(&self, subject: &str) -> Vec<MemoryToken> {
        self.get_all_tokens()
            .await
            .into_iter()
            .filter(|t| t.subject == subject)
            .collect()
    }

    async fn get_latest_frame(&self) -> Option<MemoryFrame> {
        None
    }
}