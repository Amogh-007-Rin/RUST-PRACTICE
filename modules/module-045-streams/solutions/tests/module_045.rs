//! Module 045: Streams — integration tests.

use futures::stream;
use module_045_solutions::{
    collect_stream, filter_map_stream, merge_streams, stream_from_channel, take_n,
};

#[tokio::test]
async fn collect_stream_drains_every_item() {
    let s = stream::iter(vec![1, 2, 3, 4, 5]);
    assert_eq!(collect_stream(s).await, vec![1, 2, 3, 4, 5]);
}

#[tokio::test]
async fn collect_stream_on_empty_is_empty() {
    let s = stream::iter(Vec::<i32>::new());
    assert!(collect_stream(s).await.is_empty());
}

#[tokio::test]
async fn filter_map_stream_keeps_evens_doubled() {
    let s = stream::iter(vec![1, 2, 3, 4, 5, 6]);
    assert_eq!(filter_map_stream(s).await, vec![4, 8, 12]);
}

#[tokio::test]
async fn take_n_returns_first_n_items() {
    let s = stream::iter(vec![10, 20, 30, 40, 50]);
    assert_eq!(take_n(s, 3).await, vec![10, 20, 30]);
}

#[tokio::test]
async fn take_n_more_than_available_returns_all() {
    let s = stream::iter(vec![1, 2]);
    assert_eq!(take_n(s, 10).await, vec![1, 2]);
}

#[tokio::test]
async fn merge_streams_collects_both_sources() {
    let a = stream::iter(vec![1, 3, 5]);
    let b = stream::iter(vec![2, 4, 6]);
    let mut result = merge_streams(a, b).await;
    result.sort_unstable();
    assert_eq!(result, vec![1, 2, 3, 4, 5, 6]);
}

#[tokio::test]
async fn stream_from_channel_produces_all_values() {
    let result = stream_from_channel(5).await;
    assert_eq!(result, vec![0, 1, 2, 3, 4]);
}

#[tokio::test]
async fn stream_from_channel_zero_count_is_empty() {
    let result = stream_from_channel(0).await;
    assert!(result.is_empty());
}
