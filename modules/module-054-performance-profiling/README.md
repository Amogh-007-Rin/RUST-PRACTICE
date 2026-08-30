# Module 054: Native Debugging and Performance Profiling

**Block:** Block F — Systems Programming & Performance

**Estimated time:** 75–110 min

**Prerequisites:** Module 019 (Testing in Rust)

## Learning Objectives

- You will be able to choose between compiler diagnostics, backtraces, a debugger, and a profiler.
- You will be able to build a debuggable Rust binary and launch it with `rust-lldb` or `rust-gdb`.
- You will be able to set breakpoints, step through code, inspect stack frames and variables, and print a backtrace.
- You will be able to diagnose a boundary-condition bug without guessing at the fix.
- You will be able to explain what a flamegraph is and distinguish inclusive time from exclusive time.
- You will be able to parse a call trace, compute per-function timing statistics, and identify a hotspot.

## Why This Matters

Professional debugging is not random editing followed by another build. It is an evidence-gathering process: reproduce the failure, stop execution near the fault, inspect the program's actual state, form a hypothesis, and verify the smallest fix. Rust's compiler prevents many memory and concurrency errors, but valid Rust programs can still contain incorrect comparisons, unexpected data, panics, deadlocks, and performance regressions.

A debugger answers **why is this execution wrong?** A profiler answers **where is this execution spending time?** Both expose runtime evidence, but they solve different problems. This module teaches the two workflows together so you can choose the right instrument instead of reaching for `println!` or optimization by instinct.

## Concept

### Begin with the cheapest useful evidence

Use the narrowest tool that can answer your question:

1. `cargo check` finds type, ownership, lifetime, and syntax errors without producing a final executable.
2. `cargo test -p <package>` reproduces behavioral failures and gives you a stable definition of correctness.
3. `RUST_BACKTRACE=1` shows the call chain leading to a panic.
4. `dbg!` and structured logging expose selected values when stopping the process is unnecessary.
5. `rust-lldb` or `rust-gdb` lets you pause execution and inspect changing runtime state.
6. A sampling profiler such as `perf` or `cargo flamegraph` identifies expensive code statistically.

Do not start a debugger for a compiler error, and do not start a profiler for a wrong return value. Tool choice is part of debugging.

### Build a debugger target

Cargo's default development profile includes debug information, so start with a normal build:

```bash
cargo build -p module-054-exercises --bin module-054-debug-lab
```

The resulting executable is:

```text
target/debug/module-054-debug-lab
```

Debug information maps machine instructions back to source locations, function names, and variables. On most Unix-like systems it is represented using DWARF data. Optimized release builds may inline functions, remove variables, or reorder instructions, so source-level stepping becomes less intuitive. Reproduce correctness bugs in a development build first. Profile performance in a release build later.

The Rust toolchain supplies the `rust-lldb` and `rust-gdb` wrapper commands. The wrappers load Rust-aware formatting helpers before launching LLDB or GDB. You still need the native debugger installed: LLDB is common on macOS and available on Linux; GDB is common on Linux.

### Reproduce before opening the debugger

The exercise binary contains a deterministic boundary bug. Run it normally first:

```bash
RUST_BACKTRACE=1 cargo run \
  -p module-054-exercises \
  --bin module-054-debug-lab
```

The program prints its samples, threshold, and result, then asserts the expected behavior. The backtrace tells you where the failed assertion was observed. That location is the symptom; the defect may be in a function below it.

A reproducible target matters. If the input changes on every run, debugger observations cannot be compared reliably. Reduce production failures to the smallest deterministic input you can.

### A focused `rust-lldb` session

Launch the binary:

```bash
rust-lldb target/debug/module-054-debug-lab
```

Then enter these LLDB commands:

```text
breakpoint set -n module_054_exercises::first_slow_sample
run
frame variable
next
backtrace
continue
quit
```

The lab marks this function `#[inline(never)]`, so its qualified Rust name is a stable, single breakpoint target. For unfamiliar code where the qualified name is unknown, `breakpoint set -r first_slow_sample` performs a regex search but may resolve several generated locations. At the breakpoint, `frame variable` displays arguments and local variables in the current stack frame. `next` executes the current source line without descending into called functions. `step` descends into a call when source information is available. `backtrace` (or `bt`) prints the active call stack. `continue` resumes execution until the next breakpoint or process exit.

Do not edit immediately after hitting the breakpoint. First answer concrete questions:

- What value was passed as the threshold?
- Which sample is being considered?
- Does the contract say “above” or “at least”?
- At what point does the observed result diverge from the expected result?

That sequence turns a guess into a diagnosis.

### The equivalent `rust-gdb` session

On systems where GDB is preferred:

```bash
rust-gdb target/debug/module-054-debug-lab
```

Use:

```text
rbreak first_slow_sample
run
info args
info locals
next
backtrace
continue
quit
```

`rbreak` sets breakpoints using a regular expression, which avoids depending on an exact Rust symbol name. `info args` and `info locals` inspect the current frame. LLDB and GDB use different command names, but the mental model is the same: break, run, inspect, step, and verify.

### Debugging tests

Tests are executables too. Cargo can compile them without running them:

```bash
cargo test -p module-054-exercises --no-run
```

Cargo prints each test executable's path under `target/debug/deps/`. You can pass that path to `rust-lldb` or `rust-gdb`, then supply the test name after `--` when launching. For this module, the dedicated debug-lab binary is less noisy and gives stable inputs, while the tests remain the final correctness oracle.

### Debuggers have boundaries

A debugger can change timing. This matters for race conditions, deadlocks, and latency-sensitive code. Pausing one thread may hide or create an interleaving that does not occur at full speed. Combine debugger evidence with logs, metrics, deterministic concurrency tests, or tools designed for the problem.

Containers and hardened Linux systems may restrict tracing through `ptrace`. If the debugger reports that it cannot launch or attach, inspect the host security policy rather than changing application code. Do not disable system-wide protections casually. macOS may prompt for developer-tool authorization. A hanging LLDB launch can also indicate an `lldb`/`lldb-server` transport mismatch; try the matching package version or use `rust-gdb` on Linux. These are environment failures, not Rust failures.

### What profiling tells you

Profiling answers “where is my program spending its time?” A sampling profiler interrupts execution periodically and records the current call stack. After many samples, the result is a statistical picture of runtime cost. Unlike a debugger, it normally observes without stopping at each source line.

A flamegraph represents sampled stacks as rectangles:

```text
       main(100%)─────────────────────────────────────────────────
   parse(60%)──────────────────────  process(40%)─────────────
  read(40%)──validate(20%)     sort(25%)───write(15%)
```

The width of a frame is proportional to its **inclusive time**: time in that function and its callees. The portion with no child stacked above it represents **exclusive time**, also called **self time**.

Consider these flat trace samples:

```text
main 100
parse 40
read_bytes 20
validate 20
process 60
sort 25
write_output 15
```

The module's library exercise implements three trace-analysis operations:

```rust
pub fn parse_call_trace(trace: &str) -> Vec<(&str, u64)>;
pub fn compute_self_time(traces: &[(&str, u64)], fn_name: &str) -> u64;
pub fn find_hotspot(traces: &[(&str, u64)]) -> Option<&str>;
```

`parse_call_trace` turns non-empty, non-comment lines into borrowed function names and durations. `compute_self_time` sums matching leaf samples. `find_hotspot` accumulates samples by function name and returns the largest total. Real profilers retain full stacks rather than a flat list, but the evidence-first workflow is the same.

### The profiling loop

Use profiling as a controlled experiment:

1. **Measure:** run `cargo flamegraph --release --bin my-app`, `perf`, or the platform profiler.
2. **Locate:** identify the widest relevant frames.
3. **Hypothesize:** explain why that code is expensive.
4. **Change:** make one targeted optimization.
5. **Re-measure:** confirm the cost moved or decreased.

Rust developers sometimes assume iterator chains, bounds checks, or allocation are the bottleneck. Measurements often show that formatting, I/O, synchronization, or a dependency dominates instead. Optimize evidence, not folklore.

## Common Pitfalls

- **Changing code before reproducing the failure.** Preserve the failing input and expected result first.
- **Confusing the symptom with the defect.** A panic line shows where an invariant was detected, not necessarily where it was violated.
- **Using an exact Rust symbol name.** Prefer regex breakpoints when module paths or mangling make exact names unreliable.
- **Stepping through optimized code.** Debug development builds; profile release builds.
- **Treating debugger launch failures as program bugs.** Check debugger installation, macOS authorization, container capabilities, and Linux `ptrace` policy.
- **Revealing secrets in debugger output.** Stack frames can contain credentials and personal data; do not paste raw dumps publicly.
- **Profiling debug builds.** Their performance characteristics are intentionally unlike release builds.
- **Misreading flamegraph width.** Width represents sampled time, and parent width includes callees.
- **Optimizing a cold function.** A 50% improvement to 2% of runtime yields only about 1% overall.
- **Collecting too few samples.** Short runs produce noisy profiles; extend the workload or collect more samples.

## Key Terms

- **Breakpoint:** a location or condition where the debugger pauses execution.
- **Stack frame:** one active function invocation, including its arguments and locals.
- **Backtrace:** the ordered stack of active calls leading to the current frame or panic.
- **Debug information:** metadata mapping compiled instructions to source constructs and variables.
- **DWARF:** a common debug-information format on Unix-like systems.
- **Watchpoint:** a debugger stop triggered when a memory location is read or written.
- **Flamegraph:** a visualization of sampled call stacks where width represents frequency or time.
- **Inclusive time:** time attributed to a function and everything it calls.
- **Exclusive time:** time attributed only to a function's own instructions.
- **Hotspot:** code accounting for a significant share of measured runtime.

## Exercise

In `exercises/`:

1. Implement `parse_call_trace`, `compute_self_time`, and `find_hotspot` using the existing `TODO(module-054)` markers.
2. Run `cargo test -p module-054-exercises` and separate failures in the trace analyzer from the debugger lab's boundary failure.
3. Build and run `module-054-debug-lab` with `RUST_BACKTRACE=1` to reproduce the boundary bug.
4. Launch the binary with `rust-lldb` or `rust-gdb`, break on `first_slow_sample`, and inspect the threshold and samples before editing.
5. Fix the smallest expression that contradicts the function's “strictly above” contract.
6. Re-run the binary and all exercise tests, then verify the complete module:

```bash
./scripts/verify_module.sh 054
```

Compare your implementation with `solutions/` only after you can explain the evidence that led to the fix.

## Further Reading

- [The Rust Performance Book: Profiling](https://nnethercote.github.io/perf-book/profiling.html)
- [The LLDB Tutorial](https://lldb.llvm.org/use/tutorial.html)
- [GDB: Running and Stopping Programs](https://sourceware.org/gdb/current/onlinedocs/gdb.html/Stopping.html)
- [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph)
- [Brendan Gregg's FlameGraphs](https://www.brendangregg.com/flamegraphs.html)
