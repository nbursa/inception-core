use crate::core::memory_types::MemoryToken;
use std::collections::HashMap;

#[derive(Default)]
pub struct SemanticIndex {
    subject_map: HashMap<String, Vec<MemoryToken>>,
    tag_map: HashMap<String, Vec<MemoryToken>>,
}

impl SemanticIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn insert(&mut self, token: &MemoryToken) {
        self.subject_map
            .entry(token.subject.clone())
            .or_default()
            .push(token.clone());

        for tag in &token.tags {
            self.tag_map
                .entry(tag.clone())
                .or_default()
                .push(token.clone());
        }
    }

    pub fn query_subject(&self, subject: &str) -> Vec<MemoryToken> {
        self.subject_map.get(subject).cloned().unwrap_or_default()
    }

    pub fn query_tag(&self, tag: &str) -> Vec<MemoryToken> {
        self.tag_map.get(tag).cloned().unwrap_or_default()
    }
}
