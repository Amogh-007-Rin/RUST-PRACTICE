//! Module 075: integration tests.

use module_075_solutions::{deserialize_from_json, json_to_bincode, serialize_to_json, Person};

#[test]
fn test_serialize_to_json_primitive() {
    let json = serialize_to_json(&42u32);
    assert_eq!(json, "42");
}

#[test]
fn test_serialize_to_json_string() {
    let json = serialize_to_json(&"hello");
    assert_eq!(json, "\"hello\"");
}

#[test]
fn test_deserialize_number() {
    let value: u32 = deserialize_from_json("42").unwrap();
    assert_eq!(value, 42);
}

#[test]
fn test_deserialize_string() {
    let value: String = deserialize_from_json("\"hello\"").unwrap();
    assert_eq!(value, "hello");
}

#[test]
fn test_roundtrip_json() {
    let original = vec![1, 2, 3];
    let json = serialize_to_json(&original);
    let restored: Vec<i32> = deserialize_from_json(&json).unwrap();
    assert_eq!(original, restored);
}

#[test]
fn test_json_to_bincode_roundtrip() {
    let json = r#"{"key": "value", "num": 42}"#;

    let bytes = json_to_bincode(json);
    assert!(!bytes.is_empty(), "bincode output should not be empty");

    // Deserialize back to a String, then re-parse the JSON
    let restored: String = bincode::deserialize(&bytes).expect("bincode deserialize failed");

    let value: serde_json::Value = serde_json::from_str(&restored).expect("re-parse json failed");
    assert_eq!(value["key"], "value");
    assert_eq!(value["num"], 42);
}

#[test]
fn test_person_serialization_uses_renamed_fields() {
    let person = Person {
        first: "Jane".to_string(),
        last: "Doe".to_string(),
        age: 30,
    };
    let json = serialize_to_json(&person);
    assert!(json.contains("first_name"));
    assert!(json.contains("last_name"));
    assert!(!json.contains("\"first\""));
    assert!(!json.contains("\"last\""));
}

#[test]
fn test_person_deserialization_uses_renamed_fields() {
    let json = r#"{"first_name": "John", "last_name": "Smith", "age": 25}"#;
    let person: Person = deserialize_from_json(json).unwrap();
    assert_eq!(person.first, "John");
    assert_eq!(person.last, "Smith");
    assert_eq!(person.age, 25);
}

#[test]
fn test_person_roundtrip() {
    let original = Person {
        first: "Alice".to_string(),
        last: "Jones".to_string(),
        age: 42,
    };
    let json = serialize_to_json(&original);
    let restored: Person = deserialize_from_json(&json).unwrap();
    assert_eq!(original, restored);
}
