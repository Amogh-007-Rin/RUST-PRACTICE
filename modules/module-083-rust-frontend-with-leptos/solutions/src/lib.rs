//! Module 083: Rust frontend with Leptos — reference solution.
//!
//! Compiling a real Leptos view is a wasm-target workflow, so this crate
//! implements the *engine* Leptos is built on — reactive signals, memos,
//! effects, scopes and cleanup — in pure `std`, fully testable with
//! `cargo test` on the host. Every UI you will ever write with Leptos is a
//! thin layer over exactly these primitives.

use std::any::Any;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::marker::PhantomData;

/// A memo's recompute function.
type ComputeFn = Box<dyn FnMut(&mut Runtime) -> Box<dyn Any>>;
/// An effect's body.
type EffectFn = Box<dyn FnMut(&mut Runtime)>;
/// A registered cleanup function.
type CleanupFn = Box<dyn FnMut()>;

/// Identifies a reactive node (signal, memo, or effect) inside the runtime.
pub type NodeId = usize;

/// Identifies a scope in the runtime's owner tree.
pub type ScopeId = usize;

/// What kind of reactive node a slot holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeKind {
    Signal,
    Memo,
    Effect,
}

/// One slot in the runtime's node registry.
struct Node {
    kind: NodeKind,
    /// Current value for signals and memos (type-erased).
    value: Option<Box<dyn Any>>,
    /// Recompute function for memos.
    compute: Option<ComputeFn>,
    /// Body for effects.
    effect: Option<EffectFn>,
    /// Registered via `on_cleanup`, run before reruns and on dispose.
    cleanup: Option<CleanupFn>,
    /// Nodes that depend on this node (reverse edges, used for invalidation).
    dependents: Vec<NodeId>,
    /// Nodes this node read last time it ran (refreshed on every run).
    dependencies: Vec<NodeId>,
    /// Set when the value may have changed and needs recompute/rerun.
    dirty: bool,
    /// The scope this node was created in.
    scope: Option<ScopeId>,
}

/// Bookkeeping for one owner scope.
#[derive(Debug, Clone)]
struct ScopeRecord {
    parent: Option<ScopeId>,
    nodes: Vec<NodeId>,
}

/// The reactive runtime. One instance owns all signals/memos/effects and
/// drives propagation. Everything is deterministic and single-threaded, like
/// a Leptos app running in a browser tab.
pub struct Runtime {
    nodes: Vec<Option<Node>>,
    free: Vec<NodeId>,
    dirty_queue: VecDeque<NodeId>,
    current_runner: Option<NodeId>,
    batch_depth: usize,
    scope_stack: Vec<ScopeId>,
    scopes: HashMap<ScopeId, ScopeRecord>,
    next_scope: ScopeId,
}

impl Runtime {
    /// Creates an empty runtime.
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            free: Vec::new(),
            dirty_queue: VecDeque::new(),
            current_runner: None,
            batch_depth: 0,
            scope_stack: Vec::new(),
            scopes: HashMap::new(),
            next_scope: 0,
        }
    }

    /// Allocates a node slot, records its creation scope, and returns its id.
    fn alloc_node(
        &mut self,
        kind: NodeKind,
        compute: Option<ComputeFn>,
        effect: Option<EffectFn>,
        value: Option<Box<dyn Any>>,
    ) -> NodeId {
        let id = match self.free.pop() {
            Some(id) => {
                self.nodes[id] = Some(Node {
                    kind,
                    value,
                    compute,
                    effect,
                    cleanup: None,
                    dependents: Vec::new(),
                    dependencies: Vec::new(),
                    dirty: false,
                    scope: None,
                });
                id
            }
            None => {
                self.nodes.push(Some(Node {
                    kind,
                    value,
                    compute,
                    effect,
                    cleanup: None,
                    dependents: Vec::new(),
                    dependencies: Vec::new(),
                    dirty: false,
                    scope: None,
                }));
                self.nodes.len() - 1
            }
        };
        if let Some(scope) = self.scope_stack.last().copied() {
            if let Some(record) = self.scopes.get_mut(&scope) {
                record.nodes.push(id);
            }
            if let Some(Some(node)) = self.nodes.get_mut(id) {
                node.scope = Some(scope);
            }
        }
        id
    }

    /// Records `current_runner -> to` in the reactive graph, if a runner is
    /// active. Reading a signal inside an effect or memo is what creates the
    /// edge that later triggers invalidation.
    fn record_edge(&mut self, to: NodeId) {
        let Some(from) = self.current_runner else {
            return;
        };
        if from == to {
            return;
        }
        if let Some(Some(runner)) = self.nodes.get_mut(from) {
            if !runner.dependencies.contains(&to) {
                runner.dependencies.push(to);
            }
        }
        if let Some(Some(dep)) = self.nodes.get_mut(to) {
            if !dep.dependents.contains(&from) {
                dep.dependents.push(from);
            }
        }
    }

    /// Removes all incoming edges of `id` and drops `id` from its
    /// dependencies' `dependents` lists. Called before every recompute/rerun
    /// so the graph reflects only what the node *actually* read this time.
    fn clear_incoming_edges(&mut self, id: NodeId) {
        let deps = match self.nodes.get(id) {
            Some(Some(node)) => node.dependencies.clone(),
            _ => return,
        };
        for dep in deps {
            if let Some(Some(d)) = self.nodes.get_mut(dep) {
                d.dependents.retain(|&x| x != id);
            }
        }
        if let Some(Some(node)) = self.nodes.get_mut(id) {
            node.dependencies.clear();
        }
    }

    /// Creates a signal pair. `Signal` reads the current value; `WriteSignal`
    /// pushes a new value into the reactive graph.
    pub fn create_signal<T: Clone + 'static>(&mut self, value: T) -> (Signal<T>, WriteSignal<T>) {
        let id = self.alloc_node(NodeKind::Signal, None, None, Some(Box::new(value)));
        (
            Signal {
                id,
                _marker: PhantomData,
            },
            WriteSignal {
                id,
                _marker: PhantomData,
            },
        )
    }

    /// Reads a signal's current value.
    ///
    /// Called inside an effect or memo, this also subscribes the caller to
    /// future changes.
    pub fn get<T: Clone + 'static>(&mut self, sig: Signal<T>) -> T {
        self.record_edge(sig.id);
        self.nodes[sig.id]
            .as_ref()
            .expect("signal has been disposed")
            .value
            .as_ref()
            .expect("signal has no value")
            .downcast_ref::<T>()
            .expect("signal value type mismatch")
            .clone()
    }

    /// Writes a new value and propagates the change.
    ///
    /// Without an active `batch`, propagation (recompute memos, rerun
    /// effects) happens immediately.
    pub fn set<T: Clone + 'static>(&mut self, write: WriteSignal<T>, value: T) {
        if let Some(Some(node)) = self.nodes.get_mut(write.id) {
            node.value = Some(Box::new(value));
        }
        self.mark_dirty(write.id);
        if self.batch_depth == 0 {
            self.flush();
        }
    }

    /// Mutates a signal's value in place via a closure.
    pub fn update<T: Clone + 'static>(&mut self, write: WriteSignal<T>, f: impl FnOnce(&mut T)) {
        if let Some(Some(node)) = self.nodes.get_mut(write.id) {
            if let Some(value) = node.value.as_mut() {
                if let Some(value) = value.downcast_mut::<T>() {
                    f(value);
                }
            }
        }
        self.mark_dirty(write.id);
        if self.batch_depth == 0 {
            self.flush();
        }
    }

    /// Creates a memo: a value derived from signals, recomputed lazily and
    /// only when one of its dependencies changed.
    pub fn create_memo<T: Clone + 'static>(
        &mut self,
        mut f: impl FnMut(&mut Runtime) -> T + 'static,
    ) -> Memo<T> {
        let id = self.alloc_node(
            NodeKind::Memo,
            Some(Box::new(move |rt: &mut Runtime| -> Box<dyn Any> {
                Box::new(f(rt))
            })),
            None,
            None,
        );
        if let Some(Some(node)) = self.nodes.get_mut(id) {
            node.dirty = true;
        }
        Memo {
            id,
            _marker: PhantomData,
        }
    }

    /// Reads a memo's value, computing it first if it is dirty.
    pub fn read_memo<T: Clone + 'static>(&mut self, memo: Memo<T>) -> T {
        self.record_edge(memo.id);
        let dirty = self
            .nodes
            .get(memo.id)
            .and_then(Option::as_ref)
            .map(|n| n.dirty)
            .expect("memo has been disposed");
        if dirty {
            self.clear_incoming_edges(memo.id);
            let mut compute = self
                .nodes
                .get_mut(memo.id)
                .and_then(|n| n.as_mut())
                .and_then(|n| n.compute.take())
                .expect("memo has a compute function");
            if let Some(Some(node)) = self.nodes.get_mut(memo.id) {
                node.dirty = false;
            }
            self.current_runner = Some(memo.id);
            let value = compute(self);
            self.current_runner = None;
            if let Some(Some(node)) = self.nodes.get_mut(memo.id) {
                node.compute = Some(compute);
                node.value = Some(value);
            }
        }
        self.nodes[memo.id]
            .as_ref()
            .expect("memo has been disposed")
            .value
            .as_ref()
            .expect("memo has no value")
            .downcast_ref::<T>()
            .expect("memo value type mismatch")
            .clone()
    }

    /// Creates an effect: a closure that runs once immediately and then
    /// reruns whenever a signal it reads changes.
    pub fn create_effect(&mut self, f: impl FnMut(&mut Runtime) + 'static) -> EffectHandle {
        let id = self.alloc_node(NodeKind::Effect, None, Some(Box::new(f)), None);
        self.current_runner = Some(id);
        let mut effect = self
            .nodes
            .get_mut(id)
            .and_then(|n| n.as_mut())
            .and_then(|n| n.effect.take())
            .expect("effect has a body");
        effect(self);
        self.current_runner = None;
        if let Some(Some(node)) = self.nodes.get_mut(id) {
            node.effect = Some(effect);
        }
        EffectHandle { id }
    }

    /// Registers a cleanup for the currently running effect or memo.
    ///
    /// The cleanup runs right before the next rerun and on dispose.
    pub fn on_cleanup(&mut self, f: impl FnMut() + 'static) {
        if let Some(runner) = self.current_runner {
            if let Some(Some(node)) = self.nodes.get_mut(runner) {
                node.cleanup = Some(Box::new(f));
            }
        }
    }

    /// Disposes an effect: runs its cleanup, unsubscribes it from every
    /// signal, and frees its slot.
    pub fn dispose(&mut self, handle: EffectHandle) {
        self.dispose_node(handle.id);
    }

    /// Marks `root` and everything that transitively depends on it as dirty,
    /// queueing them for the next flush.
    fn mark_dirty(&mut self, root: NodeId) {
        let mut stack = vec![root];
        while let Some(id) = stack.pop() {
            let Some(Some(node)) = self.nodes.get_mut(id) else {
                continue;
            };
            if node.dirty {
                continue;
            }
            node.dirty = true;
            self.dirty_queue.push_back(id);
            let dependents = node.dependents.clone();
            for dependent in dependents {
                stack.push(dependent);
            }
        }
    }

    /// Processes the dirty queue: clears signal flags, recomputes dirty
    /// memos, reruns dirty effects — each with refreshed dependency edges.
    pub fn flush(&mut self) {
        while let Some(id) = self.dirty_queue.pop_front() {
            let kind = match self.nodes.get(id).and_then(Option::as_ref) {
                Some(node) if node.dirty => node.kind,
                _ => continue,
            };
            match kind {
                NodeKind::Signal => {
                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        node.dirty = false;
                    }
                }
                NodeKind::Memo => {
                    self.clear_incoming_edges(id);
                    let mut compute = self
                        .nodes
                        .get_mut(id)
                        .and_then(|n| n.as_mut())
                        .and_then(|n| n.compute.take())
                        .expect("memo has a compute function");
                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        node.dirty = false;
                    }
                    self.current_runner = Some(id);
                    let value = compute(self);
                    self.current_runner = None;
                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        node.compute = Some(compute);
                        node.value = Some(value);
                    }
                }
                NodeKind::Effect => {
                    self.clear_incoming_edges(id);
                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        if let Some(mut cleanup) = node.cleanup.take() {
                            cleanup();
                        }
                        node.dirty = false;
                    }
                    let mut effect = self
                        .nodes
                        .get_mut(id)
                        .and_then(|n| n.as_mut())
                        .and_then(|n| n.effect.take())
                        .expect("effect has a body");
                    self.current_runner = Some(id);
                    effect(self);
                    self.current_runner = None;
                    if let Some(Some(node)) = self.nodes.get_mut(id) {
                        node.effect = Some(effect);
                    }
                }
            }
        }
    }

    /// Runs `f` inside a batch: writes queue up, and propagation happens
    /// exactly once when the outermost batch ends.
    pub fn batch<R>(&mut self, f: impl FnOnce(&mut Runtime) -> R) -> R {
        self.batch_depth += 1;
        let result = f(self);
        self.batch_depth -= 1;
        if self.batch_depth == 0 {
            self.flush();
        }
        result
    }

    /// Runs `f` in a child scope. Nodes created inside become owned by the
    /// scope; disposing the scope disposes them all.
    pub fn create_scope(&mut self, f: impl FnOnce(&mut Runtime)) -> Scope {
        let parent = self.scope_stack.last().copied();
        let id = self.next_scope;
        self.next_scope += 1;
        self.scopes.insert(
            id,
            ScopeRecord {
                parent,
                nodes: Vec::new(),
            },
        );
        self.scope_stack.push(id);
        f(self);
        self.scope_stack.pop();
        Scope { id }
    }

    /// Disposes a scope: child scopes first, then every node created in it
    /// (running cleanups and unsubscribing).
    pub fn dispose_scope(&mut self, scope: Scope) {
        self.dispose_scope_inner(scope.id);
    }

    /// Removes one node from the runtime: runs its cleanup, cuts all edges,
    /// and frees the slot.
    fn dispose_node(&mut self, id: NodeId) {
        let cleanup = self
            .nodes
            .get_mut(id)
            .and_then(|n| n.as_mut())
            .and_then(|n| n.cleanup.take());
        if let Some(mut cleanup) = cleanup {
            cleanup();
        }
        let dependencies = self
            .nodes
            .get(id)
            .and_then(Option::as_ref)
            .map(|n| n.dependencies.clone())
            .unwrap_or_default();
        for dep in dependencies {
            if let Some(Some(dep)) = self.nodes.get_mut(dep) {
                dep.dependents.retain(|&x| x != id);
            }
        }
        if let Some(Some(node)) = self.nodes.get_mut(id) {
            node.dependencies.clear();
            node.dependents.clear();
            node.dirty = false;
        }
        self.nodes[id] = None;
        self.free.push(id);
    }

    /// Recursively disposes a scope and its children.
    fn dispose_scope_inner(&mut self, id: ScopeId) {
        let children: Vec<ScopeId> = self
            .scopes
            .iter()
            .filter(|(_, record)| record.parent == Some(id))
            .map(|(child, _)| *child)
            .collect();
        for child in children {
            self.dispose_scope_inner(child);
        }
        let node_ids = self
            .scopes
            .get(&id)
            .map(|record| record.nodes.clone())
            .unwrap_or_default();
        for node in node_ids {
            self.dispose_node(node);
        }
        self.scopes.remove(&id);
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Read half of a signal.
#[derive(Debug, Clone, Copy)]
pub struct Signal<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// Write half of a signal.
#[derive(Debug, Clone, Copy)]
pub struct WriteSignal<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// A memoized derived value.
#[derive(Debug, Clone, Copy)]
pub struct Memo<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// Handle to a live effect.
#[derive(Debug, Clone, Copy)]
pub struct EffectHandle {
    id: NodeId,
}

/// Handle to a live scope.
#[derive(Debug, Clone, Copy)]
pub struct Scope {
    id: ScopeId,
}
