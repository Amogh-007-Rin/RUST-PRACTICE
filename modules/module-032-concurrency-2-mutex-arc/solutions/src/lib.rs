//! Module 032: reference solution.
//!
//! A `Counter` guarded by a `Mutex`, shared across threads with `Arc`.

use std::sync::{Arc, Mutex};

/// A thread-safe counter. All access to the value goes through a `Mutex`.
///
/// `clippy::mutex_atomic` is allowed deliberately: a single counter is
/// exactly the case where an `AtomicUsize` (Module 034) would be the better
/// production choice, but the point of this exercise is the `Mutex` itself.
#[allow(clippy::mutex_atomic)]
pub struct Counter {
    value: Mutex<usize>,
}

impl Counter {
    /// Creates a counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: Mutex::new(0),
        }
    }

    /// Locks the counter, adds 1, and returns the new value.
    pub fn increment(&self) -> usize {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        *guard
    }

    /// Returns the current value without modifying it.
    pub fn total(&self) -> usize {
        *self.value.lock().unwrap()
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawns `threads` workers; each calls `increment()` `per_thread` times on a
/// counter shared through an `Arc`. Returns the final total, which equals
/// `threads * per_thread`.
pub fn run_threaded_increments(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(Counter::new());
    let mut handles = Vec::new();
    for _ in 0..threads {
        let counter = Arc::clone(&counter);
        handles.push(std::thread::spawn(move || {
            for _ in 0..per_thread {
                counter.increment();
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    counter.total()
}
