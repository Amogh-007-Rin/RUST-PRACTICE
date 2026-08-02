//! Module 018: Lifetimes — annotations, elision rules, and structs holding
//! references (reference solution).
//!
//! Clippy's `needless_lifetimes` is deliberately allowed: this module teaches
//! explicit lifetime annotations, which are optional (elidable) for
//! single-input functions but required for multi-input ones like `longest`.
#![allow(clippy::needless_lifetimes)]

/// Returns the first word of `s` (up to the first whitespace).
pub fn first_word<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next().unwrap_or("")
}

/// Returns the longer of the two strings.
pub fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() {
        x
    } else {
        y
    }
}

/// Returns the last word of `s` (after the last whitespace).
pub fn last<'a>(s: &'a str) -> &'a str {
    s.split_whitespace().next_back().unwrap_or("")
}

/// Returns a reference to the longest line in `lines`.
pub fn longest_line<'a>(lines: &'a [String]) -> &'a str {
    lines
        .iter()
        .max_by_key(|line| line.len())
        .map(String::as_str)
        .unwrap_or("")
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
        self.title
    }

    /// Returns a citation like "Author — Title".
    pub fn citation(&self) -> String {
        format!("{} — {}", self.author, self.title)
    }
}

/// Returns the first and last words of `s` as a pair.
pub fn first_and_last<'a>(s: &'a str) -> (&'a str, &'a str) {
    (first_word(s), last(s))
}
