//! Module 054: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Parse a multi-line call trace into (function_name, duration_ms) pairs.
///
/// Each line has the format: `function_name duration`.
/// Blank lines and lines starting with `#` are ignored.
pub fn parse_call_trace(trace: &str) -> Vec<(&str, u64)> {
    let _ = trace;
    panic!("TODO(module-054): implement parse_call_trace")
}

/// Sum the durations for all calls to `fn_name` in the trace.
pub fn compute_self_time(traces: &[(&str, u64)], fn_name: &str) -> u64 {
    let _ = (traces, fn_name);
    panic!("TODO(module-054): implement compute_self_time")
}

/// Find the function name with the highest total (inclusive) time.
///
/// Returns `None` if the trace is empty.
pub fn find_hotspot<'a>(traces: &[(&'a str, u64)]) -> Option<&'a str> {
    let _ = traces;
    panic!("TODO(module-054): implement find_hotspot")
}
