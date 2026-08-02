# Module 042: The Tokio Runtime

**Block:** Block E — Async Rust
**Estimated time:** 60–90 min
**Prerequisites:** Module 041 (Future trait, poll, waker); Module 034 (atomics/`Send`)

## Learning Objectives

- Start a Tokio runtime two ways: `#[tokio::main]` and the `Runtime` builder API.
- Spawn independent tasks with `tokio::spawn` and collect results from `JoinHandle`s.
- Distinguish the current-thread flavor from the multi-thread flavor and know when each fits.
- Explain how the event loop schedules ready tasks and how wakers put tasks back on the queue.
- Reason about wall-clock overlap: N concurrent sleeps take ~1× the sleep, not N×.

## Why This Matters

Every serious async Rust project is a Tokio application: `axum`, `sqlx`, `tonic` all run on Tokio's runtime. The single most common first-blocker for newcomers is a runtime mismatch — spawning a task from a thread that isn't the runtime thread, or building a runtime when the macro already gave you one. If you understand the event loop itself, these errors stop being magic. And the choice between current-thread and multi-thread flavors is a real engineering decision you will make in production services.

## Concept

### The runtime is the executor, plus

Module 041 ended with a one-future executor: poll, park, repeat. Tokio is the production version of that loop with two additions:

1. **A scheduler** — many tasks share each thread, and the runtime decides which ready task runs next.
2. **OS event integration** — instead of a thread sleeping and being woken by a helper thread, the runtime hands the kernel a set of I/O interests (epoll on Linux, kqueue on macOS, IOCP on Windows) and is woken directly by kernel events. Your `Delay`'s helper thread becomes the kernel's timer wheel.

Everything else you know carries over: `Poll::Pending` means "wake me later", and the waker is still the phone number the future registers.

### The event loop

Here is the scheduling loop that runs on each runtime thread, simplified:

```
        ┌──────────────────────────────────────────────────────────┐
        │                    THE EVENT LOOP                         │
        │                                                            │
        │   ┌────────────┐      poll task A ──► Pending             │
        │   │ ready queue │◄───── poll task B ──► Pending            │
        │   │ [task A,    │      poll task C ──► Ready (return)      │
        │   │  task B,    │                                          │
        │   │  task D]    │      ...queue empty... ──► park          │
        │   └────────────┘              ▲                            │
        │         ▲                     │ unpark / wake()            │
        │         │                     │                            │
        │         └── push onto queue ──┘                            │
        │                     │                                     │
        └─────────────────────┼─────────────────────────────────────┘
                              │
        kernel timer fires / socket readable ──► runtime calls waker.wake()
```

Reading it from the bottom: some event fires (a timer elapsed, a socket got data). The runtime calls the waker that was registered for that event. The waker's job is to push the task onto that thread's ready queue. The loop pops a ready task, polls it once, and — if the task returns `Pending` — the task has already re-registered its waker for the next event, so the loop moves on to the next ready task. Only when *no* task can make progress does the thread park, and it stays parked until an event wakes it.

Notice what the loop never does: it never waits *inside* a task. A task that returns `Pending` has voluntarily given the thread back. That is the entire contract that makes thousands of "concurrent" tasks cheap — they are not parallel, they are *cooperatively interleaved* on shared threads.

### Starting a runtime: the macro

The most common way to enter the runtime is the attribute macro:

```rust
#[tokio::main]
async fn main() {
    let a = tokio::spawn(async { 20 });
    let b = tokio::spawn(async { 22 });
    println!("{}", a.await.unwrap() + b.await.unwrap());
}
```

`#[tokio::main]` is sugar for exactly this:

```rust
fn main() {
    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async { /* your async fn body */ });
}
```

Your `main` stays `fn main`; the macro builds a multi-threaded runtime, enters it with `block_on`, runs your async body, and shuts the runtime down when it finishes. Every test in this module runs under `#[tokio::test]`, which is the same macro applied to a test function.

### The builder API

When you need a specific flavor, use `Runtime`'s builder. These are the three options in practice:

```rust
// Multi-thread: a pool of worker threads, best for CPU-light,
// I/O-heavy workloads. Default for #[tokio::main].
let multi = tokio::runtime::Builder::new_multi_thread()
    .worker_threads(4)          // how many threads to run
    .enable_all()               // timers + I/O drivers
    .build()?;

// Current-thread: everything on the calling thread.
// Cheap, deterministic; great for tests and small tools.
let single = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build()?;

let answer = single.block_on(async { 6 * 7 });
assert_eq!(answer, 42);
# fn main() {}
```

Which flavor? Rule of thumb: a network service with many concurrent connections wants multi-thread (one worker thread per CPU is the default). A test that checks ordering, or a small script that just needs `async` syntax, is happy with current-thread. The current-thread runtime is also where blocking bugs are easiest to see — everything is on one thread, so one blocking call stalls *all* tasks (Module 049 will return to this).

### `tokio::spawn`

`tokio::spawn(future)` hands a new task to the runtime and returns a `JoinHandle`. Three things to know:

- The future must be `Send + 'static`: it may be moved to another worker thread, and it must not borrow from the spawning scope. The compiler enforces both.
- A spawned task runs as soon as it is polled by the loop — it does *not* start the moment you call `spawn`.
- `JoinHandle` is a future: `.await` it to get `Result<T, JoinError>`. `Err` means the task panicked or was aborted — that is how a panic in a spawned task stays contained instead of taking down your whole service.

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "finished"
    });
    println!("{}", handle.await.unwrap());
}
```

When you spawn many tasks, keep the handles in a `Vec` and await them all afterwards — that is the pattern your exercise uses, and it is how the runtime's overlap shows up: ten spawned `sleep(100ms)` tasks all finish after ~100 ms, not ~1 s.

### One runtime, many threads

A common confusion: "I spawn a task, which thread does it run on?" The answer is "whichever worker picks it up". Tasks are the unit of concurrency; threads are the unit of *parallelism*. Two tasks on a single worker thread still interleave (cooperation); two tasks on different workers can run simultaneously (true parallelism). You do not choose the thread — the scheduler does. Your job is to make tasks that yield often and share state safely (`Send`/`Sync`, from Module 034).

## Common Pitfalls

- **Calling `tokio::spawn` outside the runtime.** You get `there is no reactor running`. Either enter via `#[tokio::main]`/`block_on`, or pass the runtime around via `Handle`.
- **Awaiting spawned tasks in a loop after spawning them** — that is correct; but if you await each handle *inside* the spawn loop, you serialize the tasks and lose all overlap.
- **Panicking inside a spawned task.** The panic is caught by the runtime and surfaces as `Err(JoinError)` on the handle — `unwrap` it consciously or handle it.
- **Using `std::thread::sleep` in async code.** It parks the whole worker thread. Use `tokio::time::sleep`, which only yields the current task (details in Module 049).
- **Forgetting `enable_all()` in a manual builder.** Timers and I/O silently misbehave; `enable_all` turns on the time and I/O drivers.

## Key Terms

- **Runtime:** the executor + scheduler + I/O drivers together; you enter it once with `block_on`.
- **Task:** a `tokio::spawn`-ed future the runtime schedules; the unit of concurrency.
- **JoinHandle:** the future you await to collect a task's result (or its panic).
- **Worker thread:** a thread running an event loop; multi-thread flavor runs several.
- **Flavor:** current-thread (one loop, deterministic) vs multi-thread (worker pool, parallel).

## Exercise

Work in `exercises/` and make `cargo test -p module-042-exercises` pass. Five TODOs in `src/lib.rs`:

1. `spawn_and_sum` — spawn two tasks, await both handles, sum.
2. `spawn_many_sum` — spawn `n` tasks, collect handles, sum all outputs.
3. `multi_thread_blocking_sum` — build a multi-thread runtime with the builder, `block_on` an async block that spawns and sums.
4. `current_thread_blocking_sum` — the same with the current-thread builder.
5. `parallel_sleep_total` — spawn `n` sleeping tasks and measure the batch's wall-clock time.

The tests assert the sums are correct and that four 60 ms sleeps finish in under 150 ms — if that last test fails, your tasks are running one after another instead of concurrently. Check `solutions/` afterwards.

## Further Reading

- [Tokio docs: Tokio overview](https://docs.rs/tokio/latest/tokio/) — start with the "Runtime" section
- [Tokio tutorial: Spawning](https://tokio.rs/tokio/tutorial/spawning)
- [The Async Book: Executor & Runtime](https://rust-lang.github.io/async-book/02_execution/04_executor.html)
