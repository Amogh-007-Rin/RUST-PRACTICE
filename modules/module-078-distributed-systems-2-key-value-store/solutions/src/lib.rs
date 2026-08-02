//! Module 078: Distributed Systems Concepts II — reference solution.

use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeRole {
    Leader,
    Follower,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub index: u64,
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub struct KvStore {
    pub data: HashMap<String, String>,
    pub role: NodeRole,
    pub log: Vec<LogEntry>,
}

impl KvStore {
    pub fn new(role: NodeRole) -> Self {
        Self {
            data: HashMap::new(),
            role,
            log: Vec::new(),
        }
    }

    pub fn write(&mut self, key: &str, value: &str) {
        if self.role == NodeRole::Leader {
            self.data.insert(key.to_string(), value.to_string());
            let index = self.log.len() as u64 + 1;
            self.log.push(LogEntry {
                index,
                key: key.to_string(),
                value: Some(value.to_string()),
            });
        }
    }

    pub fn delete(&mut self, key: &str) {
        if self.role == NodeRole::Leader {
            self.data.remove(key);
            let index = self.log.len() as u64 + 1;
            self.log.push(LogEntry {
                index,
                key: key.to_string(),
                value: None,
            });
        }
    }
}

pub fn replicate_to_follower(leader: &KvStore, follower: &mut KvStore, key: &str) {
    match leader.data.get(key) {
        Some(value) => {
            follower.data.insert(key.to_string(), value.clone());
        }
        None => {
            follower.data.remove(key);
        }
    }
}

pub fn apply_log_entries(store: &mut KvStore) {
    for entry in store.log.drain(..) {
        match entry.value {
            Some(v) => {
                store.data.insert(entry.key, v);
            }
            None => {
                store.data.remove(&entry.key);
            }
        }
    }
}
