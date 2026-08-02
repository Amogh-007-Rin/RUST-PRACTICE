//! Module 076: integration tests.

use module_076_exercises::{KvOperation, KvResponse, KvStore, Service, ServiceRegistry};

#[test]
fn test_registry_dispatch_registered_service() {
    let mut registry = ServiceRegistry::new();

    // Register a handler that echoes the value
    registry.register("echo", |val: String| val);

    let result = registry.dispatch("echo", "hello".to_string());
    assert_eq!(result, Some("hello".to_string()));
}

#[test]
fn test_registry_unknown_service_returns_none() {
    let registry: ServiceRegistry<String, String> = ServiceRegistry::new();
    let result = registry.dispatch("nonexistent", "test".to_string());
    assert_eq!(result, None);
}

#[test]
fn test_registry_multiple_services() {
    let mut registry = ServiceRegistry::new();

    registry.register("double", |val: i32| val * 2);
    registry.register("triple", |val: i32| val * 3);

    assert_eq!(registry.dispatch("double", 5), Some(10));
    assert_eq!(registry.dispatch("triple", 5), Some(15));
}

#[test]
fn test_kv_store_service_set_and_get() {
    let store = KvStore::new();

    let response = store.call(KvOperation::Set("name".into(), "Alice".into()));
    assert_eq!(response, KvResponse::Stored);

    let response = store.call(KvOperation::Get("name".into()));
    assert_eq!(response, KvResponse::Found("Alice".into()));
}

#[test]
fn test_kv_store_get_nonexistent_key() {
    let store = KvStore::new();
    let response = store.call(KvOperation::Get("missing".into()));
    assert_eq!(response, KvResponse::NotFound);
}

#[test]
fn test_kv_store_delete() {
    let store = KvStore::new();

    store.call(KvOperation::Set("key".into(), "value".into()));
    let response = store.call(KvOperation::Delete("key".into()));
    assert_eq!(response, KvResponse::Deleted);

    let response = store.call(KvOperation::Get("key".into()));
    assert_eq!(response, KvResponse::NotFound);
}

#[test]
fn test_kv_store_overwrite() {
    let store = KvStore::new();

    store.call(KvOperation::Set("x".into(), "first".into()));
    store.call(KvOperation::Set("x".into(), "second".into()));
    let response = store.call(KvOperation::Get("x".into()));
    assert_eq!(response, KvResponse::Found("second".into()));
}
