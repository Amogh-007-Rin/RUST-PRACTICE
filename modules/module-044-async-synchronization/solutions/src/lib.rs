//! Module 044: Async Synchronization — reference solution.
//!
//! The four canonical Tokio sync patterns: mpsc fan-in, oneshot
//! request/response, broadcast publish/subscribe, and shared state behind
//! an async `Mutex`.

use std::sync::Arc;
use tokio::sync::{broadcast, mpsc, oneshot, Mutex};

/// Fan in: spawn `producers` tasks; producer `i` sends `per_producer`
/// values, the `k`-th value being `i * 1000 + k`. Collect **all** values
/// and return them sorted.
pub async fn fan_in(producers: u32, per_producer: u32) -> Vec<u64> {
    let (tx, mut rx) = mpsc::channel(8);
    let mut handles = Vec::new();
    for i in 0..producers {
        let tx = tx.clone();
        handles.push(tokio::spawn(async move {
            for k in 0..per_producer {
                tx.send(u64::from(i) * 1000 + u64::from(k)).await.unwrap();
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    drop(tx); // close the channel: the last sender to drop signals "done"

    let mut values = Vec::new();
    while let Some(value) = rx.recv().await {
        values.push(value);
    }
    values.sort_unstable();
    values
}

/// Request/response: spawn a task that waits for a `oneshot` message,
/// answers with `format!("echo: {message}")`, and return that answer.
pub async fn oneshot_roundtrip(message: String) -> String {
    let (tx, rx) = oneshot::channel();
    tokio::spawn(async move {
        tx.send(format!("echo: {message}")).unwrap();
    });
    rx.await.unwrap()
}

/// Publish/subscribe: create a broadcast channel with capacity 16, spawn
/// `n_subscribers` tasks each recording every message they receive, send
/// all `messages` after spawning, and return each subscriber's recording.
pub async fn broadcast_to_all(n_subscribers: usize, messages: Vec<u64>) -> Vec<Vec<u64>> {
    let (tx, _) = broadcast::channel(16);
    let mut handles = Vec::new();
    for _ in 0..n_subscribers {
        let mut rx = tx.subscribe();
        let len = messages.len();
        handles.push(tokio::spawn(async move {
            let mut received = Vec::new();
            for _ in 0..len {
                received.push(rx.recv().await.unwrap());
            }
            received
        }));
    }
    for message in messages {
        tx.send(message).unwrap();
    }
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.await.unwrap());
    }
    results
}

/// Shared state: spawn `n_tasks` tasks that each increment a
/// `Mutex<u64>` `per_task` times, then return the final count.
pub async fn shared_counter(n_tasks: u32, per_task: u32) -> u64 {
    let counter = Arc::new(Mutex::new(0u64));
    let mut handles = Vec::new();
    for _ in 0..n_tasks {
        let counter = counter.clone();
        handles.push(tokio::spawn(async move {
            for _ in 0..per_task {
                let mut guard = counter.lock().await;
                *guard += 1;
            }
        }));
    }
    for handle in handles {
        handle.await.unwrap();
    }
    let final_count = *counter.lock().await;
    final_count
}
