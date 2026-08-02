//! Module 094 — Rust-Specific Interview Questions.
//!
//! Every function here encodes a classic ownership/borrowing gotcha that
//! shows up in real interviews. The scaffold compiles but every function
//! panics: implement each one so the integration tests pass. See the README
//! for the whiteboard-style Q&A that explains *why* each one is a gotcha.

/// A two-field struct used to demonstrate split borrows.
pub struct Counter {
    pub left: u32,
    pub right: u32,
}

/// Moves a value *out* of an `Option` when you only have a `&mut` to it.
///
/// The naive `opt.unwrap()` would move out of borrowed content. The
/// interview answer: `Option::take()` swaps in `None` and hands you the
/// value. Panics when the option is `None` (by design — this is a
/// "give me the value, or this is a bug" operation).
pub fn pop_option<T>(opt: &mut Option<T>) -> T {
    // TODO(module-094): use `opt.take().expect(...)` — `take()` is the
    // only way to move the value out of the `&mut Option<T>`.
    let _ = opt; // placeholder — remove once implemented
    panic!("stub: pop_option is not implemented yet");
}

/// Removes and returns the *first* element of a `Vec`, in O(n).
///
/// `v.pop()` is O(1) but takes the last element; `v[0]` can't move out of
/// a `Vec`. The answer: `v.remove(0)`.
pub fn remove_first<T>(v: &mut Vec<T>) -> T {
    // TODO(module-094): `v.remove(0)` moves the first element out and
    // shifts the rest down.
    let _ = v; // placeholder — remove once implemented
    panic!("stub: remove_first is not implemented yet");
}

/// Returns two disjoint mutable borrows of a struct's fields.
///
/// `let a = &mut c; let b = &mut c;` fails with E0499 — but borrowing two
/// *different fields* is fine: the borrow checker splits the borrow.
/// This is called a split borrow, and it compiles.
pub fn both_mut(c: &mut Counter) -> (&mut u32, &mut u32) {
    // TODO(module-094): return `(&mut c.left, &mut c.right)` — field-level
    // borrows are disjoint, so this typechecks. A single `&mut *c` twice
    // would not.
    let _ = c; // placeholder — remove once implemented
    panic!("stub: both_mut is not implemented yet");
}

/// A struct that holds references — which means it needs a lifetime
/// parameter, because the struct outlives nothing on its own.
#[allow(dead_code)] // read once `first_match` is implemented
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
        // TODO(module-094): find the byte offset of `self.needle` inside
        // `self.haystack` and return the matching slice. The "trick": the
        // return type says `&'a str`, so index into `self.haystack`
        // directly (which is `&'a str`) rather than into `&self`.
        panic!("stub: Searcher::first_match is not implemented yet");
    }
}

/// Uppercases a string, returning an *owned* `String`.
///
/// The gotcha: `fn shout(s: &str) -> &str` is impossible for a freshly
/// *created* string — you'd return a reference to a local, which Rust
/// forbids (E0515). Returning the owned `String` is the fix; callers can
/// borrow it afterwards.
pub fn shout(s: &str) -> String {
    // TODO(module-094): `s.to_uppercase()` returns an owned `String`.
    // Return it directly — do not try to return `&` to it.
    let _ = s; // placeholder — remove once implemented
    panic!("stub: shout is not implemented yet");
}

/// Removes all even numbers from `v` in place, returning how many were
/// removed.
///
/// The gotcha: you cannot `push`/`remove` from a `Vec` while iterating it
/// (`for x in &mut v { ... v.remove(i) ... }` trips the borrow checker).
/// The answer: `Vec::retain` does exactly "keep the elements matching a
/// predicate".
pub fn remove_evens(v: &mut Vec<i32>) -> usize {
    // TODO(module-094): record the original length, call `v.retain(|x| x
    // % 2 != 0)`, and return the difference. Iterators can't safely mutate
    // the thing they borrow — `retain` exists precisely for this.
    let _ = v; // placeholder — remove once implemented
    panic!("stub: remove_evens is not implemented yet");
}

/// Splits one line off the front of a `&mut &str`, advancing the slice.
///
/// The gotcha: `let line = s.split_once('\n')?; *s = ...` borrows `*s`
/// while assigning to it (E0506). The trick: `&str` is `Copy`, so copy the
/// reference *first*, do all the reading through the copy, then assign.
pub fn pop_line<'a>(s: &mut &'a str) -> Option<&'a str> {
    // TODO(module-094): `let rest = *s;` copies the reference. Then split
    // the first line off `rest` with `split_once('\n')`, assign the
    // remainder back into `*s`, and return the line.
    let _ = s; // placeholder — remove once implemented
    panic!("stub: pop_line is not implemented yet");
}
