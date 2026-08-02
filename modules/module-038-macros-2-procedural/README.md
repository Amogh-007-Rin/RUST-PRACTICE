# Module 038: Macros II — Procedural Macros and How `#[derive(...)]` Works

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 60–90 min
**Prerequisites:** Module 037 (`macro_rules!`), Module 016 (traits), Module 009 (crates)

## Learning Objectives

- Explain the three kinds of procedural macro — derive, attribute, and function-like — and what each receives and emits.
- Describe the proc-macro pipeline: a `TokenStream` in, a `TokenStream` out, executed by the compiler before type checking.
- Read a `#[derive(...)]` expansion conceptually: the original item is preserved, and the macro appends trait impls.
- "Expand a derive by hand": given a trait, write the exact impl a derive macro would generate for a struct and an enum.
- Explain why procedural macros live in their own `proc-macro` crate, and name real-world examples (`serde`, `thiserror`, `clap`).

## Why This Matters

If `macro_rules!` (Module 037) is the screwdriver, procedural macros are the power tools of Rust's ecosystem. `#[derive(Serialize, Deserialize)]`, `#[derive(Debug, Error)]`, `#[tokio::main]` — half of what makes Rust ergonomic in production is procedural macro expansion, and knowing what the compiler *actually does* with these attributes turns "magic" into a pipeline you can trace. Even if you never write a proc-macro crate yourself (most developers don't), you will read their generated code, debug their expansion errors, and choose between crates based on their macro ergonomics. This module makes `#[derive(...)]` fully transparent.

## Concept

### Declarative vs. procedural

Module 037's `macro_rules!` is *declarative*: you write pattern-matching rules, and the macro system does the matching. A **procedural macro** is a genuine Rust function — written in Rust, with real control flow, loops, and error handling — that runs inside the compiler and transforms a token stream into another token stream. The trade-off is structural: `macro_rules!` is simple but limited (it can only do what pattern matching can express), while procedural macros can parse, analyze, and rewrite arbitrary Rust syntax.

Every procedural macro is one of three kinds:

| Kind | Looks like | Receives | Emits |
|------|-----------|----------|-------|
| **derive** | `#[derive(Describe)]` on an item | the item's tokens | new items (usually trait impls) |
| **attribute** | `#[tokio::main]` on a function | the attribute's arguments + the item | a rewritten item |
| **function-like** | `format_args!("{x}")` style | the argument tokens | replacement tokens |

All three share the same skeleton — a function marked with a specific attribute:

```rust
use proc_macro::TokenStream;

#[proc_macro_derive(Describe)]
pub fn describe_derive(input: TokenStream) -> TokenStream {
    // `input` is the tokens of the struct/enum the user annotated.
    // Return the tokens of everything that should replace it.
    TokenStream::new()
}
```

### The pipeline: tokens in, tokens out

Here is what the compiler does when it sees `#[derive(Describe)]` on a struct:

```
    user writes:                    compiler runs:                  result:
    ┌──────────────────────┐        ┌───────────────────────┐       ┌──────────────────────────┐
    │ #[derive(Describe)]  │        │ collect the tokens of  │       │ the original struct...   │
    │ struct Point {       │ ─────► │ the annotated item:    │ ────► │   AND the impl the macro │
    │     x: i32, y: i32   │        │ "struct Point { x: ... │       │   emitted:               │
    │ }                    │        │   i32, y: i32 }"       │       │ impl Describe for Point  │
    └──────────────────────┘        └───────────────────────┘       │ { fn describe(...) ... }  │
```

1. The compiler tokenizes the item — it becomes a `TokenStream`, an ordered list of tokens (`struct`, `Point`, `{`, `x`, `:`, `i32`, ...).
2. Your proc-macro function receives that stream and returns a new one. A derive macro is *expected* to echo the original item back plus the generated impls (that's why the original struct survives `#[derive(...)]`).
3. The returned tokens are spliced back into the source and compiled normally — borrow checking, type checking, and monomorphization run on the *expanded* code.

That last point is the mental model: **macros run before type checking**, so the code they emit is type-checked like any other code. A proc macro can't produce a type error silently — it just produces tokens that fail to compile, with errors pointing at the expanded code (hence `cargo expand`, the tool that shows the expansion).

### Why `#[derive]` is so powerful

A derive macro's job is narrow and immensely valuable: take a data definition and generate boilerplate that *matches it exactly* — one impl per field, per variant, per generic parameter. If you had to write that by hand, structs and enums would drift out of sync the moment you added a field; the derive keeps them synchronized by construction.

This module's exercise has you simulate exactly this: below is what a `#[derive(Describe)]` macro would generate for `Point` — you'll be writing this code yourself in the exercise:

```rust
struct Point {
    x: i32,
    y: i32,
}

trait Describe {
    fn describe(&self) -> String;
}

// This impl is the "expansion" of #[derive(Describe)] — written by hand.
impl Describe for Point {
    fn describe(&self) -> String {
        format!("Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

let p = Point { x: 1, y: 2 };
assert_eq!(p.describe(), "Point { x: 1, y: 2 }");
```

Notice the doubled braces in the format string (`{{`/`}}`) — that's the literal-brace escape in `format!`, not macro recursion. And note the shape: one `impl Describe for T` per type, with `match` arms for enum variants:

```rust
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: u32, height: u32 },
}

impl Describe for Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => {
                format!("Shape::Circle {{ radius: {} }}", radius)
            }
            Shape::Rectangle { width, height } => {
                format!("Shape::Rectangle {{ width: {}, height: {} }}", width, height)
            }
        }
    }
}
```

A real derive would *generate* this impl — inspect the input tokens, discover the type name and fields, and emit the format string. You, doing it by hand, are performing the same job the macro would: the exercise is "macro as a human."

### The `proc-macro` crate constraint

Writing a *real* procedural macro requires a crate with `[lib] proc-macro = true` in its `Cargo.toml`. That crate can only export macros — no ordinary functions, no types. Two consequences shape the ecosystem:

1. **Proc-macro crates are separate crates.** A crate cannot use its own proc macro — the macro must live in a sibling crate and be imported as a dependency. That's why your `Cargo.toml` grows lines like `serde = { version = "1", features = ["derive"] }` — `serde_derive` is the actual macro crate.
2. **Generated code must be self-contained.** The macro can't know the caller's imports, so good proc macros emit fully-qualified paths (`::std::string::String`, `$crate`-equivalent helpers) — the same discipline you learned for `#[macro_export]` in Module 037, taken further.

There's also **hygiene**, shared with declarative macros: identifiers introduced by the expansion (`fn describe`) resolve in the macro's context, while identifiers that came from the input (field names, type names) resolve in the caller's. A well-written derive can't accidentally capture the caller's variables.

### Real-world examples

- **`serde`'s `#[derive(Serialize, Deserialize)]`** — the flagship: generates serialization impls from data definitions. Reading its expansions (`cargo expand`) is a rite of passage.
- **`thiserror`'s `#[derive(Error)]`** — generates `Display` and `std::error::Error` impls from annotated fields like `#[error("status {0}")]`; Capstone 02's crate uses this pattern.
- **`clap`'s `#[derive(Parser)]`** — turns a struct into a full CLI argument parser.
- **Attribute macros** like `#[tokio::main]` rewrite `async fn main` into a runtime setup + block_on wrapper; `#[test]`-style test harnesses are the same idea inside `std`.

The rule of thumb: reach for a procedural macro when the boilerplate is *structural* — it must mirror a type's definition exactly — and when a `macro_rules!` or a generic function can't express it. For everything smaller, Module 037's tools are the better trade.

## Common Pitfalls

- **Expecting a derive macro to appear in the caller's source.** It doesn't — the expansion is spliced in before type checking. When an error message mentions a derive or a macro-generated trait, run `cargo expand` (installed via `cargo install cargo-expand`) to see the truth.
- **Writing by hand what a derive could keep in sync.** If you add a field to a struct and must remember to update two or three hand-written impls, that's exactly the drift a derive exists to prevent. Prefer derives for structural boilerplate.
- **Thinking `#[derive(Debug)]`-style magic does runtime work.** All of this is compile time; the generated code runs at normal speed, monomorphized like anything else. There's no "reflection tax" unless the macro emits one.
- **Forgetting the macro's output is ordinary code.** A proc macro that emits invalid syntax fails compilation with confusing errors — always validate what you emit. (This is why real macro crates use `syn`/`quote` to build the output, not string munging.)
- **Reaching for a proc-macro crate for a one-off.** The crate boundary, the `proc-macro` constraint, and the compile-time cost (procedural macros slow compilation) mean you should exhaust `macro_rules!` and generics first.

## Key Terms

- **procedural macro:** a Rust function running inside the compiler that transforms a `TokenStream` into another `TokenStream`.
- **TokenStream:** the ordered list of tokens that items are made of while being compiled.
- **derive macro:** `#[derive(Trait)]` — receives an item's tokens, returns the item plus generated impls.
- **attribute macro:** `#[name(...)]` — receives the attribute's arguments and the item, returns a rewritten item.
- **function-like macro:** a macro invoked like a function (`name!(...)`) that transforms its argument tokens.
- **expansion:** the output of a macro, spliced into the source before type checking.
- **`proc-macro` crate:** a crate type that can only export procedural macros.
- **hygiene:** macro-generated identifiers resolving in the macro's context while input identifiers keep the caller's meaning.

## Exercise

Open `exercises/` and fill in the `// TODO(module-038)` comments in `src/lib.rs`. This module simulates a derive macro: you'll write the impls a `#[derive(Describe)]` would generate.

1. `Describe` trait (already defined) — `fn describe(&self) -> String`.
2. `Point { x: i32, y: i32 }` — implement `Describe` so `describe()` returns exactly `"Point { x: 1, y: 2 }"` style output (the scaffold has a deliberately wrong placeholder impl).
3. `Book { title: String, pages: u32 }` — same, producing e.g. `"Book { title: The Rust Book, pages: 400 }"`.
4. `Shape` enum (variants `Circle { radius: f64 }` and `Rectangle { width, height }`) — implement `Describe` with a `match`, producing `"Shape::Circle { radius: 1.5 }"` and `"Shape::Rectangle { width: 3, height: 4 }"`.
5. `describe_all` (already implemented) maps `Describe` over slices — the tests use it to check your impls generically.

The tests assert exact strings, so field order and spacing in your `format!` calls matter. When you're done, compare with `solutions/` — this is the code a real derive macro would have emitted for you.

```bash
cargo test -p module-038-exercises
```

## Further Reading

- The Rust Book, [Chapter 19.6: Procedural Macros](https://doc.rust-lang.org/book/ch19-06-macros.html#procedural-macros-for-generating-code-from-attributes)
- The Rust Reference, [Procedural Macros](https://doc.rust-lang.org/reference/procedural-macros.html)
- [`proc_macro` crate documentation](https://doc.rust-lang.org/proc_macro/)
- [The Little Book of Rust Macros — procedural macro chapters](https://veykril.github.io/tlborm/)
