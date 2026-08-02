//! Module 077: integration tests.

use module_077_exercises::{partition_network, reconcile_partitions, run_leader_election, Node};

#[test]
fn test_bully_algorithm_highest_id_wins() {
    let mut nodes = vec![Node::new(1), Node::new(3), Node::new(2)];
    let leader = run_leader_election(&mut nodes);
    assert_eq!(leader.id, 3);
    assert!(leader.is_leader);
    assert_eq!(leader.term, 1);

    // No other node should be leader
    for node in &nodes {
        if node.id != 3 {
            assert!(!node.is_leader);
        }
    }
}

#[test]
fn test_bully_algorithm_single_node() {
    let mut nodes = vec![Node::new(42)];
    let leader = run_leader_election(&mut nodes);
    assert_eq!(leader.id, 42);
    assert!(leader.is_leader);
}

#[test]
fn test_leader_election_increments_term() {
    let mut nodes = vec![Node::new(1), Node::new(2)];
    let leader = run_leader_election(&mut nodes);
    assert_eq!(leader.term, 1);

    // Run again — same leader, but term should increment
    let leader2 = run_leader_election(&mut nodes);
    assert_eq!(leader2.id, 2);
    assert_eq!(leader2.term, 2);
}

#[test]
fn test_partition_removes_leader() {
    let mut nodes = vec![Node::new(1), Node::new(5), Node::new(3)];
    run_leader_election(&mut nodes);
    assert_eq!(nodes[1].is_leader, true); // node 5 is leader

    // Partition away node 5
    partition_network(&mut nodes, &[5]);
    assert!(!nodes[1].reachable);
    assert!(!nodes[1].is_leader);
}

#[test]
fn test_partition_does_not_affect_unpartioned_nodes() {
    let mut nodes = vec![Node::new(10), Node::new(20), Node::new(30)];
    run_leader_election(&mut nodes);
    partition_network(&mut nodes, &[10]);

    // Node 10 is partitioned
    let n0 = &nodes[0];
    assert!(!n0.reachable);
    assert!(!n0.is_leader);

    // Nodes 20 and 30 are still reachable
    assert!(nodes[1].reachable);
    assert!(nodes[2].reachable);
}

#[test]
fn test_reconcile_partitions_restores_and_elects() {
    let mut nodes = vec![Node::new(1), Node::new(3), Node::new(5), Node::new(7)];
    run_leader_election(&mut nodes);
    assert_eq!(nodes[3].id, 7);
    assert!(nodes[3].is_leader);

    // Partition away the leader (node 7)
    partition_network(&mut nodes, &[7]);
    assert!(!nodes[3].reachable);

    // Now reconcile — everything becomes reachable, leader elected
    reconcile_partitions(&mut nodes);

    for node in &nodes {
        assert!(node.reachable, "node {} should be reachable", node.id);
    }
    // Node 7 should be leader again (highest ID)
    let leader = nodes.iter().find(|n| n.is_leader).unwrap();
    assert_eq!(leader.id, 7);
}
