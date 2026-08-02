//! Module 019: Testing in Rust.
//!
//! Each function below has a deliberate bug — the tests in
//! `tests/module_019.rs` fail because of it. Fix the `TODO(module-019)` bugs,
//! and also write unit tests in the `tests` module at the bottom of this
//! file.

/// Converts Fahrenheit to Celsius: `(f - 32) * 5 / 9`.
pub fn fahrenheit_to_celsius(f: f64) -> f64 {
    // TODO(module-019): this uses the "quick approximation" formula and is
    // wrong for exact conversions. Use `(f - 32.0) * 5.0 / 9.0`.
    (f - 30.0) / 2.0
}

/// Counts the whitespace-separated words in `text`.
pub fn word_count(text: &str) -> usize {
    // TODO(module-019): this counts characters, not words. Use
    // `text.split_whitespace().count()`.
    text.chars().count()
}

/// Returns `true` if `s` reads the same forwards and backwards.
pub fn is_palindrome(s: &str) -> bool {
    // TODO(module-019): comparing only the first and last character is not
    // enough — "abca" would pass. Compare the whole string with its reverse:
    // `s.chars().eq(s.chars().rev())`.
    s.chars().next() == s.chars().last()
}

/// Returns the `n`-th Fibonacci number: 0, 1, 1, 2, 3, 5, 8, ...
pub fn fibonacci(n: u32) -> u64 {
    // TODO(module-019): the recursive case adds the value twice. The correct
    // recurrence is `fibonacci(n - 1) + fibonacci(n - 2)`.
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 1),
    }
}

/// Returns `true` if `score` is a valid grade: between 50 and 100 inclusive.
pub fn valid_grade(score: u8) -> bool {
    // TODO(module-019): the upper bound is missing — 200 would pass. Add
    // `&& score <= 100`.
    score >= 50
}

#[cfg(test)]
mod tests {
    use super::*;

    // TODO(module-019): write unit tests here for the functions above, for
    // example:
    //
    //   #[test]
    //   fn freezing_point_is_zero_celsius() {
    //       assert_eq!(fahrenheit_to_celsius(32.0), 0.0);
    //   }
    //
    // Run them with `cargo test -p module-019-exercises` and watch them
    // pass once the bugs above are fixed.
}
