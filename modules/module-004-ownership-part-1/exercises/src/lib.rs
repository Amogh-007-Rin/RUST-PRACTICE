//! Module 004: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Takes ownership of `s` (it is *moved* into this function) and returns its
/// length in bytes. `s` is dropped when this function returns.
pub fn byte_len(s: String) -> usize {
    // TODO(module-004): `s` was moved into this function. Use it — return
    // `s.len()` — and it will be dropped here.
    let _ = s;
    panic!("TODO(module-004): implement byte_len")
}

/// Returns the original `s` *and* a copy of it, so the caller gets both.
///
/// Hint: `String` is not `Copy` — returning `s` moves it out. The other
/// element must be built from a `.clone()` of `s` first.
pub fn copy_of(s: String) -> (String, String) {
    // TODO(module-004): return `(s.clone(), s)`.
    let _ = s;
    panic!("TODO(module-004): implement copy_of")
}

/// Moves both strings into this function and returns them concatenated.
pub fn concat(first: String, second: String) -> String {
    // TODO(module-004): return a new String built from both, e.g. with
    // `format!("{first}{second}")`. Both inputs are dropped when the
    // function returns.
    let _ = (first, second);
    panic!("TODO(module-004): implement concat")
}
