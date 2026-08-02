//! Module 055: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

use std::time::Duration;

/// The result of comparing two implementations.
#[derive(Debug, Clone)]
pub struct CompareResult {
    pub name1: String,
    pub time1: Duration,
    pub name2: String,
    pub time2: Duration,
    pub faster: String,
    pub speedup: f64,
}

/// Run `f` for `iterations` times.
///
/// Returns the result of the **last** call and the **average** duration
/// per iteration.
pub fn time_execution<F: FnMut() -> R, R>(f: F, iterations: u32) -> (R, Duration) {
    let _ = (f, iterations);
    panic!("TODO(module-055): implement time_execution")
}

/// Compare two closures by timing each for `iterations` calls.
///
/// Returns a [`CompareResult`] that names the faster implementation
/// and reports the speedup (slower / faster). If the times are equal
/// (within 1 ns), the `faster` field is `"tie"` and `speedup` is 1.0.
pub fn compare<F1, F2>(name1: &str, f1: F1, name2: &str, f2: F2, iterations: u32) -> CompareResult
where
    F1: FnMut() + Send,
    F2: FnMut() + Send,
{
    let _ = (name1, f1, name2, f2, iterations);
    panic!("TODO(module-055): implement compare")
}
