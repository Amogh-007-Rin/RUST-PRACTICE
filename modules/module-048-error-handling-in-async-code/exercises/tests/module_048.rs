//! Module 048: Error Handling in Async Code — integration tests.

use module_048_exercises::{fallible_operation, retry_with_backoff, run_parallel_tasks};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

#[tokio::test]
async fn fallible_operation_succeeds() {
    let result = fallible_operation(true).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "success");
}

#[tokio::test]
async fn fallible_operation_fails() {
    let result = fallible_operation(false).await;
    assert!(result.is_err());
    assert!(result.unwrap_err().to_string().contains("operation failed"));
}

#[tokio::test]
async fn retry_with_backoff_succeeds_after_failures() {
    let attempts = Arc::new(AtomicU32::new(0));
    let a = attempts.clone();
    let result = retry_with_backoff(
        move || {
            let count = a.fetch_add(1, Ordering::SeqCst);
            async move {
                if count < 2 {
                    anyhow::bail!("transient failure")
                }
                Ok(42)
            }
        },
        3,
    )
    .await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    assert!(attempts.load(Ordering::SeqCst) >= 2);
}

#[tokio::test]
async fn retry_with_backoff_exhausts_retries() {
    let result: anyhow::Result<i32> =
        retry_with_backoff(|| async { anyhow::bail!("always fails") }, 2).await;
    assert!(result.is_err());
    let err = format!("{:#}", result.unwrap_err());
    assert!(err.contains("failed after 2 retries"));
    assert!(err.contains("always fails"));
}

#[tokio::test]
async fn retry_with_backoff_succeeds_immediately() {
    let result = retry_with_backoff(|| async { Ok::<_, anyhow::Error>(99) }, 0).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 99);
}

#[tokio::test]
async fn run_parallel_tasks_all_succeed() {
    let items = vec![1, 2, 3];
    let mut results = run_parallel_tasks(items, |x| async move { Ok::<_, anyhow::Error>(x * 3) })
        .await
        .unwrap();
    results.sort();
    assert_eq!(results, vec![3, 6, 9]);
}

#[tokio::test]
async fn run_parallel_tasks_fails_on_error() {
    let items = vec![1, 2, 3];
    let result = run_parallel_tasks(items, |x| async move {
        if x == 2 {
            anyhow::bail!("error on {}", x)
        }
        Ok(x * 10)
    })
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn run_parallel_tasks_empty_input() {
    let items: Vec<i32> = vec![];
    let results = run_parallel_tasks(items, |x: i32| async move { Ok::<_, anyhow::Error>(x) })
        .await
        .unwrap();
    assert!(results.is_empty());
}
