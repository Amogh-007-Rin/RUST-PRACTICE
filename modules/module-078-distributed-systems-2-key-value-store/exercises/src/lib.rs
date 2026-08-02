//! Module 078: Distributed Systems Concepts II — exercise scaffold.
//!
//! Build a minimal distributed key-value store with single-leader replication.

use std::collections::HashMap;

/// The role a node plays in the cluster.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Leader,
    Follower,
}

/// A log entry describing an operation that has been applied or needs to be applied.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    /// A monotonically increasing index.
    pub index: u64,
    /// The key this operation targets.
    pub key: String,
    /// The value to write (None for deletes).
    pub value: Option<String>,
}

/// A minimal single-leader replicated key-value store.
#[derive(Debug, Clone)]
pub struct KvStore {
    pub data: HashMap<String, String>,
    pub role: NodeRole,
    pub log: Vec<LogEntry>,
}

impl KvStore {
    /// Create a new KvStore in the given role.
    pub fn new(role: NodeRole) -> Self {
        Self {
            data: HashMap::new(),
            role,
            log: Vec::new(),
        }
    }

    /// Leader-only: write a key-value pair and append a log entry.
    /// If this node is not the leader, do nothing (or panic).
    pub fn write(&mut self, _key: &str, _value: &str) {
        // TODO(module-078): if self.role is Leader,
        // insert into self.data, then create and push a LogEntry.
        // The log entry should use the next available index.
        panic!("TODO(module-078): implement KvStore::write")
    }

    /// Leader-only: delete a key and append a log entry.
    pub fn delete(&mut self, _key: &str) {
        // TODO(module-078): if Leader, remove from data, push LogEntry with value=None.
        panic!("TODO(module-078): implement KvStore::delete")
    }
}

/// Replicate a single key's data from the leader to a follower.
///
/// If the key exists in the leader's data, copy it to the follower.
/// If it doesn't exist in the leader, remove it from the follower (if present).
pub fn replicate_to_follower(_leader: &KvStore, _follower: &mut KvStore, _key: &str) {
    // TODO(module-078): read the key from the leader's data HashMap.
    // If Some(value), insert into follower's data.
    // If None, remove from follower's data.
    panic!("TODO(module-078): implement replicate_to_follower")
}

/// Apply all pending log entries to the store's data.
///
/// Iterate through the log and apply each entry:
/// - If entry.value is Some(v), insert (key, v) into data.
/// - If entry.value is None, remove the key from data.
pub fn apply_log_entries(_store: &mut KvStore) {
    // TODO(module-078): iterate store.log and apply each entry.
    // After applying, clear the log (it's been consumed).
    panic!("TODO(module-078): implement apply_log_entries")
}
