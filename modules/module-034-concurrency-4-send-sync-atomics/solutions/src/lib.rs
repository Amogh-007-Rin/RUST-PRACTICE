//! Module 034: reference solution.
//!
//! A lock-free `AtomicCounter`, a parallel increment fan-out, and a
//! compare-and-swap "claim once" flag.

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;

/// A lock-free counter backed by an `AtomicUsize`.
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    /// Creates a counter starting at zero.
    pub fn new() -> Self {
        Self {
            value: AtomicUsize::new(0),
        }
    }

    /// Atomically adds 1 and returns the *new* value.
    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst) + 1
    }

    /// Returns the current value.
    pub fn total(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }
}

impl Default for AtomicCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// Spawns `threads` workers; each increments a shared `Arc<AtomicCounter>`
/// `per_thread` times. Returns the final total, which equals
/// `threads * per_thread`.
pub fn run_atomic_increments(threads: usize, per_thread: usize) -> usize {
    let counter = Arc::new(AtomicCounter::new());
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

/// Atomically flips `flag` from `false` to `true` exactly once. Returns true
/// if this call performed the flip, false if the flag was already true.
pub fn try_claim(flag: &AtomicBool) -> bool {
    flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst)
        .is_ok()
}

/// Compile-time proof: `T` must be safe to move between threads and share
/// between threads. The tests call this with your types.
pub fn assert_thread_safe<T: Send + Sync>() {}
