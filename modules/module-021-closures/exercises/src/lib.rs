//! Module 021: Closures — exercise scaffold.
//!
//! Fill in every `TODO(module-021)` below so the integration tests in
//! `tests/module_021.rs` pass. The tests define "done".

/// Applies `f` to `x`, then applies `f` to the result.
pub fn apply_twice(_f: impl Fn(i32) -> i32, _x: i32) -> i32 {
    // TODO(module-021): return `f(f(x))` — the closure is `Fn`, so it can
    // be called as many times as you like.
    panic!("not implemented")
}

/// Returns a closure that adds `amount` to its argument.
pub fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    // TODO(module-021): return a closure that captures `amount`, e.g.
    // `move |x| x + amount`. The `move` keyword makes the capture explicit.
    let _ = amount;
    |_x| panic!("not implemented")
}

/// Calls `f` exactly once, consuming any `FnOnce` closure.
pub fn run_once(_f: impl FnOnce() -> usize) -> usize {
    // TODO(module-021): invoke `f` once and return its result.
    panic!("not implemented")
}

/// Calls `f` with each value in `xs` and returns how many times it ran.
pub fn call_counter(_f: impl FnMut(i32), _xs: &[i32]) -> usize {
    // TODO(module-021): iterate over `xs`, call `f(x)` for every element,
    // and return the number of calls you made. The parameter needs `mut`
    // once you actually call it.
    panic!("not implemented")
}
