//! Module 047: Structured Concurrency & Cancellation — exercise scaffold.
//!
//! `select!` races multiple futures and returns the first to complete.
//! `timeout` wraps a future with a deadline. `CancellationToken` enables
//! cooperative cancellation. `JoinSet` manages a dynamic set of tasks.

#[allow(unused_imports)]
use tokio::time::{timeout, Duration};
use tokio_util::sync::CancellationToken;

/// Race two futures: return `Ok(left)` if `left` completes first, or
/// `Ok(right)` if `right` completes first. Both futures produce `i32`.
pub async fn race_futures<L, R>(left: L, right: R) -> Result<i32, i32>
where
    L: std::future::Future<Output = i32>,
    R: std::future::Future<Output = i32>,
{
    // TODO(module-047): use `tokio::select!` to race `left` and `right`.
    // Return `Ok(left_val)` if `left` wins, `Err(right_val)` if `right`
    // wins.
    let _ = (left, right);
    panic!("TODO(module-047): implement race_futures")
}

/// Run `future` with a timeout of `millis` milliseconds. Return
/// `Ok(value)` if it completes in time, or `Err("timeout")` if it does
/// not.
pub async fn run_with_timeout<F>(future: F, millis: u64) -> Result<i32, &'static str>
where
    F: std::future::Future<Output = i32>,
{
    // TODO(module-047): use `tokio::time::timeout(Duration::from_millis(millis), future)`.
    // It returns `Result<Result<i32>, Elapsed>`. Map the outer `Err` to
    // `Err("timeout")`.
    let _ = (future, millis);
    panic!("TODO(module-047): implement run_with_timeout")
}

/// Spawn a task that loops forever, checking a `CancellationToken` on
/// each iteration. The task should increment a counter and sleep briefly.
/// When the token is cancelled, the task should exit. Return the final
/// counter value.
pub async fn cancellable_task(token: CancellationToken) -> u64 {
    // TODO(module-047): spawn a task that loops, incrementing a counter
    // each iteration. On each iteration, check `token.is_cancelled()` —
    // if true, break. Otherwise, `tokio::time::sleep` for 10ms. After
    // the loop, return the counter. Await the task's handle.
    let _ = token;
    panic!("TODO(module-047): implement cancellable_task")
}

/// Spawn `n_tasks` tasks, each computing `task_id * 10`. Collect all
/// results using a `JoinSet` and return them sorted.
pub async fn join_all_tasks(n_tasks: u32) -> Vec<u32> {
    // TODO(module-047): create a `tokio::task::JoinSet`, spawn `n_tasks`
    // tasks (each computing `task_id * 10`), then `join_next().await` in
    // a loop until the set is empty. Collect results, sort, return.
    let _ = n_tasks;
    panic!("TODO(module-047): implement join_all_tasks")
}
