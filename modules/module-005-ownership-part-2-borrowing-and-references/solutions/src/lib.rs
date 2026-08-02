//! Module 005: solution — the reference implementation.

/// Returns the first character of `s` as an `Option<char>` (`None` if empty),
/// borrowing `s` immutably.
pub fn first_char(s: &str) -> Option<char> {
    s.chars().next()
}

/// Adds one to `n` *in place*, through a mutable reference.
pub fn add_one(n: &mut i32) {
    *n += 1;
}

/// Returns the sum of the byte lengths of two borrowed strings.
pub fn total_length(a: &str, b: &str) -> usize {
    a.len() + b.len()
}

/// Swaps the values of two `i32`s through two mutable references.
pub fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}
