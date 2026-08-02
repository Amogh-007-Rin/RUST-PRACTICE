//! Module 049: Async Patterns & Pitfalls — integration tests.

use module_049_solutions::{
    blocking_computation, cpu_intensive_task, demonstrate_deadlock_avoidance, fibonacci_sync,
};

#[test]
fn fibonacci_sync_basics() {
    assert_eq!(fibonacci_sync(0), 0);
    assert_eq!(fibonacci_sync(1), 1);
    assert_eq!(fibonacci_sync(2), 1);
    assert_eq!(fibonacci_sync(10), 55);
    assert_eq!(fibonacci_sync(20), 6765);
}

#[tokio::test]
async fn cpu_intensive_task_uses_spawn_blocking() {
    assert_eq!(cpu_intensive_task(10).await, 55);
    assert_eq!(cpu_intensive_task(20).await, 6765);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
async fn demonstrate_deadlock_avoidance_works() {
    assert!(demonstrate_deadlock_avoidance().await);
}

#[tokio::test]
async fn blocking_computation_via_spawn_blocking() {
    let input = "hello".to_string();
    let result = tokio::task::spawn_blocking(move || blocking_computation(&input))
        .await
        .unwrap();
    assert_eq!(result, "olleh");
}

#[test]
fn blocking_computation_is_blocking() {
    let start = std::time::Instant::now();
    let result = blocking_computation("abc");
    let elapsed = start.elapsed();
    assert_eq!(result, "cba");
    assert!(elapsed.as_micros() > 200);
}
