//! Module 001: solution — the reference implementation.

/// Returns a greeting for the given name, e.g. `greet("Ada")` -> `"Hello, Ada!"`.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

/// Returns the length in bytes of the given message.
pub fn message_length(message: &str) -> usize {
    message.len()
}
