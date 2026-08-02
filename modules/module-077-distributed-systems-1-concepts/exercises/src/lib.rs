//! Module 077: Distributed Systems Concepts I — exercise scaffold.
//!
//! Implement leader election (bully algorithm), network partitioning, and partition reconciliation.

/// A node in a distributed system.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: u64,
    pub is_leader: bool,
    pub term: u64,
    /// Whether this node is currently reachable on the network.
    pub reachable: bool,
}

impl Node {
    pub fn new(id: u64) -> Self {
        Self {
            id,
            is_leader: false,
            term: 0,
            reachable: true,
        }
    }
}

/// Run a leader election among all *reachable* nodes using the bully algorithm:
/// the node with the highest ID becomes leader.
///
/// Return a reference to the newly elected leader.
pub fn run_leader_election(_nodes: &mut [Node]) -> &Node {
    // TODO(module-077): find the reachable node with the highest ID.
    // Set its `is_leader = true` and increment its `term`.
    // Set `is_leader = false` for all other reachable nodes.
    // Return a reference to the new leader.
    // If no nodes are reachable, panic with "no reachable nodes".
    panic!("TODO(module-077): implement run_leader_election")
}

/// Simulate a network partition: mark nodes whose IDs are in `partition` as
/// unreachable. If any of those nodes was the leader, clear its leader status.
pub fn partition_network(_nodes: &mut [Node], _partition: &[u64]) {
    // TODO(module-077): for each node in the partition list,
    // find the matching node and set reachable = false.
    // If it was the leader, set is_leader = false.
    panic!("TODO(module-077): implement partition_network")
}

/// After a network partition heals, run leader election across the now-reachable nodes.
/// First, mark all nodes as reachable, then run leader election.
pub fn reconcile_partitions(_nodes: &mut [Node]) {
    // TODO(module-077): mark all nodes as reachable,
    // then call run_leader_election.
    panic!("TODO(module-077): implement reconcile_partitions")
}
