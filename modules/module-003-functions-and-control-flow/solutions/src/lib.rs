//! Module 003: solution — the reference implementation.

/// Returns `true` if `n` is even.
pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}

/// Returns `"negative"`, `"zero"`, or `"positive"` depending on `n`.
pub fn classify(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    }
}

/// Returns the sum of all integers from `1` to `n` inclusive (`0` if `n == 0`).
pub fn sum_to(n: u32) -> u32 {
    let mut total = 0;
    for i in 1..=n {
        total += i;
    }
    total
}

/// Returns the number of Collatz steps needed to reach `1`.
pub fn collatz_steps(n: u64) -> u32 {
    let mut n = n;
    let mut steps = 0;
    while n > 1 {
        n = if n.is_multiple_of(2) {
            n / 2
        } else {
            3 * n + 1
        };
        steps += 1;
    }
    steps
}
