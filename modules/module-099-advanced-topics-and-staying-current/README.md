# Module 099: Advanced Topics & Staying Current

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 90–120 min
**Prerequisites:** Module 098 (Mock Interview & Code Review Practice)

## Learning Objectives
- You will be able to write generic code over array sizes using const generics (`<const N: usize>`).
- You will be able to define and implement a trait with a Generic Associated Type (GAT) — an associated type parameterized by a lifetime.
- You will be able to write `const fn` functions and evaluate them at compile time, eliminating runtime cost for pure computations.
- You will be able to implement a compile-time static assertion using const generics that prevents invalid code from compiling.
- You will be able to identify which Rust RFCs, release notes, and community channels to follow to stay current with the language as it evolves.

## Why This Matters

Rust moves fast. Features like const generics (stabilized in Rust 1.51), Generic Associated Types (GATs, stabilized in Rust 1.65), and `const fn` expansions (ongoing across releases) are reshaping how Rust code is written at the library, framework, and systems level. If you interviewed for a Rust role in 2022 and didn't know GATs existed, you'd be behind. If you interview in 2026 and can't write a const generic, you're behind. This module covers the features that separate someone who "learned Rust once" from someone who stays current and can discuss design tradeoffs with the language's latest capabilities.

## Concept

### Const Generics

Before const generics, if you wanted to write a function that operated on arrays of different sizes, you had two bad options: write a separate function for each size (impossible), or use slices and lose the compile-time size guarantee. Const generics let you parameterize code over *values* known at compile time, not just types.

```rust
// Without const generics: you'd need a slice, losing the compile-time size
fn sum_slice(arr: &[i32]) -> i32 { arr.iter().sum() }

// With const generics: the size is part of the type, checked at compile time
fn sum_array<const N: usize>(arr: [i32; N]) -> i32 {
    arr.into_iter().sum()
}

let a: [i32; 3] = [1, 2, 3];
let b: [i32; 5] = [1, 2, 3, 4, 5];
println!("{}", sum_array(a)); // N = 3
println!("{}", sum_array(b)); // N = 5
```

The compiler monomorphizes this (just like regular generics), generating `sum_array::<3>` and `sum_array::<5>` as separate functions. The key insight: `N` is a value, not a type, but it participates in type checking. Arrays `[i32; 3]` and `[i32; 5]` are distinct types, and the compiler knows the difference.

Const generics can also be used in struct definitions:

```rust
struct Vector<T, const DIM: usize> {
    data: [T; DIM],
}

impl<T: Default + Copy, const DIM: usize> Vector<T, DIM> {
    fn zero() -> Self {
        Self { data: [T::default(); DIM] }
    }
}
```

This is how `nalgebra`, the dominant linear algebra crate in Rust, works — every matrix and vector size is a const generic, so multiplying a 3×4 with a 4×2 matrix is checked at compile time. You cannot accidentally multiply incompatible dimensions.

The expression passed to a const generic must be evaluable at compile time. You can use literal integers, simple arithmetic on other const generics, and const functions:

```rust
fn double_array<const N: usize>(arr: [i32; N]) -> [i32; N] {
    arr.map(|x| x * 2)
}

// N * 2 is a const expression — fine
fn make_pair<const N: usize>(x: [i32; N], y: [i32; N]) -> [i32; { N * 2 }] {
    let mut out = [0; { N * 2 }];
    // ...combine x and y...
    out
}
```

Note the `{ N * 2 }` syntax. When a const generic expression is used in a type position (like an array length), it must be wrapped in braces. This is a syntactic requirement, not a semantic one — it disambiguates const expressions from type parameters.

**Limitations as of Rust 1.82:** Only integral types (`usize`, `u8`, `i32`, etc.), `bool`, and `char` are supported as const generic parameter types. You cannot use `&str`, structs, or enums (though `&'static str` is being explored). Float const generics and user-defined types are on the roadmap but not yet stable. This is the most common "why doesn't this work?" moment when learning const generics.

### Generic Associated Types (GATs)

A regular associated type is a type alias inside a trait:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

Every implementation of `Iterator` picks one `Item` type. Simple. But what if the associated type needs to vary by lifetime?

Consider a trait that returns references to internal data. Without GATs, you'd have to tie the reference lifetime to the trait's generic parameter, which leaks into every use:

```rust
// Without GATs: the lifetime is a parameter on the trait itself
trait Container<'a> {
    type Item;
    fn get(&'a self, index: usize) -> Option<Self::Item>;
}
// Every function that uses Container needs to be generic over the lifetime
fn use_container<'a, C: Container<'a>>(c: &'a C) { ... }
```

With GATs, the lifetime is a parameter on the associated type, not the trait:

```rust
// With GATs: the lifetime is on the associated type
trait Container {
    type Item<'a> where Self: 'a;
    fn get<'a>(&'a self, index: usize) -> Option<Self::Item<'a>>;
}

// Implement on Vec<T>
impl<T> Container for Vec<T> {
    type Item<'a> = &'a T where T: 'a;
    fn get<'a>(&'a self, index: usize) -> Option<Self::Item<'a>> {
        self.as_slice().get(index)
    }
}

// Usage is clean — no lifetime parameter on the trait bound
fn print_first<C: Container>(c: &C) {
    if let Some(item) = c.get(0) {
        println!("{:?}", item);
    }
}
```

The `where Self: 'a` bound on the GAT is required. It says: "this associated type can only exist while the implementing type itself is alive." Without it, you could hypothetically return a reference that outlives the container — a soundness hole the compiler prevents.

GATs are the mechanism behind several language features:
- **`LendingIterator`** (a.k.a. streaming iterator): an iterator whose `Item` borrows from the iterator itself, so you can yield `&str` slices from a `Lines` iterator without allocating.
- **Async traits:** Before `async fn` in traits (stabilized in Rust 1.75), the `async_trait` macro desugared to a GAT: `type Future<'a> where Self: 'a;`. This is still how it works under the hood — GATs are the primitive that makes async trait methods possible.
- **Scope-guarded resource handles:** a database connection that lends out a transaction handle tied to the connection's lifetime.

The key mental model shift: GATs let the *caller's* lifetime determine the associated type, not the implementor's. This is type-level currying — the associated type is a type constructor that takes a lifetime argument.

### Const Evaluation (`const fn`)

A function marked `const fn` can be evaluated at compile time. This doesn't mean it *must* be evaluated at compile time — it means it *can* be. You can call a `const fn` in a runtime context (it compiles to a regular function), or in a `const` or `static` initializer (it's evaluated by the compiler).

```rust
const fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        n => n * factorial(n - 1),
    }
}

// Compile-time evaluation: the compiler computes this, result is baked into the binary
const FIVE_FACTORIAL: usize = factorial(5);

// Runtime evaluation: same function, called at runtime
fn main() {
    let n: usize = std::env::args().len(); // runtime value
    println!("{}", factorial(n)); // still works, called at runtime
}
```

Since Rust 1.46, `const fn` supports `if`, `match`, `while`, and `loop`. Since 1.62, it supports `for` over slices. The feature set grows steadily — each Rust release expands what's allowed in `const fn`. As of 1.80+, you can allocate, use `&mut` references in const contexts, and more.

Why use const evaluation?

1. **Precomputed lookup tables.** A `const fn` can generate a table of precomputed sine values or CRC lookup tables at compile time, embedded in the binary. No runtime initialization cost.
2. **Type-level programming.** Combine const generics with const evaluation to create types that encode invariants — a `CheckedIndex<const MAX: usize>` that only compiles when `index < MAX`.
3. **Zero-runtime-overhead configuration.** Parse a config string at compile time. If it's malformed, the build fails.

### Static Assertions with Const Generics

A common pattern in Rust is the type-level assertion: "this code should only compile if condition X holds." You can enforce this with const generics and a helper type:

```rust
pub struct Assert<const COND: bool>;

impl Assert<true> {
    pub const OK: () = ();
}
// No impl for Assert<false> — accessing OK is a compile error
```

Usage:

```rust
// This compiles:
let _ = Assert::<{ std::mem::size_of::<usize>() >= 4 }>::OK;

// This would NOT compile if the condition were false:
// let _ = Assert::<{ std::mem::size_of::<usize>() < 4 }>::OK;
```

This pattern appears frequently in `no_std` and embedded Rust, where you want to assert things like "this buffer is large enough" or "this alignment is a power of two" at compile time rather than runtime. It's also common in FFI code to verify struct layouts match the C ABI:

```rust
let _ = Assert::<{ std::mem::size_of::<MyStruct>() == 16 }>::OK;
```

If someone changes the struct and the size drifts, the build breaks — a compile-time safety net.

### Staying Current with Rust

Rust releases a new stable version every six weeks. Features land first on nightly, then beta, then stable. Staying current means knowing where to look:

**Primary sources:**
- **The Rust Blog** (`blog.rust-lang.org`): official release announcements with links to stabilization PRs and detailed notes.
- **This Week in Rust** (`this-week-in-rust.org`): a curated weekly newsletter covering RFCs, blog posts, crate releases, and job postings. The single highest-signal channel for Rust ecosystem awareness.
- **Rust RFCs** (`github.com/rust-lang/rfcs`): every major language feature starts here. Reading the RFC for a recent stabilization (e.g. `async fn` in traits, `impl Trait` in return position) gives you the design rationale, alternatives considered, and edge cases — the kind of depth that comes up in senior-level interviews.
- **Rust Release Notes** (`github.com/rust-lang/rust/blob/master/RELEASES.md`): the raw changelog. Skim the "stabilized APIs" and "language" sections of each release.
- **crates.io trending / Rust Digests**: New crates gaining adoption often signal shifts in ecosystem conventions (e.g. `axum` overtaking `actix-web` in new projects, `ratatui` replacing `tui-rs`).

**Checking what your toolchain supports:**
```bash
rustup show                # active toolchain
rustup check               # updates available
rustup doc --std           # offline std docs for your version
```

The `rustup doc --std` command is underused — it opens the std documentation for your *installed* version, not the latest. This is essential when you're on a specific toolchain version and need accurate docs.

**Knowing when to adopt a feature:**
- **Stable:** use it. It won't break.
- **Beta:** safe to learn, but don't ship production code depending on it yet.
- **Nightly-only under a feature gate:** interesting, but understand it might change. Good candidates: compiler tricks you enable locally (e.g. `min_specialization` for writing more performant trait impls), `new_uninit` for low-level optimization, etc.
- **Never heard of it:** normal. There are hundreds of unstable features. You don't need to know them all — you need to know how to find them when a problem demands one.

### Async Traits: The Current State

Prior to Rust 1.75, `async fn` in trait definitions wasn't supported. The `async-trait` crate provided a workaround via proc macros that desugared each async method into a method returning a boxed future:

```rust
#[async_trait]
trait MyTrait {
    async fn fetch(&self) -> String;
}
// Desugared to:
trait MyTrait {
    fn fetch<'a>(&'a self) -> Pin<Box<dyn Future<Output = String> + Send + 'a>>;
}
```

With Rust 1.75 (and improved in 1.79+), you can write:

```rust
trait MyTrait {
    async fn fetch(&self) -> String;
}
```

This uses Return-Position Impl Trait in Traits (RPITIT). The compiler handles the associated future type implicitly. However, there are still rough edges:
- Dynamic dispatch (`dyn MyTrait`) doesn't work with async methods yet — the future type must be known at compile time.
- `Send` bounds on the hidden future type require explicit annotations in some cases.
- The `async-trait` crate is still more flexible for complex patterns (conditional `Send`, helper lifetimes).

For now, the practical advice: use `async fn` in traits for straightforward cases (non-dyn, no unusual lifetime constraints), and fall back to `async-trait` when you hit a limitation. Check the RFCs ([RFC 3185](https://rust-lang.github.io/rfcs/3185-static-async-fn-in-trait.html)) and the stabilization tracking issue for the latest status.

## Common Pitfalls
- **Forgetting the braces around const generic expressions in type position.** `[i32; N + 1]` is a syntax error; write `[i32; { N + 1 }]`.
- **Trying to use unsupported types as const generic parameters.** Only `usize`, `u8`–`u64`, `i8`–`i64`, `bool`, and `char` are stable. Floats, strings, and custom types are not.
- **Forgetting `where Self: 'a` on a GAT.** Without it, the compiler rejects the trait definition — the GAT must promise it won't outlive the container.
- **Assuming const fn can do everything a regular fn can.** `const fn` cannot (as of 1.82) use `for` with iterators other than slices, cannot allocate `Vec`/`String`/`Box`, cannot trap on errors via `?`. Check the release notes for the version you're targeting.
- **Trying to use `dyn Trait` with async methods in traits.** Dynamic dispatch of async trait methods is not yet stable. Use `async-trait` or restructure to avoid `dyn`.

## Key Terms
- **Const generic:** a generic parameter that is a compile-time-known value (e.g. `usize`) rather than a type.
- **Monomorphization:** the compiler's process of generating a separate copy of a generic function for each concrete set of type/const parameters used.
- **Generic Associated Type (GAT):** an associated type in a trait that is itself parameterized by a lifetime or type, allowing the output type to vary based on the caller's context.
- **`const fn`:** a function that can be evaluated at compile time when called with const arguments.
- **Static assertion:** a compile-time check that prevents code from compiling if a condition doesn't hold, typically implemented via const generics and a helper type like `Assert`.
- **RPITIT:** Return-Position Impl Trait in Traits — the language feature that allows `async fn` (and other `impl Trait` return types) in trait definitions, stabilized in Rust 1.75.
- **RFC:** Request for Comments — the design document format for proposing changes to the Rust language.

## Exercise

Open `exercises/` and fill in the `// TODO(module-099)` markers. You'll implement:

1. **`fixed_size_array_sum`** — a const-generic function that sums the elements of any `[i32; N]` array.
2. **`Container` implementation for `Vec<T>`** — define the GAT `Item<'a>` as `&'a T` and implement `get` using `as_slice()`.
3. **`demonstrate_const_evaluation`** — write a `const fn factorial(n: usize) -> usize` and return `factorial(5)` computed in a `const` context.
4. **`Assert<true>::OK`** — implement the `OK` constant on the `Assert<true>` specialization, then use it in `demonstrate_static_assertion` to verify a true condition and return `"static assertion passed"`.

The integration tests in `tests/module_099.rs` define "done." Run them with:

```bash
cargo test -p module-099-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading
- [The Rust Reference: Const Generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) — the formal spec for const generic syntax and restrictions.
- [RFC 1598: Generic Associated Types](https://rust-lang.github.io/rfcs/1598-generic-associated-types.html) — the original GAT design, with motivation and alternatives considered.
- [Rust Blog: GATs Stabilized](https://blog.rust-lang.org/2022/10/28/gats-stabilization.html) — the stabilization announcement with examples.
- [The Rust Reference: Const Functions](https://doc.rust-lang.org/reference/const_eval.html) — what's allowed in `const fn` as of the latest stable.
- [This Week in Rust](https://this-week-in-rust.org/) — the best way to track what's happening in the ecosystem week by week.
- [Rust RFCs Repository](https://github.com/rust-lang/rfcs) — read a recent RFC (e.g. `async fn` in traits) to understand how language design decisions are made.
