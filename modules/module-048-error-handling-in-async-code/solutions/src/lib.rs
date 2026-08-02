//! Module 048: Error Handling in Async Code — reference solution.
//!
//! `anyhow::Result` works seamlessly across `.await` boundaries.
//! Retry with exponential backoff uses a factory closure to create a
//! fresh future per attempt. `JoinSet` runs fallible tasks in parallel
//! with fail-fast behaviour.

use anyhow::Context;
use tokio::task::JoinSet;
use tokio::time::{sleep, Duration};

pub async fn fallible_operation(succeed: bool) -> anyhow::Result<String> {
    if succeed {
        Ok("success".to_string())
    } else {
        anyhow::bail!("operation failed")
    }
}

pub async fn retry_with_backoff<F, Fut, T>(mut factory: F, max_retries: u32) -> anyhow::Result<T>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = anyhow::Result<T>>,
{
    let mut attempt = 0u32;
    loop {
        match factory().await {
            Ok(val) => return Ok(val),
            Err(e) if attempt >= max_retries => {
                return Err(e).context(format!("failed after {} retries", max_retries));
            }
            Err(_) => {
                attempt += 1;
                sleep(Duration::from_millis(2u64.pow(attempt) * 10)).await;
            }
        }
    }
}

pub async fn run_parallel_tasks<I, T, F, Fut>(items: Vec<I>, f: F) -> anyhow::Result<Vec<T>>
where
    F: Fn(I) -> Fut + Send + Sync + 'static,
    Fut: std::future::Future<Output = anyhow::Result<T>> + Send + 'static,
    I: Send + 'static,
    T: Send + 'static,
{
    let f = std::sync::Arc::new(f);
    let mut set = JoinSet::new();
    for item in items {
        let f = f.clone();
        set.spawn(async move { f(item).await });
    }
    let mut results = Vec::new();
    while let Some(res) = set.join_next().await {
        results.push(res??);
    }
    Ok(results)
}
