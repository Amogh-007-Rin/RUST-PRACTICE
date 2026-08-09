use module_054_solutions::{compute_self_time, find_hotspot, first_slow_sample, parse_call_trace};

#[test]
fn parse_single_line() {
    let result = parse_call_trace("main 42");
    assert_eq!(result, vec![("main", 42)]);
}

#[test]
fn parse_multiple_lines() {
    let trace = "main 100\nfoo 20\nfoo 30\nbar 50\n";
    let result = parse_call_trace(trace);
    assert_eq!(
        result,
        vec![("main", 100), ("foo", 20), ("foo", 30), ("bar", 50)]
    );
}

#[test]
fn parse_ignores_blanks_and_comments() {
    let trace = "main 100\n\n# this is a comment\nfoo 25\n";
    let result = parse_call_trace(trace);
    assert_eq!(result, vec![("main", 100), ("foo", 25)]);
}

#[test]
fn parse_trailing_newline() {
    let result = parse_call_trace("func 10\n");
    assert_eq!(result, vec![("func", 10)]);
}

#[test]
fn self_time_sums_correctly() {
    let traces = vec![("main", 100), ("foo", 20), ("foo", 30), ("bar", 50)];
    assert_eq!(compute_self_time(&traces, "foo"), 50);
    assert_eq!(compute_self_time(&traces, "main"), 100);
    assert_eq!(compute_self_time(&traces, "bar"), 50);
    assert_eq!(compute_self_time(&traces, "baz"), 0);
}

#[test]
fn self_time_empty_trace() {
    assert_eq!(compute_self_time(&[], "anything"), 0);
}

#[test]
fn hotspot_finds_max_total() {
    let traces = vec![("main", 40), ("parse", 30), ("parse", 40), ("write", 20)];
    assert_eq!(find_hotspot(&traces), Some("parse"));
}

#[test]
fn hotspot_empty_trace() {
    assert_eq!(find_hotspot(&[]), None);
}

#[test]
fn hotspot_single_entry() {
    let traces = vec![("main", 100)];
    assert_eq!(find_hotspot(&traces), Some("main"));
}

#[test]
fn hotspot_tie_returns_one() {
    let traces = vec![("a", 50), ("b", 50)];
    let result = find_hotspot(&traces);
    assert!(result == Some("a") || result == Some("b"));
}

#[test]
fn slow_sample_returns_first_value_above_threshold() {
    let samples = [12, 18, 27, 41];
    assert_eq!(first_slow_sample(&samples, 25), Some((2, 27)));
}

#[test]
fn slow_sample_treats_threshold_as_acceptable() {
    let samples = [12, 25, 27];
    assert_eq!(first_slow_sample(&samples, 25), Some((2, 27)));
}

#[test]
fn slow_sample_handles_empty_and_fast_inputs() {
    assert_eq!(first_slow_sample(&[], 25), None);
    assert_eq!(first_slow_sample(&[10, 20, 24], 25), None);
}
