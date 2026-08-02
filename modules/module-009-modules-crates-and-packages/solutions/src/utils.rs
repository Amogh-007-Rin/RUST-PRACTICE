//! Text helpers — the `utils` module of this crate.

/// Uppercases `s` and appends an exclamation mark.
pub fn shout(s: &str) -> String {
    format!("{}!", s.to_uppercase())
}

/// Returns `true` if `s` is empty or only whitespace.
pub fn is_blank(s: &str) -> bool {
    s.trim().is_empty()
}
