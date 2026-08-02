# Module 048: Error Handling in Async Code

**Block:** Block E — Async Rust
**Estimated time:** 45–75 min
**Prerequisites:** Module 047 (Structured Concurrency & Cancellation)

## Learning Objectives

- Use `anyhow::Result` in async functions and across `.await` boundaries.
- Implement a retry function with exponential backoff for transient failures.
- Propagate errors from spawned tasks using `JoinSet` with fail-fast behaviour.
- Attach context to errors using `anyhow::Context`.

## Why This Matters

`anyhow::Result` is the de facto error type for application-level Rust code — it is used in virtually every async service, CLI tool, and prototype. In async contexts, errors compound: a spawned task can fail, a retry can exhaust its attempts, a timeout can elapse. Knowing how to wrap, propagate, and contextualise errors across `.await` points is essential for building reliable async systems. The retry-with-backoff pattern alone is required in every networked service that calls external APIs, databases, or message brokers.

## Concept

### `anyhow` in async contexts

`anyhow::Result<T>` is `Result<T, anyhow::Error>`. `anyhow::Error` is a boxed trait object (`Box<dyn Error + Send + Sync>`), so it can hold any error type. This makes it perfect for application code where you do not need to distinguish error variants at compile time — you just want to propagate or display them.

In async code, `anyhow` works exactly the same as in sync code. The `?` operator works across `.await` boundaries with no special handling:

```rust
async fn fetch_url(url: &str) -> anyhow::Result<String> {
    let response = reqwest::get(url).await?;          // ? across .await
    let body = response.text().await?;                // ? across .await
    Ok(body)
}
```

The compiler-generated state machine for `fetch_url` stores the `anyhow::Error` in whatever state variant is active when the error occurs, then propagates it when the future is polled.

`anyhow::bail!` and `anyhow::ensure!` work at any `.await` suspension point:

```rust
async fn process(n: i32) -> anyhow::Result<i32> {
    anyhow::ensure!(n > 0, "n must be positive, got {}", n);
    Ok(n * 2)
}
```

`anyhow::Context` attaches additional context to errors:

```rust
async fn load_config(path: &str) -> anyhow::Result<Config> {
    let bytes = tokio::fs::read(path)
        .await
        .context(format!("failed to read config from {path}"))?;
    serde_json::from_slice(&bytes).context("invalid JSON in config file")
}
```

The `.context()` method (from the `Context` trait imported with `use anyhow::Context`) adds a message to the error chain. When the error is displayed, all contextual messages are shown from outermost to innermost, creating a stack-trace-like chain.

### Retry with exponential backoff

Transient failures are common in distributed systems: a database connection pool is exhausted, a downstream service returns a 503, a file is temporarily locked. The standard response is to retry with increasing delays between attempts — exponential backoff.

A retry loop in async Rust follows this pattern:

```rust
use tokio::time::{sleep, Duration};

async fn with_retry<F, Fut, T>(mut factory: F, max_retries: u32) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<T>>,
{
    let mut attempt = 0;
    loop {
        match factory().await {
            Ok(value) => return Ok(value),
            Err(e) if attempt >= max_retries => {
                return Err(e).context(format!("exhausted {max_retries} retries"));
            }
            Err(_) => {
                attempt += 1;
                let delay = Duration::from_millis(2u64.pow(attempt) * 100);
                sleep(delay).await;
            }
        }
    }
}
```

```
Attempt  1: fails → wait 200ms
Attempt  2: fails → wait 400ms
Attempt  3: fails → wait 800ms
Attempt  4: succeeds → return Ok(...)

OR

Attempt  1: fails → wait 200ms
Attempt  2: fails → wait 400ms
Attempt  3: fails (max_retries=2) → return Err with context
```

Key design choices in the signature:

- `F: FnMut() -> Fut` — the factory is a *closure that returns a future*. We use `FnMut` (not `FnOnce`) because we call it multiple times. Each call creates a *new* future, so we get a fresh attempt each time, not a stale half-resolved future.
- The future's output is `anyhow::Result<T>`. When you are building a retry for a concrete operation (like an HTTP request), you wrap the operation in a closure that maps its specific error into `anyhow::Error`.

In production, you would add:
- A total timeout (wrap the entire retry in `tokio::time::timeout`).
- Jitter (add a random component to the delay) to avoid thundering-herd problems.
- Selective retry — only retry on certain error types (e.g. retry on connection-refused, but not on 404).

### Error propagation across spawned tasks

When you spawn a fallible task with `tokio::spawn`, you get a `JoinHandle<anyhow::Result<T>>`. Awaiting the handle gives `Result<anyhow::Result<T>, JoinError>` — a nested Result:

```rust
let handle = tokio::spawn(async { fallible_work().await });
let result = handle.await??;  // first ? unpacks JoinError, second ? unpacks the inner error
```

The first `?` propagates `JoinError` (which means the task panicked or was cancelled). The second `?` propagates the application-level error from the task.

With `JoinSet`, this pattern scales to many tasks:

```rust
let mut set = JoinSet::new();
for item in items {
    let f = f.clone();
    set.spawn(async move { f(item).await });
}

let mut results = Vec::new();
while let Some(res) = set.join_next().await {
    results.push(res??);  // fail-fast: first error terminates the loop
}
Ok(results)
```

This gives you fail-fast behaviour — the first task that fails causes the whole operation to fail. The remaining tasks in the `JoinSet` are aborted (dropped) when the set goes out of scope.

For a "collect all errors" approach instead of fail-fast, you would use `join_all` from `futures::future` or accumulate errors manually.

## Common Pitfalls

- **Forgetting the double `?` on `JoinHandle<Result<T, E>>`.** `handle.await?` gives you `Result<T, JoinError>`, which is the wrong type. You need `handle.await??` to unpack both the `JoinError` and the inner error.
- **Reusing a half-resolved future in retry loops.** If you `await` a future once and it fails, you cannot `await` it again — it has already resolved. Wrap the operation in a factory closure that produces a fresh future each time.
- **Blocking in the retry delay.** `sleep(Duration::from_millis(...)).await` is non-blocking. `std::thread::sleep(...)` blocks the runtime thread and starves other tasks.
- **Losing error context in generic retry code.** If your retry function swallows the original error, you lose the root cause. Always attach context with `.context()` or `anyhow::Error`'s `.chain()` facility so the developer can trace the failure.

## Key Terms

- **`anyhow::Result<T>`:** `Result<T, anyhow::Error>` — a catch-all error type for application code.
- **`anyhow::bail!(msg)`:** macro that returns `Err(anyhow::Error::msg(msg))` early from a function.
- **`Context::context(msg)`:** attaches a human-readable message to an error chain.
- **Exponential backoff:** increasing the delay between retries exponentially (2^n × base_delay) to avoid overwhelming a recovering service.
- **`JoinSet`:** a collection of spawned Tokio tasks that yields results in completion order.

## Exercise

Work in `exercises/` and make `cargo test -p module-048-exercises` pass. Three TODOs in `src/lib.rs`:

1. `fallible_operation` — return `Ok("success")` on `true`, bail with `"operation failed"` on `false`.
2. `retry_with_backoff` — loop calling `factory().await`; on success return the value; on failure increment `attempt`; if `attempt >= max_retries`, attach context and return the error; otherwise sleep with exponential backoff starting at 10ms.
3. `run_parallel_tasks` — wrap `f` in `Arc`, create a `JoinSet`, spawn one task per item (each calling `f(item).await`), drain with `join_next().await`, use `res??` to propagate errors, return collected results.

Compare with `solutions/` when done.

## Further Reading

- [`anyhow` crate docs](https://docs.rs/anyhow/latest/anyhow/)
- [Tokio docs: `JoinSet`](https://docs.rs/tokio/latest/tokio/task/struct.JoinSet.html)
- [AWS Architecture Blog: Exponential Backoff and Jitter](https://aws.amazon.com/blogs/architecture/exponential-backoff-and-jitter/)
- [`tokio-retry` crate](https://docs.rs/tokio-retry/latest/tokio_retry/) — production-grade retry with Tokio
