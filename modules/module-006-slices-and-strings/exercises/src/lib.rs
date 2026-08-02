//! Module 006: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Returns the first word of `s` (everything up to the first whitespace).
///
/// `s` is borrowed, and the returned `&str` is a *slice* of `s` — no
/// allocation happens.
pub fn first_word(s: &str) -> &str {
    // TODO(module-006): find the first space with `s.find(' ')`; return the
    // slice `&s[..index]` or the whole `s` if there is no space.
    let _ = s;
    panic!("TODO(module-006): implement first_word")
}

/// Returns the byte slice of `s` from `start` to `end` (end-exclusive).
pub fn slice_range(s: &str, start: usize, end: usize) -> &str {
    // TODO(module-006): return `&s[start..end]`.
    let _ = (s, start, end);
    panic!("TODO(module-006): implement slice_range")
}

/// Counts whitespace-separated words in `s`.
pub fn word_count(s: &str) -> usize {
    // TODO(module-006): `s.split_whitespace().count()`.
    let _ = s;
    panic!("TODO(module-006): implement word_count")
}

/// Returns an owned, uppercased `String` for `s`, so the caller keeps `s`.
pub fn shout(s: &str) -> String {
    // TODO(module-006): `s.to_uppercase()` returns an owned `String`.
    let _ = s;
    panic!("TODO(module-006): implement shout")
}
