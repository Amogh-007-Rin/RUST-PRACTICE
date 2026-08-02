//! Capstone 03: In-Memory Graph Library — reference solution.
//!
//! A reusable adjacency-list graph library. The design uses **index-based
//! node handles** (`usize`) instead of `Rc<RefCell<Node>>`: nodes live in a
//! `Vec`, and edges are lists of indices. This keeps ownership simple (no
//! cycles, no interior mutability) and is how many real graph crates are
//! built.

use std::collections::VecDeque;

/// A directed graph built on an adjacency list.
///
/// Nodes are identified by the `usize` handle returned from [`Graph::add_node`],
/// which doubles as an index into the internal storage. All methods that take
/// a node handle panic if the handle does not exist.
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
    pub fn add_node(&mut self, label: &str) -> usize {
        self.nodes.push(label.to_string());
        self.adjacency.push(Vec::new());
        self.nodes.len() - 1
    }

    /// The number of nodes in the graph.
    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    /// The number of directed edges in the graph.
    pub fn edge_count(&self) -> usize {
        self.adjacency.iter().map(Vec::len).sum()
    }

    /// The label of `node`.
    ///
    /// # Panics
    ///
    /// Panics if `node` is not a valid handle.
    pub fn label(&self, node: usize) -> &str {
        assert!(node < self.nodes.len(), "node {node} does not exist");
        &self.nodes[node]
    }

    /// The direct out-neighbors of `node`, in insertion order.
    ///
    /// # Panics
    ///
    /// Panics if `node` is not a valid handle.
    pub fn neighbors(&self, node: usize) -> &[usize] {
        assert!(node < self.nodes.len(), "node {node} does not exist");
        &self.adjacency[node]
    }

    /// Adds a directed edge `from -> to`.
    ///
    /// Duplicate edges are allowed and counted separately.
    ///
    /// # Panics
    ///
    /// Panics if either handle does not exist.
    pub fn add_edge(&mut self, from: usize, to: usize) {
        assert!(from < self.nodes.len(), "node {from} does not exist");
        assert!(to < self.nodes.len(), "node {to} does not exist");
        self.adjacency[from].push(to);
    }

    /// Returns `true` if the graph contains a cycle.
    ///
    /// Uses Kahn's algorithm: repeatedly remove nodes with no remaining
    /// incoming edges; if any nodes are left over, they form a cycle.
    /// Self-loops count as cycles.
    pub fn has_cycle(&self) -> bool {
        let n = self.nodes.len();
        let mut indegree = vec![0usize; n];
        for edges in &self.adjacency {
            for &to in edges {
                indegree[to] += 1;
            }
        }

        let mut queue: VecDeque<usize> = (0..n).filter(|&i| indegree[i] == 0).collect();
        let mut visited = 0usize;

        while let Some(node) = queue.pop_front() {
            visited += 1;
            for &to in &self.adjacency[node] {
                indegree[to] -= 1;
                if indegree[to] == 0 {
                    queue.push_back(to);
                }
            }
        }

        visited != n
    }

    /// Returns a breadth-first iterator starting at `start`.
    ///
    /// Nodes are yielded in level order (all nodes one edge away before
    /// any node two edges away). Each node is yielded exactly once.
    ///
    /// # Panics
    ///
    /// Panics if `start` is not a valid handle.
    pub fn bfs(&self, start: usize) -> Bfs<'_> {
        assert!(start < self.nodes.len(), "node {start} does not exist");
        Bfs {
            graph: self,
            visited: vec![false; self.nodes.len()],
            queue: VecDeque::from([start]),
        }
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
    pub fn dfs(&self, start: usize) -> Dfs<'_> {
        assert!(start < self.nodes.len(), "node {start} does not exist");
        Dfs {
            graph: self,
            visited: vec![false; self.nodes.len()],
            stack: vec![start],
        }
    }

    /// Finds a shortest path (fewest edges) from `from` to `to`, or
    /// `None` if `to` is unreachable from `from`.
    ///
    /// The path includes both endpoints. A node is its own path to itself.
    ///
    /// # Panics
    ///
    /// Panics if either handle does not exist.
    pub fn find_path(&self, from: usize, to: usize) -> Option<Vec<usize>> {
        let n = self.nodes.len();
        assert!(from < n, "node {from} does not exist");
        assert!(to < n, "node {to} does not exist");

        if from == to {
            return Some(vec![from]);
        }

        let mut prev = vec![None; n];
        let mut queue = VecDeque::from([from]);
        prev[from] = Some(from);

        while let Some(node) = queue.pop_front() {
            for &next in &self.adjacency[node] {
                if prev[next].is_none() {
                    prev[next] = Some(node);
                    if next == to {
                        return Some(reconstruct(prev, from, to));
                    }
                    queue.push_back(next);
                }
            }
        }

        None
    }
}

impl Default for Graph {
    fn default() -> Self {
        Self::new()
    }
}

/// Rebuilds the path `from -> ... -> to` from a parent map.
fn reconstruct(prev: Vec<Option<usize>>, from: usize, to: usize) -> Vec<usize> {
    let mut path = vec![to];
    let mut current = to;
    while current != from {
        current = prev[current].expect("every visited node has a parent");
        path.push(current);
    }
    path.reverse();
    path
}

/// A breadth-first iterator over a [`Graph`].
///
/// Created by [`Graph::bfs`]. The queue holds nodes scheduled for
/// visiting; `visited` records the nodes already yielded.
pub struct Bfs<'a> {
    graph: &'a Graph,
    visited: Vec<bool>,
    queue: VecDeque<usize>,
}

impl<'a> Iterator for Bfs<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while let Some(node) = self.queue.pop_front() {
            if !self.visited[node] {
                self.visited[node] = true;
                for &neighbor in &self.graph.adjacency[node] {
                    self.queue.push_back(neighbor);
                }
                return Some(node);
            }
        }
        None
    }
}

/// A depth-first iterator over a [`Graph`].
///
/// Created by [`Graph::dfs`]. The stack holds nodes scheduled for
/// visiting; `visited` records the nodes already yielded.
pub struct Dfs<'a> {
    graph: &'a Graph,
    visited: Vec<bool>,
    stack: Vec<usize>,
}

impl<'a> Iterator for Dfs<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<usize> {
        while let Some(node) = self.stack.pop() {
            if !self.visited[node] {
                self.visited[node] = true;
                for &neighbor in self.graph.adjacency[node].iter().rev() {
                    self.stack.push(neighbor);
                }
                return Some(node);
            }
        }
        None
    }
}
