//! Module 049: Async Patterns & Pitfalls — exercise scaffold.
//!
//! CPU-intensive work must be offloaded via `spawn_blocking` to avoid
//! starving the async runtime. Holding `std::sync::Mutex` across
//! `.await` can deadlock — use `tokio::sync::Mutex` instead. Blocking
//! the runtime thread (e.g. with `std::thread::sleep`) prevents any
//! other task on that thread from making progress.

/// Compute the nth Fibonacci number. This is a CPU-bound synchronous
/// function meant to be called via `spawn_blocking` in async context.
pub fn fibonacci_sync(n: u64) -> u64 {
    // TODO(module-049): implement iterative fibonacci:
    //   if n < 2 { return n; }
    //   a=0, b=1; for _ in 1..n { (a, b) = (b, a + b) }; b
    let _ = n;
    panic!("TODO(module-049): implement fibonacci_sync")
}

/// Run `fibonacci_sync` on a blocking thread pool so the async runtime
/// stays responsive. Return the result.
pub async fn cpu_intensive_task(n: u64) -> u64 {
    // TODO(module-049): call `tokio::task::spawn_blocking(move || fibonacci_sync(n)).await.unwrap()`
    let _ = n;
    panic!("TODO(module-049): implement cpu_intensive_task")
}

/// Demonstrate that `tokio::sync::Mutex` can be held across `.await`
/// without deadlocking (unlike `std::sync::Mutex`). Two tasks contend
/// for the mutex; the first holds it across a sleep. Return `true` if
/// the second task eventually acquires the lock and reads the value set
/// by the first task.
pub async fn demonstrate_deadlock_avoidance() -> bool {
    // TODO(module-049):
    //   1. Create `Arc<tokio::sync::Mutex<u32>>` with initial value 0.
    //   2. Spawn task A: lock, sleep 50ms, set value to 42, drop guard.
    //   3. Sleep 10ms so task A acquires the lock first.
    //   4. Spawn task B: lock, read value, return it.
    //   5. Await both tasks, return task B's result == 42.
    panic!("TODO(module-049): implement demonstrate_deadlock_avoidance")
}

/// This function performs blocking CPU work (iteration and allocation).
/// In production, it should be called via `spawn_blocking` rather than
/// directly from an async context — otherwise it stalls the runtime
/// thread.
pub fn blocking_computation(input: &str) -> String {
    // Simulate a CPU-bound operation: reverse the string character by
    // character and build a result (deliberately non-optimized).
    let mut result = String::with_capacity(input.len());
    for ch in input.chars().rev() {
        result.push(ch);
        // Tiny artificial delay to simulate heavier work.
        std::thread::sleep(std::time::Duration::from_micros(100));
    }
    result
}
