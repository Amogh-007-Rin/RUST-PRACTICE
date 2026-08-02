# Module 031: Concurrency I — Threads

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 60–90 min
**Prerequisites:** Module 021 (closures), Module 028 (ownership of heap data)

## Learning Objectives

- Spawn an OS thread with `std::thread::spawn` and receive a `JoinHandle<T>`.
- Move owned data into a thread with a `move` closure and send results back via the handle's return value.
- Block on a thread with `join()` and handle the `Result<T, Box<dyn Any + Send>>` it returns (thread panics).
- Parallelize a simple reduction (sum of squares) across a fixed number of threads and verify deterministic results.
- Explain why `'static` is required of spawned closures, and when a plain closure fails to compile.

## Why This Matters

Every Rust service you'll touch in production is multi-threaded somewhere: HTTP servers run handler work on thread pools, database clients run blocking I/O off the main task, and data pipelines split work across cores. `std::thread` is the foundation everything else builds on — `tokio` is built on top of the same OS threads, and `rayon` is a fancy parallel-iterator wrapper around the same `spawn`/`join` pair. If you don't understand threads, the rest of Block D (mutexes, channels, atomics) has no foundation to stand on.

## Concept

### What a thread is

A **thread** is a sequence of instructions the operating system schedules independently. Threads in the same process share the same address space (same heap, same statics) but each has its own stack and its own program counter. The OS is free to pause one thread and run another at any point — this is called **preemptive scheduling** — so you can never assume two threads execute in a particular interleaving. Rust's standard library exposes one flavor of thread, `std::thread`, which maps 1:1 to an OS thread. On Linux that's a pthread; on Windows a Win32 thread.

### `spawn` and `JoinHandle`

The core API is one function:

```rust
use std::thread;

let handle: thread::JoinHandle<u32> = thread::spawn(move || {
    // This closure runs on a brand-new thread.
    let x = 21u32 * 2;
    x // the value the closure returns becomes the thread's result
});

let result = handle.join().unwrap();
assert_eq!(result, 42);
```

Three things are happening:

1. `thread::spawn` takes a closure that returns some type `T`, and immediately returns a `JoinHandle<T>` while the closure starts running on a new thread.
2. The closure needs a `move` if it captures anything — see "the `'static` requirement" below.
3. `handle.join()` blocks the calling thread until the spawned thread finishes, and returns `Result<T, Box<dyn Any + Send>>`. If the closure panicked, you get `Err` with the panic payload; `unwrap()` re-panics in the caller. `join()` is how you get data *out* of a thread — the return value travels through the handle.

Here is the timeline. Time flows downward; the left column is the main thread, the right column is the worker:

```
main thread                    worker thread
───────────────────────────────────────────────────────────
thread::spawn(f) ───────────►  (closure f starts)
  returns JoinHandle          |
    │                         |   ... f computes ...
    │  main keeps running     |
    │  (does other work)      |
    │                         |
handle.join() ───────────────►  blocks until f returns
    │                         |
    │◄─────────── Ok(result)──  f's return value sent back
  unwrap() → 42               thread exits
```

Note that `join()` does two jobs: it *waits* (synchronization) and it *transfers data* (the return value). Between `spawn` and `join`, both threads run concurrently — that's the entire point.

### The `'static` requirement

`thread::spawn` has this signature in disguise:

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
```

Two constraints matter here:

- **`F: Send`** means the closure must be safe to move to another thread.
- **`F: 'static`** means the closure must not borrow anything from the spawning thread's stack — the compiler cannot know how long the new thread will live, so it demands the closure be fully self-contained.

This is why capturing by reference fails to compile:

```rust,ignore
let name = String::from("worker");
let handle = thread::spawn(|| println!("{name}"));
// error: closure may outlive the current function, but it borrows `name`,
// which is owned by the current function
```

The fix is to move the value in — the thread takes ownership, so it can't dangle:

```rust
let name = String::from("worker");
let handle = thread::spawn(move || println!("{name}"));
handle.join().unwrap();
```

After the `move`, `name` is owned by the closure; the original variable is no longer usable. If you need the same value in several threads, you need `Arc` — that's Module 032's job. For now, the rule of thumb: **give each thread its own data**, and get results back through `join()`.

### A worked example: parallel sum of squares

The natural way to use threads is to split a problem into independent chunks, hand one chunk to each thread, and merge the results in the caller. Here is a strided split: thread `t` handles values `t+1, t+1+n, t+1+2n, ...` (incrementing by the number of threads), which is easy to prove correct:

```rust
use std::thread;

fn sum_squares_parallel(n: u32, threads: usize) -> u64 {
    let mut handles = Vec::new();
    for t in 0..threads {
        handles.push(thread::spawn(move || {
            let mut sum: u64 = 0;
            let mut v = t as u32 + 1;
            while v <= n {
                sum += (v as u64) * (v as u64);
                v += threads as u32;
            }
            sum
        }));
    }
    let mut total = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
}

assert_eq!(sum_squares_parallel(100, 4), (1..=100).map(|x| (x as u64) * (x as u64)).sum());
```

Three idioms to internalize from this example:

1. **Collect the handles, then join in a second loop.** If you joined inside the first loop, you'd spawn thread 1, wait for it, spawn thread 2, wait... and get zero parallelism. The spawn loop and the join loop must be separate.
2. **`unwrap()` on join.** The only way `join()` returns `Err` is if the closure panicked. In an exercise that's a bug you want to surface loudly.
3. **Results travel through return values, not shared memory.** This is the message-passing-ish style that keeps Module 031 free of data races: no shared mutable state exists at all, so nothing can race.

### Thread panics

A panic in a spawned thread does **not** crash the process. The thread unwinds, the panic payload is stored in the `JoinHandle`, and the next `join()` reveals it:

```rust
let handle = thread::spawn(|| panic!("boom"));
let outcome = handle.join();
assert!(outcome.is_err());
```

Panicking while holding a `Mutex` (Module 032) has the same mechanics and one nasty consequence — poison — but for now, know that `join().unwrap()` turns "worker panicked" into "caller panicked at the join point," which is a traceable failure.

### When threads make sense

Spawning a thread costs: the OS allocates a stack (default 8 MiB, virtual), and the kernel schedules the new thread. So the right granularity is one thread per *coarse* unit of work (a file, a request, a chunk of a million numbers), not per number. The Module 031 exercises keep this honest by giving you `threads` as an explicit parameter.

## Common Pitfalls

- **Forgetting to `join()`.** If the main thread returns without joining, the process exits and the OS kills every remaining thread — your work silently disappears. If a result matters, `join()` it; if it doesn't, at least be aware the thread is fire-and-forget.
- **Borrowing instead of `move`-capturing.** The `'static` bound rejects `|| x` when `x` is borrowed; new learners fight this by adding `move` — do it. If you genuinely need shared access, that's `Arc` (Module 032), not a loophole.
- **Joining inside the spawn loop.** That serializes your program; parallelism only exists between the spawn loop and the join loop. Spawn all, then join all.
- **Assuming execution order.** Threads are preemptively scheduled; "thread 2" may finish before "thread 1." The only ordering you get is from `join()` calls. Never infer behavior from the order of thread ids.
- **Spawn-per-element.** `thread::spawn` for each of 10 million inputs will spend all its time creating threads. Chunk the work and spawn a few threads.

## Key Terms

- **thread:** an independently scheduled sequence of instructions sharing the process's memory, with its own stack.
- **spawn:** create and start a new thread, returning a `JoinHandle<T>`.
- **JoinHandle:** the ownership token for a thread; `join()` waits for it and extracts its result.
- **join():** block the current thread until the target thread finishes; returns `Result<T, Box<dyn Any + Send>>`.
- **move closure:** a closure that takes ownership of the variables it captures, required by `thread::spawn`.
- **preemptive scheduling:** the OS decides when each thread runs, so interleaving is outside your control.
- **'static:** a bound meaning "no borrowed data — the value lives forever or is fully owned."

## Exercise

Open `exercises/` and fill in the `// TODO(module-031)` comments in `src/lib.rs`:

1. Implement `compute_in_parallel(inputs) -> Vec<u32>`: spawn one thread per input element (moving the element into the closure), each thread squares its element, and the results are collected in **input order** after all joins.
2. Implement `sum_squares_parallel(n, threads) -> u64`: split `1..=n` across `threads` strided worker threads, sum their partials, and return the total. It must agree with the sequential sum for any `n` and `threads`.

The tests in `tests/module_031.rs` define "done": input-order preservation, empty-input handling, multi-thread correctness against the closed-form sum, and edge cases like more threads than values.

```bash
cargo test -p module-031-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 16.1: Using Threads to Run Code Simultaneously](https://doc.rust-lang.org/book/ch16-01-threads.html)
- [`std::thread` API reference](https://doc.rust-lang.org/std/thread/index.html)
- [The Rustonomicon — "Threads" section of the memory model discussion](https://doc.rust-lang.org/nomicon/threads.html)
