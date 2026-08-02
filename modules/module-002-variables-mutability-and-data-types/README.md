# Module 002: Variables, Mutability & Data Types

**Block:** Block A — Foundations I
**Estimated time:** 45–90 min
**Prerequisites:** Module 001 (toolchain, `cargo run` / `cargo test`)

> **About this module's exercise:** this is a **"make it compile"** exercise —
> the *only* one in the course. The scaffold in `exercises/` intentionally
> contains compile errors. Your task is to fix them, guided by the compiler
> itself, until `cargo test -p module-002-exercises` compiles and passes.
> Compile errors are the *point* here: by fixing them yourself you'll learn
> the rules from the inside. Every other module's scaffold compiles (its tests
> just fail) — this one is the deliberate exception.

## Learning Objectives

- You will be able to bind values with `let` and explain the difference between an immutable binding and a mutable one (`mut`).
- You will be able to declare compile-time constants with `const` and say why they must have constant initializers.
- You will be able to name the scalar types (`i32`, `u64`, `f64`, `bool`, `char`) and the compound types (`tuple`, `array`) and pick the right one.
- You will be able to explain type inference and where an explicit type annotation is required.
- You will be able to use shadowing and explain how it differs from mutation.

## Why This Matters

Every Rust job interview opens with a variation of "why is `mut` explicit?" — and it's not trivia: the compiler *uses* the answer to guarantee safety. When you read production Rust, `let` vs `let mut` tells you at a glance what can change, which is information other languages force you to hunt for. Explicit mutability is also the foundation of the ownership system (Module 004): the compiler can prove you never mutate data you don't own because you had to say `mut` out loud.

## Concept

### Binding with `let`

In Rust, variables are introduced with `let`. This is a *binding*: you are binding a name to a value. Rust is a statically typed language, but the type usually doesn't need to be written — the compiler infers it:

```rust
fn main() {
    let age = 30;          // inferred as i32
    let name = "Ada";      // inferred as &str
    let score: u64 = 99;   // annotation: force u64 explicitly
}
```

The default is **immutability**. A binding you didn't mark `mut` cannot be changed after it's created — not "shouldn't be", *cannot be*; the compiler refuses to compile code that tries:

```rust,ignore
// This will not compile: `x` is immutable.
let x = 5;
x = 6;
```

```rust
// Fix: declare intent to mutate with `mut`.
fn main() {
    let mut x = 5;
    x = 6;
}
```

Note the error message you'll get for the first version — `cannot assign twice to immutable variable` — tells you exactly what's wrong and even suggests adding `mut`. A lot of Rust learning is just reading these suggestions.

### Why is immutability the default?

Think of `let` as a promise to the compiler: "this value is never going to change while this binding is alive." The compiler can then prove things that make your code safer — for example, that no one is mutating data you're reading right now (that's the borrow checker, Module 005). It also makes *reading* code easier: every `mut` you see is a small warning sign saying "this one changes."

### Constants with `const`

`const` declares a compile-time constant. It's different from `let` in three ways: the type is always required, the value must be computable at compile time (no function calls, unless they're `const fn`), and the name lives in the value namespace — conventional style is `SCREAMING_SNAKE_CASE`:

```rust
const MAX_USERS: u32 = 100;

fn main() {
    println!("The limit is {MAX_USERS}");
}
```

The classic beginner error is putting a runtime value in a `const`:

```rust,ignore
// This will not compile: `users()` is not a compile-time constant function.
const MAX_USERS: u32 = users();
```

The fix is either to compute it at compile time or, if it genuinely depends on runtime data, to use `let` instead. In this module's exercise you'll replace the call with a literal — `const` in real codebases holds things like limits, buffer sizes, and magic numbers that are truly fixed.

### Scalar types

Rust's **scalar types** are single values:

| Type | Meaning | Example |
|---|---|---|
| `i32` | 32-bit signed integer (the default for integers) | `-42` |
| `u64` | 64-bit unsigned integer | `42` |
| `f64` | 64-bit float (the default for floats) | `3.14` |
| `bool` | true or false | `true` |
| `char` | a single Unicode character (4 bytes) | `'a'` |

Two things beginners trip on:

- **Integer literals are `i32` by default**, float literals are `f64`. Mixed arithmetic — `f64 * i32` — is a compile error, not an implicit conversion. This is intentional: Rust never silently changes the type of a number, which is the source of entire classes of bugs in C and JavaScript.
- **Unsigned vs signed matters.** `u` types can't be negative. `-1` as a `u32` is an error (or a debug-mode panic on overflow).

The exercise hits exactly this: `celsius * 9 / 5 + 32` where `celsius` is `f64` and `9`, `5`, `32` are integer literals — the compiler rejects the mixed expression until you write `9.0 / 5.0`.

### Compound types

**Tuples** group a fixed number of values of *possibly different* types; **arrays** hold a fixed number of values of *the same* type:

```rust
fn main() {
    let point: (f64, f64) = (0.0, 0.0);
    let line = ("start", (0, 0), (10, 10)); // nested tuple
    let bytes = [1, 2, 3, 4, 5];            // [i32; 5]
    let first = bytes[0];
    let (x, y) = point;                     // destructuring a tuple
}
```

Both have fixed length known at compile time — unlike `Vec` (Module 011), which grows. Tuple elements are accessed by index (`point.0`), arrays by `[index]`.

### Type inference and annotations

Rust infers types from usage, but three situations commonly require an annotation:

```rust
fn main() {
    let parsed: u32 = "42".parse().unwrap(); // parse() can produce many types
    let guess: i32 = 10;                     // when the default isn't what you want
    let value = 10i64;                       // or use a literal suffix instead
}
```

The type *inference* means you write fewer annotations than in C# or Java, but the type *system* is just as strict — the annotation is what makes the intent explicit.

### Shadowing

**Shadowing** lets you reuse a name for a new binding. It is *not* mutation — the old value still exists until it goes out of scope; you're just covering the name with a new binding:

```rust
fn main() {
    let message = "hello";        // &str
    let message = message.len();  // usize — same name, new binding, new type
    println!("{message}");
}
```

Note the trick: shadowing can even change the *type* of a name, which mutation can never do. Compiler warnings exist for accidentally shadowing without using the old value (`unused variable`), and later modules teach you to use shadowing deliberately — e.g. `let x = x + 1;` after a calculation.

### The exercise: let the compiler teach you

Open `exercises/src/lib.rs`. Four deliberate errors wait for you:

1. `double` — assigns to an immutable binding (`cannot assign twice to immutable variable`). Fix: `let mut`.
2. `MAX_USERS` — calls a function in a `const` initializer. Fix: use the literal `100`.
3. `fahrenheit` — mixes `f64` with integer literals. Fix: use `9.0 / 5.0` and `32.0`.
4. `describe_length` — type annotation says `String`, value is `usize`. Fix: annotate (or omit) correctly.

Run `cargo check -p module-002-exercises` and fix the errors one at a time — the compiler message for each error is the lesson. When `cargo check` is clean, run `cargo test` and watch all five tests pass. (Note: because the scaffold doesn't compile, `cargo clippy` and `cargo test` on the *unfixed* scaffold will report compile errors — that's expected here, and unique to this module.)

## Common Pitfalls

- **`const` vs `let`.** `const` requires a compile-time constant and an explicit type; `let` binds at runtime. Don't use `const` for values that change during execution.
- **Forgetting the default is immutable.** If the compiler says "cannot assign twice," you forgot `mut` — that's the fix, not fighting the compiler.
- **Mixing integer and float literals.** `1.5 * 2` is a compile error. Write `1.5 * 2.0` (or `2f64`).
- **Confusing shadowing with mutation.** `let x = 5; let x = 6;` creates a *new* binding; the old `5` is still there, just unreachable. `let mut x = 5; x = 6;` changes the same storage.

## Key Terms

- **binding:** the association of a name with a value via `let`.
- **immutable / mutable:** immutable bindings can't change; `mut` opts into change.
- **const:** a compile-time constant; type required, initializer must be a constant expression.
- **scalar type:** a single value: integers, floats, `bool`, `char`.
- **compound type:** a group of values: tuples (mixed types) and arrays (same type).
- **type inference:** the compiler deducing a type from usage; annotations override or disambiguate.
- **shadowing:** reusing a name for a new binding; the old binding is still alive, just hidden.

## Exercise

In `exercises/` — this is the course's only "make it compile" module:

1. Open `src/lib.rs` and read the four `// TODO(module-002)` comments.
2. Run `cargo check -p module-002-exercises` and read the first error.
3. Fix that one error per the TODO, then re-check. Repeat until `cargo check` is silent.
4. Run `cargo test -p module-002-exercises` — all five tests should pass.
5. Compare with `solutions/` to see the reference fixes.

## Further Reading

- [The Rust Book, Chapter 3: Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html) — the canonical treatment of `let`, `mut`, `const`, and shadowing.
- [The Rust Book, Chapter 3: Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html) — scalars and compounds in detail.
- [std: primitive types](https://doc.rust-lang.org/std/index.html#primitives) — reference pages for `i32`, `u32`, `f64`, `bool`, `char`, tuples, arrays.
