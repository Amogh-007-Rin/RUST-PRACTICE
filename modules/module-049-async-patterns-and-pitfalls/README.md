# Module 049: Async Patterns & Pitfalls

**Block:** Block E — Async Rust
**Estimated time:** 45–75 min
**Prerequisites:** Module 048 (Error Handling in Async Code)

## Learning Objectives

- Identify and avoid blocking operations in async contexts.
- Use `tokio::task::spawn_blocking` to offload CPU-bound work.
- Understand why `std::sync::Mutex` across `.await` can deadlock and how `tokio::sync::Mutex` avoids it.
- Recognise common async deadlock patterns and follow structured concurrency best practices.

## Why This Matters

The most common production bugs in async Rust are not logic errors — they are runtime stalls caused by blocking the async runtime. A single `std::thread::sleep` or a heavy computation on the async worker thread can delay every other task scheduled on that thread. A `std::sync::Mutex` held across an `.await` point can deadlock the entire runtime if the same thread tries to acquire it again. These are not hypothetical — they show up in production incident reports for Tokio-based services. Knowing how to avoid them is table stakes for shipping async Rust.

## Concept

### Blocking in async context: the silent killer

Tokio uses a work-stealing thread pool. Each worker thread runs a loop: dequeue a task, make progress on its future until it yields (.await), enqueue the task again, repeat. If a task never yields — because it calls `std::thread::sleep`, a blocking syscall, or a CPU-bound loop — the worker thread is occupied and cannot service other tasks.

```
┌─────────────────────────────────────────────────────────────────┐
│ Worker thread 0                                                 │
│   Task A: running (holding the thread)                          │
│   Task B: waiting in queue                                      │
│   Task C: waiting in queue                                      │
│                                                                 │
│ Task A calls std::thread::sleep(Duration::from_secs(10)).       │
│ The thread is blocked for 10 seconds.                           │
│ Task B and Task C cannot make any progress.                     │
│                                                                 │
│ Meanwhile, Worker threads 1–N are idle or underused.            │
└─────────────────────────────────────────────────────────────────┘
```

The fix for CPU-bound work is `spawn_blocking`:

```rust
// BAD: blocks the async runtime thread
async fn bad() -> u64 {
    heavy_computation()  // CPU-bound, no .await — blocks the thread
}

// GOOD: offloads to a dedicated blocking thread pool
async fn good() -> u64 {
    tokio::task::spawn_blocking(|| heavy_computation())
        .await
        .unwrap()
}
```

`spawn_blocking` runs the closure on a separate thread pool managed by Tokio specifically for blocking operations. The async task suspends (yields the worker thread) while the blocking work completes on the dedicated pool. Other tasks can use the worker thread in the meantime.

What counts as "blocking" includes:
- `std::thread::sleep`
- Heavy computation (fibonacci, crypto, image processing, large sorts)
- Synchronous I/O (`std::fs::read`, `std::net::TcpStream` — use `tokio::fs` and `tokio::net` equivalents instead)
- Any long-running loop without `.await` points

### `tokio::sync::Mutex` vs `std::sync::Mutex`

`std::sync::Mutex` uses OS-level blocking. When you call `lock()` and the mutex is already held, the current thread blocks until the mutex is released. In async code, this is disastrous because the blocked thread cannot make progress on *any* task — it is completely stuck.

The worst case is holding a `std::sync::Mutex` across an `.await`:

```rust
// DANGER: std::sync::Mutex across .await
async fn dangerous() {
    let guard = std::sync::Mutex::new(()).lock().unwrap();
    some_io().await;  // guard is held across .await
    drop(guard);
}
```

```
Step 1: Task A on thread 0 acquires the std::sync::Mutex.
Step 2: Task A reaches some_io().await and yields, still holding the mutex.
Step 3: Task B on thread 0 tries to acquire the same mutex. Thread 0 blocks.
Step 4: With only one runtime thread (e.g. #[tokio::main(flavor = "current_thread")]),
        the runtime is deadlocked — no thread can make progress.
Step 5: Even on multi-threaded runtimes, if Task A's future is polled on the
        same thread that Task B runs on, you get a deadlock.
```

`tokio::sync::Mutex` avoids this by making `lock()` asynchronous. When a task tries to acquire a held `tokio::sync::Mutex`, the task *yields* instead of blocking the thread. The runtime scheduler can run other tasks on that thread while the first task holds the lock:

```rust
// SAFE: tokio::sync::Mutex across .await
async fn safe() {
    let mut guard = tokio::sync::Mutex::new(()).lock().await;
    some_io().await;  // guard is held, but thread is not blocked
    drop(guard);
    // other tasks waiting on the mutex are woken at this point
}
```

However, `tokio::sync::Mutex` has overhead compared to `std::sync::Mutex` (it uses an async channel internally). Follow these rules of thumb:

- If you never hold the lock across `.await`, use `std::sync::Mutex` — it is faster.
- If you must hold the guard across `.await`, use `tokio::sync::Mutex`.
- If the critical section is tiny (just setting a field), consider `std::sync::Mutex` and make sure you drop the guard before any `.await`.

### Common async deadlock patterns

**Pattern 1: Single-threaded runtime + blocking lock.** If you use `#[tokio::main]` without specifying `worker_threads`, Tokio uses `multi_thread` by default. But `#[tokio::test]` defaults to `current_thread`. A deadlock that does not appear in your production multi-threaded runtime can surface in tests. Always test with `#[tokio::test(flavor = "multi_thread")]` if your code relies on multi-thread scheduling.

**Pattern 2: Holding multiple locks in different orders.** Two tasks acquiring lock A then B, and lock B then A, deadlock regardless of whether the locks are async or sync. The solution is always the same: acquire locks in a consistent global order.

**Pattern 3: Calling `block_on` inside a Tokio task.** `Handle::block_on` or `Runtime::block_on` blocks the current thread until the future completes, preventing task switching. Never call these inside a Tokio task — use `.await` directly.

**Pattern 4: Synchronous channels crossing async/sync boundaries.** Calling `std::sync::mpsc::Receiver::recv()` (which blocks) inside an async task blocks the thread. Use `tokio::sync::mpsc::Receiver::recv().await` instead.

### Structured concurrency best practices

These patterns from Module 047 apply doubly when considering pitfalls:

1. **Use `JoinSet` for dynamic task groups.** It handles join-all semantics and gives you completion-order results.
2. **Use `CancellationToken` for graceful shutdown.** Signal all tasks and wait for them to finish before dropping resources.
3. **Avoid fire-and-forget spawns.** Every `tokio::spawn` should have its `JoinHandle` awaited or stored somewhere. Unjoined tasks that panic can go unnoticed.
4. **Set timeouts on network operations.** Every `TcpStream::connect`, every HTTP request, should have a timeout. An unanswered connect call can hang indefinitely.
5. **Do not hold `std::sync::Mutex` across `.await`.** This is the number-one async deadlock cause. Use `tokio::sync::Mutex` if you must, or restructure so the lock is dropped before the await.

## Common Pitfalls

- **Blocking the runtime with sync I/O or CPU work.** The runtime has a tell-tale sign: latency spikes in unrelated requests. The fix is `spawn_blocking` or async I/O primitives.
- **`std::sync::Mutex` across `.await`.** This deadlocks on single-threaded runtimes and can deadlock on multi-threaded runtimes. Replace with `tokio::sync::Mutex` or restructure to drop the guard before `.await`.
- **Assuming `current_thread` tests reflect production behaviour.** Add `flavor = "multi_thread"` to async lock tests so a deadlocking pattern that only manifests on one thread is caught.
- **Fire-and-forget spawns with no error handling.** If a spawned task panics, the `JoinHandle` will return a `JoinError` when awaited. If you never await the handle, the panic goes unnoticed.
- **Synchronous recursion or loops in async functions with no yield points.** A long synchronous loop inside an `async fn` (with no `.await`) blocks the thread just like `std::thread::sleep`.

## Key Terms

- **`spawn_blocking`:** Tokio function that runs a CPU-bound or blocking closure on a dedicated thread pool, preventing it from stalling async worker threads.
- **`tokio::sync::Mutex`:** an async-aware mutex whose `lock()` method is a future — it yields rather than blocking the thread when the mutex is contended.
- **`current_thread` runtime:** a Tokio runtime that runs all tasks on a single OS thread; ideal for tests and simple applications but vulnerable to blocking.
- **Fire-and-forget:** spawning a task and discarding its `JoinHandle` — the task's result (including panics) is lost.
- **Work-stealing:** Tokio's scheduler strategy where idle worker threads steal queued tasks from busy threads.

## Exercise

Work in `exercises/` and make `cargo test -p module-049-exercises` pass. Three TODOs in `src/lib.rs`:

1. `fibonacci_sync` — implement an iterative fibonacci function (`n < 2` return `n`; otherwise loop computing `a, b = b, a + b`).
2. `cpu_intensive_task` — call `fibonacci_sync` via `tokio::task::spawn_blocking` to avoid blocking the runtime.
3. `demonstrate_deadlock_avoidance` — create a `tokio::sync::Mutex<u32>`, spawn two tasks where the first holds the lock across a sleep, then the second acquires it and reads the value (should be 42).

The `blocking_computation` function is already provided — it reverses a string with an artificial delay. The test calls it via `spawn_blocking` to demonstrate the correct pattern.

Compare with `solutions/` when done.

## Further Reading

- [Tokio docs: `spawn_blocking`](https://docs.rs/tokio/latest/tokio/task/fn.spawn_blocking.html)
- [Tokio tutorial: Shared State](https://tokio.rs/tokio/tutorial/shared-state) — discusses `tokio::sync::Mutex`
- [Tokio blog: Reducing Tail Latencies with Tokio](https://tokio.rs/blog/2020-04-reducing-tail-latency)
- [Rust async book: Pitfalls](https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html)
