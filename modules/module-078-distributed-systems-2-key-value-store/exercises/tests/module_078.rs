//! Module 078: integration tests.

use module_078_exercises::{apply_log_entries, replicate_to_follower, KvStore, NodeRole};

#[test]
fn test_leader_writes_to_data_and_log() {
    let mut store = KvStore::new(NodeRole::Leader);
    store.write("name", "Alice");
    assert_eq!(store.data.get("name"), Some(&"Alice".to_string()));
    assert_eq!(store.log.len(), 1);
    assert_eq!(store.log[0].index, 1);
    assert_eq!(store.log[0].key, "name");
    assert_eq!(store.log[0].value, Some("Alice".to_string()));
}

#[test]
fn test_leader_delete_adds_log_entry() {
    let mut store = KvStore::new(NodeRole::Leader);
    store.write("key", "value");
    store.delete("key");
    assert!(!store.data.contains_key("key"));
    assert_eq!(store.log.len(), 2);
    assert_eq!(store.log[1].value, None);
}

#[test]
fn test_replicate_to_follower_copies_key() {
    let mut leader = KvStore::new(NodeRole::Leader);
    leader.write("x", "hello");

    let mut follower = KvStore::new(NodeRole::Follower);
    replicate_to_follower(&leader, &mut follower, "x");

    assert_eq!(follower.data.get("x"), Some(&"hello".to_string()));
}

#[test]
fn test_replicate_to_follower_removes_delete() {
    let leader = KvStore::new(NodeRole::Leader);

    let mut follower = KvStore::new(NodeRole::Follower);
    follower.data.insert("y".to_string(), "old".to_string());

    replicate_to_follower(&leader, &mut follower, "y");
    assert!(!follower.data.contains_key("y"));
}

#[test]
fn test_apply_log_entries() {
    let mut store = KvStore::new(NodeRole::Follower);
    // Manually add log entries (simulating replication)
    store.log.push(module_078_exercises::LogEntry {
        index: 1,
        key: "a".to_string(),
        value: Some("100".to_string()),
    });
    store.log.push(module_078_exercises::LogEntry {
        index: 2,
        key: "b".to_string(),
        value: None,
    });

    apply_log_entries(&mut store);

    assert_eq!(store.data.get("a"), Some(&"100".to_string()));
    assert!(!store.data.contains_key("b"));
    assert!(store.log.is_empty(), "log should be cleared after apply");
}

#[test]
fn test_apply_log_entries_empty_log() {
    let mut store = KvStore::new(NodeRole::Follower);
    apply_log_entries(&mut store);
    assert!(store.data.is_empty());
    assert!(store.log.is_empty());
}
