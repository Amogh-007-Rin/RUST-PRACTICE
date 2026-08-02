//! Module 047: Structured Concurrency & Cancellation — integration tests.

use module_047_solutions::{cancellable_task, join_all_tasks, race_futures, run_with_timeout};
use tokio::time::{sleep, Duration};
use tokio_util::sync::CancellationToken;

#[tokio::test]
async fn race_futures_left_wins() {
    let left = async { 1 };
    let right = async {
        sleep(Duration::from_millis(50)).await;
        2
    };
    assert_eq!(race_futures(left, right).await, Ok(1));
}

#[tokio::test]
async fn race_futures_right_wins() {
    let left = async {
        sleep(Duration::from_millis(50)).await;
        1
    };
    let right = async { 2 };
    assert_eq!(race_futures(left, right).await, Err(2));
}

#[tokio::test]
async fn run_with_timeout_completes_in_time() {
    let future = async { 42 };
    assert_eq!(run_with_timeout(future, 100).await, Ok(42));
}

#[tokio::test]
async fn run_with_timeout_expires() {
    let future = async {
        sleep(Duration::from_millis(200)).await;
        42
    };
    assert_eq!(run_with_timeout(future, 50).await, Err("timeout"));
}

#[tokio::test]
async fn cancellable_task_stops_on_cancel() {
    let token = CancellationToken::new();
    let token_clone = token.clone();
    let handle = tokio::spawn(async move { cancellable_task(token_clone).await });
    sleep(Duration::from_millis(100)).await;
    token.cancel();
    let count = handle.await.unwrap();
    assert!(count > 0);
    assert!(count < 20);
}

#[tokio::test]
async fn join_all_tasks_collects_all_results() {
    let results = join_all_tasks(5).await;
    assert_eq!(results, vec![0, 10, 20, 30, 40]);
}

#[tokio::test]
async fn join_all_tasks_empty() {
    let results = join_all_tasks(0).await;
    assert!(results.is_empty());
}
