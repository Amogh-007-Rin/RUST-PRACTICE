//! Module 042: The Tokio Runtime — exercise scaffold.
//!
//! Tokio is the executor you built by hand in Module 041, industrialised:
//! a scheduler running many tasks per thread, plus integrations for
//! timers and I/O. Here you get comfortable with the two ways a runtime
//! comes to life — the `#[tokio::main]` macro and the `Runtime` builder —
//! and with `tokio::spawn`, which hands a task to the runtime.

/// Spawn two tasks that each return their argument, then await both and
/// return the sum.
pub async fn spawn_and_sum(_a: u64, _b: u64) -> u64 {
    // TODO(module-042): use `tokio::spawn` twice, `.await` both
    // `JoinHandle`s (unwrap them), and add the results.
    panic!("TODO(module-042): implement spawn_and_sum")
}

/// Spawn `n` tasks; task `i` returns `i + 1`. Await all of them and
/// return the sum of their outputs.
pub async fn spawn_many_sum(_n: u32) -> u64 {
    // TODO(module-042): push one `tokio::spawn` handle per index into a
    // `Vec`, then await every handle and add up the results.
    //
    // Remember: `u64::from(i)` converts a `u32` index.
    panic!("TODO(module-042): implement spawn_many_sum")
}

/// Build a multi-threaded runtime by hand and run a block_on on it.
pub fn multi_thread_blocking_sum(_items: Vec<u64>) -> u64 {
    // TODO(module-042): `tokio::runtime::Builder::new_multi_thread()`
    // with `.worker_threads(2)` and `.enable_all()`, then `.build()`.
    // Inside `runtime.block_on(...)`, spawn one task per item (each
    // returning its item) and sum their outputs.
    panic!("TODO(module-042): implement multi_thread_blocking_sum")
}

/// Build a current-thread runtime by hand and run a block_on on it.
pub fn current_thread_blocking_sum(_items: Vec<u64>) -> u64 {
    // TODO(module-042): same as `multi_thread_blocking_sum`, but with
    // `tokio::runtime::Builder::new_current_thread()`. Note that this
    // runtime has no extra worker threads: everything runs on the thread
    // that calls `block_on`.
    panic!("TODO(module-042): implement current_thread_blocking_sum")
}

/// Spawn `n` tasks that each sleep `millis` milliseconds, await them all,
/// and return the total wall-clock time the whole batch took.
pub async fn parallel_sleep_total(_n: u32, _millis: u64) -> std::time::Duration {
    // TODO(module-042): record `tokio::time::Instant::now()` before the
    // spawns and `start.elapsed()` after all handles are joined.
    // Spawn `n` tasks whose body is
    // `tokio::time::sleep(std::time::Duration::from_millis(millis)).await`.
    // If the sleeps overlap, the total is roughly `millis`, not `n * millis`.
    panic!("TODO(module-042): implement parallel_sleep_total")
}
