# Module 034: Concurrency IV — `Send`, `Sync`, and Atomics

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 90–120 min
**Prerequisites:** Module 032 (`Mutex`/`Arc`), Module 031 (threads), Module 016 (traits)

## Learning Objectives

- State the definitions of `Send` and `Sync`, explain how they're derived automatically, and name types that fail each bound and why.
- Explain why `T: Sync` is equivalent to `&T: Send`, and how this makes `Arc<T>` sharable.
- Use `AtomicUsize`, `AtomicBool`, and friends for lock-free shared counters and flags.
- Describe the read-modify-write (RMW) cycle and why it is atomic on the hardware.
- Use `compare_exchange` to implement a "claim once" flag, and reason about memory orderings (`Relaxed`, `SeqCst`, `Acquire`/`Release`).

## Why This Matters

`Send`/`Sync` are the compile-time backbone of every concurrent design in Rust: frameworks like Tokio and Rayon are *built* on the fact that the compiler rejects unsendable state at the type level. Atomics, meanwhile, are what `Arc`'s refcount, every lock-free queue, and every high-performance counter are made of. A senior Rust developer is distinguished by exactly this: knowing when a `Mutex` is needed, when an atomic suffices, and what ordering to pick — and this module is where that distinction crystallizes.

## Concept

### `Send` and `Sync` — the two safety properties

Rust prevents data races with two marker traits. A **marker trait** has no methods — it's a label the compiler attaches to a type and enforces.

- **`Send`**: the type is safe to *move* to another thread. Moving transfers ownership: when you `send` a value through a channel or move it into a `thread::spawn` closure, the value must be `Send`.
- **`Sync`**: the type is safe to *share* — that is, references to it (`&T`) can be used from multiple threads simultaneously. Sharing is where data races happen, so this is the stronger property.

The two are related by an identity you should memorize: **`T: Sync` if and only if `&T: Send`.** Sharing a `&T` across threads is just moving the reference; if a reference can't be sent, the type can't be sync.

Most types get these labels automatically. `u32` is both; `String` is both. The interesting ones are the exceptions:

| Type | `Send` | `Sync` | Why |
|------|:------:|:------:|-----|
| `Rc<T>` | no | no | Refcount is a plain `usize`; two threads bumping it race. |
| `RefCell<T>` | yes | no | Runtime borrow checks aren't atomic; two threads sharing `&RefCell` could double-borrow. |
| `Cell<T>` | yes | no | Same reason — no atomicity on access. |
| `Mutex<T>` | yes | yes | Internal state handles cross-thread access. |
| `AtomicUsize` | yes | yes | All ops are atomic by construction. |
| `*mut T` | no* | no* | Raw pointers have no inherent synchronization (Module 035). |
| `Arc<T>` | if `T: Send + Sync` | if `T: Send + Sync` | It's a reference-counted pointer to shared data. |

This table is why Module 032's pattern works: `Arc<Mutex<T>>` is `Send + Sync` because `Mutex<T>` is — the mutex's internal synchronization is exactly what makes concurrent access safe.

The labels are *derived*: a struct is `Send` if all its fields are. The compiler computes this for you. `unsafe impl Send` is available (Module 036) for types whose safety is guaranteed by your *contract* rather than the compiler — but for now, treat the derived bounds as law: if it compiles, it's sound; if it doesn't, your design has a real race.

### Atomics: shared state without locks

An **atomic** is a variable whose load, store, and read-modify-write operations are indivisible: the hardware guarantees no other thread observes a half-completed operation. The standard library ships one per integer width: `AtomicBool`, `AtomicI32`, `AtomicU64`, `AtomicUsize`, and more. Use `AtomicUsize` for counts and indices, `AtomicBool` for flags, `AtomicU64`/`AtomicI32` for numeric state.

Atomic access is faster than a `Mutex` (no OS-level blocking — contention resolves in hardware), but each atomic protects *only itself*, which is why the Module 032 counter's clippy suggestion makes sense: a single counter is exactly the atomic-shaped problem.

The core operations:

```rust
use std::sync::atomic::{AtomicUsize, Ordering};

let counter = AtomicUsize::new(0);

counter.fetch_add(1, Ordering::SeqCst);        // atomic read-modify-write, returns old value
counter.store(10, Ordering::SeqCst);           // atomic write
let value = counter.load(Ordering::SeqCst);    // atomic read
assert_eq!(value, 10);
```

`fetch_add` is the workhorse. Here's what "atomic" means at the hardware level — the read-modify-write (RMW) cycle:

```
                    memory cell: AtomicUsize
                    ┌───────────────────┐
   fetch_add(1) ──► │ 1. READ  → 5      │
                    │ 2. COMPUTE → 5+1  │   ← all three steps happen
                    │ 3. WRITE → 6      │     as ONE indivisible unit;
                    └───────────────────┘     nothing can interleave

   two threads calling fetch_add(1) on the same cell:

   thread A: READ 5 → COMPUTE 6 → WRITE 6      total: 6
   thread B: (interleaves AFTER A's write) 
             READ 6 → COMPUTE 7 → WRITE 7      total: 7  ← no increment lost
```

Compare this with the Module 032 non-atomic `+=`: there the three steps were separate, so two threads could both read 5 and both write 6 — losing an increment. The atomic RMW cannot lose updates; the hardware serializes competing RMWs.

### `compare_exchange` — conditional writes

The most powerful atomic operation is **compare-and-swap (CAS)**: check the current value, and only if it matches what you expect, replace it — all atomically. It returns `Ok(old)` on success and `Err(actual)` on failure:

```rust
use std::sync::atomic::{AtomicBool, Ordering};

let flag = AtomicBool::new(false);

let first_claim = flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
assert_eq!(first_claim, Ok(false));      // cell was false → wrote true → claimed!

let second_claim = flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst);
assert_eq!(second_claim, Err(true));     // cell was true → not ours → didn't write
```

```
  compare_exchange(expected, new)
  ──────────────────────────────────────────────
  if cell == expected:   write new,  return Ok(old)
  if cell != expected:   write nothing, return Err(actual)

  cell = false:  CAS(false, true) → Ok(false),  cell is now true
  cell = true:   CAS(false, true) → Err(true),  cell stays true
```

CAS is the primitive behind "claim this once," "swap this pointer," and "push this node onto a lock-free stack." If CAS keeps failing, you loop and retry — the classic *optimistic* concurrency pattern:

```rust
let mut current = counter.load(Ordering::SeqCst);
loop {
    let next = current + 1;
    match counter.compare_exchange(current, next, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(_) => break,                 // we won the race
        Err(actual) => current = actual, // someone else wrote; retry with fresh value
    }
}
```

### Memory ordering: `Relaxed`, `Acquire`/`Release`, `SeqCst`

Atomics make a *single* operation atomic — but threads also need guarantees about the order in which other memory operations become visible. That's what the `Ordering` argument controls:

- **`Relaxed`**: only the atomic itself is guaranteed atomic. No ordering of anything else. Right for pure counters where you never use the value to signal anything.
- **`Acquire`/`Release`** (paired): a `Release` store makes everything written *before* it visible to a thread that `Acquire`-loads the same variable. This is the memory-barrier pattern that implements locks, channels, and refcounts correctly.
- **`SeqCst`** (sequentially consistent): the strongest — it behaves as if all atomics in the program took place in one global order. Easiest to reason about, slightly slower, and never *wrong* when you don't know what you need.

The honest rule: **use `SeqCst` until you can prove a weaker ordering is correct.** `Relaxed` on a counter that is only ever counted is fine; `Relaxed` on a flag that gates access to shared data is a subtle data-race bug. This module's exercises use `SeqCst` on purpose — choosing orderings correctly is Module 034's stretch topic, not its core.

## Common Pitfalls

- **`Cell`/`RefCell` across threads.** They're `Send` but not `Sync` — sharing them is a compile error. The compiler *will* catch it; the pitfall is reaching for them anyway instead of `AtomicUsize`/`Mutex`.
- **`Rc` in threaded code.** Not `Send`, not `Sync` — same trap as Module 032. If any clone could reach another thread, use `Arc`.
- **Assuming "atomic" means "ordered."** An atomic counter prevents lost updates, but it does *not* prevent another thread from seeing your other writes half-completed. That's what memory orderings (and locks) are for.
- **`Relaxed` on a signaling flag.** Setting `RUNNING.store(false, Relaxed)` while a worker depends on data written before it is a real race: the worker can observe the flag change before the data. Use `Release`+`Acquire` or `SeqCst` for anything that gates other memory.
- **`fetch_add` when you need the *new* value.** `fetch_add` returns the *old* value. If you need the result after the update (e.g. an id), compute `old + 1` yourself (careful with overflow) or use a CAS loop.

## Key Terms

- **Send:** safe to move to another thread (ownership transfer).
- **Sync:** safe to share a reference across threads; `T: Sync ⟺ &T: Send`.
- **marker trait:** a trait with no methods used as a compiler label.
- **atomic:** a variable whose loads, stores, and RMW operations are indivisible.
- **read-modify-write (RMW):** the read → compute → write cycle that atomics execute as one step.
- **compare_exchange (CAS):** conditionally write a new value only if the cell still holds an expected value; returns `Ok`/`Err`.
- **memory ordering:** the argument controlling how atomics order other memory operations (`Relaxed`, `Acquire`, `Release`, `SeqCst`).
- **lock-free:** a synchronization design that never blocks threads — retries instead of sleeping.

## Exercise

Open `exercises/` and fill in the `// TODO(module-034)` comments in `src/lib.rs`:

1. Complete `AtomicCounter`: `new()` starts the `AtomicUsize` at 0, `increment()` uses `fetch_add(1, Ordering::SeqCst)` and returns the *new* value, `total()` uses `load`.
2. Implement `run_atomic_increments(threads, per_thread) -> usize` with `Arc<AtomicCounter>` — same shape as Module 032, but lock-free now. The final total must be exactly `threads * per_thread`.
3. Implement `try_claim(flag: &AtomicBool) -> bool`: return true if this call changed the flag from `false` to `true` (use `compare_exchange`), false otherwise.
4. `assert_thread_safe<T: Send + Sync>()` is already implemented — the tests use it to prove your types are sharable.

The tests check the counter under contention, that exactly one of 8 racing threads wins `try_claim`, and that the claim fails afterwards.

```bash
cargo test -p module-034-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 16.4: Extensible Concurrency with the `Sync` and `Send` Traits](https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html)
- [`std::sync::atomic` API reference](https://doc.rust-lang.org/std/sync/atomic/index.html)
- [Rust Atomics and Locks (free online book), by Mara Bos](https://marabos.nl/atomics/)
- [Rust Reference — "Behavior considered undefined" (the data-race rule)](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
