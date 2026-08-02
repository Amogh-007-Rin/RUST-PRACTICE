# Module 037: Macros I — Declarative Macros (`macro_rules!`)

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 90–120 min
**Prerequisites:** Module 009 (modules/items), Module 022 (iterators), Module 015 (generics)

## Learning Objectives

- Explain what a declarative macro is: a token-level function that runs at compile time and writes code for you.
- Match macro input with matcher fragments (`expr`, `ident`, `ty`, `tt`, literal) and repetition operators (`*`, `+`, `?`).
- Write variadic macros that expand recursively, and reason about the expansion as a tree.
- Use `#[macro_export]` and `$crate` to ship a macro usable from other crates.
- Replace real-world duplication with a `macro_rules!` helper and know when *not* to reach for one.

## Why This Matters

Every Rust codebase you will ever touch uses macros — `vec!`, `println!`, `format!`, `matches!`, and a long tail of project-specific ones. Beyond reading them, you'll *write* them: the "one match, N repetitive arms" pattern is precisely what `macro_rules!` eliminates, and it's the first tool you'll reach for before graduating to procedural macros (Module 038). Concurrency-heavy code in particular accumulates repetitive patterns (log-level parsing, message parsing, boilerplate around structs) — the Capstone 04 project has you writing exactly such a macro. This module makes you fluent in the syntax so the macros stop looking like magic.

## Concept

### What a macro is

A **declarative macro** is a compile-time function over *tokens*: it receives a stream of tokens, matches it against patterns, and emits a replacement stream of tokens. Where a normal function takes values, a macro takes syntax and produces syntax. Three consequences follow immediately:

1. Macros run **before** type checking — they can generate items (structs, impls, functions) that ordinary functions cannot.
2. Macros can match **any number** of arguments — `vec![1]`, `vec![1, 2, 3]`, `vec![]` are all the same macro.
3. Macros are **hygienic**: identifiers you introduce inside a macro body won't collide with the caller's variables, and metavariables you capture (`$x`) keep the caller's meaning.

### Anatomy of a `macro_rules!` definition

```rust
macro_rules! my_vec {
    () => {                     // arm 1: zero elements
        ::std::vec::Vec::new()
    };
    ($($elem:expr),* $(,)?) => {   // arm 2: one or more elements + optional comma
        let mut vec = ::std::vec::Vec::new();
        $(vec.push($elem);)*
        vec
    };
}

assert_eq!(my_vec![1, 2, 3], vec![1, 2, 3]);
```

Each arm is `pattern => replacement`. The pattern is matched against the tokens *after* the macro name. Inside a pattern, `$name:fragment` captures tokens and binds them to a **metavariable**:

- `$x:expr` — an expression (`1 + 2`, `foo()`, `true`)
- `$id:ident` — an identifier (`Point`, `x`, `my_fn`)
- `$t:ty` — a type (`i32`, `Vec<String>`)
- `$l:literal` — a literal (`42`, `"hi"`)
- `$tt:tt` — a single token *tree* (any single token, or a parenthesized/bracketed group)
- `$p:path`, `$s:stmt`, `$b:block`, and a few more

The `$(...)` wrappers are **repetitions** with operators: `$(...)*` zero or more, `$(...)+` one or more, `$(...)?` zero or one. Inside the replacement, `$($elem:expr),*` repeats the body once per captured element.

### Expansion is a tree

Recursion is how macros handle arbitrary-length input. Watch `sum!` chew `sum!(1, 2, 3)` — each expansion produces a smaller invocation until it bottoms out:

```
sum!(1, 2, 3)
  │  pattern: $head = 1, $($tail),* = 2, 3
  ▼
1u64 + sum!(2, 3)
        │  $head = 2, $tail = 3
        ▼
        2u64 + sum!(3)
                │  $head = 3, $tail = (none)
                ▼
                3u64 + sum!()
                        │  () arm
                        ▼
                        0u64

        = 1 + 2 + 3 + 0 = 6
```

Here's the macro:

```rust
macro_rules! sum {
    () => { 0u64 };
    ($head:expr $(, $tail:expr)* $(,)?) => {
        $head as u64 + sum!($($tail),*)
    };
}

assert_eq!(sum!(1, 2, 3, 4), 10);
assert_eq!(sum!(), 0);
```

Note the two idioms: the *base case* arm (`()`) ends the recursion, and the recursion re-wraps the captured tail (`sum!($($tail),*)`). Every recursive macro in this style has exactly those two parts.

### Token trees and `tt`

The most flexible matcher is `tt` — it matches **any** single token tree: a single token (`a`, `,`, `123`) or a complete group (`(...)`, `[...]`, `{...}`). A classic use is counting:

```rust
macro_rules! count_tt {
    () => { 0usize };
    ($_head:tt $($tail:tt)*) => { 1usize + count_tt!($($tail)*) };
}

assert_eq!(count_tt!(a b c), 3);
assert_eq!(count_tt!((a, b)), 1); // one parenthesized group = one tt
```

Because `tt` accepts *anything*, `tt`-based macros can build surprising syntax — this is how things like `matches!` and many DSLs are assembled.

### Generating items: macros that define things

Macros can define whole items. Here's a small "struct factory" — the shape of what you'll meet in Capstone 04's macro helper:

```rust
macro_rules! def_struct {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $ty),*
        }
        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}

def_struct!(Point { x: i32, y: i32 });

let p = Point::new(1, 2);
assert_eq!((p.x, p.y), (1, 2));
```

Everything in `$(...),*` is expanded once per captured repetition — including inside the struct definition, the constructor signature, and the constructor body. One definition, three synchronized repetitions: that's the power, and also the place where the repetitions must stay **in the same order** — the pattern `$field:ident: $ty:ty` binds `$field` and `$ty` in lockstep, so every repetition uses both consistently.

### Exporting: `#[macro_export]` and `$crate`

Macros defined without `#[macro_export]` are invisible outside their module (they use the module system like any item). `#[macro_export]` pushes a macro to the crate root and lets other crates use it via path: `module_037_exercises::my_vec![...]`.

Inside an exported macro, never refer to your crate's items by name — the caller's crate doesn't know them. Use `$crate`, which always resolves to *your* crate:

```rust
// lib.rs
pub fn square(x: u32) -> u32 { x * x }

#[macro_export]
macro_rules! sq {
    ($x:expr) => { $crate::square($x) };
}
```

### When to use `macro_rules!` (and when not)

Use it when: the same code shape repeats with slightly different identifiers/types/arms (log-level parsing, enum matching, struct definitions), and the duplication would drift out of sync. Don't use it for: computation you can do in a normal function or `const fn` (macros run before type checking — if it *doesn't need* to touch syntax, it doesn't need to be a macro), or code that must live in a real item namespace with real documentation. `macro_rules!` is best as a small, local, syntax-shaped helper.

## Common Pitfalls

- **Forgetting the base case.** Every recursive macro needs an arm that stops the recursion (`()` → literal). Without it, the macro recurses forever and rustc gives you "recursion limit reached."
- **`expr` fragments and the `match`-style comma trap.** `$x:expr` followed by `,` works fine — but a bare `$x:expr` followed by `$y:expr` (no comma) can silently match greedily or fail; always separate `expr` fragments with commas.
- **A macro that swallows the trailing comma.** `$(...),*` doesn't match `[1, 2, 3,]`. Add `$(,)?` — the standard "trailing comma allowed" idiom.
- **Referring to own-crate items by name inside an exported macro.** In the caller's crate those paths don't exist. Use `$crate::` for every item of yours the expansion touches.
- **Reaching for a macro when a function works.** If the repetition isn't over *syntax* (identifiers, types, arms), a generic function or `const fn` is simpler, typed, and debuggable. Macros have a cost — they're harder to read, harder to lint, harder to debug.

## Key Terms

- **declarative macro:** a compile-time token function defined with `macro_rules!`; matches input patterns and emits replacement tokens.
- **metavariable:** `$name` — a capture bound by a matcher fragment (`$x:expr`, `$id:ident`, `$t:ty`, `$tt:tt`, ...).
- **fragment:** the matcher class that defines what `$name` captures.
- **repetition:** `$(...)` with `*`, `+`, or `?` — match/emit a group zero-or-more, one-or-more, or zero-or-one times.
- **token tree (`tt`):** a single token or a complete delimited group; the most flexible matcher.
- **hygiene:** the property that identifiers introduced by a macro don't collide with caller identifiers, while captured ones keep their meaning.
- **`#[macro_export]`:** push the macro to the crate root and make it importable by other crates.
- **`$crate`:** inside an exported macro, always the defining crate — the only safe path to your own items.

## Exercise

Open `exercises/` and fill in the `// TODO(module-037)` comments in `src/lib.rs`. All five macros are exported for the tests:

1. `my_vec!` — a mini `vec!`: build a `Vec` from `$($elem:expr),*` (allow a trailing comma). Start from the empty-case behavior already there.
2. `min!` — variadic minimum: one argument returns itself; two or more return the smaller recursively via `::std::cmp::min` and `$crate::min!(...)`.
3. `sum!` — variadic sum over `expr` fragments, returning `u64`; empty input sums to 0.
4. `count_tt!` — counts token trees: base case `()` yields 0, recursive case adds 1 per `$tt` (remember a group is one `tt`).
5. `def_struct!` — generate a `pub struct` with `pub` fields plus a `new(...)` constructor. The scaffold's constructor ignores its arguments (it defaults the fields); make it store what it receives.

The tests in `tests/module_037.rs` exercise each macro, including trailing commas, empty inputs, and recursion depths.

```bash
cargo test -p module-037-exercises
```

When you're done, compare with `solutions/`.

## Further Reading

- The Rust Book, [Chapter 19.6: Macros](https://doc.rust-lang.org/book/ch19-06-macros.html)
- The Rust Reference, [Macros By Example (the full matcher grammar)](https://doc.rust-lang.org/reference/macros-by-example.html)
- [The Little Book of Rust Macros](https://veykril.github.io/tlborm/)
