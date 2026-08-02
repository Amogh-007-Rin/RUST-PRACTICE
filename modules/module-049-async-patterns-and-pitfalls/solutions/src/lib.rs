//! Module 049: Async Patterns & Pitfalls — reference solution.
//!
//! CPU-intensive work via `spawn_blocking`. `tokio::sync::Mutex`
//! instead of `std::sync::Mutex` across `.await`. Blocking functions
//! that should be called from a blocking thread pool.

use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub fn fibonacci_sync(n: u64) -> u64 {
    if n < 2 {
        return n;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 1..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    b
}

pub async fn cpu_intensive_task(n: u64) -> u64 {
    tokio::task::spawn_blocking(move || fibonacci_sync(n))
        .await
        .unwrap()
}

pub async fn demonstrate_deadlock_avoidance() -> bool {
    let mutex = Arc::new(tokio::sync::Mutex::new(0u32));

    let m1 = mutex.clone();
    let handle_a = tokio::spawn(async move {
        let mut guard = m1.lock().await;
        sleep(Duration::from_millis(50)).await;
        *guard = 42;
    });

    sleep(Duration::from_millis(10)).await;

    let m2 = mutex.clone();
    let handle_b = tokio::spawn(async move {
        let guard = m2.lock().await;
        *guard
    });

    handle_a.await.unwrap();
    handle_b.await.unwrap() == 42
}

pub fn blocking_computation(input: &str) -> String {
    let mut result = String::with_capacity(input.len());
    for ch in input.chars().rev() {
        result.push(ch);
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    result
}
