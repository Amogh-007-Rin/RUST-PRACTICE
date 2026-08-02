# Capstone 04: Multithreaded Log Processor

**Covers modules:** 031–039
**Estimated time:** 5-7 hours

## Project Brief

You're building a small but production-shaped tool: a **multithreaded log processor** that ingests several log files at once, parses each line into structured records, and aggregates statistics — total lines, error counts per level, and latency percentiles (p50/p90/p99). This is the shape of every real log-analysis utility (from `grep`-pipeline scripts to Loki/ELK-style ingestion: parse fast, in parallel, then merge deterministically).

The tool has two layers, mirroring how you'd structure a real library + CLI: a `lib` crate containing the parsing and aggregation logic (fully unit-testable), and a small `main.rs` binary that walks command-line file arguments and prints a summary. Every file is processed by its own worker thread; partial results flow back through an `mpsc` channel; a `macro_rules!` helper (your Module 037 payoff) keeps the repetitive parsing arms readable.

## Requirements

1. **Multi-file input.** `process_files` accepts any number of log-file paths and returns one merged `Stats` for all of them.
2. **Concurrency via threads.** Each file is processed on its own worker thread (one thread per file is fine; a bounded worker pool is the stretch goal). Worker threads send partial results over an `mpsc` channel; the caller merges them. You must join every worker before returning.
3. **Line format.** Each parseable line looks like:

   ```
   2026-08-02T12:00:00 ERROR request failed
   2026-08-02T12:00:01 INFO request completed latency=123ms
   ```

   - token 1: timestamp (consumed, not validated)
   - token 2: level — one of `DEBUG`, `INFO`, `WARN`, `ERROR` (via the `level_from_str!` macro)
   - remaining tokens: the message; a `latency=<n>ms` token anywhere in the message (via the `parse_latency!` macro) is extracted as `Some(n)` and removed from the message.

   Empty lines, whitespace-only lines, and lines starting with `#` are skipped (they don't count toward totals). Lines with an unknown level are skipped too.
4. **Statistics.** `Stats` tracks `total_lines` (parseable lines only), `count_by_level` (a `BTreeMap` so iteration is sorted), and `latencies`. `error_count()` reports the `ERROR` count; `percentile(p)` returns the nearest-rank percentile for `0 < p <= 1.0` over the sorted latencies (index `ceil(p * n) - 1`), or `None` when there are no samples.
5. **Merging.** `merge_stats(a, b)` combines totals, per-level counts, and latency lists — this is what makes the threaded merge exact.
6. **Deterministic output.** No timing assertions anywhere; the binary prints levels in a fixed order and counts come from a `BTreeMap`, so identical inputs always produce identical output.

## Stretch Goals

- **Bounded worker pool.** Instead of one thread per file, cap the concurrency (e.g. 4 workers) and feed file paths to workers through a channel — a real ingestion-pipeline shape.
- **Skip-and-count.** Track malformed lines in a separate `skipped` counter so the tool reports "N lines ignored."
- **Per-file reporting.** Have workers send `(file_name, Stats)` pairs and print per-file plus merged summaries.
- **More percentiles / mean.** Add a `mean_latency()` and a configurable percentile set.

## Acceptance Criteria

The provided tests in `starter/tests/capstone_04.rs` define "done":

- [ ] `level_from_str!` maps all four levels and rejects unknown ones.
- [ ] `parse_latency!` extracts `latency=123ms` and rejects malformed variants.
- [ ] `parse_line` extracts level, message, and optional latency; skips empty/comment/unknown-level lines.
- [ ] `percentile` returns exact nearest-rank values for known inputs (and `None` for no samples).
- [ ] `merge_stats` combines totals, counts, and latencies exactly.
- [ ] `process_file` aggregates a single file's lines correctly.
- [ ] `process_files` merges several files' stats across threads — totals, per-level counts, and p50/p99 all exact.
- [ ] `process_files` with zero files returns empty stats; with a missing file it returns `Err`.
- [ ] Manual: `cargo run -p capstone-04-starter -- <file>...` prints a summary (total, per-level counts, p50/p90/p99) and exits non-zero with a usage message when run without arguments.

```bash
cargo test -p capstone-04-starter
```

## Design Notes / Hints

Which Block D modules apply where:

- **Module 031 (threads):** `process_files` spawns one thread per path and joins every handle — collect the handles, join in a second loop.
- **Module 032 (`Mutex`/`Arc`):** you don't need shared mutable state here at all. If you're tempted to lock a shared `Stats`, step back — the channel design (Module 033) removes the need.
- **Module 033 (channels):** each worker sends its partial `Stats` over a cloned `mpsc::Sender`; the caller drops its own sender and receives exactly one message per file. Sender-drop semantics close the channel — but counting messages is more explicit than relying on it here.
- **Module 034 (Send/Sync, atomics):** your `Stats` and `LogEntry` types must be `Send` — they will be automatically (plain data), which is itself the lesson: the compiler only lets you ship `Stats` across threads because it's `Send`.
- **Module 035/036 (unsafe):** you should not need a single `unsafe` block in this capstone. If you find yourself writing one, redesign.
- **Module 037 (macros):** the two parsing helpers are `#[macro_export]` macros — `level_from_str!` is a `match` over the four level literals (with `$crate::LogLevel::` paths so the exported macro works from integration tests), and `parse_latency!` strips the `latency=` prefix and `ms` suffix. Keeping them as macros is overkill in production — that's the point: this is the "repetitive parsing arm" problem macros exist for.
- **Module 039 (Cargo):** the crate is a lib + binary in one package; the tests are integration tests in `starter/tests/`, and `tempfile` is a dev-dependency (test-only, not part of the shipped crate).

Start with `parse_line` (small, pure, testable), then `Stats`/`merge_stats`, then `process_file`, then the threaded `process_files`, and finally the `main.rs` summary. The tests are ordered roughly the same way.
