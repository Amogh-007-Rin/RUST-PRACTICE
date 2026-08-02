use std::path::PathBuf;

use capstone_04_starter::{process_files, LogLevel, Stats};

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: capstone-04-starter <log-file> [more log files...]");
        std::process::exit(1);
    }

    let paths: Vec<PathBuf> = args.iter().map(PathBuf::from).collect();
    match process_files(&paths) {
        Ok(stats) => print_summary(&stats),
        Err(err) => {
            eprintln!("error processing logs: {err}");
            std::process::exit(1);
        }
    }
}

fn level_name(level: LogLevel) -> &'static str {
    match level {
        LogLevel::Debug => "debug",
        LogLevel::Info => "info",
        LogLevel::Warn => "warn",
        LogLevel::Error => "error",
    }
}

fn print_summary(stats: &Stats) {
    println!("total lines: {}", stats.total_lines);
    println!("errors: {}", stats.error_count());
    for level in [
        LogLevel::Debug,
        LogLevel::Info,
        LogLevel::Warn,
        LogLevel::Error,
    ] {
        let count = stats.count_by_level.get(&level).copied().unwrap_or(0);
        println!("{:5}: {}", level_name(level), count);
    }
    match (
        stats.percentile(0.50),
        stats.percentile(0.90),
        stats.percentile(0.99),
    ) {
        (Some(p50), Some(p90), Some(p99)) => {
            println!("latency p50: {p50}ms, p90: {p90}ms, p99: {p99}ms");
        }
        _ => println!("latency: no samples"),
    }
}
