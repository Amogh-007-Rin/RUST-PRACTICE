//! Module 002: "make it compile" exercise.
//!
//! This module is special. Unlike every other module in the course, the
//! scaffold here intentionally does NOT compile. Your job is to read the
//! compiler errors, follow the TODOs, and fix the code until
//! `cargo test -p module-002-exercises` compiles AND all tests pass.
//!
//! Run `cargo check -p module-002-exercises` first: rustc will point you at
//! each error. Fix one at a time and re-check.

/// Doubles its argument. TODO: the reassignment below needs a mutable binding.
pub fn double(x: i32) -> i32 {
    let y = x;
    y = y * 2;
    y
}

/// The maximum number of users. TODO: `const` must be initialized with a
/// compile-time constant — replace the function call with a plain literal.
pub const MAX_USERS: u32 = users();

fn users() -> u32 {
    100
}

/// Converts Celsius to Fahrenheit. TODO: the arithmetic mixes `f64` and
/// integer literals, which does not compile — use float literals instead.
pub fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9 / 5 + 32
}

/// Returns the byte length of `word`. TODO: the type annotation claims the
/// value is a `String`, but `word.len()` is a `usize`. Fix the annotation.
pub fn describe_length(word: &str) -> usize {
    let description: String = word.len();
    description
}
