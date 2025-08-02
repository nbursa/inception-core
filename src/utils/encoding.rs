use serde::{Deserialize, Serialize};
use serde_json;

/// Serializes any serializable data to a JSON string.
pub fn to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

/// Deserializes JSON string into a value of type T.
pub fn from_json<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}
