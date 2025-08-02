use uuid::Uuid;

/// Generates a new unique identifier for memory tokens, frames, or episodes.
pub fn generate_id() -> Uuid {
    Uuid::new_v4()
}
