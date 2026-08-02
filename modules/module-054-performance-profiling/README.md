# Module 054: Performance Profiling

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 45–75 min
**Prerequisites:** Module 019 (Testing in Rust)

## Learning Objectives

- You will be able to explain what a flamegraph is and how to read one.
- You will be able to distinguish between inclusive (self + callees) and exclusive (self-only) time.
- You will be able to parse a call-trace file and compute per-function timing statistics.
- You will be able to identify hotspots (functions with the highest inclusive time) programmatically.

## Why This Matters

Every performance-sensitive Rust codebase relies on profiling: no amount of intuition replaces measured data. Tools like `cargo flamegraph` (built on `perf` and Brendan Gregg's flamegraph script) produce visual call-stack stacks where the width of each frame shows time spent. But flamegraphs are just visualizations of call traces. Before you generate a flamegraph, you need to understand what the underlying data means: which functions consume the most CPU time, whether the cost is "self time" or "callee time," and how to act on that information. This module teaches you to think like a profiler by writing your own trace-analysis functions — the same logic a flamegraph visualizer performs.

## Concept

### What profiling tells you

Profiling answers the question "where is my program spending its time?" A profiler samples the program's call stack at regular intervals (say, every 1 ms) and records which function is currently executing. After thousands of samples, the profile shows a statistical picture of where time went.

### Flamegraph anatomy

A flamegraph is a visual representation of stack traces. Imagine a sideways stack of rectangles:

```
       main(100%)─────────────────────────────────────────────────
   parse(60%)──────────────────────  process(40%)─────────────
  read(40%)──validate(20%)     sort(25%)───write(15%)
```

Each horizontal bar is a function. The width of a bar is proportional to its **inclusive time** — the total time spent in that function *including* any functions it calls. The portion of a bar that has nothing stacked above it represents **exclusive time** (also called **self time**) — time spent in that function's own code, not in its callees.

### Inclusive vs exclusive time

Consider this trace of function calls, each with a duration in milliseconds:

```
main 100
parse 40
read_bytes 20
validate 20
process 60
sort 25
write_output 15
```

- **Inclusive time** for `main`: 100 ms (everything happens inside main).
- **Exclusive time** for `main`: 100 - (40 + 60) = 0 ms (main does nothing directly; it only delegates).
- **Inclusive time** for `parse`: 40 ms.
- **Exclusive time** for `parse`: 40 - (20 + 20) = 0 ms (parse only calls read_bytes and validate).
- **Exclusive time** for `read_bytes`: 20 ms (it has no callees in the trace).

In real flamegraphs, inclusive time is the full width of a bar; exclusive time is the "leftover" width not covered by child bars.

### Building a trace analyzer

For this module you'll implement three functions that work on a flat list of (function_name, duration) pairs — simulating a trace file before it's turned into a flamegraph:

```rust
/// Parse a multi-line trace. Each line is "function_name duration_ms".
/// Blank lines and lines starting with '#' are ignored.
pub fn parse_call_trace(trace: &str) -> Vec<(&str, u64)>;

/// Sum all durations for calls named `fn_name`.
pub fn compute_self_time(traces: &[(&str, u64)], fn_name: &str) -> u64;

/// Return the function name with the highest total inclusive time.
pub fn find_hotspot(traces: &[(&str, u64)]) -> Option<&str>;
```

**`parse_call_trace`** takes a multi-line string and splits each non-empty, non-comment line into a function name and a duration. Use `lines()`, `filter()`, and `split_once()` to avoid intermediate allocations. The returned `&str` borrows from the input, so no heap allocation is needed for the names.

**`compute_self_time`** iterates over the parsed trace and sums durations where the function name matches. This is exclusive self-time in a flat trace — the simplest metric.

**`find_hotspot`** builds a `HashMap` keyed by function name, accumulating total time for each function, then returns the name with the highest total. This finds the function where the most wall-clock time is spent.

### The profiling workflow

Real-world profiling follows a loop:

1. **Profile**: `cargo flamegraph --bin my-app -- --input data.csv` (or `perf record` / `sample` on macOS).
2. **Analyze**: open the SVG flamegraph and look for the widest bars.
3. **Hypothesize**: identify a bottleneck and guess why it's slow (e.g., "parsing is 60% of runtime — maybe the regex is expensive").
4. **Optimize**: make a targeted change.
5. **Verify**: re-profile and confirm the bar shrunk.

This module's exercise gives you the "analyze" step — given a trace, find the hotspot.

### Why flat traces are a simplification

A real profiler records full call stacks at each sample point, producing a tree, not a flat list. The flat trace in this exercise represents the *leaf* samples: the function actually on top of the stack when the sample was taken. A real flamegraph uses the full stack to build the hierarchical visualization, but computing self-time at the leaf level still works the same way.

### When profiling beats guessing

Rust developers often assume "iterator chains are slow" or "Box allocations are the problem." Profiling frequently proves these intuitions wrong. A common finding: the bottleneck is in a dependency's string formatting or in IO wait, not in the algorithmic Rust code at all. Measuring first, optimizing second, is the professional approach.

## Common Pitfalls

- **Profiling debug builds.** Debug mode disables optimizations, making tiny functions appear disproportionately expensive. Always profile `--release`.
- **Misreading flamegraph width.** A wide bar means *inclusive* time (this function + callees). To find what *this function* contributes, look at the gaps above it.
- **Optimizing the wrong thing.** The widest bar is the right place to start. Speeding up a function that takes 2% of runtime by 50% yields a 1% overall improvement.
- **Not collecting enough samples.** If your program runs for 200 ms and you take samples every 10 ms, you have only 20 data points — statistical noise can dominate. Longer runs or higher sampling rates improve accuracy.
- **Calling `cargo flamegraph` without `perf` installed.** On Linux, you need `linux-perf` or equivalent kernel facility.

## Key Terms

- **Flamegraph:** a visualization of profiler output where function calls are stacked rectangles; width = inclusive time.
- **Inclusive time:** total time spent in a function including all functions it calls.
- **Exclusive time (self time):** time spent in a function's own code, excluding callees.
- **Hotspot:** the function that accounts for the largest fraction of runtime.
- **Sampling profiler:** a profiler that periodically interrupts execution to record the current call stack.
- **Call trace:** a record of which function was executing, typically with timing data.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-054)` comments.
2. Implement `parse_call_trace()` to parse a multi-line trace string into `Vec<(&str, u64)>`. Each line has the format `function_name duration`. Ignore blank lines and lines starting with `#`.
3. Implement `compute_self_time()` to sum durations for a given function name.
4. Implement `find_hotspot()` to find the function with the highest total inclusive time. Return `None` for an empty trace.
5. Run `cargo test -p module-054-exercises` until all tests pass.
6. Compare with `solutions/` afterwards.

## Further Reading

- [cargo-flamegraph crate](https://github.com/flamegraph-rs/flamegraph) — Rust wrapper around Brendan Gregg's flamegraph tools.
- [Brendan Gregg's FlameGraphs](https://www.brendangregg.com/flamegraphs.html) — the original article introducing flamegraph visualization.
- [The Rust Performance Book — Profiling](https://nnethercote.github.io/perf-book/profiling.html) — profiling guidance specific to Rust.
