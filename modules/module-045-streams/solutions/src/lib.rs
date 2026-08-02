//! Module 045: Streams — reference solution.
//!
//! The five canonical stream operations: collect, filter/map, take, merge,
//! and building a stream from a channel.

use futures::StreamExt;
use tokio_stream::{wrappers::ReceiverStream, Stream};

/// Collect every item from `stream` into a `Vec` and return it.
pub async fn collect_stream<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    stream.collect::<Vec<i32>>().await
}

/// Return a vector containing only the even numbers from `stream`, each
/// doubled.
pub async fn filter_map_stream<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    stream
        .filter(|n| futures::future::ready(n % 2 == 0))
        .map(|n| n * 2)
        .collect::<Vec<i32>>()
        .await
}

/// Take the first `n` items from `stream` and return them as a `Vec`.
pub async fn take_n<S>(stream: S, n: usize) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    stream.take(n).collect::<Vec<i32>>().await
}

/// Merge two streams: every item from `a` and every item from `b`, in
/// whatever order they arrive. Return the collected items sorted so the
/// test can compare deterministically.
pub async fn merge_streams<A, B>(a: A, b: B) -> Vec<i32>
where
    A: Stream<Item = i32> + Unpin + Send + 'static,
    B: Stream<Item = i32> + Unpin + Send + 'static,
{
    let mut merged = futures::stream::select(a, b).collect::<Vec<i32>>().await;
    merged.sort_unstable();
    merged
}

/// Build a stream from a `tokio::sync::mpsc` channel: spawn a task that
/// sends the values `0..count` into the channel, then convert the
/// receiver into a stream with `tokio_stream::wrappers::ReceiverStream`
/// and collect the items.
pub async fn stream_from_channel(count: u32) -> Vec<u32> {
    let (tx, rx) = tokio::sync::mpsc::channel(8);
    tokio::spawn(async move {
        for i in 0..count {
            tx.send(i).await.unwrap();
        }
    });
    ReceiverStream::new(rx).collect::<Vec<u32>>().await
}
