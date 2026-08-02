# Module 044: Async Synchronization

**Block:** Block E — Async Rust
**Estimated time:** 60–90 min
**Prerequisites:** Module 042 (spawn), Module 032–033 (`Arc`, `Mutex`, `std` channels), Module 041

## Learning Objectives

- Choose the right channel for a job: `mpsc` (fan-in), `oneshot` (request/response), `broadcast` (pub/sub).
- Share mutable state across tasks with `tokio::sync::Mutex` and explain why the `.await` version exists.
- Close channels deliberately to signal completion, and drain receivers without deadlocking.
- Explain when `std::sync` primitives are the right call inside async code.

## Why This Matters

Real services are pipelines: HTTP handlers produce events, workers consume them, one service talks to another with request/response. Tokio's channels are the plumbing. The most common production async bug — a task that waits on a channel nobody will ever fill because a sender was accidentally dropped — becomes tractable the moment you can trace a channel's ownership: *the receiver returns `None`/`Err` exactly when the last sender is gone*, and that fact is the protocol.

## Concept

### A map of the channel family

Three channel flavors cover almost everything:

```
  mpsc (multi-producer, single-consumer)        oneshot (one message)
  ┌────────┐  ┌────────┐  ┌────────┐            ┌────────┐
  │ task 1 ├─►│        │  │ task 2 ├───────────►│ task 3 │
  │ task 2 ├─►│  FIFO  ├─►│        │            │        │
  │ task 3 ├─►│ queue  │  └────────┘            └────────┘
  └────────┘  └────┬───┘     │                    │  │
                   ▼         ▼                    ▼  │
               mpsc::Receiver                 oneshot::Receiver
               (exactly one)                  (exactly one message)

  broadcast (publish/subscribe)
        ┌────────────┐  send(msg)          ┌──────────────┐
        │  Sender    │────────────────────►│ subscriber 1 │  every message,
        │  (publisher)│                    └──────────────┘  in order, to
        └────────────┘                     ┌──────────────┐  every subscriber
              │  send(msg)────────────────►│ subscriber 2 │
                                           └──────────────┘
```

- **`mpsc`** is a FIFO queue with many producers and one consumer. The `Sender` is cheaply cloneable (`Clone`), so each task gets its own — but the receiver returns `None` only when *every* sender (including your original) has been dropped. This is why the canonical pattern ends with `drop(tx)` after spawning: you must give up your own sender or the consumer waits forever.
- **`oneshot`** is a "one message, one consumer" mailbox. It is the async version of a promise: the receiver errors if the sender was dropped without sending. Every `JoinHandle` result and most RPC patterns reduce to this shape.
- **`broadcast`** copies each message to every subscriber, in order. If a subscriber is too slow, it is ejected with an error — broadcast has a bounded buffer and *drops* laggards rather than blocking the publisher.

### The `.await` in the middle

Both sides of every channel call are async:

```rust
let (tx, mut rx) = tokio::sync::mpsc::channel(8);
tx.send(42).await.unwrap();   // yields if the buffer is full
let value = rx.recv().await.unwrap(); // yields until a message arrives
```

This is the deep difference from `std::sync::mpsc`: Tokio's `send`/`recv` return `Pending` when the channel is full/empty, so the task yields the thread instead of parking it. One task blocked on `recv()` does not stall its siblings — the runtime just polls other tasks until a message lands.

`broadcast` and `oneshot` follow the same rule, with one twist: `broadcast::Sender::send` is synchronous (it only copies into the buffer and never waits), while receiving is async.

### Shared state: the async `Mutex`

Module 032 introduced `std::sync::Mutex`: a lock you hold across *threads*. Tokio's `Mutex` is a lock you hold across `.await` points:

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

async fn increment(counter: &Arc<Mutex<u64>>) {
    let mut guard = counter.lock().await;
    *guard += 1;
}
```

Why can't you hold a `std::sync::MutexGuard` across an `.await`? The guard would block the current thread if the lock is contended — and blocking a worker thread is exactly what async code must never do. `tokio::sync::Mutex` yields the task on contention instead, which is correct-but-slower (it is a bit heavier than the std one). The rule of thumb:

- Guarding state briefly, with no `.await` while holding — use `std::sync::Mutex` (cheap, no allocation).
- Guarding state across an `.await` — use `tokio::sync::Mutex`.

The exercise's `shared_counter` also shows the discipline that makes this safe: acquire, mutate, *release* the guard before the next iteration. Holding a guard across an unrelated `.await` is how you accidentally serialize tasks or (worse, on a single-threaded runtime) deadlock — Module 049 returns to this.

## Common Pitfalls

- **Never `drop(tx)`.** The receiver then waits forever on `recv()`, and your program hangs silently. Drop your own sender after spawning clones.
- **Using `oneshot::Receiver::recv().await`** — it is `rx.await`, not `recv()`. The receiver itself is a future.
- **Missing the broadcast buffer**: a slow subscriber gets kicked with `RecvError::Lagged`. Spawn subscribers *before* publishing (as the exercise does) to avoid dropping the first messages.
- **Holding an async `MutexGuard` across a long `.await`.** You serialize every other locker for the whole wait. Narrow the guard's scope.
- **Reaching for `Mutex` when the data is append-only.** A channel or a single `AtomicU64` is usually the better tool.

## Key Terms

- **`mpsc`:** multi-producer, single-consumer FIFO channel; `None` when all senders drop.
- **`oneshot`:** one-message channel; `Err` when the sender drops before sending.
- **`broadcast`:** copies every message to all subscribers; drops laggards.
- **`tokio::sync::Mutex`:** an async-aware lock that yields on contention.
- **Channel capacity:** the buffer size `channel(n)` — bounded, never grows.

## Exercise

Work in `exercises/` and make `cargo test -p module-044-exercises` pass. Four TODOs in `src/lib.rs`:

1. `fan_in` — the producer/consumer pipeline: `mpsc`, cloned senders, join, `drop(tx)`, drain.
2. `oneshot_roundtrip` — a spawned task answering through a one-shot mailbox.
3. `broadcast_to_all` — subscribers spawned before publishing, then joined.
4. `shared_counter` — `Arc<tokio::sync::Mutex<u64>>` incremented from ten tasks; the final count must be exactly 10,000.

Tests check every fan-in value arrives (sorted), the echo round-trips, every subscriber sees the same ordered broadcast, and the shared counter is exactly correct — no lost increments. Compare with `solutions/` when done.

## Further Reading

- [Tokio docs: tokio::sync](https://docs.rs/tokio/latest/tokio/sync/index.html) — the canonical usage notes for every primitive
- [Tokio tutorial: Shared state](https://tokio.rs/tokio/tutorial/shared-state)
- [The Rust Book: Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html) (std side)
