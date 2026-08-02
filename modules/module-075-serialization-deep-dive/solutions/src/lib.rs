//! Module 075: Serialization Deep Dive — reference solution.

use serde::{Deserialize, Serialize};

pub fn serialize_to_json<T: Serialize>(value: &T) -> String {
    serde_json::to_string(value).expect("serialization failed")
}

pub fn deserialize_from_json<T: for<'de> Deserialize<'de>>(
    json: &str,
) -> Result<T, serde_json::Error> {
    serde_json::from_str(json)
}

/// Parse and validate JSON, then encode it as compact bincode bytes.
/// The JSON string itself is serialized with bincode, so the result
/// can be deserialized back to a String and re-parsed.
pub fn json_to_bincode(json: &str) -> Vec<u8> {
    // Validate the JSON is parseable first
    let _: serde_json::Value = serde_json::from_str(json).expect("invalid JSON");
    // Serialize the raw JSON string with bincode for compact storage
    bincode::serialize(&json.to_string()).expect("bincode serialization failed")
}

/// A person record with serde field renaming.
///
/// The `#[serde(rename = "...")]` attributes tell serde to use different
/// names in the serialized output. In JSON, `first` becomes `"first_name"`.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Person {
    #[serde(rename = "first_name")]
    pub first: String,
    #[serde(rename = "last_name")]
    pub last: String,
    pub age: u8,
}
