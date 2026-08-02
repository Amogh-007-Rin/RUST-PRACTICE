//! Module 094 — Rust-Specific Interview Questions.
//!
//! Reference solution. Compare against your `exercises/` implementation
//! after you have made a genuine attempt.

/// A two-field struct used to demonstrate split borrows.
pub struct Counter {
    pub left: u32,
    pub right: u32,
}

/// Moves a value *out* of an `Option` when you only have a `&mut` to it.
pub fn pop_option<T>(opt: &mut Option<T>) -> T {
    opt.take().expect("pop_option called on None")
}

/// Removes and returns the *first* element of a `Vec`, in O(n).
pub fn remove_first<T>(v: &mut Vec<T>) -> T {
    v.remove(0)
}

/// Returns two disjoint mutable borrows of a struct's fields.
pub fn both_mut(c: &mut Counter) -> (&mut u32, &mut u32) {
    (&mut c.left, &mut c.right)
}

/// A struct that holds references — which means it needs a lifetime
/// parameter, because the struct outlives nothing on its own.
pub struct Searcher<'a> {
    haystack: &'a str,
    needle: &'a str,
}

impl<'a> Searcher<'a> {
    /// Creates a searcher borrowing `haystack` and `needle` for `'a`.
    pub fn new(haystack: &'a str, needle: &'a str) -> Self {
        Self { haystack, needle }
    }

    /// Returns the first occurrence of the needle inside the haystack, or
    /// `None` when absent. The returned slice borrows from the *original*
    /// haystack string, with lifetime `'a` — not from `&self`.
    pub fn first_match(&self) -> Option<&'a str> {
        let offset = self.haystack.find(self.needle)?;
        Some(&self.haystack[offset..offset + self.needle.len()])
    }
}

/// Uppercases a string, returning an *owned* `String`.
pub fn shout(s: &str) -> String {
    s.to_uppercase()
}

/// Removes all even numbers from `v` in place, returning how many were
/// removed.
pub fn remove_evens(v: &mut Vec<i32>) -> usize {
    let before = v.len();
    v.retain(|x| x % 2 != 0);
    before - v.len()
}

/// Splits one line off the front of a `&mut &str`, advancing the slice.
pub fn pop_line<'a>(s: &mut &'a str) -> Option<&'a str> {
    let rest = *s; // &str is Copy: read everything through this copy
    let (line, after) = rest.split_once('\n')?;
    *s = after; // now safe: no live borrow of `*s`
    Some(line)
}
