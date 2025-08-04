use crate::core::memory_types::{MemoryToken, RecallQuery};

pub struct QueryEngine;

impl QueryEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn run(&self, query: &RecallQuery, tokens: &[MemoryToken]) -> Vec<MemoryToken> {
        let conditions: Vec<_> = query
            .query
            .split("AND")
            .map(|s| s.trim())
            .filter_map(|cond| cond.split_once('='))
            .map(|(k, v)| (k.trim(), v.trim()))
            .collect();

        tokens
            .iter()
            .cloned()
            .filter(|t| {
                conditions.iter().all(|(k, v)| match *k {
                    "subject" => t.subject == *v,
                    "relation" => t.relation == *v,
                    "object" => t.object == *v,
                    _ => false,
                })
            })
            .collect()
    }
}
