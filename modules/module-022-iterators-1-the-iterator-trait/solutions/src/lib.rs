//! Module 022: Iterators I — reference solution.

/// Yields the inclusive range `start..=end`, one value per call to `next`.
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
        if self.current > self.end {
            return None;
        }
        let value = self.current;
        self.current += 1;
        Some(value)
    }
}

/// Yields the Fibonacci numbers `1, 1, 2, 3, 5, 8, ...` forever — it never
/// returns `None`.
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
        let current = self.a;
        (self.a, self.b) = (self.b, self.a + self.b);
        Some(current)
    }
}

/// Sums the even numbers in `v`, using a `for` loop.
pub fn sum_evens(v: &[i32]) -> i32 {
    let mut total = 0;
    for &x in v {
        if x % 2 == 0 {
            total += x;
        }
    }
    total
}

/// Returns the first value in `v` greater than `threshold`, driving an
/// iterator by hand with `next()`.
pub fn first_greater(v: &[i32], threshold: i32) -> Option<i32> {
    let mut it = v.iter();
    loop {
        match it.next() {
            Some(&x) if x > threshold => return Some(x),
            Some(_) => {}
            None => return None,
        }
    }
}
