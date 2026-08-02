# Module 041: Async Fundamentals — The Future Trait Before Tokio

**Block:** Block E — Async Rust
**Estimated time:** 45–90 min
**Prerequisites:** Modules 016–017 (traits), 031–034 (threads, `Arc`/`Mutex`, atomics)

## Learning Objectives

- Explain what a `Future` is and what `async`/`.await` desugar to.
- Implement the `Future` trait by hand for a real, poll-based type.
- Write a working `block_on` executor using `std::task::Waker` and `thread::park`/`unpark`.
- Describe the poll state machine: `Pending` → waker fires → re-poll → `Ready`.
- Understand why this machinery exists *before* any runtime is introduced.

## Why This Matters

Every async library you will use for the rest of your career — Tokio, axum, `sqlx` — is built on one trait: `Future`. `async fn` and `.await` are compiler sugar over it. When you understand the poll state machine, error messages like "future is not `Send`" and "value was dropped while borrowed" stop being magic, and you can reason about *why* async code blocks one thread but not another. This module builds that foundation with zero dependencies, so the machinery is fully visible.

## Concept

### The problem async solves

Threads are expensive: each one costs stack memory (typically MBs of virtual address space), context switches, and kernel involvement. A server handling 10,000 simultaneous connections with one thread per connection would be dead on arrival. Async Rust gives you *concurrency without threads*: many tasks share one thread, and a task pauses itself (yields) whenever it would otherwise wait — on a socket read, a timer, a lock. While it waits, the thread runs other tasks.

The price of this trick is that nothing is free: the runtime needs a precise contract from each task about *when* it wants to run again. That contract is the `Future` trait.

### The `Future` trait

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

A `Future` is a *state machine you ask "are you done yet?"*. `poll` is that question:

- `Poll::Ready(value)` — done, here is the result.
- `Poll::Pending` — not done; call me again **only** when something I'm waiting on makes progress.

The `Context` carries a `Waker`. The waker is the future's phone number: the future registers it with whatever it's waiting on (a timer, a socket), and that thing calls `waker.wake()` when progress is possible. The runtime then knows to re-poll the future. If no waker ever fires, a `Pending` future can sit forever — and that is fine, because it means *nothing it depends on has changed*.

The most important discipline: **every time `poll` returns `Pending`, it must have ensured a waker is registered somewhere.** Returning `Pending` without registering a waker is a bug — the future will never be polled again and your program stalls.

### The poll state machine

Your exercise implements a `Delay` future. Every poll follows this state machine:

```
        poll()
          |
          v
    deadline passed? ---- yes ----> Poll::Ready(())   <-- fire the waker
          |                                          ^      once, then
          no                                         |      re-poll: now
          |                                          |      the deadline
          v                                          |      has passed
    first poll? ---- yes ----> spawn sleeper thread ------+
          |                    (sleeps until deadline,
          no                   then wakes the waker)
          |
          v
    store waker, return Poll::Pending
```

Reading the diagram from the top: a fresh `Delay` is polled, the deadline has not passed, so we arm the sleeper thread (only once — `swap(true)` on an `AtomicBool` makes the first poll distinguishable from later ones), store the caller's waker, and answer `Pending`. The sleeper thread wakes us when the deadline arrives. Re-polled, the deadline has passed, so we answer `Ready(())`. The first branch (deadline already passed) also makes `Delay::new(Duration::ZERO)` complete instantly on the very first poll.

Note the handoff in the middle: the future and the thread do not share a `&mut` to the same data. They share an `Arc<DelayState>`; the thread only needs the `Waker`, which it takes out of the `Mutex` with `.take()` so a stale clone can't double-wake.

### `async`/`.await` is sugar

You never write `poll` yourself when you use `async`/`.await`; the compiler writes it for you. This:

```rust
async fn fetch_two_things() -> u32 {
    let a = Delay::new(Duration::from_millis(10)).await;
    let b = Delay::new(Duration::from_millis(20)).await;
    a + b
}
```

desugars into a struct that stores its local state (the two `Delay`s, plus a marker of how far it got), and a `poll` whose state machine is *your function body cut at each `.await`*:

```
Poll 1: first Delay -> Pending
        (waker registered with the timer)
Poll 2 (woken): first Delay done -> run second Delay -> Pending
        (waker registered again)
Poll 3 (woken): second Delay done -> Poll::Ready(30)
```

Each `.await` becomes a "pause point": the compiler saves the intermediate values and continues on the next poll. That is why an `async fn` is not executed at all until something polls it — `async { }` and `async fn` only *build the state machine*.

### The executor loop

Something has to keep calling `poll`. That something is an executor. The whole of it fits in a loop:

```rust
loop {
    if let Poll::Ready(v) = fut.as_mut().poll(&mut cx) {
        return v;
    }
    thread::park();
}
```

Poll; if not done, park the thread; when a waker fires it `unpark`s the thread; loop. `thread::park`/`unpark` are the primitive that makes "no lost wakeups" work: if `unpark` happens before `park` runs, the next `park` returns immediately instead of sleeping. This `block_on` is a real, correct executor — just one that can only run one future at a time, on one thread. Tokio's runtime is this loop plus two superpowers: a scheduler that runs *many* tasks on each thread, and event sources (epoll/kqueue/IOCP) that integrate directly with the OS so `wake()` can fire from an I/O event.

### What a waker is made of

A waker is just a trait object with one method, `wake`:

```rust
struct ThreadWaker(thread::Thread);

impl Wake for ThreadWaker {
    fn wake(self: Arc<Self>) {
        self.0.unpark();
    }
}
```

`Waker::from(Arc::new(ThreadWaker(thread::current())))` hands out an owned, clonable "wake this thread" handle. Any thread can hold it — which is exactly what your `Delay`'s sleeper thread does.

One common misconception to kill now: awaiting `Delay::new(30ms)` does **not** busy-wait. The executor parks; the sleeper thread sleeps at the OS level; CPU is consumed only when a waker fires. "Async" does not mean "faster" — it means *more tasks per thread*.

## Common Pitfalls

- **Returning `Pending` without registering a waker.** The future will never be polled again and the program hangs forever. Register the waker on *every* `Pending` return, or at least once before the first `Pending`.
- **Spawning the helper thread on every poll.** Polling can happen many times; use `swap(true)` on an `AtomicBool` so the "first poll" setup runs once.
- **Calling `poll` with a moved future.** A future must stay put between polls — that is what `Pin` is for. This module sidesteps the details with `Box::pin`; Module 046 covers *why*.
- **Believing `.await` blocks the thread.** `.await` *yields* the thread. Blocking the thread inside async code is the sin Module 049 exists to fix.

## Key Terms

- **Future:** a pollable state machine; `poll` asks it "done yet?"
- **Poll::Pending:** "not done — wake me when progress is possible".
- **Waker:** a clonable handle passed via `Context`; calling `wake()` tells the executor to re-poll.
- **Executor (runtime):** the loop that polls futures and parks when nothing can progress.
- **`block_on`:** the simplest executor: poll, park, repeat, on the current thread.

## Exercise

Work in `exercises/` and make `cargo test -p module-041-exercises` pass. There are two TODOs in `src/lib.rs`:

1. `TODO(module-041)` in `impl Future for Delay` — implement the poll state machine described in the Concept section (deadline check, one-shot thread spawn, waker registration).
2. `TODO(module-041)` in `block_on` — pin the future, build a `ThreadWaker` from `std::task::Wake`, and run the poll/park loop.

The tests in `tests/module_041.rs` check that a `Delay` completes, that an `async` block runs through your `block_on`, that the first poll reports `Pending`, that a poll after the deadline is `Ready`, and that the delay actually waits. Compare with `solutions/` after you have a passing run. Note: if a test *hangs* rather than fails, a future is returning `Pending` without a waker — check that path first.

## Further Reading

- [The Rust Book: Async and Await](https://doc.rust-lang.org/book/ch20-01-multi-threaded-web-server.html) (and the newer [Async Book chapter](https://rust-lang.github.io/async-book/02_execution/01_chapter.html))
- [std::task::Wake documentation](https://doc.rust-lang.org/std/task/trait.Wake.html) — the docs include this exact `block_on` example
- [The Async Book: Executors & Wakers](https://rust-lang.github.io/async-book/02_execution/03_wakeups.html)
