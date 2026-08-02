//! Module 047: Structured Concurrency & Cancellation — reference solution.
//!
//! `select!` races futures, `timeout` enforces deadlines,
//! `CancellationToken` enables cooperative cancellation, `JoinSet`
//! manages dynamic task sets.

use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

/// Race two futures: return `Ok(left)` if `left` completes first, or
/// `Ok(right)` if `right` completes first.
pub async fn race_futures<L, R>(left: L, right: R) -> Result<i32, i32>
where
    L: std::future::Future<Output = i32>,
    R: std::future::Future<Output = i32>,
{
    tokio::select! {
        val = left => Ok(val),
        val = right => Err(val),
    }
}

/// Run `future` with a timeout of `millis` milliseconds.
pub async fn run_with_timeout<F>(future: F, millis: u64) -> Result<i32, &'static str>
where
    F: std::future::Future<Output = i32>,
{
    match timeout(Duration::from_millis(millis), future).await {
        Ok(value) => Ok(value),
        Err(_) => Err("timeout"),
    }
}

/// Spawn a task that loops forever, checking a `CancellationToken`.
pub async fn cancellable_task(token: CancellationToken) -> u64 {
    let handle = tokio::spawn(async move {
        let mut counter = 0u64;
        loop {
            if token.is_cancelled() {
                break;
            }
            counter += 1;
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        counter
    });
    handle.await.unwrap()
}

/// Spawn `n_tasks` tasks, each computing `task_id * 10`. Collect all
/// results using a `JoinSet` and return them sorted.
pub async fn join_all_tasks(n_tasks: u32) -> Vec<u32> {
    let mut set = tokio::task::JoinSet::new();
    for task_id in 0..n_tasks {
        set.spawn(async move { task_id * 10 });
    }
    let mut results = Vec::new();
    while let Some(result) = set.join_next().await {
        results.push(result.unwrap());
    }
    results.sort_unstable();
    results
}
