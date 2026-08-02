//! Module 045: Streams — exercise scaffold.
//!
//! Streams are the async equivalent of iterators. The `Stream` trait (from
//! the `futures` crate) yields items one at a time, asynchronously. You
//! will use `StreamExt` combinators (`next`, `filter`, `map`, `take`,
//! `collect`) and build streams from channels and iterators.

use tokio_stream::Stream;

/// Collect every item from `stream` into a `Vec` and return it.
pub async fn collect_stream<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    // TODO(module-045): use `stream.collect::<Vec<i32>>().await` (from
    // `StreamExt`) to drain the stream into a vector.
    let _ = stream;
    panic!("TODO(module-045): implement collect_stream")
}

/// Return a vector containing only the even numbers from `stream`, each
/// doubled.
pub async fn filter_map_stream<S>(stream: S) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    // TODO(module-045): chain `.filter(|n| futures::future::ready(n % 2 == 0))`
    // then `.map(|n| n * 2)` then `.collect::<Vec<i32>>().await`.
    // Note: `filter` and `map` take closures that return a future for
    // `filter` (use `future::ready`) and a plain value for `map`.
    let _ = stream;
    panic!("TODO(module-045): implement filter_map_stream")
}

/// Take the first `n` items from `stream` and return them as a `Vec`.
pub async fn take_n<S>(stream: S, n: usize) -> Vec<i32>
where
    S: Stream<Item = i32> + Unpin,
{
    // TODO(module-045): `.take(n).collect::<Vec<i32>>().await`.
    let _ = (stream, n);
    panic!("TODO(module-045): implement take_n")
}

/// Merge two streams: every item from `a` and every item from `b`, in
/// whatever order they arrive. Return the collected items sorted so the
/// test can compare deterministically.
pub async fn merge_streams<A, B>(a: A, b: B) -> Vec<i32>
where
    A: Stream<Item = i32> + Unpin + Send + 'static,
    B: Stream<Item = i32> + Unpin + Send + 'static,
{
    // TODO(module-045): use `tokio_stream::StreamExt::merge(a, b)` (or
    // `futures::stream::select(a, b)`) to interleave the two streams,
    // then collect and sort.
    let _ = (a, b);
    panic!("TODO(module-045): implement merge_streams")
}

/// Build a stream from a `tokio::sync::mpsc` channel: spawn a task that
/// sends the values `0..count` into the channel, then convert the
/// receiver into a stream with `tokio_stream::wrappers::ReceiverStream`
/// and collect the items.
pub async fn stream_from_channel(count: u32) -> Vec<u32> {
    // TODO(module-045): `tokio::sync::mpsc::channel(8)`, spawn a task
    // that sends `0..count`, then wrap the receiver in
    // `ReceiverStream::new(rx)` and `.collect::<Vec<u32>>().await`.
    let _ = count;
    panic!("TODO(module-045): implement stream_from_channel")
}
