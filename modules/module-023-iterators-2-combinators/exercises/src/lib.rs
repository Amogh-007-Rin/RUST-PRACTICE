//! Module 023: Iterators II — exercise scaffold.
//!
//! Fill in every `TODO(module-023)` below so the integration tests in
//! `tests/module_023.rs` pass. The tests define "done".

/// Returns a new vector with the squares of the even numbers in `v`,
/// in the original relative order.
pub fn squares_of_evens(_v: &[i32]) -> Vec<i32> {
    // TODO(module-023): `v.iter()` -> `filter` for even values -> `map` to
    // the square -> `collect`.
    panic!("not implemented")
}

/// Returns the sum of the squares of all values in `v`.
pub fn sum_of_squares(_v: &[i32]) -> i64 {
    // TODO(module-023): `map` each value to its square, then `sum` the
    // resulting iterator.
    panic!("not implemented")
}

/// Counts the whitespace-separated words in `s`.
pub fn count_words(_s: &str) -> usize {
    // TODO(module-023): `s.split_whitespace()` then `count`.
    panic!("not implemented")
}

/// Counts the words in `s` whose length is at most `max_len`.
pub fn count_short_words(_s: &str, _max_len: usize) -> usize {
    // TODO(module-023): like `count_words`, but `filter` by length first.
    panic!("not implemented")
}

/// Returns the longest word in `s`, or `None` if `s` has no words.
pub fn longest_word(_s: &str) -> Option<&str> {
    // TODO(module-023): `max_by_key(|w| w.len())`.
    panic!("not implemented")
}

/// Counts the positive values in `v` using `fold`.
pub fn count_positive(_v: &[i32]) -> usize {
    // TODO(module-023): `fold` starting at `0`, adding 1 when a value is
    // positive. Keep the closure non-trivial — clippy will suggest `sum`
    // or `count` for a plain `acc + x` fold.
    panic!("not implemented")
}

/// Computes the dot product of `a` and `b`, or `None` if their lengths
/// differ.
pub fn dot_product(_a: &[i32], _b: &[i32]) -> Option<i64> {
    // TODO(module-023): check lengths first, then `zip`, `map` to the
    // product, and `sum`.
    panic!("not implemented")
}

/// Returns `true` if `haystack` contains any of the `needles`.
pub fn contains_any(_needles: &[&str], _haystack: &str) -> bool {
    // TODO(module-023): `any` over `needles`, testing `haystack.contains`.
    panic!("not implemented")
}

/// Returns `true` if every value in `v` is even.
pub fn is_all_even(_v: &[i32]) -> bool {
    // TODO(module-023): `all` over `v`, testing evenness.
    panic!("not implemented")
}
