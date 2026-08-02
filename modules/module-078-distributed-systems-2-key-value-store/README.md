# Module 078: Distributed Systems Concepts II

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 077 (Distributed Systems Concepts I)

## Learning Objectives
- Build a minimal single-leader replicated key-value store
- Understand the write-ahead log pattern and its role in replication
- Implement log-based replication from leader to follower
- Apply a sequence of log entries to bring a follower up to date

## Why This Matters
Single-leader replication is the backbone of Postgres read replicas, MySQL replication, MongoDB replica sets, Kafka partitions, and Raft-based stores like etcd and TiKV. The pattern — leader accepts writes, appends to a log, followers consume the log — is universal. Understanding it at the Rust level prepares you to work with any replicated database and to build custom replication logic when needed.

## Concept

Replication means keeping copies of data on multiple nodes. The goal: if one node fails, another can serve requests without data loss. The simplest approach is **single-leader replication** (also called primary-backup or master-slave).

### The write path

```
Client → Leader → Log entry → Forward to Followers → Acknowledge
```

1. The client sends a write to the leader
2. The leader writes the data locally and appends a **log entry** (write-ahead log / WAL)
3. The leader forwards the log entry to all followers
4. Followers apply the log entry to their local state
5. The leader acknowledges the write to the client once a quorum of followers confirms

### The data model

Our key-value store has three pieces:

```rust
#[derive(Debug, Clone)]
enum NodeRole {
    Leader,
    Follower,
}

#[derive(Debug, Clone)]
struct LogEntry {
    index: u64,        // monotonically increasing
    key: String,       // the key affected
    value: Option<String>, // Some(v) = write, None = delete
}

#[derive(Debug, Clone)]
struct KvStore {
    data: HashMap<String, String>,
    role: NodeRole,
    log: Vec<LogEntry>,
}
```

The `log` is an ordered sequence of all mutations. It serves as the authoritative record of state changes — if the in-memory `data` is lost, you can replay the log to reconstruct it. This is the same pattern used by every replicated database.

### Writing on the leader

When the leader receives a write:

```rust
impl KvStore {
    fn write(&mut self, key: &str, value: &str) {
        if self.role != NodeRole::Leader {
            return; // followers don't accept direct writes
        }
        self.data.insert(key.to_string(), value.to_string());
        let index = self.log.len() as u64 + 1;
        self.log.push(LogEntry {
            index,
            key: key.to_string(),
            value: Some(value.to_string()),
        });
    }

    fn delete(&mut self, key: &str) {
        if self.role != NodeRole::Leader {
            return;
        }
        self.data.remove(key);
        let index = self.log.len() as u64 + 1;
        self.log.push(LogEntry {
            index,
            key: key.to_string(),
            value: None,  // None means "delete"
        });
    }
}
```

Notice the `value: Option<String>` in `LogEntry`: `Some(v)` is a write, `None` is a delete. This unified representation is called **operation-based replication** — each log entry is an operation, not the full state.

### Replicating to followers

Replication happens in two steps:

**Step 1: The leader sends log entries to followers.** In a real system this is done via network (gRPC, TCP). For our exercise, we simulate it with direct function calls:

```rust
fn replicate_to_follower(leader: &KvStore, follower: &mut KvStore, key: &str) {
    match leader.data.get(key) {
        Some(value) => {
            follower.data.insert(key.to_string(), value.clone());
        }
        None => {
            follower.data.remove(key);
        }
    }
}
```

This is a simplified **state transfer** — we copy the current value rather than replaying the log. Real systems typically use **log shipping**: the leader sends log entries, and the follower replays them independently. Our function models the end result.

**Step 2: The follower applies log entries.** When a follower receives log entries (either from the leader or from a snapshot), it applies them:

```rust
fn apply_log_entries(store: &mut KvStore) {
    for entry in store.log.drain(..) {
        match entry.value {
            Some(v) => { store.data.insert(entry.key, v); }
            None => { store.data.remove(&entry.key); }
        }
    }
}
```

`drain(..)` consumes the vec — after applying, the log is empty. In a production system, you'd truncate the log up to a committed index (Raft's `commit_index`) rather than clearing the entire log.

### Why a log?

The log provides:

- **Durability:** if the in-memory `data` is lost, replay the log to rebuild it
- **Ordering:** all nodes apply entries in the same order → deterministic, consistent state
- **Audit trail:** you can examine the log to understand what happened and when
- **Incremental replication:** send only new entries since the follower's last known index

### The leader-follower lifecycle

```
1. Node starts as Follower
2. No leader heartbeat → timeout → starts election (Module 077)
3. Wins election → becomes Leader, increments term
4. Accepts writes, appends to log, replicates to followers
5. Leader crashes → followers detect heartbeat timeout → new election
6. Old leader recovers → discovers higher term → steps down to Follower
```

Steps 2 and 6 involve the term from Module 077. A node with a stale term cannot act as leader — followers reject its commands.

### State machine replication

The pattern we've built is **state machine replication**: each node has the same initial state, applies the same sequence of operations, and therefore reaches the same final state. This is the theoretical foundation of Raft, Paxos, and blockchain consensus.

```
Node A:  [] --op1--> [k1=v1] --op2--> [k1=v1, k2=v2] --op3--> [k1=v1]
Node B:  [] --op1--> [k1=v1] --op2--> [k1=v1, k2=v2] --op3--> [k1=v1]
```

Both nodes started empty, applied `op1, op2, op3` in the same order, and ended up with identical state. Determinism guarantees consistency.

## Common Pitfalls
- **Followers accepting direct writes**: only the leader should accept writes. Otherwise you get conflicting data. Our `write` and `delete` methods guard against this.
- **Not draining the log after applying**: if you leave entries in the log, you'll double-apply them next time.
- **Log index not monotonic**: gaps or duplicates in indexes cause confusion during crash recovery. Always use the next sequential number.
- **Mixing state transfer and log shipping**: in production, pick one approach. Mixing them leads to inconsistent state when there are race conditions.

## Key Terms
- **Single-leader replication**: one node accepts writes, others follow (also called primary-backup)
- **Write-ahead log (WAL)**: an ordered, append-only record of all mutations applied before updating state
- **Log entry**: a single mutation record (index + operation)
- **State machine replication**: applying the same sequence of deterministic operations to identical starting states guarantees identical final states
- **State transfer**: replicating the entire current state (our simplified model)
- **Log shipping**: replicating individual log entries for the follower to apply

## Exercise

In `exercises/`, fill in the `TODO(module-078)` markers to:

1. **`KvStore::write`** — insert data and append a `LogEntry` (leader only)
2. **`KvStore::delete`** — remove data and append a `LogEntry` with `value: None` (leader only)
3. **`replicate_to_follower`** — copy or remove a key on the follower based on the leader's data
4. **`apply_log_entries`** — replay all pending log entries and clear the log

Run `cargo test -p module-078-exercises` to verify.

## Further Reading
- [Raft paper (Ongaro, 2014)](https://raft.github.io/raft.pdf)
- [Designing Data-Intensive Applications, Chapter 5 (Replication)](https://dataintensive.net/)
- [Why is database replication important? (CockroachDB blog)](https://www.cockroachlabs.com/blog/what-is-database-replication/)
