# Module 035: Unsafe Rust I — Why Unsafe Exists, Raw Pointers

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 90–120 min
**Prerequisites:** Module 028 (smart pointers/`Drop`), Module 004 (ownership), Module 018 (lifetimes)

## Learning Objectives

- Explain what `unsafe` does and — just as importantly — what it does *not* do (it does not disable the borrow checker).
- List the five "unsafe superpowers" and identify which superpower a given `unsafe` block uses.
- Create raw pointers (`*const T`, `*mut T`) from references and understand how they differ from references: no ownership, no lifetime, no aliasing rules.
- Dereference raw pointers soundly, and state the soundness invariants (non-null, aligned, in-bounds, initialized, no aliasing, no data race).
- Read the safety section of an `unsafe fn` and uphold its preconditions.

## Why This Matters

Virtually every serious Rust codebase contains some `unsafe` — FFI bindings (Modules 052–053), high-performance data structures, `std` itself. Your job as a Rust developer is not to avoid `unsafe` but to confine it: wrap it in safe APIs, document the invariants, and keep it small. Interviewers and reviewers probe this skill hard, because it's exactly the line between "Rust developer" and "Rust developer who understands what the compiler actually protects." This module teaches you to read, write, and reason about raw pointers; Module 036 extends that to statics, unions, and FFI.

## Concept

### What `unsafe` is — and isn't

The phrase "unsafe Rust" is a contract, not a mood. Safe Rust is the language with the rules you've learned in Modules 001–034: references can't dangle, data can't race, `Option` can't be null. Those rules are enforced *for you*. `unsafe` is the escape hatch: **you take over responsibility for the rules the compiler would otherwise check.** The compiler still enforces everything else — memory is still freed, ownership still moves, the borrow checker still runs. `unsafe` does not turn anything off except the specific checks the marked operation needs to skip.

That's why the standard formulation holds: an `unsafe` block is a promise to the compiler — "I know the invariants this operation requires, and I uphold them." If you're wrong, you get undefined behavior (UB): the program may crash, corrupt memory, or silently misbehave, and the compiler is no longer on the hook for explaining it.

There are exactly five things that require `unsafe`. Everything else in the language is safe:

1. **Dereference a raw pointer** (`*ptr` on a `*const T`/`*mut T`).
2. **Call an `unsafe fn`** (a function whose *caller* must uphold preconditions).
3. **Access or modify a mutable static** (`static mut`).
4. **Implement an `unsafe trait`** (e.g. `Send`/`Sync` by hand — see Module 036).
5. **Access a field of a `union`** (Module 036).

Note what is *not* on the list: defining a `struct`, calling a safe function, running normal code. You cannot write an `unsafe` block around ordinary code and gain anything.

### Raw pointers

A **raw pointer** is the C-style pointer Rust provides when references aren't enough: `*const T` (read-only) or `*mut T` (writable). It is *literally an address* — nothing more. References come with guarantees (non-null, aligned, in-bounds, no aliasing); raw pointers come with none, and the compiler will not check any of them for you.

```
   stack                              heap (the allocation)
   ─────                              ─────────────────────
   ptr: *const u32 ────────────────►  ┌─────┬─────┬─────┬─────┐
   [ 0x7ffd...a4 ]                    │ 42  │ 17  │  9  │ 100 │
                                      └─────┴─────┴─────┴─────┘
   ptr.add(1) ──────────────────────►   ^     ^     ^     ^
                                       ptr  ptr+1  ptr+2  ptr+3
   (address arithmetic: move forward
    by N * size_of::<T>() bytes)
```

You create raw pointers from references with a cast:

```rust
let value = 42u32;
let ptr_const: *const u32 = &value as *const u32;   // read-only view

let mut slot = 7u32;
let ptr_mut: *mut u32 = &mut slot as *mut u32;      // writable view
```

References to pointers coerce (`&mut u32` can pass where `*const u32` is expected), and slices give you their pointers via `as_ptr()` and `as_mut_ptr()`. But once you have a raw pointer, all safety is on you.

### Dereferencing: the soundness invariants

Dereferencing a raw pointer is the unsafe superpower you'll use most. Note that a function that dereferences a *caller-provided* pointer cannot guarantee safety on its own, so it must be an `unsafe fn` whose contract is documented under `# Safety`:

```rust
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn read_via_raw(ptr: *const u32) -> u32 {
    unsafe { *ptr }
}

/// # Safety
///
/// `ptr` must be non-null, aligned, and point to writable, initialized
/// `u32` storage with no other aliasing reference.
pub unsafe fn write_via_raw(ptr: *mut u32, value: u32) {
    unsafe { *ptr = value; }
}
```

Soundness — the property that no UB occurs no matter how the program runs — requires you to prove all of these:

1. **Non-null.** Dereferencing a null pointer is instant UB (well, instant crash — but still UB).
2. **Aligned.** The address must be a multiple of `align_of::<T>()`. An unaligned read can fault or tear on real hardware. (If you can't control alignment, use `read_unaligned`.)
3. **In-bounds.** The pointer must point into a live allocation, and the object you touch must fit entirely inside it.
4. **Initialized.** The memory must hold a valid `T` — no reading uninitialized bytes.
5. **No aliasing.** While a `&mut T` (or exclusive raw pointer) exists, no other reference may access the same memory.
6. **No data races.** If the pointer is shared across threads, all access must be synchronized (Modules 032/034).

The key insight: **these are the invariants the borrow checker was enforcing for you.** When you dereference a raw pointer, you re-assume them yourself.

### Walking a slice with pointers

Here's the pattern you'll see constantly in optimized code — iterate a slice via pointer arithmetic instead of indexing:

```rust
fn sum_slice_via_raw(slice: &[u32]) -> u32 {
    let ptr = slice.as_ptr();       // points at element 0
    let mut sum = 0u32;
    for i in 0..slice.len() {
        // SAFETY: `i` is in 0..len, so `ptr.add(i)` stays inside the
        // allocation, is aligned (same alignment as the slice), and
        // points to initialized data of type u32.
        unsafe { sum = sum.wrapping_add(*ptr.add(i)); }
    }
    sum
}

assert_eq!(sum_slice_via_raw(&[10, 20, 30]), 60);
```

`ptr.add(i)` moves the address forward by `i * size_of::<u32>()` bytes (4 bytes each). The loop bounds are what make this sound: `add` itself is safe to *call*, it's the later dereference that needs the pointer to be in-bounds. Compare with `ptr.offset(i)`, which additionally requires you to prove the arithmetic doesn't overflow the address space — `add` is what you want for simple in-bounds stepping.

### `unsafe fn`: moving the burden to the caller

An `unsafe fn` is a function that says: "my *caller* must prove preconditions I can't check." The `unsafe` marker lives at the call site, not inside:

```rust
/// Doubles the value behind `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn unsafe_double(ptr: *const u32) -> u32 {
    unsafe { *ptr * 2 }   // the block is needed even inside an unsafe fn
}

let value = 21u32;
let ptr: *const u32 = &value;
assert_eq!(unsafe { unsafe_double(ptr) }, 42);
```

Two details are easy to miss:

- Every raw-pointer dereference needs its own `unsafe` block, even inside an `unsafe fn` — the `unsafe fn` marker only moves the obligation to the call site; it doesn't license the body.
- The `# Safety` section is the *contract*. Read it before calling; write it before publishing. Reviewers treat a missing safety section as a bug in itself.

The standard production shape: a safe wrapper around a small `unsafe` core. The wrapper checks what it can (non-null, length, alignment) and the `unsafe` core does the raw work. That way most of your codebase never sees `unsafe` at all, and the bits that do are tiny and auditable.

## Common Pitfalls

- **Treating `unsafe` as "turn off the borrow checker."** It doesn't. You can still trigger borrow errors inside `unsafe` blocks. The permission it grants is narrow: only the five superpowers above.
- **An empty `unsafe` block.** `unsafe {}` that contains only safe operations is either dead code or a security theater. Clippy flags it; reviewers flag it.
- **Dereferencing without proving in-bounds.** A hand-written loop that stops one element short of `len` is a classic off-by-one heap corruption. Derive your bounds from the allocation's size, always.
- **Creating raw pointers from references and letting the reference die.** The pointer doesn't keep anything alive. If the reference goes out of scope (or the `Vec` is dropped/reallocated), your raw pointer dangles — and dereferencing it later is UB, exactly the bug raw pointers were invented to express.
- **Using `offset` where `add` is right.** `offset` demands overflow-correctness arguments (it can move *outside* the allocation in an intermediate step); `add` only permits in-bounds stepping. Use `add` for slice walking.
- **An `unsafe fn` with no `# Safety` docs.** An undocumented unsafe contract is a ticking bomb for the next caller. If you can't write the contract, the function shouldn't be `unsafe`.

## Key Terms

- **safe Rust:** the language with compiler-enforced memory safety (Modules 001–034).
- **unsafe Rust:** the same language, plus five superpowers whose soundness you must prove.
- **undefined behavior (UB):** behavior the language forbids and does not define; the program can do anything.
- **raw pointer (`*const T`, `*mut T`):** an address with no enforced guarantees.
- **soundness invariants:** the properties (non-null, aligned, in-bounds, initialized, no aliasing, no data race) your raw-pointer use must uphold.
- **`unsafe fn`:** a function whose callers must uphold preconditions documented under `# Safety`.
- **`add`/`offset`:** raw-pointer address arithmetic; `add` for in-bounds stepping, `offset` when you can prove arithmetic-level correctness.

## Exercise

Open `exercises/` and fill in the `// TODO(module-035)` comments in `src/lib.rs`:

1. `read_via_raw` — an `unsafe fn` (caller-provided pointer) with a proper `# Safety` section; dereference and return the value.
2. `write_via_raw` — an `unsafe fn` that writes through the caller's `*mut u32`.
3. `sum_slice_via_raw(slice: &[u32]) -> u32` — a *safe* function (the pointer comes from the slice, so it can guarantee soundness itself): sum the elements by walking `as_ptr()` with `add(i)` inside a loop; it must match the iterator sum for any slice, including empty ones.
4. `swap_via_raw(a: &mut i32, b: &mut i32)` — swap two values through `as_mut_ptr()` without using the safe `std::mem::swap` (the deliberate `clippy::manual_swap` allow is already in place).
5. `unsafe_double` — an `unsafe fn` with a `# Safety` section that doubles the value behind a `*const u32`; remember the dereference still needs its own `unsafe` block.

Every `unsafe` block you write must be **sound**: prove each dereference is non-null, aligned, in-bounds, and initialized. The tests in `tests/module_035.rs` exercise both directions of every function.

```bash
cargo test -p module-035-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 19.1: Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- The Rustonomicon, [the "Dereferencing" section on raw-pointer invariants](https://doc.rust-lang.org/nomicon/raw-pointers.html)
- [`std::ptr` API reference](https://doc.rust-lang.org/std/ptr/index.html)
- [Rust Reference — "Behavior considered undefined"](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
