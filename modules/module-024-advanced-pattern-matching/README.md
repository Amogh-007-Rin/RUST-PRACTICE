# Module 024: Advanced Pattern Matching

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Module 008 (enums and pattern matching), Module 007 (structs)

## Learning Objectives

- Write match arms with `if` guards and `@` bindings.
- Destructure nested structures — enums inside structs, tuples inside options — in one pattern.
- Use slice patterns (`[a, b]`, `rest @ ..`) to match collections by shape.
- Choose between `match`, `if let`, and `let ... else` for the job at hand.

## Why This Matters

In most languages, extracting data from nested structures is a trail of `if (obj.role != null && obj.role.type == "member")` checks. Rust folds the whole inspection into a single pattern — the compiler checks it for exhaustiveness and field validity at compile time. This shows up constantly in real code: parsing command-line input, walking JSON-shaped data, handling `Option<Result<T, E>>` chains in service code, and — as you'll see in Module 026 — matching on trait object variants. Pattern matching is arguably the feature Rust developers use most.

## Concept

Module 008 gave you the basics: `match` on an enum, `if let`, `_` as a wildcard. This module is about making patterns *precise*.

### Match guards: conditions inside a pattern

A guard is an `if` attached to an arm that runs only when the pattern itself matched. It's how you distinguish values that have the same shape:

```rust
fn main() {
    fn classify(n: i32) -> &'static str {
        match n {
            0 => "zero",
            n if n < 0 => "negative",
            n @ 1..=9 => "single digit",
            _ => "big",
        }
    }
    assert_eq!(classify(0), "zero");
    assert_eq!(classify(-5), "negative");
    assert_eq!(classify(7), "single digit");
    assert_eq!(classify(100), "big");
}
```

Two details matter. First, guards can only use variables the pattern *binds* — in `n if n < 0`, `n` is bound by the pattern. Second, order matters: arms are tried top to bottom, so `0` must come before the `n if n < 0` arm (a guard arm still matches a zero, its guard just fails, and matching continues).

### `@` bindings: match *and* bind at the same time

Sometimes you want to test a value against a pattern *and* use the value itself. `@` binds the value to a name while the pattern around it narrows what it can be:

```rust
fn main() {
    let text = String::from("hello");
    match text.as_str() {
        s @ "hello" => println!("greeting: {s}"),
        s => println!("other: {s}"),
    }
}
```

The `s @ "hello"` arm matches only the exact string `"hello"`, and `s` is bound to that string for the arm's body. Without `@`, you'd match `"hello"` and lose access to the value; without the literal, you'd match anything. `@` is especially useful with ranges and with `..` in slice patterns (below).

### Nested destructuring: one pattern, whole shape

Patterns compose. You can destructure a struct *and* an enum *and* a tuple inside a single arm — the pattern must then name every field it touches, down to the leaves:

```rust
struct User {
    name: String,
    role: Role,
}

enum Role {
    Admin,
    Member { joined_year: u32 },
}

fn main() {
    let user = User {
        name: "ada".into(),
        role: Role::Member { joined_year: 2024 },
    };

    match &user {
        User { role: Role::Admin, name } => println!("admin {name}"),
        User { role: Role::Member { joined_year: 2024 }, name } => println!("new member {name}"),
        User { name, .. } => println!("hi {name}"),
    }
}
```

Reading arm two from the outside in: "a `User` whose `role` is a `Member` variant whose `joined_year` is exactly 2024; bind `name` for the body." The third arm's `..` says "and whatever else `User` contains, I don't care" — essential when a struct grows new fields (which is also why `..` is a default-arms panacea). Note the arms bind through the reference: matching `&user` gives you `name: &String`, and `{name}` prints through the reference.

### Slice patterns: matching collections by shape

You can match a slice's *structure*: exact length, head/tail split, or prefix/suffix:

```rust
fn main() {
    fn head_and_tail(v: &[i32]) -> Option<(i32, i32)> {
        match v {
            [first, rest @ ..] => Some((*first, rest.len() as i32)),
            [] => None,
        }
    }
    assert_eq!(head_and_tail(&[10, 20, 30]), Some((10, 2)));
    assert_eq!(head_and_tail(&[7]), Some((7, 0)));
    assert_eq!(head_and_tail(&[]), None);
}
```

`[first, rest @ ..]` means "at least one element; bind `first` to it and `rest` to everything after." `rest @ ..` is the slice analog of the `@` binding: the pattern `..` matches "the rest of anything", and `@` keeps it in a variable. Slice patterns are how you'll write parsers and validators without indexing gymnastics — and why `parse_i32_pair` in the exercise can distinguish `"a, b"` from `"1,2,3"` purely by shape.

### `if let` and `let ... else`: matching when one arm is enough

`match` is the general tool; when you only care about one shape, `if let` is shorter, and `let ... else` lets you bail out early:

```rust
fn main() {
    let pair: Option<(i32, &str)> = Some((5, "five"));
    if let Some((n, s)) = pair {
        assert_eq!(s.len() as i32, n);
    }

    fn first_digit(s: &str) -> Option<u32> {
        let Some(c) = s.chars().next() else {
            return None;
        };
        c.to_digit(10)
    }
    assert_eq!(first_digit("7x"), Some(7));
    assert_eq!(first_digit(""), None);
}
```

`let Some(...) = ... else { ... }` is the "extract or return" idiom — the `else` block must diverge (return, break, panic). It reads as: "this pattern *must* match, and here's what happens if it doesn't."

### Broken: refutable patterns in `let`

This will not compile — a plain `let` requires an *irrefutable* pattern (one that always matches), and `Some(x)` isn't one:

```rust,ignore
let opt = Some(5);
let Some(x) = opt; // error: refutable pattern in local binding
```

The fix is to use `let ... else` (above), `if let`, or a `match`. The compiler's error message will literally list the alternatives — this is one of Rust's friendliest errors.

## Common Pitfalls

- **Forgetting that guards don't affect exhaustiveness.** An arm `(x, 0) if x > 0` still *matches* `(0, 0)` shape-wise; if you don't have a later arm covering `(0, 0)`, you'll fall through incorrectly. Cover the shape, then narrow with guards.
- **Binding the whole field when you only need a part.** `User { role, name }` binds the whole `Role`; if you meant to check its variant, destructure it inline (`role: Role::Member { .. }`).
- **Using `..` where you should name the field.** `User { .. }` matches everything; if a new field is added, arms with `..` silently keep matching — usually fine, but it can mask missing handling.
- **Ordering specific arms after generic ones.** `_` and wildcard-heavy arms placed early swallow everything below them. Specific patterns first.
- **Forgetting the `else` block must diverge in `let ... else`.** It's `return`, `break`, `panic!`, or `continue` — a plain expression is a compile error.

## Key Terms

- **match guard:** an `if` condition on a match arm, evaluated after the pattern matches.
- **`@` binding:** pattern syntax (`x @ pattern`) that matches `pattern` and binds the whole value to `x`.
- **nested destructuring:** combining struct, enum, and tuple patterns in a single arm.
- **slice pattern:** matching on a slice's shape, e.g. `[first, rest @ ..]`, `[a, b]`, `[]`.
- **refutable / irrefutable:** whether a pattern can fail to match. `let` needs irrefutable; `match`/`if let`/`while let` accept refutable.
- **`let ... else`:** irrefutable-with-escape-hatch: extract a value or run a diverging `else` block.

## Exercise

In `exercises/`, four functions are stubbed out. Fill in each `TODO(module-024)`:

1. `describe_point` — tuple patterns + guards to cover origin, axes, and quadrants. Arms must be ordered so later arms still catch `(0, 0)`-shaped leftovers.
2. `describe_shape` — `@` bindings for the radius, a guard for `width == height` squares.
3. `greeting` — nested destructuring of `User` and `Role`, with a literal pattern for `joined_year: 2024`.
4. `parse_i32_pair` — slice pattern `[a, b]` on `parts.as_slice()`, using `?` for the parse.

Run `cargo test -p module-024-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 18: Patterns and Matching](https://doc.rust-lang.org/book/ch18-00-patterns.html)
- [Rust Reference: Patterns](https://doc.rust-lang.org/reference/patterns.html)
- [The Rust Book, Chapter 6.3: Concise Control Flow with `if let`](https://doc.rust-lang.org/book/ch06-03-if-let.html)
