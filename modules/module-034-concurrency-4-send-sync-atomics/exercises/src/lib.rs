//! Module 034: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

use std::sync::atomic::{AtomicBool, AtomicUsize};

/// A lock-free counter backed by an `AtomicUsize`.
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    /// Creates a counter starting at zero.
    pub fn new() -> Self {
        // TODO(module-034): return `Self { value: AtomicUsize::new(0) }`.
        panic!("TODO(module-034): implement AtomicCounter::new")
    }

    /// Atomically adds 1 and returns the *new* value.
    pub fn increment(&self) -> usize {
        // TODO(module-034): use `self.value.fetch_add(1, Ordering::SeqCst)`
        // and return the new value (the fetch returns the old one).
        let _ = &self.value;
        panic!("TODO(module-034): implement AtomicCounter::increment")
    }

    /// Returns the current value.
    pub fn total(&self) -> usize {
        // TODO(module-034): use `self.value.load(Ordering::SeqCst)`.
        panic!("TODO(module-034): implement AtomicCounter::total")
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawns `threads` workers; each increments a shared `Arc<AtomicCounter>`
/// `per_thread` times. Returns the final total, which must equal
/// `threads * per_thread`.
pub fn run_atomic_increments(_threads: usize, _per_thread: usize) -> usize {
    // TODO(module-034): rename `_threads` and `_per_thread` (dropping the
    // underscores), wrap an `AtomicCounter::new()` in an `Arc`, clone the
    // `Arc` into each thread, and join everything before reading `total()`.
    panic!("TODO(module-034): implement run_atomic_increments")
}

/// Atomically flips `flag` from `false` to `true` exactly once. Returns true
/// if this call performed the flip, false if the flag was already true.
pub fn try_claim(_flag: &AtomicBool) -> bool {
    // TODO(module-034): rename `_flag` to `flag`, then use
    // `flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)`
    // and return whether it succeeded.
    panic!("TODO(module-034): implement try_claim")
}

/// Compile-time proof: `T` must be safe to move between threads and share
/// between threads. The tests call this with your types.
pub fn assert_thread_safe<T: Send + Sync>() {}
