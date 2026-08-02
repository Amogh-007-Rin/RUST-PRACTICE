# Module 083: Building a Rust Frontend App with Leptos

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 90–120 min
**Prerequisites:** Module 082 (wasm-bindgen & JS interop). Modules 001–080.

## Learning Objectives

- You will be able to explain the reactive programming model — signals as sources of truth, memos as derived values, and effects as side-effect consumers.
- You will be able to implement signal/memo/effect primitives from scratch and describe how the dependency graph drives invalidation and recomputation.
- You will be able to explain scoped ownership for automatic DOM cleanup when components unmount.
- You will be able to implement batched updates and explain why they improve efficiency in a reactive UI framework.

## Why This Matters

Module 082 showed you how Rust crosses the wasm boundary. Leptos (along with Yew, Sycamore, and Dioxus) takes the next step: it gives you component-based UI programming in Rust compiled to wasm, replacing the hand-rolled DOM manipulation you did in Module 082 with a declarative reactive model. The framework tracks which parts of the DOM depend on which pieces of state and updates only what changed — exactly the pattern that React popularized in JS and that Leptos refined for Rust. Every `#[component]` annotation, every `create_signal` call, and every `create_effect` in a real Leptos app is powered by the primitives you implement in this module. Understanding the engine means you'll never be mystified by "why didn't this rerender?" — you'll have built the graph that decides.

## Concept

### The reactive paradigm

Imagine a spreadsheet. Cell A1 contains the number 5. Cell B1 contains `=A1*2`. When you change A1 to 7, B1 automatically recomputes to 14. You didn't write any "update B1" code — the spreadsheet's formula engine handles it. Reactive UI frameworks work the same way: you declare relationships between data and the DOM, and the runtime propagates changes automatically.

The three primitives are:

- **Signal** — a mutable cell that holds a value. This is the source of truth. `create_signal(0)` returns a reader handle and a writer handle. Writing a new value (`set`) invalidates everything that depends on it.
- **Memo** — a value derived from signals (and other memos). `create_memo(|| a * 2)` only recomputes when one of its dependencies changed and only when someone actually reads it — lazy evaluation.
- **Effect** — a closure that runs when its dependencies change, producing side effects. In a browser, an effect typically updates the DOM. In this module's engine, effects are just closures you can observe from tests.

Here is the same idea expressed with the engine you'll build:

```rust
let mut rt = Runtime::new();

// Source of truth
let (count, set_count) = rt.create_signal(0);

// Derived value — recomputed lazily
let doubled = rt.create_memo(move |rt| rt.get(count) * 2);

// Side effect — runs now and on every change
rt.create_effect(move |rt| {
    println!("count is {}", rt.get(count));
});

rt.set(set_count, 5);  // prints "count is 5"
assert_eq!(rt.read_memo(doubled), 10);
```

### The dependency graph

Behind the scenes, every `get(read_handle)` call inside a running memo or effect creates a directed edge `runner → signal`. The signal records the runner in its `dependents` list — these are the nodes that must be invalidated when the signal changes. The runner records the signal in its `dependencies` list — these are what it read last time it ran.

When you call `set(write_handle, new_value)`:

1. The signal's value is stored.
2. `mark_dirty(signal_id)` walks the `dependents` graph transitively, setting `dirty = true` on every downstream node and pushing them onto a `dirty_queue`.
3. `flush()` drains the queue:
   - For **signals**: just clear the dirty flag — the value already changed.
   - For **memos**: if dirty, clear incoming edges (the old dependency list), run the compute closure with `current_runner` set to the memo's id (so reads inside re-subscribe), store the result, and clear the dirty flag.
   - For **effects**: if dirty, clear incoming edges, run any registered cleanup from the previous run, then run the effect body with `current_runner` set.

Here is that flow as an ASCII diagram:

```
set(signal, 6)
  │
  ├─ store value "6" on signal node
  ├─ mark_dirty(signal_id)
  │    └─ walk dependents[signal] → memo1 → memo2 → effect1
  │       set dirty=true, push to dirty_queue (signal, memo1, memo2, effect1)
  └─ flush()
       ├─ pop signal:   clear dirty
       ├─ pop memo1:    clear edges, recompute (reads signal → edge rebuilt), clear dirty
       ├─ pop memo2:    clear edges, recompute (reads memo1 → edge rebuilt), clear dirty
       └─ pop effect1:  clear edges, run cleanup(), run body (reads memo2 → edge rebuilt), clear dirty
```

The critical detail is **edge clearing before every run**. When a memo or effect runs, its old dependency edges are torn down and rebuilt based on what it *actually* reads this time. This is why effects can conditionally read different signals on different runs — a pattern called "dynamic dependency tracking":

```rust
rt.create_effect(move |rt| {
    if rt.get(show_details) {
        println!("details: {}", rt.get(data));
    }
});
// First run: edge from effect → show_details only
// After show_details becomes true: edge from effect → show_details AND → data
// After show_details becomes false again: edge from effect → show_details only
```

### Scopes and cleanup

In a browser, components mount and unmount. When a component unmounts, every signal, memo, and effect created inside it must be disposed — otherwise effects keep running on data that no longer corresponds to DOM, leaking both memory and CPU.

This module's engine models this with **scopes**. `create_scope(f)` pushes a new scope onto a stack, runs `f` (which typically creates signals and effects inside), then pops the scope. Nodes record which scope created them. Calling `dispose_scope(scope)`:

1. Recursively disposes all child scopes first (inner before outer).
2. Disposes every node owned by the scope: runs the node's cleanup if any, removes it from all dependency lists (so no future writes trigger it), and frees the slot.
3. Removes the scope record from the runtime.

Scopes mirror the component tree of a real framework. In Leptos, every component is a scope — when the component is removed from the view tree, Leptos calls `dispose_scope` internally.

Run `on_cleanup(f)` inside an effect to register a function that runs before the next rerun *or* when the effect is disposed. This is the mechanism behind Leptos's automatic DOM cleanup: the previous DOM nodes generated by an effect are removed before the effect creates new ones.

### Batching

Without batching, every `set(...)` call triggers a full flush — recomputing memos, rerunning effects. If you're updating five signals to reflect new application state, five flushes means five DOM updates, five frame paints, and potentially wasted work (early flushes might use half the state while the other half hasn't been set yet).

`batch(f)` increments a depth counter, runs `f` (inside which `set` calls still queue dirty nodes but skip the flush), and only flushes once when the outermost batch ends. Nested `batch` calls are safe — only the outermost boundary triggers propagation.

```rust
rt.batch(|rt| {
    rt.set(set_name, "Alice");
    rt.set(set_age, 30);
    rt.set(set_role, "admin");
    // No effects have run yet — they see three stale values
});
// One flush: all effects run once with the final ("Alice", 30, "admin") state
```

This is exactly how Leptos handles state updates inside event handlers — the framework wraps every event callback in a batch so that multiple state mutations produce a single DOM update.

### Putting it all together: how Leptos uses these primitives

A Leptos `#[component]` desugars to roughly:

```rust,ignore
fn MyComponent(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);
    let doubled = create_memo(cx, move |_| count.get() * 2);
    create_effect(cx, move |_| {
        // DOM mutation: set text content of some element to doubled.get()
    });
    view! { cx,
        <button on:click=move |_| set_count.update(|n| *n += 1)>
            "Count: " {doubled}
        </button>
    }
}
```

Under the hood, `create_signal` allocates a reactive node in the runtime. `create_memo` registers a derived node. `create_effect` subscribes to signal changes and runs the DOM mutation closure. The `cx: Scope` parameter is the owner scope — when the component unmounts, Leptos calls `dispose_scope` on it, which cleans up everything the component created.

The exercise in this module implements exactly this runtime — not the Leptos DSL or the DOM layer, but the reactive engine that makes it all work. Once you've built it, the framework's behavior will stop feeling like magic.

## Common Pitfalls

- **Creating cycles in the dependency graph.** A memo that reads itself (directly or transitively through `get` while it's recomputing) creates infinite recursion. The `record_edge` helper in the scaffold prevents direct self-edges, but indirect cycles (A → B → A) are still possible.
- **Forgetting to clear edges before rerunning.** If a runner retains old dependencies, inline branches in effects (reading signal A only when B is true) will keep A as a dependency forever, causing unnecessary reruns.
- **Reading a signal without subscribing.** When you read a signal outside an active memo or effect (no `current_runner`), `record_edge` is a no-op — fine for one-shot queries, but the reader won't react to future changes.
- **Forgetting to run cleanup in `flush` for effects.** If the previous run registered a `on_cleanup` that tears down DOM but you skip it before the next run, you leak DOM nodes.
- **Disposing a scope without disposing child scopes first.** Data from child scopes may still reference parent nodes.

## Key Terms

- **Signal:** a mutable reactive value with separate read/write handles. Writing a signal invalidates every node that depends on it.
- **Memo:** a lazily-computed derived reactive value. Stale when a dependency changed, recomputed on next read.
- **Effect:** a closure that runs eagerly when its dependencies change. The mechanism for driving side effects (DOM updates, logging, network calls).
- **Reactive graph:** the directed acyclic graph where edges run from readers (memos/effects) to their sources (signals/other memos). Edges are rebuilt on every run.
- **Scope:** an ownership boundary for reactive nodes. Disposing a scope disposes all nodes created within it, mirroring component unmount in a UI.
- **Batch:** a transaction that defers propagation until the outermost batch boundary, reducing redundant work.
- **Flush:** the process of draining the dirty queue — recomputing stale memos and rerunning stale effects.
- **Dirty flag:** a boolean on each node indicating whether its value may be stale. Set by `mark_dirty`, cleared by `flush` or on recompute.

## Exercise

In `exercises/src/lib.rs` you'll find a full reactive runtime scaffold with `// TODO(module-083)` stubs. The scaffold provides:
- `alloc_node`, `record_edge`, `clear_incoming_edges` — graph manipulation helpers.
- `create_signal`, `create_scope` — already implemented as reference for the pattern.
- Type-erased component storage (`Node`) and handle types (`Signal<T>`, `WriteSignal<T>`, `Memo<T>`, `EffectHandle`, `Scope`).

Implement the following TODOs:

1. **`Runtime::get`** — record the edge from the current runner, then downcast and clone the stored value.
2. **`Runtime::set`** — store the new value, call `mark_dirty`, and flush unless batching.
3. **`Runtime::update`** — downcast the value mutably, run the closure, then propagate like `set`.
4. **`Runtime::create_memo`** — allocate a Memo node with a wrapped compute closure, starting dirty so the first read computes.
5. **`Runtime::read_memo`** — record the edge, recompute if dirty (as the current runner with edge clearing), then return the value.
6. **`Runtime::create_effect`** — allocate an Effect node, run the body once immediately with the effect as current runner.
7. **`Runtime::on_cleanup`** — store the cleanup closure on the current runner's node.
8. **`Runtime::dispose`** — delegate to `dispose_node`.
9. **`Runtime::mark_dirty`** — walk the dependents graph transitively, set dirty flags, and push onto the dirty queue.
10. **`Runtime::flush`** — drain the dirty queue: clear signals, recompute memos, rerun effects (running cleanups first).
11. **`Runtime::batch`** — increment depth, run the closure, decrement, and flush when depth returns to zero.
12. **`Runtime::dispose_scope`** — dispose child scopes recursively, then dispose all nodes owned by this scope.
13. **`Runtime::dispose_node`** — run cleanup, cut all graph edges, free the slot.

The integration tests in `tests/module_083.rs` cover signals, memos (caching, chaining, ignoring unrelated writes), effects (initial run, rerun, dependency switching, disposal), batching (nested, combined with effects), cleanup lifecycle, and scoped disposal. Run with `cargo test -p module-083-exercises` — they'll fail until your implementations are complete.

## Further Reading

- [Leptos Book — "Reactivity" chapter](https://book.leptos.dev/reactivity/index.html) — the official Leptos documentation on signals, memos, and effects.
- [SolidJS Reactivity Guide](https://www.solidjs.com/guides/reactivity) — SolidJS is the JS framework that inspired Leptos's core reactive model; the concepts translate directly.
- [Leptos source — `reactive_graph` module](https://github.com/leptos-rs/leptos/tree/main/reactive_graph) — the real implementation you're approximating here.
- [Module 082 — wasm-bindgen & JS Interop](../module-082-wasm-bindgen-and-js-interop/README.md) — the boundary layer Leptos sits on top of.
