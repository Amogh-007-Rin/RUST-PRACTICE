# Module 016: Traits I — Defining & Implementing Traits

**Block:** Block B — Foundations II
**Estimated time:** 45–90 min
**Prerequisites:** Module 007 (structs), Module 015 (generics — trait *bounds* are the connective tissue)

## Learning Objectives

- Define a trait with required methods and implement it for your own types.
- Add default method bodies that implementations can inherit or override.
- Call trait methods via the `.method()` syntax and explain when a trait must be `use`d.
- Distinguish trait *definition* from trait *implementation*, and know the orphan rule at a high level.
- Recognize the standard traits you've already used: `Debug`, `Clone`, `PartialEq`, `Hash`, `Default`.

## Why This Matters

Traits are Rust's answer to interfaces (Java), protocols (Swift), and typeclasses (Haskell) — but they're also the *primary* mechanism for code reuse and abstraction. When you `#[derive(Debug, Clone)]`, you're implementing standard traits. When a library says "anything implementing `Serialize` works here", that's a trait. In later modules you'll use traits as generic bounds (`T: PartialOrd`, Module 015/017), as trait objects (`dyn Trait`, Module 017), and for operator overloading (Module 025). This module gives you the raw material: defining a trait, implementing it, and default methods.

## Concept

### The problem traits solve

You have three types that should all support the same operations:

```rust
struct Person { name: String }
struct Robot { model: String }
struct Cat { name: String }
```

Naive Rust would give each its own methods:

```rust
impl Person { fn greet(&self) -> String { ... } }
impl Robot { fn greet(&self) -> String { ... } }
impl Cat { fn greet(&self) -> String { ... } }
```

But then there's no *shared name* for "something that can greet" — no way to write one function that accepts a `Person` *or* a `Robot`. A trait declares that shared contract:

```rust
pub trait Greeter {
    fn name(&self) -> &str;
    fn farewell(&self) -> String;
}
```

And each type promises to fulfill it:

```rust
impl Greeter for Person {
    fn name(&self) -> &str {
        &self.name
    }

    fn farewell(&self) -> String {
        "Goodbye, human.".to_string()
    }
}
```

Once implemented, the methods are callable with the familiar dot syntax, and the trait gives you a type you can write functions against:

```rust
fn announce<T: Greeter>(greeter: &T) -> String {
    format!("{} says: {}", greeter.name(), greeter.farewell())
}

let alice = Person { name: "Alice".to_string() };
assert_eq!(announce(&alice), "Alice says: Goodbye, human.");
```

`T: Greeter` is the trait bound you met in Module 015 — "any type that implements `Greeter`". The trait name is the abstraction; the impls are the concrete behaviors.

### Anatomy of a trait

```rust
pub trait Greeter {
    fn name(&self) -> &str;          // signature only: required
    fn farewell(&self) -> String;    // signature only: required

    fn greet(&self) -> String {      // body provided: default method
        format!("Hello, {}!", self.name())
    }
}
```

Two kinds of members:

- **Required methods** — just a signature. Every implementor must write a body. The contract is: "every `Greeter` has a `name()` returning `&str`".
- **Default methods** — signature *plus* a body. Implementors inherit it for free and may override it. Defaults are where shared behavior lives: most greeters say `"Hello, {name}!"`, so write it once in the trait.

Notice the `&self` receiver: the first parameter of every trait method says *what* the method applies to. `&self` means "read-only access" (like Module 005's immutable borrows); `&mut self` means exclusive access; `self` consumes the value. The syntax `fn name(&self) -> &str` desugars to `fn name(self: &Self) -> &str` — `Self` being whatever type implements the trait. A trait can also declare methods with no receiver at all (associated functions), but those are rare in trait definitions.

### Implementing: overriding and inheriting

Each type implements the trait independently, and it may override any default:

```rust
impl Greeter for Robot {
    fn name(&self) -> &str {
        &self.model
    }

    // farewell() is required, so we must write it
    fn farewell(&self) -> String {
        "Beep. Shutting down.".to_string()
    }
    // greet() is inherited from the default: "Hello, {model}!"
}
```

`Robot` gets `greet()` for free; `Person` can override it with a custom greeting. A diagram of the relationship:

```
trait Greeter
+----------------------+          implement        struct Person
| fn name(&self)->&str | <-------------------------+  name: String
| fn farewell(&self)   |    impl Greeter for Person +-- name()      -> &self.name
|   -> String          |                            +-- farewell()  -> "Goodbye, human."
| fn greet(&self)      |                            +-- greet()     -> "Hi, I'm {name}!"  (override)
|   -> String { ... }  |            implement
+----------------------+          impl Greeter for Robot
                                  +  name()      -> &self.model
                                  +  farewell()  -> "Beep. Shutting down."
                                  +  greet()     -> inherited default
```

One type, many traits:

```rust
pub trait Describable {
    fn describe(&self) -> String;

    fn summary(&self) -> String {
        self.describe()
    }
}

impl Describable for Person {
    fn describe(&self) -> String {
        format!("Person named {}", self.name)
    }
}
```

`Person` now implements both `Greeter` and `Describable`. A type can implement any number of traits, and traits from anywhere (yours, std, a crate) can be implemented for your types. Method name collisions between traits are resolved by calling with fully-qualified syntax: `<Person as Greeter>::name(&alice)` — you'll rarely need it, but it exists.

### `use` and the standard traits

To call trait methods on a type, the trait must be *in scope*. Your own traits are in scope in your crate; traits from other crates need a `use`:

```rust
use module_016_exercises::{Describable, Greeter}; // brings the methods in

let alice = Person { name: "Alice".to_string() };
let _ = alice.greet();   // works because Greeter is in scope
```

The derives you've seen since Module 007 are implementations of standard traits:

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
struct Config {
    port: u16,
    hostname: String,
}
```

- `Debug` — `{:?}` printing.
- `Clone` — `.clone()`.
- `PartialEq` — `==` / `assert_eq!`.
- `Hash` — usable as a `HashMap` key (Module 012).
- `Default` — `Config::default()`.

Each derive is shorthand for `impl Debug for Config { ... }` written by the compiler. Standard traits are exactly the same machinery as your `Greeter` — no special status, just well-known names the ecosystem agrees on.

### A note on the orphan rule

"Implement a trait for a type" — but not every combination. Rust's **orphan rule** says: you may implement a trait for a type only if *either* the trait *or* the type is defined in your crate. You can't write `impl Display for Vec<u8>` (both foreign) — that's what the `newtype` pattern (Module 025) is for. The rule exists so that two crates can't both implement the same trait for the same type and silently disagree.

## Common Pitfalls

- **Forgetting the receiver.** `fn name(&self) -> &str` — the `&self` is mandatory. `fn name() -> &str` declares an associated function with no instance, and `person.name()` won't compile.
- **Implementing a trait without all required methods.** The compiler errors with a list of missing methods. Fix: implement every required method (defaults count as implemented).
- **Traits not in scope.** `alice.greet()` fails with "no method named `greet` found" if `Greeter` isn't `use`d. Fix: bring the trait into scope.
- **`Self` vs concrete type.** Inside an impl, `Self` is the implementing type — `fn new() -> Self` returns `Person`. Using a concrete name instead works but is less flexible.
- **Implementing a foreign trait for a foreign type.** `impl Debug for Vec<u8>` — orphan rule violation. Fix: wrap in a newtype (Module 025) or define your own trait.

## Key Terms

- **Trait:** a set of method signatures (plus optional defaults) that types can implement.
- **Receiver (`self`):** the first parameter of a trait method, stating which value the method operates on (`&self`, `&mut self`, `self`).
- **Default method:** a trait method with a body, inherited unless overridden.
- **Required method:** a trait method with no body; every implementor must provide one.
- **`Self`:** inside a trait or impl, the type implementing it.
- **Trait bound (`T: Trait`):** a generic constraint meaning "any type implementing `Trait`".
- **Orphan rule:** you may implement a trait for a type only if you define the trait or the type.
- **Derive:** `#[derive(Trait)]` auto-implements standard traits.

## Exercise

Open `exercises/src/lib.rs`. The traits (`Greeter`, `Describable`) and the three structs are defined; fill in the `TODO(module-016)` bodies in the `impl` blocks:

1. `Person`: custom `greet()`, `name()`, `farewell()`, `describe()`.
2. `Robot`: `name()`, `farewell()`, `describe()`, and an overridden `summary()` — note it inherits the default `greet()`.
3. `Cat`: `name()`, `farewell()`, `describe()` — also inheriting the default `greet()`.

The tests in `tests/module_016.rs` define "done":

```bash
cargo test -p module-016-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 10.2 — Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Book, Appendix C — Derivable Traits](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
- [std::fmt::Display — the trait behind `{}` printing](https://doc.rust-lang.org/std/fmt/trait.Display.html)
- [Rust Reference — trait items and the orphan rule](https://doc.rust-lang.org/reference/items/traits.html)
