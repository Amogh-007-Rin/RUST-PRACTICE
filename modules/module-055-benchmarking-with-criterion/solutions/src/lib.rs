//! Module 055: solution — the reference implementation.

use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct CompareResult {
    pub name1: String,
    pub time1: Duration,
    pub name2: String,
    pub time2: Duration,
    pub faster: String,
    pub speedup: f64,
}

pub fn time_execution<F: FnMut() -> R, R>(mut f: F, iterations: u32) -> (R, Duration) {
    let start = Instant::now();
    let mut result = f();
    for _ in 1..iterations {
        result = f();
    }
    let total = start.elapsed();
    (result, total / iterations)
}

pub fn compare<F1, F2>(
    name1: &str,
    mut f1: F1,
    name2: &str,
    mut f2: F2,
    iterations: u32,
) -> CompareResult
where
    F1: FnMut() + Send,
    F2: FnMut() + Send,
{
    let (_, time1) = time_execution(&mut f1, iterations);
    let (_, time2) = time_execution(&mut f2, iterations);

    let (faster, speedup) = if time1 < time2 {
        let ratio = time2.as_nanos() as f64 / time1.as_nanos().max(1) as f64;
        (name1.to_string(), ratio)
    } else if time2 < time1 {
        let ratio = time1.as_nanos() as f64 / time2.as_nanos().max(1) as f64;
        (name2.to_string(), ratio)
    } else {
        ("tie".to_string(), 1.0)
    };

    CompareResult {
        name1: name1.to_string(),
        time1,
        name2: name2.to_string(),
        time2,
        faster,
        speedup,
    }
}
