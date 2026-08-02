//! Capstone 04: reference solution.
//!
//! A multithreaded log processor: parse lines with `macro_rules!` helpers,
//! aggregate statistics per file, and merge per-file results across worker
//! threads connected by an `mpsc` channel.

use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::mpsc;

/// The severity levels a log line may carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error,
}

/// One successfully parsed log line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub latency_ms: Option<u64>,
}

/// Aggregated statistics for any number of log files.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Stats {
    /// Number of successfully parsed lines.
    pub total_lines: u64,
    /// Per-level line counts (a `BTreeMap`, so iteration is sorted).
    pub count_by_level: BTreeMap<LogLevel, u64>,
    /// All latency samples, in parse order.
    pub latencies: Vec<u64>,
}

/// Maps a level literal to its `LogLevel`, or `None`.
#[macro_export]
macro_rules! level_from_str {
    ($s:expr) => {
        match $s {
            "DEBUG" => Some($crate::LogLevel::Debug),
            "INFO" => Some($crate::LogLevel::Info),
            "WARN" => Some($crate::LogLevel::Warn),
            "ERROR" => Some($crate::LogLevel::Error),
            _ => None,
        }
    };
}

/// Extracts the millisecond value from a `latency=<n>ms` token, or `None`.
#[macro_export]
macro_rules! parse_latency {
    ($s:expr) => {{
        $s.strip_prefix("latency=")
            .and_then(|rest| rest.strip_suffix("ms"))
            .and_then(|digits| digits.parse::<u64>().ok())
    }};
}

/// Parses one log line into a `LogEntry`, or `None` if it should be
/// skipped (empty, comment, or unknown level).
pub fn parse_line(line: &str) -> Option<LogEntry> {
    let trimmed = line.trim();
    if trimmed.is_empty() || trimmed.starts_with('#') {
        return None;
    }

    let mut tokens = trimmed.split_whitespace();
    let _timestamp = tokens.next()?;
    let level = crate::level_from_str!(tokens.next()?)?;

    let mut message_parts = Vec::new();
    let mut latency_ms = None;
    for token in tokens {
        if let Some(ms) = crate::parse_latency!(token) {
            latency_ms = Some(ms);
        } else {
            message_parts.push(token);
        }
    }

    Some(LogEntry {
        level,
        message: message_parts.join(" "),
        latency_ms,
    })
}

/// Merges two `Stats` values into one.
pub fn merge_stats(mut a: Stats, b: Stats) -> Stats {
    a.total_lines += b.total_lines;
    for (level, count) in b.count_by_level {
        *a.count_by_level.entry(level).or_insert(0) += count;
    }
    a.latencies.extend(b.latencies);
    a
}

impl Stats {
    /// The number of `ERROR` lines.
    pub fn error_count(&self) -> u64 {
        self.count_by_level
            .get(&LogLevel::Error)
            .copied()
            .unwrap_or(0)
    }

    /// Nearest-rank percentile over the latency samples: index
    /// `ceil(p * n) - 1` of the sorted samples. `None` when there are no
    /// samples or when `p` is outside `0 < p <= 1.0`.
    pub fn percentile(&self, p: f64) -> Option<u64> {
        if self.latencies.is_empty() || !(p > 0.0 && p <= 1.0) {
            return None;
        }
        let mut sorted = self.latencies.clone();
        sorted.sort_unstable();
        let index = (p * sorted.len() as f64).ceil() as usize - 1;
        Some(sorted[index])
    }
}

/// Reads one file and aggregates its statistics.
pub fn process_file(path: &Path) -> io::Result<Stats> {
    let content = fs::read_to_string(path)?;
    let mut stats = Stats::default();
    for line in content.lines() {
        if let Some(entry) = parse_line(line) {
            stats.total_lines += 1;
            *stats.count_by_level.entry(entry.level).or_insert(0) += 1;
            if let Some(ms) = entry.latency_ms {
                stats.latencies.push(ms);
            }
        }
    }
    Ok(stats)
}

/// Processes every path on its own worker thread and merges the partial
/// stats received over a channel. An `Err` is returned if any file fails.
pub fn process_files(paths: &[PathBuf]) -> io::Result<Stats> {
    if paths.is_empty() {
        return Ok(Stats::default());
    }

    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    for path in paths.iter().cloned() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            let result = process_file(&path);
            tx.send((path, result)).unwrap();
        }));
    }
    drop(tx);

    let mut results = Vec::new();
    for _ in 0..paths.len() {
        let (path, result) = rx.recv().unwrap();
        results.push((path, result));
    }
    for handle in handles {
        handle.join().unwrap();
    }

    let mut merged = Stats::default();
    for (_, result) in results {
        merged = merge_stats(merged, result?);
    }
    Ok(merged)
}
