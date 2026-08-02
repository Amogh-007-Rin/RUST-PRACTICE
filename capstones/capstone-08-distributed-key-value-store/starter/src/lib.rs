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

#[allow(dead_code)]
pub struct KvStore {
    data: HashMap<String, String>,
    role: NodeRole,
    term: u64,
    node_id: String,
    leader_id: Option<String>,
}

impl KvStore {
    pub fn new(_node_id: String) -> Self {
        todo!("initialize KvStore with node_id, default role Follower, term 0")
    }

    pub fn role(&self) -> &NodeRole {
        todo!("return current role")
    }

    pub fn term(&self) -> u64 {
        todo!("return current term")
    }

    pub fn node_id(&self) -> &str {
        todo!("return node id")
    }

    pub fn leader_id(&self) -> Option<&str> {
        todo!("return leader id if known")
    }

    pub fn handle_command(&mut self, _cmd: KvCommand) -> KvResponse {
        todo!("implement handle_command")
    }

    pub fn serialize_command(_cmd: &KvCommand) -> String {
        todo!("implement serialize_command")
    }

    pub fn deserialize_command(_data: &str) -> Result<KvCommand, serde_json::Error> {
        todo!("implement deserialize_command")
    }

    pub fn serialize_response(_resp: &KvResponse) -> String {
        todo!("implement serialize_response")
    }

    pub fn deserialize_response(_data: &str) -> Result<KvResponse, serde_json::Error> {
        todo!("implement deserialize_response")
    }

    pub fn become_leader(&mut self) {
        todo!("transition to leader role")
    }

    pub fn set_term(&mut self, _term: u64) {
        todo!("set current term")
    }

    pub fn apply_replicated(&mut self, _key: String, _value: String, _term: u64) -> KvResponse {
        todo!("implement apply_replicated")
    }
}
