//! Module 083: Rust frontend with Leptos — exercise scaffold.
//!
//! Compiling a real Leptos view is a wasm-target workflow, so this crate
//! implements the *engine* Leptos is built on — reactive signals, memos,
//! effects, scopes and cleanup — in pure `std`, fully testable with
//! `cargo test` on the host. Every UI you will ever write with Leptos is a
//! thin layer over exactly these primitives.
//!
//! Fill in every `// TODO(module-083)` below.

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
///
/// `#[allow(dead_code)]` is scaffold-only: the fields are written by
/// `Runtime::alloc_node` and are read by the TODO functions you implement.
#[allow(dead_code)]
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
///
/// `#[allow(dead_code)]` is scaffold-only: `parent` is read by the TODO
/// `dispose_scope` implementation.
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ScopeRecord {
    parent: Option<ScopeId>,
    nodes: Vec<NodeId>,
}

/// The reactive runtime. One instance owns all signals/memos/effects and
/// drives propagation. Everything is deterministic and single-threaded, like
/// a Leptos app running in a browser tab.
///
/// `#[allow(dead_code)]` is scaffold-only: `dirty_queue`/`batch_depth` are
/// written and read by the TODO functions you implement.
#[allow(dead_code)]
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
    ///
    /// `#[allow(dead_code)]` is scaffold-only: these helpers are called by
    /// the TODO functions you implement.
    #[allow(dead_code)]
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
    #[allow(dead_code)]
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
        // TODO(module-083): record the edge from the current runner to this
        // signal (see `record_edge`), then return a clone of the stored
        // value (downcast to `T`).
        let _ = sig;
        panic!("TODO(module-083): implement Runtime::get");
    }

    /// Writes a new value and propagates the change.
    ///
    /// Without an active `batch`, propagation (recompute memos, rerun
    /// effects) happens immediately.
    pub fn set<T: Clone + 'static>(&mut self, write: WriteSignal<T>, value: T) {
        // TODO(module-083): store the new value on the node, mark it dirty
        // (`mark_dirty`), and flush unless a batch is active.
        let _ = (write, value);
        panic!("TODO(module-083): implement Runtime::set");
    }

    /// Mutates a signal's value in place via a closure.
    pub fn update<T: Clone + 'static>(&mut self, write: WriteSignal<T>, f: impl FnOnce(&mut T)) {
        // TODO(module-083): downcast the stored value to `T`, run `f` on it,
        // then propagate exactly like `set`.
        let _ = (write, f);
        panic!("TODO(module-083): implement Runtime::update");
    }

    /// Creates a memo: a value derived from signals, recomputed lazily and
    /// only when one of its dependencies changed.
    pub fn create_memo<T: Clone + 'static>(
        &mut self,
        f: impl FnMut(&mut Runtime) -> T + 'static,
    ) -> Memo<T> {
        // TODO(module-083): allocate a Memo node whose compute closure wraps
        // `f` (erasing T into `Box<dyn Any>`), starting out dirty so the
        // first read computes it. See `alloc_node`'s signature.
        let _ = f;
        panic!("TODO(module-083): implement Runtime::create_memo");
    }

    /// Reads a memo's value, computing it first if it is dirty.
    pub fn read_memo<T: Clone + 'static>(&mut self, memo: Memo<T>) -> T {
        // TODO(module-083): record the edge from the current runner, and if
        // the memo is dirty, recompute it (as the current runner) before
        // returning its value.
        let _ = memo;
        panic!("TODO(module-083): implement Runtime::read_memo");
    }

    /// Creates an effect: a closure that runs once immediately and then
    /// reruns whenever a signal it reads changes.
    pub fn create_effect(&mut self, f: impl FnMut(&mut Runtime) + 'static) -> EffectHandle {
        // TODO(module-083): allocate an Effect node and run `f` once right
        // now, with the effect as the current runner so reads subscribe.
        let _ = f;
        panic!("TODO(module-083): implement Runtime::create_effect");
    }

    /// Registers a cleanup for the currently running effect or memo.
    ///
    /// The cleanup runs right before the next rerun and on dispose.
    pub fn on_cleanup(&mut self, f: impl FnMut() + 'static) {
        // TODO(module-083): store `f` on the current runner's node, so it
        // can be run later. No-op when no runner is active.
        let _ = f;
        panic!("TODO(module-083): implement Runtime::on_cleanup");
    }

    /// Disposes an effect: runs its cleanup, unsubscribes it from every
    /// signal, and frees its slot.
    pub fn dispose(&mut self, handle: EffectHandle) {
        // TODO(module-083): delegate to `dispose_node`.
        panic!("TODO(module-083): implement Runtime::dispose ({handle:?})");
    }

    /// Marks `root` and everything that transitively depends on it as dirty,
    /// queueing them for the next flush.
    #[allow(dead_code)]
    fn mark_dirty(&mut self, root: NodeId) {
        // TODO(module-083): walk the `dependents` graph from `root`, set
        // `dirty = true` on each node, and push each newly-dirty node onto
        // `self.dirty_queue` (order matters for deterministic tests).
        panic!("TODO(module-083): implement Runtime::mark_dirty (root {root})");
    }

    /// Processes the dirty queue: clears signal flags, recomputes dirty
    /// memos, reruns dirty effects — each with refreshed dependency edges.
    pub fn flush(&mut self) {
        // TODO(module-083): pop ids from `self.dirty_queue`. For signals,
        // just clear the flag. For memos and effects, clear incoming edges,
        // run the node's function with `current_runner` set to the node id,
        // then restore the runner.
        panic!("TODO(module-083): implement Runtime::flush");
    }

    /// Runs `f` inside a batch: writes queue up, and propagation happens
    /// exactly once when the outermost batch ends.
    pub fn batch<R>(&mut self, f: impl FnOnce(&mut Runtime) -> R) -> R {
        // TODO(module-083): increment `batch_depth`, run `f`, decrement, and
        // flush once the depth returns to zero.
        let _ = f;
        panic!("TODO(module-083): implement Runtime::batch");
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
        // TODO(module-083): dispose the scope's children (see
        // `ScopeRecord::parent`), then every node created in the scope, then
        // remove the scope record.
        let _ = scope;
        panic!("TODO(module-083): implement Runtime::dispose_scope");
    }

    /// Removes one node from the runtime: runs its cleanup, cuts all edges,
    /// and frees the slot.
    #[allow(dead_code)]
    fn dispose_node(&mut self, id: NodeId) {
        // TODO(module-083): run the node's cleanup if any, remove it from
        // its dependencies' `dependents` lists, clear its own edge lists,
        // drop the slot, and push the id onto `self.free`.
        panic!("TODO(module-083): implement Runtime::dispose_node (id {id})");
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

/// Read half of a signal.
///
/// `#[allow(dead_code)]` is scaffold-only: the id is read by the TODO
/// functions you implement.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Signal<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// Write half of a signal.
///
/// `#[allow(dead_code)]` is scaffold-only: the id is read by the TODO
/// functions you implement.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct WriteSignal<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// A memoized derived value.
///
/// `#[allow(dead_code)]` is scaffold-only: the id is read by the TODO
/// functions you implement.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Memo<T> {
    id: NodeId,
    _marker: PhantomData<fn() -> T>,
}

/// Handle to a live effect.
///
/// `#[allow(dead_code)]` is scaffold-only: the id is read by the TODO
/// functions you implement.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct EffectHandle {
    id: NodeId,
}

/// Handle to a live scope.
///
/// `#[allow(dead_code)]` is scaffold-only: the id is read by the TODO
/// functions you implement.
#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Scope {
    id: ScopeId,
}
