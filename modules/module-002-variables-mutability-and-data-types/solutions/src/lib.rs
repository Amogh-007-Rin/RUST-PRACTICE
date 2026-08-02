//! Module 002: solution — the reference implementation.

/// Doubles its argument.
pub fn double(x: i32) -> i32 {
    let mut y = x;
    y *= 2;
    y
}

/// The maximum number of users.
pub const MAX_USERS: u32 = 100;

/// Converts Celsius to Fahrenheit.
pub fn fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

/// Returns the byte length of `word`.
pub fn describe_length(word: &str) -> usize {
    word.len()
}
