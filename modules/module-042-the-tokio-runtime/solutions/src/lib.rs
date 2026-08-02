//! Module 042: The Tokio Runtime — reference solution.
//!
//! Two ways to start a runtime (`#[tokio::main]`-style macros and the
//! `Builder` API), `tokio::spawn` for fire-and-forget tasks, and the
//! demonstration that concurrent sleeps overlap in wall-clock time.

use std::time::Duration;

/// Spawn two tasks that each return their argument, then await both and
/// return the sum.
pub async fn spawn_and_sum(a: u64, b: u64) -> u64 {
    let task_a = tokio::spawn(async move { a });
    let task_b = tokio::spawn(async move { b });
    task_a.await.unwrap() + task_b.await.unwrap()
}

/// Spawn `n` tasks; task `i` returns `i + 1`. Await all of them and
/// return the sum of their outputs.
pub async fn spawn_many_sum(n: u32) -> u64 {
    let mut handles = Vec::new();
    for i in 0..n {
        handles.push(tokio::spawn(async move { u64::from(i) + 1 }));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.await.unwrap();
    }
    total
}

/// Build a multi-threaded runtime by hand and run a block_on on it.
pub fn multi_thread_blocking_sum(items: Vec<u64>) -> u64 {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let mut handles = Vec::new();
        for item in items {
            handles.push(tokio::spawn(async move { item }));
        }
        let mut total = 0;
        for handle in handles {
            total += handle.await.unwrap();
        }
        total
    })
}

/// Build a current-thread runtime by hand and run a block_on on it.
pub fn current_thread_blocking_sum(items: Vec<u64>) -> u64 {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    runtime.block_on(async {
        let mut handles = Vec::new();
        for item in items {
            handles.push(tokio::spawn(async move { item }));
        }
        let mut total = 0;
        for handle in handles {
            total += handle.await.unwrap();
        }
        total
    })
}

/// Spawn `n` tasks that each sleep `millis` milliseconds, await them all,
/// and return the total wall-clock time the whole batch took.
pub async fn parallel_sleep_total(n: u32, millis: u64) -> Duration {
    let start = tokio::time::Instant::now();
    let mut handles = Vec::new();
    for _ in 0..n {
        handles.push(tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(millis)).await;
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    start.elapsed()
}
