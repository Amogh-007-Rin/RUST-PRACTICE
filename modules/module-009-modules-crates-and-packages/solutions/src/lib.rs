//! Module 009: solution — the reference implementation.
//!
//! This crate is organized into multiple files: `lib.rs` declares the child
//! modules, and each module lives in its own file (`math.rs`, `utils.rs`).

pub mod math;
pub mod utils;

use utils::shout;

/// Adds `a` and `b` via the `math` module, then shouts the result via
/// `utils::shout`. Demonstrates how modules combine through `use`.
pub fn shout_sum(a: i32, b: i32) -> String {
    let sum = math::add(a, b);
    shout(&sum.to_string())
}
