use capstone_03_solution::Graph;

fn diamond() -> Graph {
    let mut g = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    let d = g.add_node("d");
    g.add_edge(a, b);
    g.add_edge(a, c);
    g.add_edge(b, d);
    g.add_edge(c, d);
    g
}

#[test]
fn add_node_returns_increasing_handles() {
    let mut g = Graph::new();
    assert_eq!(g.add_node("x"), 0);
    assert_eq!(g.add_node("y"), 1);
    assert_eq!(g.node_count(), 2);
    assert_eq!(g.label(0), "x");
    assert_eq!(g.label(1), "y");
}

#[test]
fn add_edge_records_neighbors() {
    let mut g = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b);
    g.add_edge(a, c);
    assert_eq!(g.neighbors(a), &[1, 2]);
    assert!(g.neighbors(b).is_empty());
    assert_eq!(g.edge_count(), 2);
}

#[test]
#[should_panic(expected = "node 5 does not exist")]
fn add_edge_panics_on_invalid_target() {
    let mut g = Graph::new();
    g.add_node("a");
    g.add_edge(0, 5);
}

#[test]
#[should_panic(expected = "node 9 does not exist")]
fn add_edge_panics_on_invalid_source() {
    let mut g = Graph::new();
    g.add_node("a");
    g.add_edge(9, 0);
}

#[test]
fn bfs_visits_in_level_order() {
    let g = diamond();
    assert_eq!(g.bfs(0).collect::<Vec<_>>(), vec![0, 1, 2, 3]);
}

#[test]
fn bfs_from_isolated_node_visits_only_it() {
    let mut g = Graph::new();
    g.add_node("a");
    g.add_node("b");
    let isolated = g.add_node("c");
    g.add_edge(0, 1);
    assert_eq!(g.bfs(isolated).collect::<Vec<_>>(), vec![2]);
}

#[test]
fn bfs_next_driven_by_hand() {
    let g = diamond();
    let mut it = g.bfs(0);
    assert_eq!(it.next(), Some(0));
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
}

#[test]
fn dfs_visits_in_preorder() {
    let g = diamond();
    assert_eq!(g.dfs(0).collect::<Vec<_>>(), vec![0, 1, 3, 2]);
}

#[test]
fn dfs_from_isolated_node_visits_only_it() {
    let mut g = Graph::new();
    g.add_node("a");
    g.add_node("b");
    let isolated = g.add_node("c");
    g.add_edge(0, 1);
    assert_eq!(g.dfs(isolated).collect::<Vec<_>>(), vec![2]);
}

#[test]
fn cycle_detection_rejects_acyclic_graphs() {
    assert!(!diamond().has_cycle());
}

#[test]
fn cycle_detection_finds_cycles() {
    let mut g = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    let c = g.add_node("c");
    g.add_edge(a, b);
    g.add_edge(b, c);
    g.add_edge(c, a);
    assert!(g.has_cycle());
}

#[test]
fn cycle_detection_finds_self_loops() {
    let mut g = Graph::new();
    let a = g.add_node("a");
    g.add_edge(a, a);
    assert!(g.has_cycle());
}

#[test]
fn cycle_detection_ignores_disconnected_components() {
    let mut g = Graph::new();
    let a = g.add_node("a");
    let b = g.add_node("b");
    g.add_edge(a, b);
    let c = g.add_node("c");
    let d = g.add_node("d");
    g.add_edge(c, d);
    assert!(!g.has_cycle());
}

#[test]
fn find_path_returns_a_shortest_path() {
    let g = diamond();
    assert_eq!(g.find_path(0, 3), Some(vec![0, 1, 3]));
}

#[test]
fn find_path_is_none_for_unreachable_targets() {
    let g = diamond();
    assert_eq!(g.find_path(3, 0), None);
}

#[test]
fn find_path_from_node_to_itself() {
    let g = diamond();
    assert_eq!(g.find_path(2, 2), Some(vec![2]));
}

#[test]
fn find_path_is_none_between_disconnected_components() {
    let mut g = Graph::new();
    g.add_node("a");
    g.add_node("b");
    let isolated = g.add_node("c");
    g.add_edge(0, 1);
    assert_eq!(g.find_path(0, isolated), None);
}
