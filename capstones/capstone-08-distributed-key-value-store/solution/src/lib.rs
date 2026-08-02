use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KvCommand {
    Get {
        key: String,
    },
    Set {
        key: String,
        value: String,
    },
    Delete {
        key: String,
    },
    Keys,
    Replicate {
        key: String,
        value: String,
        term: u64,
    },
    Heartbeat {
        term: u64,
        leader_id: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KvResponse {
    Ok,
    Value { value: Option<String> },
    Keys { keys: Vec<String> },
    Error { message: String },
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeRole {
    Leader,
    Follower,
}

pub struct KvStore {
    data: HashMap<String, String>,
    role: NodeRole,
    term: u64,
    node_id: String,
    leader_id: Option<String>,
}

impl KvStore {
    pub fn new(node_id: String) -> Self {
        KvStore {
            data: HashMap::new(),
            role: NodeRole::Follower,
            term: 0,
            node_id,
            leader_id: None,
        }
    }

    pub fn role(&self) -> &NodeRole {
        &self.role
    }

    pub fn term(&self) -> u64 {
        self.term
    }

    pub fn node_id(&self) -> &str {
        &self.node_id
    }

    pub fn leader_id(&self) -> Option<&str> {
        self.leader_id.as_deref()
    }

    pub fn handle_command(&mut self, cmd: KvCommand) -> KvResponse {
        match cmd {
            KvCommand::Get { key } => KvResponse::Value {
                value: self.data.get(&key).cloned(),
            },
            KvCommand::Set { key, value } => {
                self.data.insert(key, value);
                KvResponse::Ok
            }
            KvCommand::Delete { key } => match self.data.remove(&key) {
                Some(_) => KvResponse::Ok,
                None => KvResponse::Error {
                    message: format!("key not found: {}", key),
                },
            },
            KvCommand::Keys => KvResponse::Keys {
                keys: self.data.keys().cloned().collect(),
            },
            KvCommand::Replicate { key, value, term } => self.apply_replicated(key, value, term),
            KvCommand::Heartbeat {
                term: leader_term,
                leader_id,
            } => {
                if leader_term >= self.term {
                    self.term = leader_term;
                    self.role = NodeRole::Follower;
                    self.leader_id = Some(leader_id);
                }
                KvResponse::Ok
            }
        }
    }

    pub fn serialize_command(cmd: &KvCommand) -> String {
        let mut json = serde_json::to_string(cmd).expect("serialization should not fail");
        json.push('\n');
        json
    }

    pub fn deserialize_command(data: &str) -> Result<KvCommand, serde_json::Error> {
        serde_json::from_str(data.trim())
    }

    pub fn serialize_response(resp: &KvResponse) -> String {
        let mut json = serde_json::to_string(resp).expect("serialization should not fail");
        json.push('\n');
        json
    }

    pub fn deserialize_response(data: &str) -> Result<KvResponse, serde_json::Error> {
        serde_json::from_str(data.trim())
    }

    pub fn become_leader(&mut self) {
        self.role = NodeRole::Leader;
        self.term += 1;
        self.leader_id = Some(self.node_id.clone());
    }

    pub fn set_term(&mut self, term: u64) {
        self.term = term;
    }

    pub fn apply_replicated(&mut self, key: String, value: String, term: u64) -> KvResponse {
        if term < self.term {
            return KvResponse::Error {
                message: format!("stale term {} (current term {})", term, self.term),
            };
        }
        self.term = term;
        self.data.insert(key, value);
        KvResponse::Ok
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_store_defaults() {
        let store = KvStore::new("node-1".into());
        assert_eq!(store.role(), &NodeRole::Follower);
        assert_eq!(store.term(), 0);
        assert_eq!(store.node_id(), "node-1");
        assert_eq!(store.leader_id(), None);
    }

    #[test]
    fn test_get_set_delete() {
        let mut store = KvStore::new("n1".into());

        assert_eq!(
            store.handle_command(KvCommand::Get { key: "a".into() }),
            KvResponse::Value { value: None }
        );

        assert_eq!(
            store.handle_command(KvCommand::Set {
                key: "a".into(),
                value: "1".into()
            }),
            KvResponse::Ok
        );

        assert_eq!(
            store.handle_command(KvCommand::Get { key: "a".into() }),
            KvResponse::Value {
                value: Some("1".into())
            }
        );

        assert_eq!(
            store.handle_command(KvCommand::Delete { key: "a".into() }),
            KvResponse::Ok
        );

        assert_eq!(
            store.handle_command(KvCommand::Get { key: "a".into() }),
            KvResponse::Value { value: None }
        );
    }

    #[test]
    fn test_keys() {
        let mut store = KvStore::new("n1".into());
        store.handle_command(KvCommand::Set {
            key: "x".into(),
            value: "1".into(),
        });
        store.handle_command(KvCommand::Set {
            key: "y".into(),
            value: "2".into(),
        });

        let mut keys = match store.handle_command(KvCommand::Keys) {
            KvResponse::Keys { keys } => keys,
            _ => panic!("expected Keys response"),
        };
        keys.sort();
        assert_eq!(keys, vec!["x", "y"]);
    }

    #[test]
    fn test_delete_missing_key() {
        let mut store = KvStore::new("n1".into());
        assert_eq!(
            store.handle_command(KvCommand::Delete {
                key: "missing".into()
            }),
            KvResponse::Error {
                message: "key not found: missing".into()
            }
        );
    }

    #[test]
    fn test_serialization_roundtrip() {
        let cmds = vec![
            KvCommand::Get { key: "k".into() },
            KvCommand::Set {
                key: "k".into(),
                value: "v".into(),
            },
            KvCommand::Delete { key: "k".into() },
            KvCommand::Keys,
            KvCommand::Replicate {
                key: "k".into(),
                value: "v".into(),
                term: 1,
            },
            KvCommand::Heartbeat {
                term: 2,
                leader_id: "leader".into(),
            },
        ];

        for cmd in cmds {
            let wire = KvStore::serialize_command(&cmd);
            let parsed = KvStore::deserialize_command(&wire).expect("deserialize should succeed");
            assert_eq!(cmd, parsed);
        }
    }

    #[test]
    fn test_become_leader() {
        let mut store = KvStore::new("node-1".into());
        store.become_leader();
        assert_eq!(store.role(), &NodeRole::Leader);
        assert_eq!(store.term(), 1);
        assert_eq!(store.leader_id(), Some("node-1"));
    }

    #[test]
    fn test_replication_stale_term() {
        let mut store = KvStore::new("follower".into());
        store.term = 5;
        let resp = store.handle_command(KvCommand::Replicate {
            key: "k".into(),
            value: "v".into(),
            term: 3,
        });
        assert!(matches!(resp, KvResponse::Error { .. }));
        // key should not have been inserted with stale term
        assert_eq!(
            store.handle_command(KvCommand::Get { key: "k".into() }),
            KvResponse::Value { value: None }
        );
    }

    #[test]
    fn test_replication_accepts_fresh_term() {
        let mut store = KvStore::new("follower".into());
        let resp = store.handle_command(KvCommand::Replicate {
            key: "k".into(),
            value: "v".into(),
            term: 2,
        });
        assert_eq!(resp, KvResponse::Ok);
        assert_eq!(store.term(), 2);
        assert_eq!(
            store.handle_command(KvCommand::Get { key: "k".into() }),
            KvResponse::Value {
                value: Some("v".into())
            }
        );
    }

    #[test]
    fn test_heartbeat_updates_follower() {
        let mut store = KvStore::new("follower".into());
        store.become_leader();
        assert_eq!(store.role(), &NodeRole::Leader);

        store.handle_command(KvCommand::Heartbeat {
            term: 10,
            leader_id: "new-leader".into(),
        });

        assert_eq!(store.role(), &NodeRole::Follower);
        assert_eq!(store.term(), 10);
        assert_eq!(store.leader_id(), Some("new-leader"));
    }
}
