//! Module 004: solution — the reference implementation.

/// Takes ownership of `s` (it is *moved* into this function) and returns its
/// length in bytes. `s` is dropped when this function returns.
pub fn byte_len(s: String) -> usize {
    s.len()
}

/// Returns the original `s` *and* a copy of it, so the caller gets both.
pub fn copy_of(s: String) -> (String, String) {
    (s.clone(), s)
}

/// Moves both strings into this function and returns them concatenated.
pub fn concat(first: String, second: String) -> String {
    format!("{first}{second}")
}
