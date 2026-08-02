# Module 008: Enums & Pattern Matching

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 007 (structs, methods)

## Learning Objectives

- You will be able to define an enum with and without associated data (`enum Command { Add(String), List }`).
- You will be able to write exhaustive `match` expressions and know what the compiler enforces about them.
- You will be able to bind matched data to names in patterns and combine patterns with `|` and `_`.
- You will be able to use `if let` and `while let` for single-pattern cases.
- You will be able to work with `Option<T>` — Rust's "maybe a value" type — fluently.

## Why This Matters

Enums are Rust's superpower for modeling *states and choices* — HTTP responses (`Ok`/`Err`), parser results, UI events, and (in Capstone 01) CLI commands are all enums. Where other languages model "either/or" with nullable pointers and strings-that-mean-things, Rust makes every alternative a named, typed case, and the compiler *forces* you to handle all of them. That's why production Rust has dramatically fewer "what if it's the other case" bugs: the cases are in the type, and the type checker won't let you forget one.

## Concept

### Enums: a type with a fixed set of alternatives

An enum declares a type whose value is *exactly one of* a fixed set of variants:

```rust
enum Direction {
    North,
    South,
    East,
    West,
}

fn main() {
    let d = Direction::North; // the variant is namespaced under the enum
}
```

Every variant is a distinct, first-class value. But the real power is that variants can *carry data* — this is what makes enums "sum types", the union of their variants' payloads:

```rust
enum Command {
    Add(String),      // carries a name
    List,             // carries nothing
    Remove(usize),    // carries an id
    Search(String),   // carries a query
}
```

`Command::Add("Ada".to_string())` and `Command::List` are both `Command`, yet they carry completely different payloads — and you can't mistake one for the other, because the *variant* says which payload is present. The `usize` in `Remove` isn't a generic "number field": the type system knows it's the id, because it lives in that variant. Contrast with a C-style union or a `String`-typed command: same flexibility, none of the safety.

Under the hood, an enum takes only as much memory as its largest variant (plus a discriminant tag — the "which variant is this" marker). The compiler tracks the tag; your code never touches it directly.

### Struct variants: named payloads

Variants can carry *named* fields — struct variants — which read better when a payload has several parts. Capstone 01 uses exactly this shape for its `Add` command:

```rust
enum Command {
    Add { name: String, email: Option<String>, phone: Option<String> },
    List,
    Remove(u32),
}

fn main() {
    let cmd = Command::Add {
        name: "Ada".to_string(),
        email: None,
        phone: None,
    };
    match cmd {
        Command::Add { name, email, phone } => {
            println!("adding {name} ({email:?}, {phone:?})");
        }
        _ => {}
    }
}
```

Matching a struct variant destructures its fields by name — the mirror image of Module 007's named-field reads. Tuple variants (positional, `Remove(u32)`) stay the right choice for single-payload cases; struct variants take over once there are two or more fields to name.

### `Option` has convenience helpers (preview)

`match` is always the fallback, but `Option` ships shortcuts: `is_some()` / `is_none()` for checks, `unwrap_or(default)` for a fallback value, and `map` / `and_then` (Modules 021–023) for chaining transformations without unwrapping. `unwrap()` and `expect()` exist too, but they *panic* on `None` — handy in small programs, a smell in production code. You'll replace them with proper error handling in Modules 013–014; for now, prefer matching and `if let`.

### `match`: exhaustive pattern matching

`match` is the "switch" that's actually safe: it checks *every* variant and *binds* payloads:

```rust
enum Command {
    Add(String),
    List,
    Remove(usize),
    Search(String),
}

fn describe(c: &Command) -> String {
    match c {
        Command::Add(name) => format!("add contact {name}"),
        Command::List => "list all contacts".to_string(),
        Command::Remove(id) => format!("remove contact {id}"),
        Command::Search(query) => format!("search for {query}"),
    }
}
```

Three things to note:

- **Every arm is `pattern => value`.** The pattern names the variant and *binds* its payload (`name`, `id`, `query` get the inner values). The value after `=>` is what the arm produces — `match` is an expression, like `if` from Module 003.
- **`match` must be exhaustive.** Leave out `Command::List` and the compiler refuses: `non-exhaustive patterns: `List` not covered`. Exhaustiveness is checked *at compile time* — the classic "the compiler made me handle every case" moment is this rule. When you add a variant later, every `match` on the type breaks until you handle it. This is a feature: the compiler walks you through your own codebase.
- **The catch-all `_`** matches anything, for cases where you genuinely don't care:

```rust
enum Command {
    Add(String),
    List,
    Remove(usize),
    Search(String),
}

fn is_removal(c: &Command) -> bool {
    match c {
        Command::Remove(_) => true, // `_` = "match this variant, ignore its data"
        _ => false,
    }
}
```

### Binding and combining patterns

Patterns are a small language. The pieces you need now:

```rust
fn describe_opt(o: Option<i32>) -> String {
    match o {
        Some(7) => "the lucky number!".to_string(), // literal pattern
        Some(n) => format!("some({n})"),            // binding pattern
        None => "nothing".to_string(),
    }
}
```

Order matters: `Some(7)` is checked first; more general `Some(n)` catches the rest. You can also combine alternatives with `|`:

```rust
fn is_end_of_line(ch: char) -> bool {
    match ch {
        '\n' | '\r' => true,
        _ => false,
    }
}
```

And `match` can match on *tuples* of values — perfect for "two things at once":

```rust
enum Unit {
    Celsius,
    Fahrenheit,
}

fn convert(value: f64, from: Unit, to: Unit) -> f64 {
    match (from, to) {
        (Unit::Celsius, Unit::Fahrenheit) => value * 9.0 / 5.0 + 32.0,
        (Unit::Fahrenheit, Unit::Celsius) => (value - 32.0) * 5.0 / 9.0,
        _ => value, // same unit: unchanged
    }
}
```

### `Option<T>`: the enum you'll use every day

Rust has no `null`. Instead, the standard library defines:

```rust
enum Option<T> {
    Some(T),
    None,
}
```

`Option<String>` is "a `String` that might not be there" — and the compiler forces you to handle both cases wherever you use one. That's how Rust makes the billion-dollar mistake (null references) a *type error*. Two quick idioms:

```rust
fn main() {
    let maybe: Option<i32> = Some(42);

    match maybe {
        Some(n) => println!("got {n}"),
        None => println!("nothing there"),
    }

    if let Some(n) = maybe {
        println!("also {n}"); // runs only when the pattern matches
    }
}
```

### `if let` and `while let`: matching without the ceremony

When you only care about *one* pattern, `match`'s boilerplate is noise. `if let` runs a block only if the pattern matches:

```rust
fn main() {
    let contact = Some("Ada");
    if let Some(name) = contact {
        println!("hello {name}");
    }
}
```

It's the sugar for `match x { Pattern => ..., _ => () }`. `while let` is the loop version — keep processing while the pattern matches:

```rust
fn main() {
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        println!("popped {top}");
    }
}
```

The `?` operator (Module 014) is the third member of this family: "unpack `Some`, or return `None` early" — you'll see a preview of it in this module's `parse_command`. One clippy note: a `if let` whose `else` returns `None` and whose body just rewraps a value (`Some(n + 1)`) gets flagged as a manual `Option::map` — a closure-based idiom you'll learn in Module 021. If you see that suggestion, don't worry about it yet; building the value in a local binding first (`let next = n + 1; Some(next)`) keeps the `if let` form and stays clippy-clean.

### This module's exercise

Four functions plus a `Command` enum (whose shape returns in Capstone 01):

1. `parse_command(line) -> Option<Command>` — split the line, `match` the first word, and use `?`/`.ok()?` for the fallible parts.
2. `convert(value, from, to)` — `match (from, to)` over the four combinations.
3. `describe(o) -> String` — `match` on `Option<i32>` with a binding.
4. `bump(o) -> Option<i32>` — the same idea with `if let`.

The tests cover each variant of `Command`, both conversion directions, identity, and the `None` paths — exhaustive, like every good test suite of a `match`.

## Common Pitfalls

- **Non-exhaustive `match`.** Miss a variant and the compiler errors. Either add the arm or cover it with `_`.
- **Forgetting the `=>` arms are expressions.** Each arm's value must have the same type; a `;` after a string literal in an arm makes it `()` and the match breaks.
- **Using `if let` when `match` is clearer.** For two-or-more patterns, `match` wins. Reserve `if let` for single-pattern cases.
- **Patterns that are too specific, too early.** `Some(7)` before `Some(n)` makes `Some(7)` unreachable if ordered wrongly — actually the reverse: specific patterns must come first, or the compiler warns about unreachable patterns.
- **Treating `Option` like a nullable pointer.** `Option<T>` is a *value* — you can't use the inner value until you match it. That's the point.

## Key Terms

- **enum:** a type with a fixed set of named variants, optionally carrying data.
- **variant:** one of an enum's alternatives (`Command::Add`).
- **pattern:** the syntax that matches a value's shape and binds its parts (`Some(n)`).
- **exhaustive match:** a `match` covering every variant — enforced by the compiler.
- **`Option<T>`:** the enum encoding "maybe a value" — `Some(T)` or `None`.
- **`if let` / `while let`:** sugar for single-pattern matching and matching loops.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the four `// TODO(module-008)` comments.
2. Implement `parse_command(line)` — match the first word, build each variant, use `?` for failure.
3. Implement `convert(value, from, to)` — `match (from, to)` with the two formulas and an identity fallback.
4. Implement `describe(o)` — `match` on the `Option`, bind the inner value.
5. Implement `bump(o)` — `if let Some(n) = o`.
6. Run `cargo test -p module-008-exercises` until all 10 tests pass.
7. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 6: Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html) — the canonical chapter.
- [The Rust Book, Chapter 6: The `match` Control Flow Operator](https://doc.rust-lang.org/book/ch06-02-match.html) — arms, bindings, and exhaustiveness.
- [The Rust Book, Chapter 6: Concise Control Flow with `if let`](https://doc.rust-lang.org/book/ch06-03-if-let.html) — sugar for single patterns.
- [std: `Option`](https://doc.rust-lang.org/std/option/enum.Option.html) — the type behind every "maybe".
