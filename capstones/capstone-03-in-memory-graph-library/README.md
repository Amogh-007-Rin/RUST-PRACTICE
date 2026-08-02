# Capstone 03: In-Memory Graph Library

**Covers modules:** 021–029
**Estimated time:** 4-6 hours

## Project Brief

You're building a reusable graph library crate — the kind of thing a routing engine, a build-system dependency resolver, or a social-network analysis tool would depend on. It stores nodes (with labels) and directed edges in memory, and exposes the operations every graph consumer needs: traversal in breadth-first and depth-first order, cycle detection, and path finding. The twist is that this is a *library*: the public API is your product, so it gets real doc comments and returns custom iterators instead of ad-hoc result vectors. This is a take-home-assignment-shaped artifact: a small, complete, documented crate that exercises everything from Block C — closures and iterators (021–023), pattern matching and traits (024–026), and the ownership discipline of smart pointers (028–029).

## Requirements

1. **`Graph::new()`** creates an empty graph.
2. **`add_node(&mut self, label: &str) -> usize`** adds a node and returns its handle; handles start at 0 and increase by one per node.
3. **`add_edge(&mut self, from: usize, to: usize)`** adds a directed edge; invalid handles panic with a clear message.
4. **`neighbors(&self, node: usize) -> &[usize]`** returns the out-neighbors of a node in insertion order.
5. **`bfs(&self, start: usize) -> Bfs<'_>`** returns a custom iterator that yields nodes in breadth-first (level) order, each exactly once.
6. **`dfs(&self, start: usize) -> Dfs<'_>`** returns a custom iterator that yields nodes in depth-first pre-order, each exactly once.
7. **`has_cycle(&self) -> bool`** detects cycles, including self-loops, on graphs with multiple disconnected components.
8. **`find_path(&self, from, to) -> Option<Vec<usize>>`** returns a shortest path (fewest edges) including both endpoints, or `None` when unreachable.
9. **Storage is adjacency-list-based** — a `Vec` of nodes plus a `Vec<Vec<usize>>` of edges per node.
10. **Every public item is documented** with `///` doc comments stating behavior and panic conditions.

## Stretch Goals

- **Weighted edges + Dijkstra.** Extend `add_edge` (or add `add_weighted_edge`) with an `i64`/`u32` weight per edge and implement `shortest_path` using Dijkstra's algorithm. The BFS-based `find_path` becomes the unweighted special case.
- **Topological sort.** For acyclic graphs, return a linear ordering of all nodes (`None` if the graph has a cycle). Kahn's algorithm you used for `has_cycle` almost does this already — return the removal order instead of a boolean.
- **`IntoIterator` for `&Graph`.** Implement `IntoIterator for &Graph` yielding node handles, so `for node in &graph` works — a natural companion to the custom `Bfs`/`Dfs` iterators.
- **Reachability queries.** A `reachable(from, to) -> bool` that reuses the BFS machinery instead of allocating a full path.

## Acceptance Criteria

Checklist mirrored by the tests in `starter/tests/capstone_03.rs` (and identical in `solution/tests/`):

- [ ] `cargo test -p capstone-03-starter` passes (17 tests): construction and handles, neighbor and edge accounting, panic behavior on invalid handles, BFS/DFS visit order (including manual `next()` driving and isolated nodes), cycle detection on acyclic/cyclic/self-loop/disconnected graphs, and path finding (found, unfound, self, cross-component).
- [ ] `cargo clippy -p capstone-03-starter -- -D warnings` is clean.
- [ ] Every public item has a doc comment; methods with panic behavior document it under a `# Panics` section.
- [ ] Duplicate edges and self-loops don't corrupt traversal or cycle detection.

## Design Notes / Hints

- **Node handles over `Rc<RefCell<Node>>`.** The tempting graph design is `Rc<RefCell<Node>>` with `Vec<Rc<RefCell<Node>>>` children (Module 029) — it's also a trap: cycles leak (refcounts never reach zero) and every access needs a borrow. Prefer **index-based handles**: nodes live in a `Vec`, an edge is just a `usize` in a neighbor list, and `&mut Graph` is the only mutable access needed. This is simpler, faster, and how `petgraph` — the de-facto real-world graph crate — works. Module 029's `Rc`/`RefCell` still applies: it's the tool for *shared mutable* state, which a `&mut Graph` API doesn't need.
- **Custom iterators over result vectors.** Requirements 5–6 demand real `Iterator` implementations (Module 022), not `bfs()` returning a `Vec`. The iterator struct holds `&'a Graph`, a `Vec<bool>` "visited" map, and a work list (`VecDeque` for BFS, `Vec` stack for DFS); implement `next()` to pop, skip already-visited nodes, expand neighbors, and return the node. Mark visited when *yielding*, not when enqueuing — it makes duplicate-edge graphs harmless.
- **Deterministic order.** Your tests assert exact visit orders, so decide (and document) the tie-breaking: BFS visits neighbors in insertion order; DFS pushes neighbors in *reverse* order so lower-indexed neighbors are explored first. `find_path` should return the path discovered by BFS parent pointers — on the diamond graph, `find_path(0, 3)` is `[0, 1, 3]`.
- **Cycle detection with Kahn's algorithm** (Module 011–012 collections + a `VecDeque`): count in-degrees, repeatedly pop nodes with in-degree 0, decrement their neighbors; if you process fewer nodes than exist, a cycle remains. Handles self-loops and multi-edges correctly for free.
- **Pattern matching (Module 024) is light here** — this capstone is mostly iterators and ownership — but `find_path`'s parent-map walk is a natural place for `let Some(...) = ... else` and while-let style.
- **`#[must_use]` and `#[allow(dead_code)]`:** the starter's fields are only read once you implement the TODOs; the scaffold marks that with `#[allow(dead_code)]`, which the solution removes.
