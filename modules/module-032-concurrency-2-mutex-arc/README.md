# Module 032: Concurrency II — `Mutex<T>` and `Arc<T>`

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 60–90 min
**Prerequisites:** Module 031 (threads), Module 029 (`Rc`/interior mutability), Module 028 (`Drop`/RAII)

## Learning Objectives

- Protect shared data with `Mutex<T>` and explain what the lock actually protects (the *data*, not the code).
- Use `lock()` to obtain a `MutexGuard`, mutate through it via `DerefMut`, and understand that dropping the guard releases the lock (RAII).
- Share one `Mutex<T>` across threads with `Arc<T>`, and explain why `Rc<T>` cannot cross threads.
- Handle lock poisoning with `lock().unwrap()` and know when to recover instead.
- Build a thread-safe counter and verify exact totals under concurrent increments.

## Why This Matters

Message passing (Module 033) covers some concurrency, but real codebases are full of shared mutable state: request counters, connection pools, caches, configuration that gets reloaded. `Mutex<T>` + `Arc<T>` is the workhorse — every connection pool, every `Arc<Mutex<T>>`-wrapped app state in a web framework, every in-process cache uses this exact pair. When people say "Rust makes data races impossible," `Mutex` and `Arc` are two of the mechanisms that make it true — and they're also where learners write their first deadlocks, so this module deserves your full attention.

## Concept

### The problem: two threads, one variable

In Module 031 you avoided the hard part by keeping all data thread-local and moving results through `join()`. But consider a shared counter. Two threads both execute `counter += 1`. On the CPU that's really three steps: read the value, add one, write it back. If thread A reads 5, then thread B reads 5, then A writes 6 and B writes 6, the counter went from 5 to 6 even though two increments ran. That's a **data race** — the single worst kind of bug: intermittent, timing-dependent, and impossible to reproduce on demand.

Rust prevents you from writing racy code at compile time — you literally cannot share a `&mut usize` with another thread — but it can't prevent *synchronized* access patterns from racing if *you* get the synchronization wrong. That's what `Mutex` is for: it serializes access so the read-modify-write triple becomes atomic with respect to other threads.

### How `Mutex<T>` works

A `Mutex<T>` is a box with a lock. The lock has two states: **unlocked** and **locked**. A thread that wants the data must first acquire the lock; if another thread holds it, the acquirer blocks (sleeps) until it's released. Here's the state diagram:

```
                lock() succeeds                       lock() blocks
   unlocked ──────────────────► locked ◄────────────────────┐
      ▲                          │    │                     │
      │                          │    │  (held by thread A) │  thread B calls
      │                          │    │                     │  lock(); sleeps
      │                     drop(guard) │  ...mutate data...│  until unlocked
      │                          │    ▼                     │
      └──────────────────────────┴─── unlocked ◄────────────┘
            guard dropped:            (thread B wakes,
            lock released             acquires, proceeds)
```

The crucial idea: **the lock protects the data, not the code.** You can't write a racy `counter += 1` in Rust, but the rule to internalize is: *any* read or write of the guarded data must happen while holding the lock. Rust enforces this by construction — the data is only reachable through the `MutexGuard` — which is the entire trick.

In code:

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);

{
    // lock() returns Result<MutexGuard<T>, PoisonError<T>>
    let mut guard = counter.lock().unwrap();
    *guard += 1;            // DerefMut: guard behaves like &mut T
    // guard is dropped here (end of block) → lock released
}

let value = *counter.lock().unwrap();
assert_eq!(value, 1);
```

Three details matter:

1. **`lock()` returns a `Result`.** The error case is poisoning — see below. `unwrap()` is the standard "panic loudly" choice.
2. **`MutexGuard` derefs to the data.** `*guard += 1` works because `guard` implements `Deref`/`DerefMut` (Module 028). The guard is a smart pointer that hands you `&mut T` *only while you hold the lock* — that's what makes the lock enforceable.
3. **Unlocking is automatic.** `MutexGuard` implements `Drop`; when the guard goes out of scope, the lock is released. This RAII behavior (Module 028) means there is no way to forget to unlock — which is exactly why Rust's `Mutex` is far less deadlock-prone than C's, where unlocking is a manual call you can skip on every error path.

### Sharing across threads: `Arc<T>`

To share the `Mutex` with several threads, every thread needs access to the same heap allocation. `Rc<T>` (Module 029) does shared ownership on one thread — but `Rc`'s reference count is a plain integer, and two threads bumping it concurrently would race. So `Rc<T>` is not `Send`, and the compiler rejects it in `thread::spawn`:

```rust,ignore
use std::rc::Rc;
use std::sync::Mutex;

let counter = Rc::new(Mutex::new(0));
std::thread::spawn(move || { *counter.lock().unwrap() += 1; });
// error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
```

The fix is `Arc<T>` ("atomically reference counted"): a cloneable reference counter whose counter itself is an atomic (a sneak preview of Module 034). Cloning an `Arc` is cheap — it bumps the count, it does not copy the data:

```rust
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let mut handles = Vec::new();

for _ in 0..4 {
    let counter = Arc::clone(&counter);   // cheap: refcount++ only
    handles.push(std::thread::spawn(move || {
        for _ in 0..1000 {
            let mut guard = counter.lock().unwrap();
            *guard += 1;
        }
    }));
}

for handle in handles {
    handle.join().unwrap();
}

assert_eq!(*counter.lock().unwrap(), 4000);
```

For this to compile, two bounds must hold (these get formal names in Module 034): the closure must be `Send` (safe to move to another thread), and the shared data must be `Sync` (safe to be referenced from multiple threads at once). `Arc<Mutex<T>>` satisfies both whenever `T` is `Send`. That's the point: **Rust's type system makes the "share data safely" route the only route that compiles.**

### Poisoning

What if a thread panics *while holding the lock*? The guard is dropped during unwinding, so the lock is released — but the data may be in a half-updated state. Rust marks the mutex **poisoned** so the rest of the program can't silently read garbage. Every subsequent `lock()` returns `Err(PoisonError)`.

The standard handling is `lock().unwrap()`: "if the data is poisoned, panic here — a bug already happened." If you have a design where the data is always valid at the panic point (e.g. it's just a counter), you can recover:

```rust
use std::sync::{Mutex, PoisonError};

let counter: Mutex<usize> = Mutex::new(0);
let guard = counter.lock().unwrap_or_else(PoisonError::into_inner);
// `into_inner` hands you the guard anyway; the data is yours.
```

For the exercises, `lock().unwrap()` is the right call. In production you decide per mutex whether poisoning is recoverable.

### A note you will meet in this module's exercise

`cargo clippy` on a `Mutex<usize>` counter suggests `AtomicUsize` — `clippy::mutex_atomic`. That's correct advice for production: atomics (Module 034) are the right tool for a single counter. The exercise keeps `Mutex` anyway because the *point* is the lock, not the counter — so the exercise crate carries an `#[allow(clippy::mutex_atomic)]` with a comment explaining exactly that. When you see a lint allowed with an explanatory comment, that's usually a deliberate trade, not laziness.

## Common Pitfalls

- **Holding a guard across an unrelated wait.** Keep the critical section as small as possible. If you hold `Mutex` A while acquiring `Mutex` B and another thread does the reverse, you have a deadlock: each waits for the other. Rust can't detect this — it's purely your design.
- **`lock()` in a loop or hot path.** Locking is cheap-ish but not free; if a mutex is contended, threads sleep and wake, which costs microseconds. Prefer bigger critical sections over frequent lock/unlock, and consider read-write locking for read-heavy data.
- **Locking the wrong mutex.** The lock only protects *its* data. Two threads mutating `counter` while one of them also reads a different `Mutex`-guarded value can still produce inconsistent views — the locks don't compose automatically.
- **Returning the guard.** If a function returns `MutexGuard<'_, T>` it's a red flag — you're handing the lock (not just the data) to the caller, and the critical section length becomes unpredictable. Return the data, or restructure.
- **Using `Rc` across threads.** `Rc` is not `Send` — the compiler will stop you, but learners still try because "it compiled on one thread." Reach for `Arc` whenever any clone might cross a thread boundary.

## Key Terms

- **Mutex:** mutual exclusion — a lock guarding data; at most one thread holds it at a time.
- **MutexGuard:** the RAII token returned by `lock()`; derefs to the data, releases the lock on drop.
- **Critical section:** the region of code between acquiring and releasing a lock.
- **Data race:** two threads accessing the same memory without synchronization, at least one write — undefined behavior.
- **Deadlock:** threads each holding a lock the other needs, all blocking forever.
- **Arc:** atomically reference-counted pointer; clones share ownership and are safe across threads.
- **Poisoning:** a mutex marked permanently broken after a thread panicked while holding it.
- **Send / Sync:** the compile-time safety properties that let `Arc<Mutex<T>>` be shared (full treatment in Module 034).

## Exercise

Open `exercises/` and fill in the `// TODO(module-032)` comments in `src/lib.rs`:

1. Complete the `Counter` struct and its methods: `new()` wraps 0 in a `Mutex<usize>`; `increment()` locks, adds 1, and returns the *new* value; `total()` locks and returns the current value.
2. Implement `run_threaded_increments(threads, per_thread) -> usize`: create an `Arc<Counter>`, spawn `threads` workers each calling `increment()` `per_thread` times, join all, and return the final total. The result must be exactly `threads * per_thread` — if your locking is wrong, this test fails.

The tests in `tests/module_032.rs` check single-thread behavior, the exact final total under contention, and edge cases like zero threads.

```bash
cargo test -p module-032-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 16.3: Shared-State Concurrency](https://doc.rust-lang.org/book/ch16-03-shared-state.html)
- [`std::sync::Mutex` API reference](https://doc.rust-lang.org/std/sync/struct.Mutex.html)
- [`std::sync::Arc` API reference](https://doc.rust-lang.org/std/sync/struct.Arc.html)
