//! Module 018: Lifetimes — annotations, elision rules, and structs holding
//! references.
//!
//! The signatures are fixed (the `'a` annotations are part of the exercise —
//! read them and understand them). Fill in the `TODO(module-018)` bodies so
//! the integration tests in `tests/module_018.rs` pass.
//!
//! Clippy's `needless_lifetimes` is deliberately allowed: this module teaches
//! explicit lifetime annotations, which are optional (elidable) for
//! single-input functions but required for multi-input ones like `longest`.
#![allow(clippy::needless_lifetimes)]

/// Returns the first word of `s` (up to the first whitespace).
pub fn first_word<'a>(s: &'a str) -> &'a str {
    // TODO(module-018): `s.split_whitespace().next().unwrap_or("")` gives the
    // first word, or the empty string when there is none.
    s
}

/// Returns the longer of the two strings.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO(module-018): compare with `x.len() >= y.len()` and return the
    // longer. The `'a` is why this compiles at all: the returned reference
    // is valid for as long as *both* inputs are.
    let _y_len = y.len();
    x
}

/// Returns the last word of `s` (after the last whitespace).
pub fn last<'a>(s: &'a str) -> &'a str {
    // TODO(module-018): `s.split_whitespace().next_back().unwrap_or("")`.
    s
}

/// Returns a reference to the longest line in `lines`.
pub fn longest_line<'a>(lines: &'a [String]) -> &'a str {
    // TODO(module-018): `lines.iter().max_by_key(|line| line.len())` gives
    // `Option<&String>`; `.map(String::as_str)` turns it into `Option<&str>`;
    // `.unwrap_or("")` handles the empty slice.
    let _count = lines.len();
    ""
}

/// A book whose title and author are borrowed strings.
pub struct Book<'a> {
    pub title: &'a str,
    pub author: &'a str,
}

impl<'a> Book<'a> {
    /// Creates a book borrowing `title` and `author`.
    pub fn new(title: &'a str, author: &'a str) -> Self {
        Book { title, author }
    }

    /// Returns the book's title.
    ///
    /// No annotation needed: the elision rules tie the returned reference to
    /// `&self` — there is only one input lifetime.
    pub fn title(&self) -> &str {
        // TODO(module-018): return `self.title`.
        panic!("not implemented")
    }

    /// Returns a citation like "Author — Title".
    pub fn citation(&self) -> String {
        // TODO(module-018): `format!("{} — {}", self.author, self.title)`.
        String::new()
    }
}

/// Returns the first and last words of `s` as a pair.
pub fn first_and_last<'a>(s: &'a str) -> (&'a str, &'a str) {
    // TODO(module-018): combine `first_word` and `last` — both return
    // references valid for `'a`, so the tuple type-checks.
    (s, s)
}
