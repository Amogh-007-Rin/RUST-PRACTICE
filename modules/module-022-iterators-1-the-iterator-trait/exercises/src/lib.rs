//! Module 022: Iterators I — exercise scaffold.
//!
//! Fill in every `TODO(module-022)` below so the integration tests in
//! `tests/module_022.rs` pass. The tests define "done".

/// Yields the inclusive range `start..=end`, one value per call to `next`.
///
/// Fields are not read until you implement `next` — that is the TODO.
#[allow(dead_code)]
pub struct Step {
    current: i64,
    end: i64,
}

impl Step {
    /// Creates a `Step` that yields `start`, `start + 1`, ..., `end`.
    /// If `start > end`, it yields nothing.
    pub fn new(start: i64, end: i64) -> Self {
        Self {
            current: start,
            end,
        }
    }
}

impl Iterator for Step {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        // TODO(module-022): if `current > end`, return `None`. Otherwise
        // return `Some(current)` and advance `current` by one.
        panic!("not implemented")
    }
}

/// Yields the Fibonacci numbers `1, 1, 2, 3, 5, 8, ...` forever — it never
/// returns `None`.
///
/// Fields are not read until you implement `next` — that is the TODO.
#[allow(dead_code)]
pub struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    /// Creates a fresh Fibonacci iterator.
    pub fn new() -> Self {
        Self { a: 1, b: 1 }
    }
}

impl Default for Fibonacci {
    fn default() -> Self {
        Self::new()
    }
}

impl Iterator for Fibonacci {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        // TODO(module-022): return the next Fibonacci number and advance
        // the pair `(a, b)` to `(b, a + b)`.
        panic!("not implemented")
    }
}

/// Sums the even numbers in `v`, using a `for` loop.
pub fn sum_evens(_v: &[i32]) -> i32 {
    // TODO(module-022): `for &x in v`, add `x` to a running total only
    // when `x % 2 == 0`.
    panic!("not implemented")
}

/// Returns the first value in `v` greater than `threshold`, driving an
/// iterator by hand with `next()`.
pub fn first_greater(_v: &[i32], _threshold: i32) -> Option<i32> {
    // TODO(module-022): `let mut it = v.iter();` then drive it by hand with
    // `it.next()` in a `loop`/`match` (or any equivalent), returning the
    // first value strictly greater than `threshold`.
    panic!("not implemented")
}
