use std::fs;
use std::path::PathBuf;

use capstone_04_solution::{merge_stats, parse_line, process_file, process_files, LogLevel, Stats};

fn write_log(dir: &tempfile::TempDir, name: &str, lines: &[&str]) -> PathBuf {
    let path = dir.path().join(name);
    fs::write(&path, lines.join("\n")).unwrap();
    path
}

#[test]
fn level_from_str_macro_parses_levels() {
    assert_eq!(
        capstone_04_solution::level_from_str!("DEBUG"),
        Some(LogLevel::Debug)
    );
    assert_eq!(
        capstone_04_solution::level_from_str!("INFO"),
        Some(LogLevel::Info)
    );
    assert_eq!(
        capstone_04_solution::level_from_str!("WARN"),
        Some(LogLevel::Warn)
    );
    assert_eq!(
        capstone_04_solution::level_from_str!("ERROR"),
        Some(LogLevel::Error)
    );
    assert_eq!(
        capstone_04_solution::level_from_str!("FATAL"),
        None::<LogLevel>
    );
}

#[test]
fn parse_latency_macro_parses_ms_values() {
    assert_eq!(
        capstone_04_solution::parse_latency!("latency=123ms"),
        Some(123)
    );
    assert_eq!(capstone_04_solution::parse_latency!("latency=0ms"), Some(0));
    assert_eq!(
        capstone_04_solution::parse_latency!("latency=ms"),
        None::<u64>
    );
    assert_eq!(
        capstone_04_solution::parse_latency!("latency=123"),
        None::<u64>
    );
    assert_eq!(
        capstone_04_solution::parse_latency!("latency=abcms"),
        None::<u64>
    );
    assert_eq!(
        capstone_04_solution::parse_latency!("duration=99ms"),
        None::<u64>
    );
}

#[test]
fn parse_line_extracts_level_and_message() {
    let entry = parse_line("2026-08-02T12:00:00 ERROR request failed").unwrap();
    assert_eq!(entry.level, LogLevel::Error);
    assert_eq!(entry.message, "request failed");
    assert_eq!(entry.latency_ms, None);
}

#[test]
fn parse_line_extracts_latency() {
    let entry = parse_line("2026-08-02T12:00:00 INFO request completed latency=123ms").unwrap();
    assert_eq!(entry.level, LogLevel::Info);
    assert_eq!(entry.message, "request completed");
    assert_eq!(entry.latency_ms, Some(123));
}

#[test]
fn parse_line_latency_in_any_token_position() {
    let entry = parse_line("2026-08-02T12:00:00 WARN latency=99ms slow path").unwrap();
    assert_eq!(entry.message, "slow path");
    assert_eq!(entry.latency_ms, Some(99));
}

#[test]
fn parse_line_handles_empty_and_comment_lines() {
    assert_eq!(parse_line(""), None);
    assert_eq!(parse_line("   "), None);
    assert_eq!(parse_line("# a comment"), None);
}

#[test]
fn parse_line_rejects_unknown_level() {
    assert_eq!(parse_line("2026-08-02T12:00:00 FATAL boom"), None);
}

#[test]
fn stats_percentiles_are_exact() {
    let mut stats = Stats::default();
    for ms in [10u64, 20, 30, 40] {
        stats.latencies.push(ms);
    }
    assert_eq!(stats.percentile(0.50), Some(20));
    assert_eq!(stats.percentile(0.90), Some(40));
    assert_eq!(stats.percentile(1.0), Some(40));
}

#[test]
fn stats_percentiles_empty() {
    let stats = Stats::default();
    assert_eq!(stats.percentile(0.50), None);
}

#[test]
fn stats_error_count_counts_errors() {
    let mut stats = Stats::default();
    *stats.count_by_level.entry(LogLevel::Error).or_insert(0) += 2;
    *stats.count_by_level.entry(LogLevel::Info).or_insert(0) += 5;
    assert_eq!(stats.error_count(), 2);
}

#[test]
fn merge_stats_combines_exactly() {
    let mut a = Stats {
        total_lines: 3,
        ..Stats::default()
    };
    *a.count_by_level.entry(LogLevel::Info).or_insert(0) += 2;
    a.latencies.push(10);

    let mut b = Stats {
        total_lines: 2,
        ..Stats::default()
    };
    *b.count_by_level.entry(LogLevel::Error).or_insert(0) += 1;
    b.latencies.push(20);

    let merged = merge_stats(a, b);
    assert_eq!(merged.total_lines, 5);
    assert_eq!(merged.count_by_level.get(&LogLevel::Info), Some(&2));
    assert_eq!(merged.error_count(), 1);
    assert_eq!(merged.latencies, vec![10, 20]);
    assert_eq!(merged.percentile(1.0), Some(20));
}

#[test]
fn process_file_aggregates_a_single_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_log(
        &dir,
        "app.log",
        &[
            "2026-08-02T12:00:00 INFO boot complete",
            "2026-08-02T12:00:01 ERROR db connection failed latency=150ms",
            "2026-08-02T12:00:02 INFO request completed latency=30ms",
            "2026-08-02T12:00:03 WARN retrying latency=45ms",
            "# a comment line",
            "",
        ],
    );
    let stats = process_file(&path).unwrap();
    assert_eq!(stats.total_lines, 4);
    assert_eq!(stats.error_count(), 1);
    assert_eq!(stats.count_by_level.get(&LogLevel::Info), Some(&2));
    assert_eq!(stats.count_by_level.get(&LogLevel::Warn), Some(&1));
    assert_eq!(stats.percentile(0.50), Some(45));
    assert_eq!(stats.percentile(0.99), Some(150));
}

#[test]
fn process_files_merges_across_threads() {
    let dir = tempfile::tempdir().unwrap();
    let a = write_log(
        &dir,
        "a.log",
        &[
            "2026-08-02T12:00:00 ERROR first failure latency=100ms",
            "2026-08-02T12:00:01 INFO ok",
        ],
    );
    let b = write_log(
        &dir,
        "b.log",
        &[
            "2026-08-02T12:00:00 ERROR second failure latency=200ms",
            "2026-08-02T12:00:01 INFO ok",
            "2026-08-02T12:00:02 INFO done latency=50ms",
        ],
    );
    let c = write_log(&dir, "c.log", &["2026-08-02T12:00:00 DEBUG trace spam"]);

    let stats = process_files(&[a, b, c]).unwrap();
    assert_eq!(stats.total_lines, 6);
    assert_eq!(stats.error_count(), 2);
    assert_eq!(stats.count_by_level.get(&LogLevel::Info), Some(&3));
    assert_eq!(stats.count_by_level.get(&LogLevel::Debug), Some(&1));
    assert_eq!(stats.percentile(0.50), Some(100));
    assert_eq!(stats.percentile(0.99), Some(200));
}

#[test]
fn process_files_empty_input() {
    assert_eq!(process_files(&[]).unwrap(), Stats::default());
}

#[test]
fn process_files_missing_file_errors() {
    let result = process_files(&[PathBuf::from("/nonexistent/definitely-not-here.log")]);
    assert!(result.is_err());
}

#[test]
fn process_files_one_file_matches_process_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = write_log(&dir, "single.log", &["2026-08-02T12:00:00 WARN lonely"]);
    let direct = process_file(&path).unwrap();
    let threaded = process_files(&[path]).unwrap();
    assert_eq!(threaded, direct);
}
