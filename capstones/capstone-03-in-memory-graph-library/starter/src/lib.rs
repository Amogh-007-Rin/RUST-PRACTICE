//! Capstone 03: In-Memory Graph Library — starter scaffold.
//!
//! A reusable adjacency-list graph library. Every `TODO(capstone-03)`
//! below marks a piece of the public API you must implement so the
//! integration tests in `tests/capstone_03.rs` pass.
//!
//! The design uses **index-based node handles** (`usize`) instead of
//! `Rc<RefCell<Node>>`: nodes live in a `Vec`, and edges are lists of
//! indices. This keeps ownership simple (no cycles, no interior
//! mutability) and is how many real graph crates are built.

use std::collections::VecDeque;

/// A directed graph built on an adjacency list.
///
/// Nodes are identified by the `usize` handle returned from [`Graph::add_node`],
/// which doubles as an index into the internal storage. All methods that take
/// a node handle panic if the handle does not exist.
///
/// Fields are only read once you implement the TODO methods below.
#[allow(dead_code)]
pub struct Graph {
    nodes: Vec<String>,
    adjacency: Vec<Vec<usize>>,
}

impl Graph {
    /// Creates an empty graph with no nodes and no edges.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            adjacency: Vec::new(),
        }
    }

    /// Adds a node with the given `label`, returning its handle.
    ///
    /// Handles are assigned in increasing order starting at zero.
    pub fn add_node(&mut self, _label: &str) -> usize {
        // TODO(capstone-03): push the label onto `nodes`, push an empty
        // adjacency row, and return the new node's index.
        panic!("not implemented")
    }

    /// The number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        // TODO(capstone-03): the number of nodes.
        panic!("not implemented")
    }

    /// The number of directed edges in the graph.
    pub fn edge_count(&self) -> usize {
        // TODO(capstone-03): the sum of all adjacency-list lengths.
        panic!("not implemented")
    }

    /// The label of `node`.
    ///
    /// # Panics
    ///
    /// Panics if `node` is not a valid handle.
    pub fn label(&self, _node: usize) -> &str {
        // TODO(capstone-03): index into `nodes`.
        panic!("not implemented")
    }

    /// The direct out-neighbors of `node`, in insertion order.
    ///
    /// # Panics
    ///
    /// Panics if `node` is not a valid handle.
    pub fn neighbors(&self, _node: usize) -> &[usize] {
        // TODO(capstone-03): index into `adjacency`.
        panic!("not implemented")
    }

    /// Adds a directed edge `from -> to`.
    ///
    /// Duplicate edges are allowed and counted separately.
    ///
    /// # Panics
    ///
    /// Panics if either handle does not exist.
    pub fn add_edge(&mut self, _from: usize, _to: usize) {
        // TODO(capstone-03): validate both handles, then push `to` onto
        // `adjacency[from]`.
        panic!("not implemented")
    }

    /// Returns `true` if the graph contains a cycle.
    ///
    /// Uses Kahn's algorithm: repeatedly remove nodes with no remaining
    /// incoming edges; if any nodes are left over, they form a cycle.
    /// Self-loops count as cycles.
    pub fn has_cycle(&self) -> bool {
        // TODO(capstone-03): Kahn's algorithm.
        panic!("not implemented")
    }

    /// Returns a breadth-first iterator starting at `start`.
    ///
    /// Nodes are yielded in level order (all nodes one edge away before
    /// any node two edges away). Each node is yielded exactly once.
    ///
    /// # Panics
    ///
    /// Panics if `start` is not a valid handle.
    pub fn bfs(&self, _start: usize) -> Bfs<'_> {
        // TODO(capstone-03): validate `start`, then seed a `Bfs` with a
        // queue containing only `start`.
        panic!("not implemented")
    }

    /// Returns a depth-first iterator starting at `start`.
    ///
    /// Nodes are yielded in pre-order (a node before its descendants).
    /// Neighbors are pushed onto the stack in reverse order so that
    /// lower-indexed neighbors are visited first. Each node is yielded
    /// exactly once.
    ///
    /// # Panics
    ///
    /// Panics if `start` is not a valid handle.
    pub fn dfs(&self, _start: usize) -> Dfs<'_> {
        // TODO(capstone-03): validate `start`, then seed a `Dfs` with a
        // stack containing only `start`.
        panic!("not implemented")
    }

    /// Finds a shortest path (fewest edges) from `from` to `to`, or
    /// `None` if `to` is unreachable from `from`.
    ///
    /// The path includes both endpoints. A node is its own path to itself.
    ///
    /// # Panics
    ///
    /// Panics if either handle does not exist.
    pub fn find_path(&self, _from: usize, _to: usize) -> Option<Vec<usize>> {
        // TODO(capstone-03): breadth-first search recording a `prev` map,
        // then walk it back from `to` and reverse.
        panic!("not implemented")
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// A breadth-first iterator over a [`Graph`].
///
/// Created by [`Graph::bfs`]. The queue holds nodes scheduled for
/// visiting; `visited` records the nodes already yielded.
///
/// The fields are only read once you implement `Iterator::next`.
#[allow(dead_code)]
pub struct Bfs<'a> {
    graph: &'a Graph,
    visited: Vec<bool>,
    queue: VecDeque<usize>,
}

impl<'a> Iterator for Bfs<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        // TODO(capstone-03): pop from the front of the queue until you
        // find an unvisited node; mark it visited, enqueue all its
        // neighbors, and return it. Return `None` when the queue drains.
        panic!("not implemented")
    }
}

/// A depth-first iterator over a [`Graph`].
///
/// Created by [`Graph::dfs`]. The stack holds nodes scheduled for
/// visiting; `visited` records the nodes already yielded.
///
/// The fields are only read once you implement `Iterator::next`.
#[allow(dead_code)]
pub struct Dfs<'a> {
    graph: &'a Graph,
    visited: Vec<bool>,
    stack: Vec<usize>,
}

impl<'a> Iterator for Dfs<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        // TODO(capstone-03): pop from the stack until you find an
        // unvisited node; mark it visited, push its neighbors in
        // *reverse* order (so lower indices are visited first), and
        // return it. Return `None` when the stack drains.
        panic!("not implemented")
    }
}
