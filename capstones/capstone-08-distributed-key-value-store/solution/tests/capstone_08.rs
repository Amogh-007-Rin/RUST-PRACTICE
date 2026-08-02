use capstone_08_solution::{KvCommand, KvResponse, KvStore, NodeRole};

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
fn test_response_serialization_roundtrip() {
    let responses = vec![
        KvResponse::Ok,
        KvResponse::Value { value: None },
        KvResponse::Value {
            value: Some("hello".into()),
        },
        KvResponse::Keys {
            keys: vec!["a".into(), "b".into()],
        },
        KvResponse::Error {
            message: "oops".into(),
        },
    ];

    for resp in responses {
        let wire = KvStore::serialize_response(&resp);
        let parsed = KvStore::deserialize_response(&wire).expect("deserialize should succeed");
        assert_eq!(resp, parsed);
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
    store.set_term(5);
    let resp = store.handle_command(KvCommand::Replicate {
        key: "k".into(),
        value: "v".into(),
        term: 3,
    });
    assert!(matches!(resp, KvResponse::Error { .. }));
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
