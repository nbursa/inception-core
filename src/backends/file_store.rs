use std::fs::{OpenOptions};
use std::io::{BufReader, BufWriter, Write};
use std::sync::RwLock;
use crate::core::memory_store::MemoryBackend;
use crate::core::memory_types::{MemoryToken, MemoryFrame};
use async_trait::async_trait;
use serde_json;

const FILE_PATH: &str = "memory_store.jsonl";

pub struct FileStoreBackend {
    tokens: RwLock<Vec<MemoryToken>>,
}

impl FileStoreBackend {
    pub fn new() -> Self {
        let tokens = read_all_tokens_from_file().unwrap_or_default();
        Self {
            tokens: RwLock::new(tokens),
        }
    }
}

#[async_trait]
impl MemoryBackend for FileStoreBackend {
    async fn store_token(&self, token: MemoryToken) {
        {
            let mut lock = self.tokens.write().unwrap();
            lock.push(token.clone());
        }

        let file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(FILE_PATH)
            .unwrap();

        let mut writer = BufWriter::new(file);
        let json = serde_json::to_string(&token).unwrap();
        writer.write_all(json.as_bytes()).unwrap();
        writer.write_all(b"\n").unwrap();
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
        None
    }
}

fn read_all_tokens_from_file() -> Result<Vec<MemoryToken>, std::io::Error> {
    if !std::path::Path::new(FILE_PATH).exists() {
        return Ok(vec![]);
    }

    let file = std::fs::File::open(FILE_PATH)?;
    let reader = BufReader::new(file);
    let mut tokens = Vec::new();

    for line in reader.lines().flatten() {
        if let Ok(token) = serde_json::from_str(&line) {
            tokens.push(token);
        }
    }

    Ok(tokens)
}