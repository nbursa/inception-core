use serde::{Deserialize, Serialize};
use time::serde::rfc3339;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryToken {
    pub id: Uuid,
    pub subject: String,
    pub relation: String,
    pub object: String,
    #[serde(with = "rfc3339")]
    pub timestamp: OffsetDateTime,
    pub tags: Vec<String>,
}

impl MemoryToken {
    pub fn new(subject: &str, relation: &str, object: &str) -> Self {
        Self {
            id: Uuid::new_v4(),
            subject: subject.to_string(),
            relation: relation.to_string(),
            object: object.to_string(),
            timestamp: OffsetDateTime::now_utc(),
            tags: vec![],
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryFrame {
    pub id: Uuid,
    pub tokens: Vec<MemoryToken>,
    pub timestamp: OffsetDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryEpisode {
    pub id: Uuid,
    pub frames: Vec<MemoryFrame>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecallQuery {
    pub query: String, // DSL: "subject=ball AND object=red"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryResult {
    pub token: MemoryToken,
    pub match_score: f32,
}
