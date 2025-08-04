use serde::{Deserialize, Serialize};
use serde_json;

pub fn to_json<T: Serialize>(data: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(data)
}

pub fn from_json<T: for<'de> Deserialize<'de>>(s: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(s)
}
