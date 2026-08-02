# Module 047: Structured Concurrency & Cancellation

**Block:** Block E — Async Rust
**Estimated time:** 60–90 min
**Prerequisites:** Module 046 (Pinning & Async Internals)

## Learning Objectives

- Use `tokio::select!` to race multiple async futures and handle the non-selected branch.
- Wrap any future with `tokio::time::timeout` to enforce a deadline.
- Implement cooperative cancellation with `CancellationToken` for graceful task shutdown.
- Manage a dynamic set of spawned tasks with `JoinSet` and collect all results.

## Why This Matters

Every production async service needs timeouts, graceful shutdown, and the ability to handle multiple concurrent operations cleanly. The `select!` macro is the backbone of Tokio-based request handlers that race a handler against a timeout. `CancellationToken` is the standard pattern for propagating shutdown signals to in-flight tasks — it is used in every `actix-web` and `axum` server shutdown hook. `JoinSet` is the tool you reach for when you want to spawn a variable number of tasks (e.g., one per incoming request) and wait for all of them to complete. Mastering these patterns is what moves you from "I can write an async function" to "I can build a robust async service."

## Concept

### Racing futures with `select!`

The `tokio::select!` macro waits on multiple futures concurrently and returns as soon as *any one* of them completes. The remaining futures are dropped — they will not continue executing. This is the single most important concurrency primitive in async Rust after `tokio::spawn`.

```rust
tokio::select! {
    result = async_operation_1() => {
        println!("operation 1 completed: {:?}", result);
    }
    result = async_operation_2() => {
        println!("operation 2 completed: {:?}", result);
    }
}
```

Each branch evaluates a future and binds its output to a pattern. At most one branch executes its body — the one whose future resolved first. All other futures are cancelled. This is exactly what you want when implementing timeouts, cancellation, or "first response wins" logic.

Branch ordering matters: `select!` evaluates branches in order, giving priority to earlier branches if multiple futures resolve simultaneously (a "biased" select — the default in Tokio).

### What happens to the losing future?

When `select!` picks a winner, the losing future is dropped. This means any resources held by that future (file handles, network connections, in-memory buffers) are freed. However, this also means that *the losing future does not get a chance to clean up gracefully* — it is simply cancelled. If the future owns a database transaction that should be rolled back, you need to handle that via `Drop` implementations on the types involved.

```
┌──────────────────────────────────────────────────────────────┐
│ tokio::select! {                                             │
│   val = slow_operation()  =>  { use val }   // cancelled     │
│   val = fast_operation()  =>  { use val }   // WINS          │
│ }                                                            │
│                                                              │
│ slow_operation's future is dropped.                          │
│ Resources held by it are freed via Drop.                     │
└──────────────────────────────────────────────────────────────┘
```

A common pattern is to `select!` over a future and a `CancellationToken`:

```rust
tokio::select! {
    _ = token.cancelled() => {
        return Err(anyhow::anyhow!("cancelled"));
    }
    result = do_work() => {
        Ok(result)
    }
}
```

### `timeout` — a deadline wrapper

`tokio::time::timeout` wraps any future with a duration. If the future completes within the duration, you get `Ok(value)`. If not, you get `Err(Elapsed)`. Under the hood, `timeout` is implemented with `select!` — it races the wrapped future against a timer.

```rust
use tokio::time::{timeout, Duration};

match timeout(Duration::from_secs(5), fetch_data()).await {
    Ok(data) => process(data),
    Err(_)    => return Err("request timed out".into()),
}
```

The timeout error type is `tokio::time::error::Elapsed`. In your own APIs you almost always want to map this to a domain error like `"timeout"` or a custom error variant. The lossy conversion from `Elapsed` to a string is expected — `Elapsed` carries no extra information beyond "time ran out."

### Cooperative cancellation with `CancellationToken`

A `CancellationToken` is a lightweight signal that one or more tasks can watch. One task calls `token.cancel()`; all tasks that are awaiting `token.cancelled()` or polling `token.is_cancelled()` see the cancellation and can shut down.

This is *cooperative*: tasks must explicitly check the token. Tokio does not kill tasks from the outside. This is by design — forced cancellation would leave resources in an unknown state. Cooperative cancellation lets each task clean up its own resources.

```rust
use tokio_util::sync::CancellationToken;

async fn worker(token: CancellationToken) {
    loop {
        if token.is_cancelled() {
            println!("shutting down worker");
            break;
        }
        // do a unit of work...
        tokio::time::sleep(Duration::from_millis(10)).await;
    }
}

let token = CancellationToken::new();
let worker_token = token.clone();
let handle = tokio::spawn(worker(worker_token));

// Later, signal cancellation:
token.cancel();
handle.await.unwrap(); // worker exits its loop and finishes
```

Key points:
- `CancellationToken::new()` creates a fresh token in the "not cancelled" state.
- `.clone()` produces a child token that references the same underlying signal — a shallow, cheap clone.
- `.cancel()` sets the signal. It is idempotent — calling it twice has the same effect as calling it once.
- `.is_cancelled()` returns instantly. Use it in poll loops.
- `.cancelled()` returns a future that resolves when the token is cancelled. Use it in `select!`.

`CancellationToken` is typically used for graceful shutdown: the main task holds the root token, spawns worker tasks with cloned tokens, and on receiving SIGTERM or a shutdown command, calls `.cancel()`. Workers detect the cancellation, finish their current unit of work, and return.

### Managing dynamic tasks with `JoinSet`

`tokio::task::JoinSet` is like a `Vec<JoinHandle<T>>` with an async iterator interface. You spawn tasks into the set and then call `.join_next().await` to get the result of the next task that finishes, in completion order.

```rust
use tokio::task::JoinSet;

let mut set = JoinSet::new();

for i in 0..10 {
    set.spawn(async move {
        some_work(i).await
    });
}

let mut results = Vec::new();
while let Some(result) = set.join_next().await {
    results.push(result.unwrap());
}
```

Unlike `futures::future::join_all`, `JoinSet` does not require you to know all tasks up front — you can add tasks dynamically. This is critical for servers that spawn one task per connection and want to wait for all connections to drain during shutdown.

`.join_next()` returns `Option<Result<T, JoinError>>`. It returns `None` when the set is empty (all tasks have been joined). Tasks are removed from the set when joined, so `join_next` in a `while let` loop naturally drains the set.

`JoinSet` also supports `.shutdown()` which aborts all remaining tasks. Combined with `CancellationToken`, this gives you both graceful and forced shutdown in one type.

## Common Pitfalls

- **Forgetting to handle the non-selected branch.** When `select!` completes, the losing future is dropped silently. If that future held resources that need explicit cleanup, those resources are dropped via `Drop` — make sure their `Drop` impl does the right thing. For network connections, Tokio's types handle this correctly by closing the underlying socket on drop.
- **Dropping a `CancellationToken` while tasks still reference it.** The cloned tokens are independent child handles — dropping the root token does not cancel the children. Only `.cancel()` propagates the signal. If you drop the root without calling `.cancel()`, child tokens will wait forever for a signal that never comes.
- **Holding a `JoinSet` open indefinitely.** A `JoinSet` has no built-in bound on the number of tasks. In a server, this can lead to unbounded memory growth. Always pair `JoinSet` with a semaphore or channel if you need to limit concurrency.
- **Assuming `timeout` cancels the inner future cleanly.** `timeout` drops the future if it elapses, which triggers `Drop`. This is usually fine, but if the future is in the middle of a critical section, the drop happens at the next `.await` point — not immediately. The future runs until its next suspension point before being dropped.
- **Using `async` blocks without thinking about lifetime captures in `select!`.** Each branch's future borrows from the enclosing scope. If two branches borrow the same mutable data, you get a borrow-checker error at compile time — but if one branch borrows data that the other branch drops (by completing), the compiler rejects it. Structure your branches to be independent.

## Key Terms

- **`select!`:** macro that races multiple futures, executing the body of the first one to complete and dropping the rest.
- **`timeout`:** wraps a future with a deadline; returns `Ok(value)` on success or `Err(Elapsed)` if the deadline expires.
- **`CancellationToken`:** a signal object that tasks poll to detect a shutdown or cancellation request; owned by the coordinator and cloned to workers.
- **`JoinSet`:** a collection of spawned Tokio tasks that yields results in completion order via `.join_next()`.

## Exercise

Work in `exercises/` and make `cargo test -p module-047-exercises` pass. Four TODOs in `src/lib.rs`:

1. `race_futures` — use `tokio::select!` to race `left` and `right`; return `Ok(left_val)` if left wins, `Err(right_val)` if right wins.
2. `run_with_timeout` — use `tokio::time::timeout(Duration::from_millis(millis), future)`; map the outer `Err` to `Err("timeout")`.
3. `cancellable_task` — spawn a task that loops, incrementing a counter and sleeping 10ms per iteration. On each iteration, check `token.is_cancelled()` — break if true. Return the counter.
4. `join_all_tasks` — create a `JoinSet`, spawn `n_tasks` tasks each computing `task_id * 10`, drain with `join_next()`, collect results, sort, return.

Compare with `solutions/` when done.

## Further Reading

- [Tokio tutorial: Select](https://tokio.rs/tokio/tutorial/select)
- [Tokio docs: `JoinSet`](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)
- [`tokio_util::sync::CancellationToken` docs](https://docs.rs/tokio-util/latest/tokio_util/sync/struct.CancellationToken.html)
- [Tokio blog: Graceful Shutdown](https://tokio.rs/blog/2021-04-graceful-shutdown)
