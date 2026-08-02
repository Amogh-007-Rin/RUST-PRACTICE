//! Module 032: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

use std::sync::Mutex;

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
        // TODO(module-032): return `Self { value: Mutex::new(0) }`.
        panic!("TODO(module-032): implement Counter::new")
    }

    /// Locks the counter, adds 1, and returns the new value.
    pub fn increment(&self) -> usize {
        // TODO(module-032): lock, `*guard += 1`, and return the new value
        // while the guard is still held.
        let _ = &self.value;
        panic!("TODO(module-032): implement Counter::increment")
    }

    /// Returns the current value without modifying it.
    pub fn total(&self) -> usize {
        // TODO(module-032): lock and return the value behind the guard.
        panic!("TODO(module-032): implement Counter::total")
    }
}

impl Default for Counter {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawns `threads` workers; each calls `increment()` `per_thread` times on a
/// counter shared through an `Arc`. Returns the final total, which must equal
/// `threads * per_thread`.
pub fn run_threaded_increments(_threads: usize, _per_thread: usize) -> usize {
    // TODO(module-032): rename `_threads` and `_per_thread` (dropping the
    // underscores), wrap a `Counter::new()` in an `Arc`, and clone the `Arc`
    // into each spawned thread. Join every handle before reading the total.
    panic!("TODO(module-032): implement run_threaded_increments")
}
