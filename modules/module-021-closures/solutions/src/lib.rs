//! Module 021: Closures — reference solution.

/// Applies `f` to `x`, then applies `f` to the result.
pub fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

/// Returns a closure that adds `amount` to its argument.
pub fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |x| x + amount
}

/// Calls `f` exactly once, consuming any `FnOnce` closure.
pub fn run_once(f: impl FnOnce() -> usize) -> usize {
    f()
}

/// Calls `f` with each value in `xs` and returns how many times it ran.
pub fn call_counter(mut f: impl FnMut(i32), xs: &[i32]) -> usize {
    let mut calls = 0;
    for &x in xs {
        f(x);
        calls += 1;
    }
    calls
}
