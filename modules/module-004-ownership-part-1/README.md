# Module 004: Ownership Part 1

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 003 (functions, expressions); comfortable with what a variable is

## Learning Objectives

- You will be able to explain the difference between the stack and the heap and say which of your values live where.
- You will be able to state the three ownership rules from memory and apply them.
- You will be able to predict when a value is *moved* and what that means for the variable you moved it from.
- You will be able to use `.clone()` to get a second independent copy and explain the cost.
- You will be able to explain why integers copy but `String` moves, and what the `Copy` trait has to do with it.

## Why This Matters

Ownership is the single idea that makes Rust Rust — the reason a Rust backend can serve millions of requests without a garbage collector, and the reason Rust was the most-admired language for eight years running. Every Rust interview has at least one ownership question; every Rust codebase is organized around it (functions that take `&str` vs `String`, `clone()` decisions in hot paths). Modules 004–006 are the most important three modules in this entire course — if you internalize them, everything else (borrowing, lifetimes, `Arc`, unsafe) follows naturally.

## Concept

### The stack and the heap

Every running program has two regions of memory it hands out to functions and values.

- The **stack** is a LIFO structure — like a stack of trays in a cafeteria. Push a value, pop it later; pushing and popping happen in exactly the reverse order. The stack is fast (a single pointer bump to push, a decrement to pop), and values on it have a strictly nested lifetime. Local variables and function call frames live here.
- The **heap** is a pool of memory where you can allocate a block of any size, at any time, and free it whenever. Allocation is slower (the allocator must find a free block, track it, and eventually reclaim it). The heap exists so values can outlive the function that created them and so sizes can be dynamic.

The classic picture:

```text
                        ┌──────────────────────────┐
   stack (fast, LIFO)   │                          │      heap (pool, dynamic)
                        │                          │
┌─────────────────────┐ │                          │   ┌─────────────────────┐
│ frame for main      │ │                          │   │                     │
│   s ────────────────┼─┼──────────────────────────┼──▶│ "hello" (5 bytes)   │
│   ptr │ len │ cap   │ │                          │   │                     │
└─────────────────────┘ │                          │   └─────────────────────┘
                        │                          │
   A String is a stack │                          │
   struct holding a    │                          │
   pointer, a length,  │                          │
   and a capacity; the │                          │
   actual text lives   │                          │
   on the heap.        │                          │
```

A `String` is exactly this: a small struct on the stack — a pointer to heap memory, a length (bytes in use), and a capacity (bytes allocated) — with the text itself on the heap. An `i32`, `f64`, or `bool` is a plain value that fits entirely on the stack. Which category a value falls into decides how it behaves when you pass it around.

### The three ownership rules

Rust's ownership system is three rules. Everything else in this module is a consequence:

1. **Each value in Rust has a single owner.**
2. **When the owner goes out of scope, the value is dropped** (its heap memory is freed).
3. **Ownership can be transferred (a move) or borrowed (Modules 005), but never shared ambiguously.**

There is no garbage collector walking the heap periodically. When the owner dies, the memory is freed *immediately, deterministically* — that's why Rust programs have predictable memory usage. The compiler enforces the rules at compile time, so the runtime cost is zero.

### Move semantics

Look at this function:

```rust
fn print_length(s: String) {
    println!("{}", s.len());
}

fn main() {
    let name = String::from("Ada");
    print_length(name); // `name` is MOVED into the function
    // println!("{name}"); // <-- this would not compile
}
```

When `print_length(name)` runs, `name`'s ownership is *transferred* to the parameter `s`. The binding `name` is no longer valid — the compiler tracks this and refuses to let you use `name` afterwards. This is the famous compile error: `value borrowed here after move` / `use of moved value`.

Why does Rust do this? Think about what happens with the heap. When `print_length` returns, `s` goes out of scope and the heap text is freed. If `name` were still usable, you'd have a dangling pointer — a use-after-free. In C this is a silent, exploitable bug; in Rust it's a compile error with a clear message. **Moves are how Rust prevents double-frees and use-after-free while still freeing memory eagerly.**

Diagram of the move — the stack frame transfers, the heap data never copies:

```text
BEFORE the call:                        AFTER the call:
┌───────────────┐                        ┌───────────────┐
│ main          │                        │ main          │
│   name ───────┼──┐                     │   (name no    │
└───────────────┘  │                     │    longer     │
                   ▼                     │    valid)     │
              ┌─────────┐                └───────────────┘
              │ "Ada"   │                   ┌───────────────┐
              └─────────┘                   │ print_length  │
                                            │   s ────────┼──┐
                                            └───────────────┘  │
                                                               ▼
                                                          ┌─────────┐
                                                          │ "Ada"   │
                                                          └─────────┘
    One String, one owner. Ownership slides from `name`   Only `s` owns it now;
    to `s`; the heap block is never duplicated.           dropped when the fn ends.
```

### Moves happen implicitly

In Rust, *assignment* moves, and *passing an argument* moves. Three examples:

```rust
fn main() {
    let a = String::from("one");
    let b = a;      // MOVE: `a` is now invalid; `b` owns the string
    // println!("{a}"); // compile error: use of moved value

    let x = 42;
    let y = x;      // NOT a move: 42 is a plain integer; both x and y work
    println!("{x} {y}"); // fine!
}
```

Why the difference? Integers are small enough to live entirely on the stack, so there's nothing to double-free — copying the bits is free and safe. `String` contains a pointer to the heap; copying the struct would give *two* owners of the same heap block, and both would try to free it when they die (double free!). So Rust says: for heap-owning types, *assignment transfers ownership*; for tiny stack-only types, *assignment copies*.

That distinction is the `Copy` trait. A type that implements `Copy` (all integers, floats, `bool`, `char`, tuples of `Copy` types) is copied on assignment; a type that doesn't (`String`, `Vec` — anything owning a heap allocation) is moved. You'll implement traits yourself in Module 016; for now, remember the rule: **`Copy` types duplicate freely, everything else moves.**

### Getting a real copy: `.clone()`

When you *do* want a second independent copy of a moved type, call `.clone()` — it copies the stack struct *and* the heap data:

```rust
fn main() {
    let a = String::from("hello");
    let b = a.clone(); // deep copy: a new heap block
    println!("{a} {b}"); // both alive and independent
    b_does_something(b);
}

fn b_does_something(s: String) {
    println!("{s}");
}
```

The two strings are fully independent: pushing to `b` does not affect `a`. The cost is a heap allocation plus a copy of the bytes — cheap for a name, expensive in a hot loop. Rust developers treat `.clone()` as a *visible, auditable* cost: when you see it in a codebase, you know someone deliberately chose to duplicate data, and reviewers will ask why. (`.clone()` is so common that clippy has a lint for *redundant* clones.)

### When the owner dies, memory is freed

Every binding is dropped when it goes out of scope — the end of the block, or the end of the function. For a `String`, that means freeing the heap block; for an `i32`, nothing happens. This is deterministic and automatic, and it's exactly why Rust needs no GC: the compiler knows each value's owner and inserts the frees itself.

```rust
fn main() {
    let s = String::from("temporary");
    // ...s used here...
} // ← s is dropped here: heap memory freed, no leak
```

### What this module's exercise asks of you

Three functions in `exercises/src/lib.rs` make the theory concrete:

1. `byte_len(s: String) -> usize` — you move `s` *in*, use it, and it drops inside. Return `s.len()`.
2. `copy_of(s: String) -> (String, String)` — you must return *two* strings; only one can be the moved original, so the other is `s.clone()`.
3. `concat(first: String, second: String) -> String` — two strings move in, one combined string moves out; the inputs drop on return.

The tests call these functions with `String::from(...)` literals, so the moves happen right before your eyes. Notice the tests can't observe moves directly — they observe the *values* — so the TODOs are your real guide. When all tests pass, try writing a version that uses `a` after moving it, and read the error the compiler gives you: that error message is the lesson.

## Common Pitfalls

- **Using a moved value.** After `let b = a;` (with `a: String`), `a` is dead. Fix: clone, borrow (Module 005), or restructure.
- **Cloning everything.** `.clone()` everywhere is safe but slow. Move by default; clone only when you need the original too.
- **Forgetting `String` and `Vec` aren't `Copy`.** `let y = x;` for integers copies, for `String` moves — the same syntax, two behaviors. Know which types are `Copy`.
- **Thinking moves "copy" the data.** A move transfers the stack struct and leaves the heap block untouched — O(1), not O(n). Only `.clone()` copies the heap.

## Key Terms

- **stack:** fast, LIFO memory for local values; freed automatically when frames pop.
- **heap:** a pool for dynamic, long-lived allocations; freed when the owner drops.
- **owner:** the binding that holds a value and frees it on scope exit.
- **move:** transferring ownership; the source binding becomes invalid.
- **clone:** a deep copy of the value (stack struct *and* heap data).
- **`Copy` trait:** marker saying a type may be duplicated on assignment (stack-only types).

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the three `// TODO(module-004)` comments.
2. Implement `byte_len(s)` — move `s` in, return its length.
3. Implement `copy_of(s)` — return `(s.clone(), s)`.
4. Implement `concat(first, second)` — return a combined `String` with `format!`.
5. Run `cargo test -p module-004-exercises` until all 7 tests pass.
6. Bonus: try using a moved variable after the move in a scratch file and read the compiler's error message.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 4: Understanding Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) — the canonical chapter, including stack vs heap and moves.
- [The Rust Book, Chapter 4: What Is Ownership?](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html) — the three rules in depth.
- [std: `String::clone`](https://doc.rust-lang.org/std/string/struct.String.html#method.clone) — the docs for the operation you just used.
- [Rust Reference: Ownership](https://doc.rust-lang.org/reference/ownership.html) — the formal description of ownership and moves.
