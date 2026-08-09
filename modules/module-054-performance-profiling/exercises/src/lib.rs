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

/// Return the index and duration of the first sample strictly above `threshold`.
///
/// This scaffold contains an intentional boundary bug. Use the `debug_lab`
/// binary and a native debugger to inspect the comparison, then fix it.
#[inline(never)]
pub fn first_slow_sample(samples: &[u64], threshold: u64) -> Option<(usize, u64)> {
    // TODO(module-054): debug why a sample equal to the threshold is reported
    // as slow. Keep the function non-inlined so debuggers can break on it.
    samples
        .iter()
        .copied()
        .enumerate()
        .find(|(_, duration)| *duration >= threshold)
}
