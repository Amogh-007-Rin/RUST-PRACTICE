//! Module 077: Distributed Systems Concepts I — reference solution.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub id: u64,
    pub is_leader: bool,
    pub term: u64,
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

/// Run a leader election among all reachable nodes using the bully algorithm:
/// the node with the highest ID becomes leader.
pub fn run_leader_election(nodes: &mut [Node]) -> &Node {
    let reachable: Vec<&mut Node> = nodes.iter_mut().filter(|n| n.reachable).collect();

    if reachable.is_empty() {
        panic!("no reachable nodes");
    }

    // Find the node with the highest ID
    let max_id = reachable.iter().map(|n| n.id).max().unwrap();

    for node in nodes.iter_mut() {
        if node.reachable {
            node.is_leader = node.id == max_id;
        }
    }

    // Increment the term on the leader
    if let Some(leader) = nodes.iter_mut().find(|n| n.is_leader) {
        leader.term += 1;
    }

    // Return a reference to the leader
    nodes.iter().find(|n| n.is_leader).unwrap()
}

/// Simulate a network partition: mark nodes in the partition as unreachable.
pub fn partition_network(nodes: &mut [Node], partition: &[u64]) {
    for &id in partition {
        if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
            node.reachable = false;
            node.is_leader = false;
        }
    }
}

/// After a network partition heals, mark all nodes reachable and run leader election.
pub fn reconcile_partitions(nodes: &mut [Node]) {
    for node in nodes.iter_mut() {
        node.reachable = true;
    }
    run_leader_election(nodes);
}
