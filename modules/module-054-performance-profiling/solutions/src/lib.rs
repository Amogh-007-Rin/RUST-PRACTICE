//! Module 054: solution — the reference implementation.

use std::collections::HashMap;

pub fn parse_call_trace(trace: &str) -> Vec<(&str, u64)> {
    trace
        .lines()
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| {
            let (name, dur) = line.rsplit_once(' ')?;
            let dur = dur.parse::<u64>().ok()?;
            Some((name, dur))
        })
        .collect()
}

pub fn compute_self_time(traces: &[(&str, u64)], fn_name: &str) -> u64 {
    traces
        .iter()
        .filter(|(name, _)| *name == fn_name)
        .map(|(_, dur)| dur)
        .sum()
}

pub fn find_hotspot<'a>(traces: &[(&'a str, u64)]) -> Option<&'a str> {
    let mut totals: HashMap<&str, u64> = HashMap::new();
    for (name, dur) in traces {
        *totals.entry(name).or_default() += dur;
    }
    totals
        .into_iter()
        .max_by_key(|(_, dur)| *dur)
        .map(|(name, _)| name)
}
