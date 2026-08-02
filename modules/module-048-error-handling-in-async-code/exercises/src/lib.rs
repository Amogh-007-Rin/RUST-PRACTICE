//! Module 048: Error Handling in Async Code — exercise scaffold.
//!
//! `anyhow::Result` works across `.await` boundaries. Retry with
//! exponential backoff is a standard pattern for transient failures.
//! `JoinSet` lets you run fallible tasks in parallel and fail fast.

#[allow(unused_imports)]
use anyhow::Context;
#[allow(unused_imports)]
use tokio::task::JoinSet;
#[allow(unused_imports)]
use tokio::time::{sleep, Duration};

/// An async fallible operation. Returns `Ok("success")` if `succeed`,
/// otherwise `Err` with a message.
pub async fn fallible_operation(succeed: bool) -> anyhow::Result<String> {
    // TODO(module-048): if `succeed`, return Ok("success".to_string());
    // otherwise, use `anyhow::bail!("operation failed")`.
    let _ = succeed;
    panic!("TODO(module-048): implement fallible_operation")
}

/// Call `factory` repeatedly until it succeeds or `max_retries` is
/// exhausted. Between attempts, sleep with exponential backoff starting
/// at 10ms and doubling each attempt (10, 20, 40, …). On final failure,
/// attach context with `Context::context` showing the attempt count.
#[allow(unused_mut)]
pub async fn retry_with_backoff<F, Fut, T>(mut factory: F, max_retries: u32) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<T>>,
{
    // TODO(module-048): implement retry loop:
    //   let mut attempt = 0u32;
    //   loop {
    //       match factory().await {
    //           Ok(val) => return Ok(val),
    //           Err(e) if attempt >= max_retries => {
    //               return Err(e).context(format!("failed after {} retries", max_retries));
    //           }
    //           Err(_) => {
    //               attempt += 1;
    //               sleep(Duration::from_millis(2u64.pow(attempt) * 10)).await;
    //           }
    //       }
    //   }
    let _ = (factory, max_retries);
    panic!("TODO(module-048): implement retry_with_backoff")
}

/// Run multiple async tasks in parallel using `JoinSet`. Each item is
/// passed through `f` and spawned as a Tokio task. All results are
/// collected. If any task fails, return the error (fail-fast).
pub async fn run_parallel_tasks<I, T, F, Fut>(items: Vec<I>, f: F) -> anyhow::Result<Vec<T>>
where
    F: Fn(I) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = anyhow::Result<T>> + Send + 'static,
    I: Send + 'static,
    T: Send + 'static,
{
    // TODO(module-048):
    //   1. Wrap `f` in `std::sync::Arc::new(f)`.
    //   2. Create a `JoinSet::new()`.
    //   3. For each item, clone the Arc and `set.spawn(async move { f(item).await })`.
    //   4. Drain the set with `while let Some(res) = set.join_next().await`.
    //   5. Use `res??` to propagate both the JoinError and the inner error.
    //   6. Return the collected Vec<T>.
    let _ = (items, f);
    panic!("TODO(module-048): implement run_parallel_tasks")
}
