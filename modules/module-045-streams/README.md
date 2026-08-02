# Module 045: Streams

**Block:** Block E — Async Rust
**Estimated time:** 60–90 min
**Prerequisites:** Module 041 (async/await), Module 042 (Tokio runtime), Module 044 (channels)

## Learning Objectives

- Explain what a `Stream` is and how it relates to `Iterator`.
- Use `StreamExt` combinators (`next`, `filter`, `map`, `take`, `collect`) to transform async sequences.
- Build a `Stream` from a `tokio::sync::mpsc` channel with `ReceiverStream`.
- Merge multiple streams with `select` / `merge` and reason about interleaving.

## Why This Matters

Most real async Rust is not one request, one response — it is a flow of events: messages on a channel, rows from a database cursor, lines from a WebSocket, ticks from a timer. A `Stream` is the abstraction that turns "a sequence of things arriving over time" into something you can `.filter()`, `.map()`, and `.collect()` just like an iterator. Every streaming API in the Tokio ecosystem — `tokio-stream`, `async-trait` services, `tonic` gRPC streams — is built on this trait.

## Concept

### From `Iterator` to `Stream`

You already know `Iterator`:

```rust
let doubled: Vec<i32> = vec![1, 2, 3].into_iter().map(|x| x * 2).collect();
```

Each call to `.next()` returns `Some(item)` synchronously. A `Stream` is the same idea, but each `.next()` returns a *future* that resolves to `Some(item)`:

```
Iterator::next(&mut self) -> Option<Item>
Stream::poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Item>>
```

Because `Stream` lives in the `futures` crate (not `std`), you pull it in with `use futures::stream::Stream;` and the combinators via `use futures::StreamExt;`.

### The five operations you will use every day

```
┌──────────────────────────────────────────────────────────────┐
│  collect     drain every item into a Vec / HashMap / etc.    │
│  filter      keep items matching a predicate                 │
│  map         transform each item                             │
│  take        stop after N items                              │
│  next        pull one item at a time (like Iterator::next)   │
└──────────────────────────────────────────────────────────────┘
```

Every one of these is an `.await` point. That is the entire point: the runtime can poll other tasks while waiting for the next item.

```rust
use futures::{stream, StreamExt};

async fn sum_of_even_squares() -> i32 {
    stream::iter(1..=10)
        .filter(|n| futures::future::ready(n % 2 == 0))
        .map(|n| n * n)
        .collect::<Vec<i32>>()
        .await
        .into_iter()
        .sum()
}
```

Note the `future::ready` inside `filter`. The `filter` combinator takes an *async* predicate — it returns a future that resolves to `bool`. For a synchronous check, wrap it in `future::ready`.

### Building streams from channels

The most common stream you will create comes from a channel. Tokio's receiver is not itself a `Stream`, but `tokio_stream::wrappers::ReceiverStream` adapts it:

```rust
use tokio_stream::wrappers::ReceiverStream;
use futures::StreamExt;

let (tx, rx) = tokio::sync::mpsc::channel::<String>(8);
let mut stream = ReceiverStream::new(rx);

tokio::spawn(async move {
    tx.send("hello".to_string()).await.unwrap();
    tx.send("world".to_string()).await.unwrap();
});

while let Some(msg) = stream.next().await {
    println!("{msg}");
}
```

When every sender is dropped, the stream yields `None` and the loop ends — the same "close on last sender drop" protocol from Module 044, now as a stream terminator.

### Merging streams: `select` and `merge`

When you have two streams and want whichever item arrives first, use `futures::stream::select(a, b)` (or `StreamExt::merge`). Items are interleaved in arrival order:

```
   stream a:  ──1──────3──────5──►
   stream b:  ────2──────4──────6──►
   select:    ──1──2──3──4──5──6──►
```

This is the building block for multiplexing: a timer stream merged with a message stream, a control channel merged with a data channel, and so on.

```rust
use futures::{stream, StreamExt};

async fn merged() -> Vec<i32> {
    let a = stream::iter(vec![1, 3, 5]);
    let b = stream::iter(vec![2, 4, 6]);
    futures::stream::select(a, b)
        .collect::<Vec<i32>>()
        .await
}
```

The order depends on polling; for deterministic tests, sort the result.

### `Unpin` and why it shows up everywhere

Stream combinators require `S: Stream + Unpin`. Most streams you build from channels, iterators, and `Box::pin` are `Unpin` by construction. If you hit a bound error, wrapping in `Box::pin(stream)` always works — you will see why in Module 046.

## Common Pitfalls

- **Forgetting that `filter` takes an async closure.** The predicate returns a future; use `futures::future::ready(bool)` for synchronous checks.
- **Expecting merge order to be deterministic.** `select`/`merge` interleave by arrival; sort the collected output in tests.
- **Using `std::iter` combinators on a stream.** They do not exist — you must `use futures::StreamExt;`.
- **Missing the `Unpin` bound.** If your function takes a generic stream, add `+ Unpin` or `Box::pin` it.
- **Holding a stream across an `.await` in a loop without `next()`.** You must `.await` each `.next()` call to drive it.

## Key Terms

- **`Stream`:** the async `Iterator`; each `poll_next` returns `Poll<Option<Item>>`.
- **`StreamExt`:** the combinator trait (`next`, `filter`, `map`, `take`, `collect`, `merge`).
- **`ReceiverStream`:** adapts a `tokio::sync::mpsc::Receiver` into a `Stream`.
- **`select` / `merge`:** interleave two streams by arrival order.
- **`Unpin`:** marker trait most constructed streams satisfy; required by generic combinators.

## Exercise

Work in `exercises/` and make `cargo test -p module-045-exercises` pass. Five TODOs in `src/lib.rs`:

1. `collect_stream` — drain a stream into a `Vec` with `.collect().await`.
2. `filter_map_stream` — keep evens, double them, using `filter` + `map`.
3. `take_n` — take the first `n` items.
4. `merge_streams` — interleave two streams and return sorted output.
5. `stream_from_channel` — spawn a sender task, wrap the receiver in `ReceiverStream`, collect.

Tests check exact values for `collect`, `filter_map`, `take`; sorted equality for `merge`; and exact sequences for the channel stream. Compare with `solutions/` when done.

## Further Reading

- [`futures::stream::Stream` docs](https://docs.rs/futures/latest/futures/stream/trait.Stream.html)
- [`tokio-stream` crate](https://docs.rs/tokio-stream/latest/tokio_stream/) — wrappers for channels, intervals, signals
- [Tokio tutorial: Streams](https://tokio.rs/tokio/tutorial/streams)
- [The `futures` crate book](https://rust-lang.github.io/futures-rs/futures/tutorial/index.html)
