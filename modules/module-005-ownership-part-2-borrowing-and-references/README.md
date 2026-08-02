# Module 005: Ownership Part 2 — Borrowing & References

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 004 (ownership, moves, clone)

## Learning Objectives

- You will be able to read a function taking `&T` and say exactly what it can and cannot do with the data.
- You will be able to write functions that mutate data through `&mut T` without taking ownership.
- You will be able to state the borrow checker's two rules and predict which code they reject.
- You will be able to explain what a dangling reference is and why Rust makes it impossible.
- You will be able to write functions with multiple parameters that borrow in different ways.

## Why This Matters

Ownership says "one owner, moves to transfer"; borrowing says "anyone else may use it, on *my* terms, for a *limited time*." This is the mechanism behind Rust's two biggest promises: thread safety (Module 031+ builds directly on it) and fearless refactoring (the compiler tells you *exactly* where a change would break someone). Every `&str` you see in a real codebase is a borrowing decision, and interviews love to probe it — "why does this function take `&self` not `self`?" is a daily code-review question.

## Concept

### Borrowing: using without owning

Last module: passing a `String` to a function *moves* it. But most functions don't need to take the string — they just need to *look at it*. Moving would be wasteful and annoying (you'd have to move it back or clone). So Rust lets you **borrow**: pass a *reference*, `&String`, that points at the value without owning it.

```rust
fn look_at(s: &String) -> usize {
    s.len()
}

fn main() {
    let name = String::from("Ada");
    let len = look_at(&name); // borrow; `name` still owned by main
    println!("{name} has {len} chars"); // still usable!
}
```

The `&` creates the reference; the `&String` type is "an immutable reference to a String". The borrower gets read access for as long as the borrow lasts, and the owner keeps ownership the whole time. No move, no clone, no cost beyond the reference itself (a pointer).

Two kinds of references exist:

| Reference | Reads | Writes | Simultaneous borrowers |
|---|---|---|---|
| `&T` (shared/immutable) | yes | no | any number |
| `&mut T` (mutable) | yes | yes | exactly one |

That table *is* the borrow checker's two rules:

1. **You may have any number of `&T` borrows, or exactly one `&mut T`, at any moment — not both kinds mixed.**
2. **A reference must never outlive the value it points to** (no dangling references).

### Immutable borrows: many readers

Because `&T` can't change anything, having several at once is safe — nobody can surprise anybody:

```rust
fn total(a: &String, b: &String) -> usize {
    a.len() + b.len()
}

fn main() {
    let s = String::from("hello");
    println!("{}", total(&s, &s)); // two immutable borrows of the same value: fine
}
```

### Mutable borrows: exactly one writer

When you need to change data you don't own, you take a `&mut T`. The caller must have a mutable variable to lend:

```rust
fn add_one(n: &mut i32) {
    *n += 1;
}

fn main() {
    let mut counter = 41;
    add_one(&mut counter);
    println!("{counter}"); // 42
}
```

The `*` in `*n += 1` is *dereferencing*: following the reference to the value it points at and acting on that. For `+=` Rust lets you skip the `*` (`n += 1` compiles too), but for plain reads you always need it: `*n` is the value, `n` is the pointer.

The rule bites when two `&mut` references target the same data:

```rust,ignore
// This will not compile: two mutable borrows of `v` are alive at once.
let mut v = vec![1, 2, 3];
let a = &mut v[0];
let b = &mut v[1]; // error[E0499]: cannot borrow `v` as mutable more than once
println!("{a} {b}");
```

Why so strict? Think of what *could* happen in C: two pointers to the same memory, one writes while the other reads, and the reader sees half-updated garbage — a **data race**. The borrow checker prevents the *possibility* of data races at compile time, with zero runtime cost. The rule isn't "no two mutable pointers" — it's "no two mutable pointers that can *both be reached and used* while both are alive." This diagram shows why sequential uses are fine and overlapping ones aren't:

```text
OK — sequential borrows:                    NOT OK — overlapping mutable borrows:

let v = ...;                               let mut v = ...;
let a = &mut v[0];   ── writer #1 active   let a = &mut v[0];   ── writer #1 active
use a;               ── a done             let b = &mut v[1];   ── writer #2 active
let b = &mut v[1];   ── writer #2 active   use a;               ── #1 still alive!
use b;               ── b done                       ✗ compiler: two live
                                                  writers on the same data
   The borrows don't overlap, so the
   compiler knows they can't interfere.
```

The borrow checker is not guessing — it's doing a precise analysis of when each reference is used (the same machinery later modules will call *non-lexical lifetimes*). Two `&mut` borrows that never overlap are perfectly legal, as the exercise's `swap` demonstrates:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}

fn main() {
    let mut x = 1;
    let mut y = 2;
    swap(&mut x, &mut y); // two mutable borrows, different owners: fine
}
```

`a` and `b` point at *different* variables, so there's no conflict. Writing this by hand is the right way to feel the dereference syntax:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    let tmp = *a;
    *a = *b;
    *b = tmp;
}
```

...but modern `clippy` will flag the manual version (`clippy::manual_swap`) and suggest the standard library's version, which does the same thing safely:

```rust
fn swap(a: &mut i32, b: &mut i32) {
    std::mem::swap(a, b);
}
```

In this module's exercise you'll implement `swap` — start with the manual version to feel the dereferences, then switch to `std::mem::swap` so your code stays clippy-clean.

### Dangling references: the one thing Rust forbids

A **dangling reference** points at memory whose owner is already gone. In C this is a use-after-free; in Rust, this program is *rejected at compile time*:

```rust,ignore
// This will not compile: `s` is dropped when the function returns, but
// the returned reference would outlive it (error[E0106]: missing lifetime
// specifier, and the borrow checker will refuse this pattern).
fn bad() -> &String {
    let s = String::from("gone");
    &s
}
```

Rust's answer is that every reference carries a **lifetime** — a compile-time notion of "how long this borrow is valid." The compiler verifies that the reference never outlives its target, so dangling references simply can't be written. (You'll write lifetimes by hand in Module 018; for now, know that the compiler checks them for you automatically in everyday code.)

### What borrowing means for your function design

A practical habit emerges immediately. Looking at a signature tells you the intent:

- `fn f(s: &str)` — reads, does not own, caller keeps it. *Most common in real code.*
- `fn f(s: &mut String)` — changes it in place, caller keeps it.
- `fn f(s: String)` — takes ownership; caller loses it.

Prefer borrowing unless you genuinely need to own. That's why this course's code uses `&str` everywhere a function just wants to inspect text — and why `String` vs `&str` is the question every Rust dev asks themselves while designing an API.

### This module's exercise

Four functions in `exercises/src/lib.rs`, each demonstrating one borrowing pattern:

1. `first_char(s: &str) -> Option<char>` — immutable borrow; `s.chars().next()`.
2. `add_one(n: &mut i32)` — mutates through `&mut`, returns nothing.
3. `total_length(a: &str, b: &str) -> usize` — two simultaneous immutable borrows.
4. `swap(a: &mut i32, b: &mut i32)` — two mutable borrows of *different* variables.

The tests deliberately do things the borrow checker approves of — calling `add_one(&mut x)` twice in a row (sequential borrows) and using `a` *after* passing `&a` to `total_length` (the borrow ends when the call ends). If you get a borrow-checker error while implementing, stop and read it: the compiler tells you both where the conflicting borrow is and often how to fix it.

## Common Pitfalls

- **Forgetting the `*`.** `*n += 1` dereferences; `n += 1` won't compile on a `&mut i32` parameter (except for `+=` sugar, which hides it). For reads, `*n` is the value.
- **Taking `&mut` when you only read.** A `&mut` borrow blocks everyone else for no reason. Use `&T` unless you must mutate.
- **Passing `&mut x` where the callee only needs `&x`.** The callee's signature decides; match it. (`&mut` coerces to `&` only in some positions.)
- **Returning a reference to a local.** That's a dangling reference — the compiler rejects it. Return an owned value (or take the input as a parameter and borrow *that*).
- **Mutating while reading.** `let a = &v[0]; v.push(1);` — the push would invalidate the borrow; the compiler blocks it. Restructure: do the push first or after.

## Key Terms

- **reference:** a pointer that borrows; `&T` (read-only) or `&mut T` (writable).
- **borrow checker:** the compile-time analysis enforcing the two borrowing rules.
- **dereference (`*`):** following a reference to the value it points at.
- **dangling reference:** a reference to freed memory — impossible in safe Rust.
- **lifetime:** the compile-time span during which a borrow is valid.
- **data race:** concurrent access where at least one access writes — prevented at compile time.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the four `// TODO(module-005)` comments.
2. Implement `first_char(s)` — immutable borrow, `s.chars().next()`.
3. Implement `add_one(n)` — `*n += 1` through a mutable reference.
4. Implement `total_length(a, b)` — sum of the two borrowed lengths.
5. Implement `swap(a, b)` — manual swap via a temporary.
6. Run `cargo test -p module-005-exercises` until all 6 tests pass.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 4: References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html) — the two rules, mutable vs immutable, and dangling references.
- [The Rust Book, Chapter 4: The Slice Type](https://doc.rust-lang.org/book/ch04-03-slices.html) — the most borrowed type of all (next module).
- [Rust RFC 2094: Non-lexical lifetimes](https://rust-lang.github.io/rfcs/2094-nll.html) — why sequential borrows like `swap`'s are allowed.
- [Rust Reference: Behavior considered undefined](https://doc.rust-lang.org/reference/behavior-considered-undefined.html) — what the borrow checker is protecting you from.
