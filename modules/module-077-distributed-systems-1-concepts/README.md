# Module 077: Distributed Systems Concepts I

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 031 (Threads), Module 074 (Raw Networking)

## Learning Objectives
- Understand the CAP theorem and its implications for distributed system design
- Implement the bully algorithm for leader election
- Simulate network partitions and their effect on cluster leadership
- Reconcile a cluster after a network partition heals

## Why This Matters
Every production distributed system — Kafka, etcd, CockroachDB, TiKV — must handle node failures, network partitions, and leader election. The fundamental patterns (CAP trade-offs, consensus, leader election) appear in every distributed database you'll work with. Understanding the bully algorithm and partition handling in code makes the theory concrete: it's not just a whiteboard concept — it's a few dozen lines of Rust.

## Concept

Distributed systems are programs that run on multiple computers (nodes) that communicate over a network. The big challenge: the network can fail, nodes can crash, and messages can be delayed. A distributed system must continue operating despite these failures.

### The CAP theorem

The CAP theorem states that a distributed data store can provide at most two of these three guarantees:

| Guarantee | Meaning |
|-----------|---------|
| **C**onsistency | Every read receives the most recent write |
| **A**vailability | Every request receives a (non-error) response |
| **P**artition tolerance | The system continues operating despite network partitions |

Since network partitions are inevitable (cables get cut, switches fail), you must choose between **C** (CP system — sacrifice availability during partitions) or **A** (AP system — allow stale reads). No real system achieves all three simultaneously.

**CP example (etcd/ZooKeeper):** during a partition, the minority side refuses writes. Consistency is preserved, but availability is lost for the minority.

**AP example (Cassandra/DynamoDB):** all nodes accept writes during a partition. When the partition heals, conflicts are resolved (typically last-write-wins).

Rust's role: CP systems are often implemented in Rust (TiKV, a CP key-value store, is Rust) because they need strong correctness guarantees and the borrow checker helps enforce them.

### Consensus and leader election

In a CP system, nodes must agree on a single source of truth. **Consensus** algorithms (Raft, Paxos) elect a **leader** that coordinates writes. If the leader fails, a new leader is elected.

The simplest leader election algorithm is the **bully algorithm**:

1. When a node detects the leader is down, it calls an election
2. It sends an "election" message to all nodes with higher IDs
3. If no higher-ID node responds, it becomes the leader
4. If a higher-ID node responds, that node calls its own election

Implemented as pure logic:

```rust
#[derive(Debug, Clone)]
struct Node {
    id: u64,
    is_leader: bool,
    term: u64,
    reachable: bool,
}

fn run_leader_election(nodes: &mut [Node]) -> &Node {
    let reachable: Vec<_> = nodes.iter().filter(|n| n.reachable).collect();
    let max_id = reachable.iter().map(|n| n.id).max().unwrap();

    for node in nodes.iter_mut() {
        if node.reachable {
            node.is_leader = node.id == max_id;
        }
    }

    let leader = nodes.iter_mut().find(|n| n.is_leader).unwrap();
    leader.term += 1;
    leader
}
```

Key details:
- Only reachable nodes vote. Partitioned nodes cannot participate.
- The **term** is a monotonically increasing number. Every new election increments it — this prevents a stale leader from issuing commands after a new leader is elected.
- Highest ID wins. In real systems, IDs might encode node age or data freshness.

### Network partitions

A **network partition** occurs when some nodes can't communicate with others. This is simulated by setting `reachable = false` on affected nodes:

```rust
fn partition_network(nodes: &mut [Node], partition: &[u64]) {
    for id in partition {
        if let Some(node) = nodes.iter_mut().find(|n| n.id == *id) {
            node.reachable = false;
            node.is_leader = false; // can't lead if isolated
        }
    }
}
```

During a partition, the system typically has two groups: a majority partition (which can elect a new leader) and a minority partition (which cannot, in a CP system).

### Split-brain and partition reconciliation

**Split-brain** happens when both sides of a partition elect their own leaders, leading to conflicting writes. Raft and similar algorithms prevent this by requiring a majority quorum for leader election — a minority partition can't elect a leader.

When the partition heals:

```rust
fn reconcile_partitions(nodes: &mut [Node]) {
    for node in nodes.iter_mut() {
        node.reachable = true;
    }
    run_leader_election(nodes); // re-elect the highest-ID node
}
```

In a real system (Raft), reconciliation is more complex: the old leader from the majority partition may have committed entries that need replicating to the previously partitioned nodes. But the principle is the same: all nodes become reachable, a single leader emerges, and the cluster heals.

### The bully algorithm in practice

The bully algorithm is simple but has flaws (it doesn't handle message delays well, and is O(n) per election). Production systems use Raft or Multi-Paxos. However, the bully algorithm teaches the core concepts — leader election, terms, reachability — that underpin those more sophisticated algorithms.

```
Before election:        After election:

Node 1 (id=1)  ✓        Node 1  leader=false
Node 3 (id=3)  ✓   →    Node 3  leader=true ← highest reachable ID
Node 2 (id=2)  ✗        Node 2  unreachable
```

## Common Pitfalls
- **Not checking reachability during election**: partitioned nodes should not participate. Otherwise you get multiple leaders (split-brain).
- **Forgetting to increment the term**: if the term doesn't change, a stale leader can pretend it still has authority.
- **Assuming perfect network**: every distributed system must handle partitions. Design for it from the start.
- **Using node ID as the only criterion in real systems**: real leader election considers log completeness (Raft's last-log-index), not just node ID.

## Key Terms
- **CAP theorem**: a distributed store can provide at most 2 of Consistency, Availability, Partition-tolerance
- **CP system**: prioritizes consistency over availability (etcd, TiKV, ZooKeeper)
- **AP system**: prioritizes availability over consistency (Cassandra, DynamoDB)
- **Consensus**: agreement among distributed nodes on a single value or leader
- **Bully algorithm**: leader election where the highest-ID reachable node wins
- **Term**: a monotonically increasing epoch number, incremented on each election
- **Split-brain**: two leaders existing simultaneously due to a partition
- **Network partition**: a subset of nodes that can only communicate with each other, not the rest

## Exercise

In `exercises/`, fill in the `TODO(module-077)` markers to:

1. **`run_leader_election`** — implement the bully algorithm (highest reachable ID wins)
2. **`partition_network`** — mark nodes as unreachable, clear leader status if partitioned
3. **`reconcile_partitions`** — restore all nodes to reachable and re-elect

Run `cargo test -p module-077-exercises` to verify.

## Further Reading
- [CAP theorem (Brewer, 2000)](https://dl.acm.org/doi/10.5555/343477.343502)
- [Raft Consensus Algorithm](https://raft.github.io/)
- [The bully algorithm (Garcia-Molina, 1982)](https://en.wikipedia.org/wiki/Bully_algorithm)
