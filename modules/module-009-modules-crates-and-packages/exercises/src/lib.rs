//! Module 009: exercise scaffold.
//!
//! This crate is organized into multiple files: `lib.rs` declares the child
//! modules, and each module lives in its own file (`math.rs`, `utils.rs`).
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

pub mod math;
pub mod utils;

/// Adds `a` and `b` via the `math` module, then shouts the result via
/// `utils::shout`. Demonstrates how modules combine through `use`.
pub fn shout_sum(a: i32, b: i32) -> String {
    // TODO(module-009): first add `use utils::shout;` at the top of this file
    // (above `pub mod`), then implement: `let sum = math::add(a, b);` and
    // return `shout(&sum.to_string())`.
    let _ = (a, b);
    panic!("TODO(module-009): implement shout_sum")
}
