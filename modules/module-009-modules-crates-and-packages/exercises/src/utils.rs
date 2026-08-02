//! Text helpers — the `utils` module of this crate.

/// Uppercases `s` and appends an exclamation mark.
pub fn shout(s: &str) -> String {
    // TODO(module-009): `format!("{}!", s.to_uppercase())`.
    let _ = s;
    panic!("TODO(module-009): implement utils::shout")
}

/// Returns `true` if `s` is empty or only whitespace.
pub fn is_blank(s: &str) -> bool {
    // TODO(module-009): `s.trim().is_empty()`.
    let _ = s;
    panic!("TODO(module-009): implement utils::is_blank")
}
