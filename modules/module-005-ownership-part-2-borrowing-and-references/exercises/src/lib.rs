//! Module 005: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//!
//! Note: all functions here take *references* — no ownership changes hands.

/// Returns the first character of `s` as an `Option<char>` (`None` if empty),
/// borrowing `s` immutably.
pub fn first_char(s: &str) -> Option<char> {
    // TODO(module-005): `s.chars().next()` — an immutable borrow is enough.
    let _ = s;
    panic!("TODO(module-005): implement first_char")
}

/// Adds one to `n` *in place*, through a mutable reference.
pub fn add_one(n: &mut i32) {
    // TODO(module-005): dereference with `*n` and mutate it: `*n += 1`.
    let _ = n;
    panic!("TODO(module-005): implement add_one")
}

/// Returns the sum of the byte lengths of two borrowed strings.
pub fn total_length(a: &str, b: &str) -> usize {
    // TODO(module-005): multiple immutable borrows are fine at the same time.
    let _ = (a, b);
    panic!("TODO(module-005): implement total_length")
}

/// Swaps the values of two `i32`s through two mutable references.
pub fn swap(a: &mut i32, b: &mut i32) {
    // TODO(module-005): the manual way is `let tmp = *a; *a = *b; *b = tmp;`
    // — but clippy's `manual_swap` lint flags that, so idiomatic code calls
    // the std function: `std::mem::swap(a, b);`.
    let _ = (a, b);
    panic!("TODO(module-005): implement swap")
}
