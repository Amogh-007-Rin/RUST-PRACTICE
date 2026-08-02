//! Module 044: Async Synchronization — integration tests.

use module_044_solutions::{broadcast_to_all, fan_in, oneshot_roundtrip, shared_counter};

#[tokio::test]
async fn fan_in_collects_every_value() {
    let values = fan_in(3, 5).await;
    let mut expected = Vec::new();
    for i in 0..3u64 {
        for k in 0..5u64 {
            expected.push(i * 1000 + k);
        }
    }
    expected.sort_unstable();
    assert_eq!(values, expected);
}

#[tokio::test]
async fn fan_in_with_one_producer_is_ordered() {
    let values = fan_in(1, 10).await;
    assert_eq!(values, (0..10u64).map(u64::from).collect::<Vec<_>>());
}

#[tokio::test]
async fn oneshot_roundtrip_returns_the_answer() {
    assert_eq!(oneshot_roundtrip("hi".to_string()).await, "echo: hi");
}

#[tokio::test]
async fn broadcast_reaches_every_subscriber() {
    let results = broadcast_to_all(2, vec![1, 2, 3]).await;
    assert_eq!(results.len(), 2);
    for subscriber in &results {
        assert_eq!(subscriber, &vec![1, 2, 3]);
    }
}

#[tokio::test]
async fn shared_counter_reaches_exactly_the_sum() {
    assert_eq!(shared_counter(10, 1000).await, 10_000);
}
