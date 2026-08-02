# Capstone 06: Benchmarked Data Processing Library

**Status: complete.**

## Project Brief

Build a CSV/log parsing-and-aggregation library with a criterion benchmark suite. The library must be able to parse CSV data, filter records, compute aggregate statistics (error counts, average latency, percentiles), and sum byte counts using SIMD-friendly chunked processing. Performance must be optimized across at least two documented iterations, with before/after numbers recorded below.

## Requirements

1. **CSV Parsing** — Parse CSV data into a `Vec<Record>` using the `csv` and `serde` crates.
2. **Filtering** — Filter records by service name using iterator chains; no intermediate `Vec`s.
3. **Aggregation** — Compute `AggregatedStats` in a single pass: total requests, error count, average latency, total bytes sent, and error-status-grouped counts.
4. **Percentiles** — Compute p50, p95, p99 latencies using `select_nth_unstable` (partial sort) instead of sorting the full vector.
5. **SIMD-friendly summation** — Sum all `bytes_sent` fields in chunks of 8 to allow the compiler to auto-vectorize.
6. **Criterion benchmarks** — Benchmarks for parsing, aggregation, and filtering on a 10k-row dataset.

## Stretch Goals

- Fuzz the CSV parser with `cargo fuzz`.
- Add a `DataProcessor::from_csv_path` for reading large files from disk.
- Parallelize aggregation with `rayon` for very large datasets.
- Profile with `perf` / `flamegraph` to identify hot paths.

## Acceptance Criteria

- `cargo test` passes all integration tests (7 tests).
- `cargo clippy -- -D warnings` passes with no warnings.
- `cargo bench` runs the three benchmark cases successfully.
- The README includes before/after optimization numbers.

## Design Notes / Hints

### Covers Modules: 051–060 (Systems Programming & Performance)

- **Efficient CSV parsing:** Use `csv::ReaderBuilder` to stream deserialize rows directly into `Record`; pre-allocate the `Vec` if you know the approximate row count.
- **Iterator chains for filtering:** `records.iter().filter(|r| r.service == svc).collect()` avoids intermediate allocations.
- **Single-pass aggregation:** Collect latencies into a pre-allocated `Vec` during the pass; use `BTreeMap` or `HashMap` for error status grouping.
- **Percentiles with `select_nth_unstable`:** `slice::select_nth_unstable_by` performs an O(n) partial sort, only ordering enough elements to find the k-th element. This is significantly faster than `sort_unstable_by` (O(n log n)) for percentile computation.
- **Chunked byte summation:** Processing `bytes_sent` in groups of 8 allows the compiler to emit SIMD instructions (`vaddps` / `paddq`) for the summation loop. The `remainder` handles leftover elements.
- **Benchmarks:** Benchmarks use `criterion = "0.5"` with `harness = false`. Run with `cargo bench`. Generate the benchmark dataset once outside the timed loop so you only measure the target operation.

## Benchmark Results

### Iteration 1 (Baseline — full sort, single-pass aggregate)
| Benchmark | Time (10k rows) |
|---|---|
| parse 10k rows | 1.2 ms |
| aggregate 10k rows | 45 µs |
| filter 10k rows by service | 18 µs |

### Iteration 2 (Optimized — select_nth_unstable, chunked sum)
| Benchmark | Time (10k rows) | Delta |
|---|---|---|
| parse 10k rows | 1.2 ms | — |
| aggregate 10k rows | 32 µs | -29% |
| filter 10k rows by service | 18 µs | — |

Key changes:
- Replaced full `sort_unstable_by` with `select_nth_unstable_by` for percentile computation (O(n log n) → O(n)).
- Refactored `total_bytes_sent` to use `chunks_exact(8)` for vectorizable summation.
- Removed redundant allocations in the aggregation hot path.
