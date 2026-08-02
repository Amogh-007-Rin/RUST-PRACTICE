//! Module 003: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Returns `true` if `n` is even.
pub fn is_even(n: i32) -> bool {
    // TODO(module-003): return `n % 2 == 0`. (On signed integers that's the
    // idiomatic form — the modern `.is_multiple_of()` method only exists on
    // unsigned integers in the current toolchain.)
    let _ = n;
    panic!("TODO(module-003): implement is_even")
}

/// Returns `"negative"`, `"zero"`, or `"positive"` depending on `n`.
pub fn classify(n: i32) -> &'static str {
    // TODO(module-003): use if/else. Remember: `if` is an expression, so each
    // branch is a value (here: a string literal) with no trailing `;`.
    let _ = n;
    panic!("TODO(module-003): implement classify")
}

/// Returns the sum of all integers from `1` to `n` inclusive (`0` if `n == 0`).
pub fn sum_to(n: u32) -> u32 {
    // TODO(module-003): loop with `for i in 1..=n { ... }` accumulating into a
    // mutable variable.
    let _ = n;
    panic!("TODO(module-003): implement sum_to")
}

/// Returns the number of Collatz steps needed to reach `1`:
/// while `n > 1`, halve it if even, else compute `3 * n + 1`; count the steps.
pub fn collatz_steps(n: u64) -> u32 {
    // TODO(module-003): use a `while n > 1` loop and a mutable counter. Watch
    // out: the loop body must rebind `n` itself — think about shadowing. Use
    // `n.is_multiple_of(2)` to test evenness (unsigned, so the method exists).
    let _ = n;
    panic!("TODO(module-003): implement collatz_steps")
}
