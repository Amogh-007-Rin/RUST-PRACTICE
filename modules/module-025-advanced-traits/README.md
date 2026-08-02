# Module 025: Advanced Traits

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Modules 016–017 (traits, bounds), Module 015 (generics), Module 024 (patterns)

## Learning Objectives

- Declare and use associated types in your own traits.
- Implement `std::ops::Add` and friends to overload operators for your types.
- Express "must also implement X" constraints with supertraits.
- Apply the newtype pattern to give a plain value a distinct, safe type.

## Why This Matters

These four mechanisms are what let you build *your own std*. Associated types power `Iterator` (which you implemented in Module 022) and every error-handling ecosystem crate; operator overloading is how crates like `nalgebra` (linear algebra) and `chrono` (dates) make math and time readable; supertraits appear wherever a trait promises "and it's printable, too" — think any `Display`-bound logging trait; and the newtype pattern is the standard defense against mixing up `u64`s that mean very different things (user IDs vs. timestamps) across an entire codebase.

## Concept

### Associated types: "this trait works on *some* type"

An associated type is a placeholder type each *implementation* chooses. The trait promises "I deal in a type called `Item`" and leaves it to each `impl` to decide which:

```rust
trait Container {
    type Item;

    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Container for Vec<T> {
    type Item = T;

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

fn first<C: Container>(c: &C) -> Option<&C::Item> {
    c.get(0)
}

fn main() {
    assert_eq!(first(&vec![10, 20]), Some(&10));
}
```

Compare with a generic trait parameter, `trait Container<T>`: associated types are *single* — a type can implement `Container` exactly once, with exactly one `Item`. That's why `Iterator` uses an associated type: `impl Iterator for Step` with `type Item = i64` is unambiguous, whereas `impl Iterator<i64> for Step` would allow (and require deciding) several.

### Operator overloading

The `std::ops` traits let you make your own types work with `+`, `*`, `[]`, `-`, and the rest. Each trait has an associated type naming the result. `Add` is a great first example:

```rust
use std::ops::{Add, Mul};

#[derive(Debug, PartialEq)]
struct Vector(f64, f64);

impl Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}

fn main() {
    assert_eq!(Vector(1.0, 2.0) + Vector(3.0, 4.0), Vector(4.0, 6.0));
    assert_eq!(Vector(2.0, 3.0) * 2.0, Vector(4.0, 6.0));
}
```

A step-by-step view of what implementing an operator does:

```
  code written              after implementing                 compiler inserts
  ------------------------  ---------------------------------  ------------------
  Vector(1,2) + Vector(3,4)  Vector::add(Vector(1,2), Vector(3,4))  add(self, rhs)
  Vector(2,3) * 2.0          Vector::mul(Vector(2,3), 2.0)          mul(self, rhs)

  Both dispatch statically (Module 026) — the operator call is inlined
  into the trait method you wrote, with zero runtime cost.
```

Note `Mul<f64>` is *generic*: this impl lets `Vector * f64` work. You could add a second impl `Mul<Vector> for Vector` for a different meaning — generic trait parameters multiply the possibilities, associated types keep them coherent.

### Supertraits: "implements X *and* Y"

A supertrait bound demands that implementors also implement another trait. It's how a trait can *use* that other trait's methods in its own defaults or expose them through its bounds:

```rust
use std::fmt::Display;

trait Summarizable: Display {
    fn summary(&self) -> String;
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

impl Summarizable for Book {
    fn summary(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}

fn print_summary<S: Summarizable>(item: &S) -> String {
    format!("{} | {}", item.summary(), item)
}

fn main() {
    let book = Book {
        title: "Rust in Action".into(),
        author: "Tim McNamara".into(),
        pages: 382,
    };
    assert_eq!(
        print_summary(&book),
        "Rust in Action by Tim McNamara (382 pages) | Rust in Action by Tim McNamara"
    );
}
```

The payoff is in `print_summary`: its bound is `S: Summarizable`, but inside it can call `item.summary()` *and* use `item` with `{}` — the supertrait guarantee (Book's `Display` impl) is available wherever the bound is. Attempting `impl Summarizable for Book` without `Display` fails with a clear "the trait bound `Book: Display` is not satisfied" error.

### The newtype pattern: wrapping a value in intent

A **newtype** is a tuple struct with one field — `struct Celsius(f64)`. It gives you a distinct type the compiler treats as unrelated to `f64`, so you can't accidentally pass Celsius where a raw `f64` (or another newtype) is expected:

```rust
struct Celsius(f64);
struct Fahrenheit(f64);

impl Celsius {
    fn to_fahrenheit(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(celsius: Celsius) -> Self {
        Fahrenheit(celsius.to_fahrenheit())
    }
}

fn main() {
    let boiling = Celsius(100.0);
    let f: Fahrenheit = boiling.into();
    assert!((f.0 - 212.0).abs() < 1e-9);
}
```

`Celsius` and `Fahrenheit` are both `f64` under the hood, yet mixing them is a compile error — the error message is exactly the point. Newtypes typically get their own `impl` blocks (methods), conversions (`From`), and `Display`. Any trait you want — `Add`, `Deref` (Module 028), `Serialize` (Block G) — you implement *deliberately* for the newtype, which is the other virtue: `f64`'s whole API doesn't leak through unless you want it to.

### Broken: forgetting the supertrait

This will not compile — `Summarizable` demands `Display`, and `Temperature` doesn't provide it:

```rust,ignore
struct Temperature(f64);

impl Summarizable for Temperature { // error: the trait bound
    fn summary(&self) -> String {   // `Temperature: Display` is not satisfied
        format!("{}°", self.0)
    }
}
```

The fix is to implement `Display` for `Temperature` (or drop the supertrait). The compiler's error names the missing trait exactly, and rust-analyzer can even insert the skeleton for you.

## Common Pitfalls

- **Choosing a generic trait parameter when you want an associated type.** If a type should have *one* meaningful implementation of the trait, use an associated type; generic params invite accidental multiple implementations (and downstream ambiguity).
- **Forgetting `type Output` in an operator impl.** Every `std::ops` trait requires it; the compiler tells you the moment you forget.
- **Adding a newtype but not giving it methods or conversions.** A bare `struct Id(u64)` that nobody can do anything with isn't better than the `u64`; add `From`, `Display`, and the handful of operations it needs.
- **Assuming the newtype inherits traits.** It does *not* — `Celsius` isn't `Copy`, `Add`, or even `Debug` unless you implement or derive them. That's deliberate, but it's also the most common "why doesn't this work" surprise.
- **Supertrait on the wrong side.** The bound goes on the *trait that requires* the other one (`trait Summarizable: Display`), not on implementors.

## Key Terms

- **associated type:** a type chosen per-implementation inside a trait, named like `type Item;`.
- **`std::ops`:** the module of operator traits (`Add`, `Mul`, `Index`, `Neg`, ...) that power overloading.
- **`type Output`:** the associated type operator traits use to name their result type.
- **supertrait:** a trait bound on a trait definition (`trait A: B`), requiring implementors of `A` to also implement `B`.
- **newtype pattern:** a one-field tuple struct wrapping an existing type to give it a distinct identity and a tailored API.

## Exercise

In `exercises/`, the types are all defined — what's missing is the trait machinery. Fill in each `TODO(module-025)`:

1. `Container` impls for `Vec<T>` and `String` (associated types) and the `first` helper.
2. `Add` and `Mul<f64>` for `Vector` — write the operator bodies and `type Output`.
3. `Celsius`/`Fahrenheit` conversion methods and `From<Celsius> for Fahrenheit` (newtype pattern).
4. `Display` + `Summarizable` for `Book` and the `print_summary` helper (supertrait).

Run `cargo test -p module-025-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 19.3: Advanced Traits](https://doc.rust-lang.org/book/ch19-03-advanced-traits.html)
- [std docs: `std::ops`](https://doc.rust-lang.org/std/ops/index.html)
- [Rust by Example: Operator Overloading](https://doc.rust-lang.org/rust-by-example/trait/ops.html)
