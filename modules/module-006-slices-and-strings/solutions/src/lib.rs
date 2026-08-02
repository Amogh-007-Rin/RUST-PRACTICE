//! Module 006: solution — the reference implementation.

/// Returns the first word of `s` (everything up to the first whitespace).
pub fn first_word(s: &str) -> &str {
    match s.find(' ') {
        Some(index) => &s[..index],
        None => s,
    }
}

/// Returns the byte slice of `s` from `start` to `end` (end-exclusive).
pub fn slice_range(s: &str, start: usize, end: usize) -> &str {
    &s[start..end]
}

/// Counts whitespace-separated words in `s`.
pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

/// Returns an owned, uppercased `String` for `s`, so the caller keeps `s`.
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}
