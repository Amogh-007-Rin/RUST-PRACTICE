# Module 003: Functions & Control Flow

**Block:** Block A — Foundations I
**Estimated time:** 45–90 min
**Prerequisites:** Module 002 (variables, mutability, data types)

## Learning Objectives

- You will be able to define functions with `fn`, type their parameters and return values, and explain the difference between an expression and a statement.
- You will be able to use `if`/`else` as an expression that produces a value.
- You will be able to choose the right loop (`loop`, `while`, `for`) for a job and write each of them correctly.
- You will be able to break out of loops with `break` (including with a value) and skip iterations with `continue`.
- You will be able to trace control flow through a small program step by step.

## Why This Matters

Functions and control flow are the skeleton of every program you'll write — an `axum` handler is a function, a game loop is a `loop`, and a batch job is a `for` over rows. But Rust has one twist that trips even experienced developers: `if` and `loop` are *expressions* that produce values, which shapes how idiomatic Rust code is written (the `match` in Module 008 builds directly on this). Getting expressions-vs-statements straight now prevents a whole class of subtle bugs later.

## Concept

### Functions

A function takes typed parameters and returns a typed value:

```rust
fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn main() {
    println!("{}", area(10, 5));
}
```

Two syntax details matter. First, the return type comes after `->`. Second, look at the body: `width * height` has **no semicolon**. That trailing expression is the function's return value. If you add a semicolon, `area` returns `()` (the unit type, "nothing") and the program won't compile — the type error says `mismatched types: expected u32, found ()`.

An explicit `return` works too and is needed for early exits, but idiomatic Rust prefers the trailing-expression form:

```rust
fn describe(n: i32) -> &'static str {
    if n > 0 {
        "positive"
    } else {
        "not positive"
    }
}
```

### Expressions vs statements

This is the single most important idea in this module:

- A **statement** is an instruction that *does* something and produces no value (`let x = 5;`, a function call ending in `;`, an item definition).
- An **expression** is anything that *evaluates to a value* — literals, arithmetic, function calls, blocks, `if`, loops.

Every Rust program is a pile of statements made of expressions. A block `{ ... }` is an expression whose value is its final, semicolon-less expression — which is exactly why function bodies "return" their last expression:

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // the block's value: 4
    };
    println!("{y}");
}
```

If the last line of that block had a `;`, the block's value would be `()` and `let y = ();` would be a type error downstream. Rule of thumb: **semicolons discard values; the absence of a semicolon produces one.**

### `if` / `else` is an expression

Because `if` is an expression, both branches must produce values of the *same type*:

```rust
fn sign(n: i32) -> &'static str {
    if n < 0 {
        "negative"
    } else if n > 0 {
        "positive"
    } else {
        "zero"
    }
}
```

There's no ternary operator (`cond ? a : b`) in Rust — this form replaces it. The `else if` chain is just nesting. And unlike C, the condition must be a `bool` — no truthy integers:

```rust,ignore
// This will not compile: Rust requires a `bool` condition.
if 1 {
    println!("nope");
}
```

### Early returns: `return` as a guard

The trailing expression is the default shape, but `return` exists for early exits — typically a guard at the top of a function or a mid-loop bailout:

```rust
fn first_even(values: &[i32]) -> i32 {
    for &v in values {
        if v % 2 == 0 {
            return v; // leave the whole function immediately
        }
    }
    -1 // fallback if nothing matched
}
```

Use `return` when it reads better than nesting several `else` blocks; keep the trailing-expression style for the *normal* exit path. Both compile — reviewers in real codebases expect the expression form as the default.

### Loops

Rust has three loop constructs:

**1. `loop` — forever, until you `break`.** Use it when the exit condition is somewhere in the middle of the body:

```rust
fn main() {
    let mut line = String::new();
    loop {
        let trimmed = line.trim();
        if trimmed == "quit" {
            break;
        }
        line = String::from("quit"); // pretend: read more input
    }
}
```

`break` can also *carry a value* out of the loop:

```rust
fn main() {
    let mut n = 0;
    let found = loop {
        n += 1;
        if n * n > 50 {
            break n; // the loop's value: the first n whose square exceeds 50
        }
    };
    println!("{found}");
}
```

**2. `while` — test a condition before each iteration.** The classic counting-down loop:

```rust
fn main() {
    let mut n = 3;
    while n > 0 {
        println!("{n}");
        n -= 1;
    }
}
```

**3. `for` — iterate over a range or collection.** This is the workhorse; `while`-with-index loops are rare in idiomatic Rust:

```rust
fn main() {
    for i in 1..=3 {
        println!("{i}");
    }
}
```

`1..3` excludes 3; `1..=3` includes it. `for` works on anything that implements the `Iterator` trait — which you'll master in Modules 022–023; for now, `for x in 0..10` is a plain counted loop.

Choosing a loop is a habit, not a rule:

| Situation | Loop |
|---|---|
| You know the iteration count (or you're walking a collection) | `for` |
| The exit depends on a condition re-checked before each run | `while` |
| The exit is mid-body, or the body must run at least once | `loop` + `break` |

If you catch yourself writing `while i < n { ...; i += 1; }`, a `for` was almost certainly what you wanted — the counter lives in the loop syntax and can't be forgotten or duplicated. And when the exit condition is genuinely mid-body — read input, check it, maybe bail — `loop` with a `break` (optionally carrying a value) is the clearest spelling, because `while` would need an awkward duplicated check.

`continue` skips the rest of the current iteration:

```rust
fn main() {
    let mut sum = 0;
    for i in 1..=10 {
        if i % 2 == 0 {
            continue; // skip even numbers
        }
        sum += i;
    }
    println!("sum of odd numbers 1..=10: {sum}");
}
```

### Control flow through a real example

Here's the Collatz function from the exercise, traced step by step. The rule: if `n` is even, halve it; if odd, compute `3n + 1`; count until you reach 1. Starting from `n = 6`:

```text
n = 6   even → 3      (1 step)
n = 3   odd  → 10     (2)
n = 10  even → 5      (3)
n = 5   odd  → 16     (4)
n = 16  even → 8      (5)
n = 8   even → 4      (6)
n = 4   even → 2      (7)
n = 2   even → 1      (8 steps total)
```

The Rust shape: a `while n > 1` loop, an `if` expression that computes the *next* `n`, and a mutable step counter:

```rust
fn collatz_steps(mut n: u64) -> u32 {
    let mut steps = 0;
    while n > 1 {
        n = if n.is_multiple_of(2) { n / 2 } else { 3 * n + 1 };
        steps += 1;
    }
    steps
}
```

Two idioms to note: `mut n` in the parameter list makes the parameter itself mutable (a convenient shadowing-style trick), and `n.is_multiple_of(2)` is the modern way to test evenness on unsigned integers (this toolchain's `clippy` flags `n % 2 == 0` there in favor of it). Signed integers still use `n % 2 == 0` — the `%` operator appears all over Rust code, so you'll see both forms in this course. (Every starting value tested so far eventually reaches 1; proving it is the famous open problem — you're in good company.)

### Statement-position habits that bite

Because expressions are everywhere, Rust code rewards a specific habit: when an `if` or block is used *as a statement*, make sure the branches' last lines have semicolons or are `()`-typed, or you'll compute a value you then throw away. And when a function should return a value, make sure the *final* expression has no semicolon. Roughly half of all beginner "expected `()`, found integer" errors trace back to exactly one of these two slips.

## Common Pitfalls

- **Semicolon kills the return.** `fn f() -> u32 { 42; }` returns `()` — a type error. Drop the semicolon to return `42`.
- **`else` not attached to `if`.** The `else` must follow the closing brace of the `if` body on the same line's chain: `} else { ... }`.
- **Integer condition.** `if 1 { ... }` doesn't compile; write `if 1 != 0 { ... }` or compare properly.
- **Off-by-one with ranges.** `for i in 0..3` visits 0, 1, 2 — *not* 3. Use `0..=3` to include the end.
- **Mutating the loop variable of a `for`.** `for i in 0..10 { i += 1 }` won't compile; use a separate `mut` counter or, better, rethink the loop.

## Key Terms

- **statement:** code that does something and yields no value (usually ends in `;`).
- **expression:** code that evaluates to a value (literals, calls, blocks, `if`, loops).
- **unit type `()`:** the "no value" type — what a function returns when it has no `-> Type`.
- **shadowing:** reusing a name for a new binding (used here to mutate `n` inside the Collatz loop).
- **`break` / `continue`:** leave a loop (optionally with a value) / skip to the next iteration.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the four `// TODO(module-003)` comments.
2. Implement `is_even(n)` — one expression, no `if` needed.
3. Implement `classify(n)` with an `if`/`else if`/`else` *expression* — each branch is a string literal without a semicolon.
4. Implement `sum_to(n)` with a `for` loop over `1..=n` and a mutable accumulator.
5. Implement `collatz_steps(n)` with a `while` loop, an `if` expression for the next value, and shadowed `mut n`.
6. Run `cargo test -p module-003-exercises` until all 12 tests pass.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 3: Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html) — expressions vs statements.
- [The Rust Book, Chapter 3: Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html) — `if` and all three loops.
- [The Collatz conjecture (Wikipedia)](https://en.wikipedia.org/wiki/Collatz_conjecture) — the famous open problem behind `collatz_steps`.
