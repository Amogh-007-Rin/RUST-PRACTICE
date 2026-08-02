//! Module 044: Async Synchronization — exercise scaffold.
//!
//! Tokio's async primitives: `mpsc` for many-to-one fan-in, `oneshot` for
//! request/response, `broadcast` for publish/subscribe, and the async
//! `Mutex` for shared state. All of them have an `.await` in the middle —
//! that is what makes them safe to use from async code.

/// Fan in: spawn `producers` tasks; producer `i` sends `per_producer`
/// values, the `k`-th value being `i * 1000 + k`. Collect **all** values
/// and return them sorted.
pub async fn fan_in(_producers: u32, _per_producer: u32) -> Vec<u64> {
    // TODO(module-044): create a `tokio::sync::mpsc::channel(8)` and
    // clone the sender into every spawned task. Each task sends its
    // `per_producer` values with `tx.send(value).await`.
    //
    // After every handle is joined, `drop(tx)` to close the channel —
    // the receiver then sees `None` once the last sender is gone — and
    // drain `rx.recv().await` into a `Vec`. Sort the result before
    // returning it so the tests can compare exactly.
    panic!("TODO(module-044): implement fan_in")
}

/// Request/response: spawn a task that waits for a `oneshot` message,
/// answers with `format!("echo: {message}")`, and return that answer.
pub async fn oneshot_roundtrip(_message: String) -> String {
    // TODO(module-044): create a `tokio::sync::oneshot::channel()`,
    // spawn a task that holds the sender, sends the echo via
    // `tx.send(...)`, and await the receiver. Receiver errors mean the
    // sender was dropped: `rx.await.unwrap()`.
    panic!("TODO(module-044): implement oneshot_roundtrip")
}

/// Publish/subscribe: create a broadcast channel with capacity 16, spawn
/// `n_subscribers` tasks each recording every message they receive, send
/// all `messages` after spawning, and return each subscriber's recording.
pub async fn broadcast_to_all(_n_subscribers: usize, _messages: Vec<u64>) -> Vec<Vec<u64>> {
    // TODO(module-044): `tokio::sync::broadcast::channel(16)`, then
    // `tx.subscribe()` once per subscriber inside a spawned task. Each
    // task does `rx.recv().await.unwrap()` once per message.
    // Send the messages *after* all subscribers are spawned (so nobody
    // misses the start), then join every handle.
    panic!("TODO(module-044): implement broadcast_to_all")
}

/// Shared state: spawn `n_tasks` tasks that each increment a
/// `Mutex<u64>` `per_task` times, then return the final count.
pub async fn shared_counter(_n_tasks: u32, _per_task: u32) -> u64 {
    // TODO(module-044): `Arc::new(tokio::sync::Mutex::new(0u64))`, clone
    // the `Arc` into every task, and increment with
    // `*counter.lock().await += 1` inside a loop. Join every handle,
    // then return the value: `*counter.lock().await`.
    panic!("TODO(module-044): implement shared_counter")
}
