//! Module 075: Serialization Deep Dive — exercise scaffold.
//!
//! Implement JSON serialization, deserialization, and bincode conversion.
//! The Person struct below demonstrates serde field renaming.

use serde::{Deserialize, Serialize};

/// Serialize a value to a JSON string.
pub fn serialize_to_json<T: Serialize>(_value: &T) -> String {
    // TODO(module-075): use serde_json::to_string
    panic!("TODO(module-075): implement serialize_to_json")
}

/// Deserialize a JSON string to a value of type T.
pub fn deserialize_from_json<T: for<'de> Deserialize<'de>>(
    _json: &str,
) -> Result<T, serde_json::Error> {
    // TODO(module-075): use serde_json::from_str
    panic!("TODO(module-075): implement deserialize_from_json")
}

/// Parse and validate JSON, then encode it as compact bincode bytes.
/// The JSON string itself is serialized with bincode, so the result
/// can be deserialized back to a String and re-parsed.
pub fn json_to_bincode(_json: &str) -> Vec<u8> {
    // TODO(module-075): validate JSON is parseable with serde_json::from_str::<Value>(),
    // then serialize the JSON string itself with bincode::serialize().
    panic!("TODO(module-075): implement json_to_bincode")
}

/// A person record with serde field renaming.
///
/// The `#[serde(rename = "...")]` attributes on each field tell serde to use
/// different names in the serialized output than in the Rust struct.
/// In JSON, `first` becomes `"first_name"`, `last` becomes `"last_name"`.
/// The `age` field keeps its name.
///
/// TODO(module-075): study this pattern — in a real project you would add these
/// attributes yourself. For this exercise, the struct is fully wired up so you
/// can focus on implementing the three helper functions above.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "first_name")]
    pub first: String,
    #[serde(rename = "last_name")]
    pub last: String,
    pub age: u8,
}
