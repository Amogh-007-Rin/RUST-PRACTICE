//! Module 042: The Tokio Runtime — integration tests.

use module_042_exercises::{
    current_thread_blocking_sum, multi_thread_blocking_sum, parallel_sleep_total, spawn_and_sum,
    spawn_many_sum,
};

use std::time::Duration;

#[tokio::test]
async fn spawn_and_sum_adds_two_spawned_tasks() {
    assert_eq!(spawn_and_sum(21, 42).await, 63);
}

#[tokio::test]
async fn spawn_many_sum_adds_every_task_output() {
    assert_eq!(spawn_many_sum(20).await, 210);
}

#[tokio::test]
async fn many_small_tasks_all_complete() {
    assert_eq!(spawn_many_sum(1000).await, 500_500);
}

#[test]
fn manual_multi_thread_runtime_runs_block_on() {
    assert_eq!(multi_thread_blocking_sum(vec![1, 2, 3, 4]), 10);
}

#[test]
fn manual_current_thread_runtime_runs_block_on() {
    assert_eq!(current_thread_blocking_sum(vec![1, 2, 3, 4]), 10);
}

#[tokio::test]
async fn parallel_sleeps_overlap_in_wall_clock_time() {
    let elapsed = parallel_sleep_total(4, 60).await;
    // Four serial sleeps would take at least 240 ms; a running runtime
    // should overlap them into roughly one sleep's duration.
    assert!(
        elapsed < Duration::from_millis(150),
        "sleeps did not overlap: took {elapsed:?}"
    );
}
