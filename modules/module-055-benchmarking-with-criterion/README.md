# Module 055: Benchmarking

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 45–75 min
**Prerequisites:** Module 019 (Testing in Rust), Module 054 (Performance Profiling)

## Learning Objectives

- You will be able to measure function execution time using `std::time::Instant`.
- You will be able to write a simple benchmarking harness that runs multiple iterations.
- You will be able to compare two implementations and report which is faster with a speedup ratio.
- You will understand common benchmarking traps: insufficient warmup, unstable noise, and micro-benchmark narrowness.

## Why This Matters

"How fast is my code?" is a question Rust developers ask constantly. Whether you're choosing between `HashSet` or `BTreeSet`, between a regex engine or a hand-written parser, or between two sorting algorithms, you need numbers, not hunches. The `criterion` crate is Rust's gold-standard benchmarking framework — it handles warmup, statistical analysis, HTML reports, and comparison graphs. But before you reach for a heavyweight framework, you need to understand what benchmarking actually does under the hood: measure wall-clock time, run enough iterations for statistical stability, and compare two measurements meaningfully. This module teaches you to build a minimal benchmarking harness from scratch so you internalize how `criterion` and similar tools work.

## Concept

### The anatomy of a benchmark

A benchmark measures how long a piece of code takes to run. At minimum, you:

1. Record the start time.
2. Execute the code under test.
3. Record the end time and compute the elapsed duration.
4. Repeat N times and compute statistics (mean, standard deviation, etc.).

In Rust, `std::time::Instant::now()` gives you a monotonically non-decreasing clock suitable for measuring intervals:

```rust
use std::time::Instant;

let start = Instant::now();
expensive_computation();
let elapsed = start.elapsed();
println!("{:?}", elapsed); // e.g., "2.34ms"
```

### Running multiple iterations

A single measurement is noisy — the OS might schedule another process, the CPU might throttle, or cache effects might unfairly penalize the first run. Running many iterations and taking the average reduces noise:

```rust
use std::time::{Duration, Instant};

fn time_execution<F: FnMut() -> R, R>(mut f: F, iterations: u32) -> (R, Duration) {
    let start = Instant::now();
    let mut result = f();
    for _ in 1..iterations {
        result = f();
    }
    let total = start.elapsed();
    (result, total / iterations)
}
```

This returns the **average** duration per iteration. Keep in mind that `Duration` division (`/`) truncates to integer nanoseconds, which is fine for millisecond-scale benchmarks but loses precision for nanosecond-scale operations.

### Comparing two implementations

Once you can time one function, you can compare two:

```rust
pub struct CompareResult {
    pub name1: String,
    pub time1: Duration,
    pub name2: String,
    pub time2: Duration,
    pub faster: String,
    pub speedup: f64,
}
```

The `speedup` field is `slower_time / faster_time` — a value of 2.0 means the faster implementation is twice as quick. If both are equal within measurement noise, the speedup is approximately 1.0.

The `faster` field names the faster implementation or says `"tie"` if the difference is negligible.

### Benchmark traps

Rust makes certain benchmarking mistakes especially easy:

**1. The optimizer deletes your code.** If the result of your computation is unused, the compiler might eliminate the entire benchmark loop. This is why benchmarks often use `std::hint::black_box()` (or criterion's equivalent) to prevent dead-code elimination:

```rust
let result = expensive_computation();
std::hint::black_box(result); // prevent optimization
```

In our simplified harness, we return the value and let the caller consume it — the test reading the return value prevents the optimizer from removing the computation.

**2. Warmup matters.** The first few iterations of a benchmark include cold-cache penalties (instruction cache, data cache, branch prediction). Production benchmarking frameworks like `criterion` include a warmup phase that's discarded from results.

**3. Background noise.** A single OS thread preempting your benchmark can inflate a single iteration by orders of magnitude. Using the **median** instead of the **mean** is a common defense, though mean-with-many-iterations works for most cases.

**4. Micro-benchmarks don't predict system behavior.** A function that benchmarks well in isolation might interact poorly with the rest of the program — e.g., polluting the CPU cache with large data structures that displace data the caller needed. Always measure at the integration level too.

### ASCII diagram: benchmark flow

```
┌─────────────────────────────────────────────┐
│  for i in 0..iterations:                    │
│    start = Instant::now()                   │
│    result = f()        ◄── code under test  │
│    elapsed = start.elapsed()                │
│    records[i] = elapsed                     │
│                                             │
│  avg = sum(records) / len(records)          │
│  return (last_result, avg)                  │
└─────────────────────────────────────────────┘
```

### When to benchmark vs when to profile

- **Benchmark** when you have a specific hypothesis: "Is implementation A faster than implementation B for workload W?"
- **Profile** when you don't know where the time goes: "My server is slow under load — which function is the bottleneck?"

Module 054 taught profiling. This module teaches the complementary skill: measuring and comparing known alternatives.

### The exercise in a sentence each

- `time_execution()` — runs a closure `iterations` times, returns the final result and the average duration.
- `compare()` — times two closures and reports which is faster, by how much.

## Common Pitfalls

- **Forgetting `mut` on the closure binding.** You need `mut f` to call `FnMut` multiple times.
- **Optimized-out code.** Always ensure the benchmark result is consumed (e.g., returned and checked in the test). An unused return value invites the compiler to eliminate the work.
- **Too few iterations.** Single-digit iterations yield noisy, unreliable results. Use hundreds or thousands for stable measurements.
- **Comparing across different runs.** CPU frequency scaling, thermal throttling, and other processes differ between runs. Compare two implementations in the same process invocation for fairness.
- **Benchmarking debug builds.** Always benchmark `--release`. Debug-mode benchmarks are essentially meaningless for performance comparison.

## Key Terms

- **Criterion:** a Rust benchmarking crate providing statistical analysis, HTML reports, and comparison charts.
- **Warmup:** initial benchmark iterations discarded because of cold-cache effects.
- **Speedup:** the ratio of the slower time to the faster time (values > 1.0 indicate improvement).
- **`Instant`:** a monotonically non-decreasing clock from `std::time`, suitable for measuring elapsed time.
- **`black_box`:** a function that prevents compiler optimizations from eliminating "dead" computations in benchmarks.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-055)` comments.
2. Implement `time_execution()` — run `f` for `iterations` times and return `(final_result, average_duration)`.
3. Implement `compare()` — time both closures with `iterations` each and return a `CompareResult` comparing them.
4. Run `cargo test -p module-055-exercises` until all tests pass.
5. Compare with `solutions/` afterwards.

## Further Reading

- [The Criterion Book](https://bheisler.github.io/criterion.rs/book/) — the definitive guide to Rust benchmarking.
- [`std::time::Instant` documentation](https://doc.rust-lang.org/std/time/struct.Instant.html) — the core timing primitive.
- [`std::hint::black_box`](https://doc.rust-lang.org/std/hint/fn.black_box.html) — prevents optimizations from eliminating benchmark code.
- [The Rust Performance Book — Benchmarking](https://nnethercote.github.io/perf-book/benchmarking.html) — Rust-specific benchmarking advice.
