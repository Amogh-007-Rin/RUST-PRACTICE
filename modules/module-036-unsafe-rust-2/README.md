# Module 036: Unsafe Rust II — Mutable Statics, Unions, and the FFI Preview

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 90–120 min
**Prerequisites:** Module 035 (raw pointers), Module 034 (`Send`/`Sync`, atomics), Module 032 (`Mutex`)

## Learning Objectives

- Distinguish `static` from `const`, and explain why `static mut` is so restricted that every access needs `unsafe`.
- Build a *sound* mutable-static pattern: all access serialized by a lock, so the statics are thread-safe without a single atomic.
- Write `unsafe impl Send`/`Sync` deliberately — and explain the contract that makes them sound.
- Define a `union`, write fields safely, and read them under `unsafe` (reinterpretation, not conversion).
- Perform sound raw-pointer casts (`.cast()`, `read_unaligned`) and preview `extern "C"` FFI declarations.

## Why This Matters

Module 035 gave you the mechanics of raw pointers. This module covers the three remaining unsafe superpowers you'll meet in real code: **mutable statics** (global state — usually an anti-pattern, but you'll see them in legacy code and in `lazy_static`-style machinery), **unions** (the C-style memory overlay — pervasive in FFI and in efficient serialization code), and the **FFI boundary** itself (`extern "C"`, which Modules 052–053 build out fully). Knowing when a `static mut` is *actually sound* — and being able to say precisely why — is exactly the skill reviewers probe in production code review.

## Concept

### `const` vs `static`

Both are compile-time-defined values, but they differ in one decisive way:

- `const` has **no address**. It's inlined everywhere it's used — semantically it's just a name for a value.
- `static` is **one memory cell in the process**, shared by all code. `static NAME: &str = "srv";` allocates one string in static memory; every reference points at the same bytes.

Statics are also special in Rust: they have no destructor (they live for the whole process), they're `'static` (surprise), and — the key part — a `static` must be either immutable or accessed under `unsafe`. You can read a plain `static` safely from any thread (that's what makes `&str`/`&'static T` constants work), but a **`static mut`** is a mutable memory cell with no compiler-enforced synchronization. Reading or writing it requires `unsafe`, and *you* are responsible for proving that concurrent access cannot race:

```
   static mut GLOBAL_COUNT: usize        const MAX: usize = 1000
   ┌──────────────────┐                  (no address — inlined at every use)
   │        42        │  ← one process-    MAX  MAX  MAX
   └──────────────────┘    wide cell      every use is a literal value
      every thread reads and writes
      THIS cell — must be serialized
```

### The sound `static mut` pattern

Accessing a mutable static directly from multiple threads without synchronization is a data race — undefined behavior. The classic *sound* pattern: keep the mutable static, but gate **every single access** behind a lock that lives in a plain (safe) `static`:

```rust
use std::sync::Mutex;

static GLOBAL_LOCK: Mutex<()> = Mutex::new(());
static mut GLOBAL_COUNT: usize = 0;

fn global_increment(by: usize) -> usize {
    let _guard = GLOBAL_LOCK.lock().unwrap();   // safe: plain static, no lock-free tricks
    // SAFETY: every access to GLOBAL_COUNT happens while GLOBAL_LOCK is
    // held, so reads and writes are serialized — no data race, and no
    // aliasing is possible because the lock grants exclusive access.
    unsafe {
        GLOBAL_COUNT += by;
        GLOBAL_COUNT
    }
}
```

Why is this sound? The two requirements for safe shared access are (1) no data race and (2) no aliasing violation. The `Mutex` provides both: only one thread is inside the critical section at a time, so the read-modify-write is exclusive, and the `unsafe` block is *tiny* — exactly one statement, with its proof written next to it. This is the pattern this module's exercise builds on: mutable static + `unsafe` + a `Sync` wrapper.

### `unsafe impl Send` / `unsafe impl Sync`

In Module 034 you learned `Send`/`Sync` are derived automatically. The fourth unsafe superpower lets you *declare* them by hand when your design's contract guarantees the safety:

```rust
pub struct GlobalCounter;

// SAFETY: this handle has no data of its own; every method serializes
// access through `GLOBAL_LOCK`, so sharing it across threads cannot race.
unsafe impl Send for GlobalCounter {}
unsafe impl Sync for GlobalCounter {}
```

When is this sound? Exactly when **the type's own operations** provide the synchronization that makes concurrent use safe — e.g. because every method takes the lock. When is it a bug? When you slap `unsafe impl Sync` on a type that hands out unsynchronized access to shared data — that converts a compile-time race into silent UB. The rule: *an `unsafe impl Sync` is a claim that your type's API makes sharing sound; if you can't write the argument in one sentence, don't write the impl.*

### Unions: overlapping memory

A `union` is a struct where **all fields share the same bytes**. Writing one field doesn't touch the others; reading a field that wasn't last-written reinterprets the bits. That reinterpretation is the entire point — and why reading is `unsafe` (the compiler can't know which field the current bits are valid for):

```
   union IntOrFloat { i: i32, f: f32 }      size: 4 bytes (max of fields)

   ┌──────────────────────┐
   │   i: i32    OR   f: f32  │  ← SAME 4 bytes, two interpretations
   └──────────────────────┘
     IntOrFloat { i: -1 }  then reading .f
     → an f32 whose BIT PATTERN is 0xFFFF_FFFF
     → NOT -1.0 — bits are not converted, only re-looked-at
```

```rust
#[repr(C)]
pub union IntOrFloat {
    pub i: i32,
    pub f: f32,
}

impl IntOrFloat {
    pub fn from_int(value: i32) -> Self {
        Self { i: value }   // writing a union field is SAFE
    }

    pub fn as_float(&self) -> f32 {
        unsafe { self.f }   // reading is unsafe: type validity is on you
    }
}

let u = IntOrFloat::from_int(-1);
assert_eq!(u.as_float().to_bits(), (-1i32) as u32); // bits survive intact
```

Two rules of thumb: (1) prefer `#[repr(C)]` on unions so the layout is predictable when crossing FFI; (2) reading a union field is only sound if the bits in there are a valid value *of the type you read*. This is the overlap trick C code uses for type-punning — Rust makes you say "I know what I'm doing" at each read instead.

### Raw-pointer casts: `.cast()` and `read_unaligned`

Module 035 taught pointer *arithmetic*. Casting is the other pointer skill: changing what the address is interpreted as. The idiomatic cast is `.cast::<U>()` (equivalent to `as *const U` but more explicit and type-safe about provenance). The canonical sound use — reading an integer from bytes without alignment guarantees:

```rust
pub fn read_u32_unaligned(bytes: &[u8]) -> u32 {
    assert!(bytes.len() >= 4, "need at least 4 bytes");
    // SAFETY: `bytes` is at least 4 bytes long, and `read_unaligned`
    // tolerates any alignment; the read stays inside the slice.
    unsafe { bytes.as_ptr().cast::<u32>().read_unaligned() }
}

assert_eq!(read_u32_unaligned(&[1, 2, 3, 4]), u32::from_ne_bytes([1, 2, 3, 4]));
```

Two details: `as_ptr().cast::<u32>()` reinterprets the address as `*const u32`, and `read_unaligned()` dereferences without requiring the 4-byte alignment a normal `*ptr` dereference demands. This is the portable way to deserialize from byte buffers — you'll see its big sibling, `serde`'s internal reader, doing exactly this.

### FFI preview: `extern "C"`

The last superpower family peek: calling foreign functions. An `extern "C"` block declares functions with the C calling convention — the compiler must trust that the symbol exists, has that signature, and follows the C ABI:

```rust
extern "C" {
    // Declared only — linking happens at build time; never called unless
    // we actually need the symbol. (Calling it is unsafe: no signature
    // checking across the boundary.)
    fn abs(input: i32) -> i32;
}

let result = unsafe { abs(-5) }; // every FFI call is an unsafe call
assert_eq!(result, 5);
```

Rules you'll live by: every call into an `extern` block is `unsafe` (the signature is unverified), FFI data must be `#[repr(C)]` (unions included — see above), and soundness means the C side upholds the same invariants you'd demand of Rust: valid pointers, no races, no dangling returns. Modules 052–053 build this out completely. For now: *declare, don't call, unless you need it.*

## Common Pitfalls

- **`static mut` without a lock.** Two threads reading/writing the same mutable static without serialization is a data race — UB, even if it "works" in practice. If you keep `static mut`, every access must be gated (as the exercise does with `GLOBAL_LOCK`).
- **`unsafe impl Sync` on a type that shares unsynchronized data.** The wrapper is not a free pass; it's a claim the methods make sharing safe. If the methods hand out `&mut` to shared state without a lock, you've just moved the race out of the compiler's sight.
- **Reading a union field that holds an invalid bit pattern for that type.** Reading `f` when the last write was `i` is only sound if those bits are a valid `f32`. This is why reading is `unsafe` — think about what the bits actually are.
- **Using `transmute` when a cast or union is fine.** `std::mem::transmute` is the bluntest tool in Rust; prefer `.cast()`, `read_unaligned`, or a union for reinterpretation. (And check for `clippy::useless_transmute` before reaching for it.)
- **Calling FFI functions without understanding the ABI.** Missing `#[repr(C)]` on the data you pass, wrong argument order in the extern declaration, or crossing a panic across the boundary — all UB. When you get to Module 052, this list grows; for now, remember the extern block is a *declaration*, and every call is an `unsafe` call.

## Key Terms

- **const:** a compile-time value with no address; inlined at every use.
- **static:** one process-wide memory cell; a `static mut` is a mutable one needing `unsafe` for every access.
- **mutable static pattern:** the sound way to use `static mut`: gate every access behind a lock held in a safe static.
- **unsafe impl Send/Sync:** declaring a type shareable by contract rather than by derivation; the contract must be true.
- **union:** a struct whose fields overlap in the same memory; reads are `unsafe`, writes are safe.
- **reinterpretation:** re-reading the same bits as a different type (unions, casts) — distinct from *conversion*.
- **read_unaligned:** dereference a pointer without requiring its target's alignment.
- **extern "C":** a block declaring foreign functions with the C ABI; calling them is `unsafe`.
- **FFI:** the foreign-function interface — Rust code calling (or being called by) code written in other languages.

## Exercise

Open `exercises/` and fill in the `// TODO(module-036)` comments in `src/lib.rs`:

1. `global_increment(by)`, `global_value()`, `global_reset()` — implement the sound mutable-static pattern: take `GLOBAL_LOCK`, then read/modify/`GLOBAL_COUNT` inside a minimal `unsafe` block with a `SAFETY` comment.
2. `GlobalCounter` — a `Clone + Copy` handle whose `*mut usize` field points at `GLOBAL_COUNT` (via `&raw mut`), with `unsafe impl Send`/`Sync` (sound because every method locks `GLOBAL_LOCK` before touching the pointer). Implement `increment(&self, by)` and `total(&self)`.
3. `IntOrFloat` union — implement `from_int`/`from_float` (safe writes) and `as_int`/`as_float` (unsafe reads).
4. `read_u32_unaligned(bytes)` — a safe function that asserts the length and uses `.cast::<u32>().read_unaligned()`.

All `unsafe` code in this crate must be sound. The tests check sequential and parallel accumulation on the global counter (exactly `threads * per_thread`), bit-exact union reinterpretation, and unaligned reads — and they use a `std::sync::Arc<GlobalCounter>` across threads, which only compiles because your `unsafe impl` contract is honest.

```bash
cargo test -p module-036-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rustonomicon, ["Data Races and Race Conditions" and the mutable-static rules](https://doc.rust-lang.org/nomicon/races.html)
- The Rust Reference, [`static`/`static mut` semantics](https://doc.rust-lang.org/reference/items/static-items.html)
- The Rust Reference, [unions](https://doc.rust-lang.org/reference/items/unions.html)
- [`std::ptr` — `read_unaligned` and friends](https://doc.rust-lang.org/std/ptr/fn.read_unaligned.html)
- The Rust Book, [Chapter 19.1: FFI preview (`extern`)](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#calling-unsafe-functions-or-methods)
