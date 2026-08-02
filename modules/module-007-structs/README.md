# Module 007: Structs

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 006 (slices, `String` vs `&str`)

## Learning Objectives

- You will be able to define a named struct, instantiate it, and access or mutate its fields.
- You will be able to implement methods (`&self`, `&mut self`, `self`) and associated functions (`Self::new` style) in an `impl` block.
- You will be able to use the field-init shorthand and update syntax.
- You will be able to define tuple structs and unit structs and say when each is useful.
- You will be able to explain how `self` relates to ownership and borrowing from Module 005.

## Why This Matters

Structs are how you give a name and a shape to your data — and they are everywhere in real Rust: an `axum` handler's state, a `sqlx` database row, a Bevy component, a blockchain account. The `impl` block pattern (`new`, methods taking `&self`) is the single most common API shape in the ecosystem, and it's exactly what Capstone 01's `ContactBook` is built around. If you can read `impl X { pub fn y(&self) -> Z }`, you can read most Rust library code.

## Concept

### Defining and instantiating structs

A struct bundles named fields under one type:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}
```

This declares *what* a Book is. To get one, you *instantiate* it, listing every field:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    let b = Book {
        title: String::from("The Rust Programming Language"),
        author: String::from("Steve Klabnik"),
        pages: 500,
    };
    println!("{}", b.title);
    println!("{}", b.pages);
}
```

A struct's data lives wherever you put it — the struct itself sits on the stack by default, and any heap-owning fields (`String`) point at the heap. Fields keep the ownership rules: `b.title` is a `String`, and moving it out of `b` moves it (partial moves are fine — you just can't use `b` as a whole afterwards).

The **field-init shorthand** saves typing when a variable has the same name as the field:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn build(title: String, author: String, pages: u32) -> Book {
    Book { title, author, pages }
}
```

And **struct update syntax** copies "the rest" from another instance:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    let original = Book {
        title: String::from("A"),
        author: String::from("B"),
        pages: 100,
    };
    let sequel = Book {
        title: String::from("A2"),
        ..original // everything else from `original`
    };
    println!("{} by {}", sequel.title, sequel.author);
}
```

### Methods and associated functions

Functions tied to a type live in an `impl` block. Methods take a `self` parameter in one of three flavors, which map exactly onto Modules 004–005:

| Method flavor | Borrows | Use case |
|---|---|---|
| `fn f(&self)` | reads the receiver | `book.summary()` |
| `fn f(&mut self)` | mutates the receiver | `book.set_pages(600)` |
| `fn f(self)` | consumes the receiver | `book.into_parts()` |

The `&self`/`&mut self` flavors are really `self: &Self` and `self: &mut Self` — borrowing, exactly as in Module 005. And *associated functions* are `impl`-block functions without a `self`, called with `Type::function(...)` — the constructor idiom:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Book {
    fn new(title: &str, author: &str, pages: u32) -> Book {
        Book {
            title: title.to_string(),
            author: author.to_string(),
            pages,
        }
    }

    fn summary(&self) -> String {
        format!("\"{}\" by {} ({} pages)", self.title, self.author, self.pages)
    }

    fn is_long(&self) -> bool {
        self.pages > 400
    }
}

fn main() {
    let b = Book::new("Foundations", "Ada", 250); // associated function: Book::new
    println!("{}", b.summary());                  // method: receiver `&self`
    println!("{}", b.is_long());
}
```

Inside a method, `self` *is* the instance: `self.title`, `self.pages`. When a method only reads, take `&self`; the caller keeps full control of the instance.

### Tuple structs and unit structs

A **tuple struct** names a tuple — fields are anonymous, accessed by `.0`, `.1`:

```rust
struct Point(f64, f64);

fn main() {
    let p = Point(3.0, 4.0);
    println!("{}", p.0);
    println!("{}", p.1);
}
```

Tuple structs are perfect for small wrappers where naming each field would be noise — a coordinate, an RGB color, a pair of counts. They also give you a *distinct type*: `struct Meters(u64)` and `struct Seconds(u64)` are different types, and the compiler won't let you pass one where the other is expected — a zero-cost defense against unit mix-ups.

A **unit struct** has no fields at all — `struct Marker;`. It occupies no space and is used as a type-level flag or to implement traits on (Module 016). You'll see it in real code mainly as a "namespace" for associated functions or as a zero-sized marker.

### Structs, ownership, and the derive macro (preview)

A struct is subject to every rule you've learned: constructing one moves values into it, reading a field borrows the struct, and a struct is `Drop`-ped when its owner goes out of scope. One convenience you'll see everywhere: `#[derive(Debug)]` (and friends) auto-implement traits:

```rust
#[derive(Debug)]
struct Point(f64, f64);

fn main() {
    let p = Point(1.0, 2.0);
    println!("{p:?}"); // prints: Point(1.0, 2.0)
}
```

You'll write your own `impl Debug` later (Module 016); for now, know that `{p:?}` prints a struct's fields — indispensable when debugging, and required by the exercise crates' `assert_eq!` comparisons in later modules.

### Moving out of a struct: partial moves

Struct fields obey the ownership rules from Module 004. You can move a single field out of a struct — a *partial move* — as long as you don't treat the struct as a whole afterwards:

```rust
struct Book {
    title: String,
    author: String,
    pages: u32,
}

fn main() {
    let b = Book {
        title: String::from("A"),
        author: String::from("B"),
        pages: 1,
    };
    let title = b.title; // move the String field out
    let pages = b.pages; // moving a different field is fine
    println!("{title}, {pages}");
}
```

The struct is no longer usable "as a whole" (for example, you can't borrow `&b` or move it into a function afterwards), but its remaining fields stay perfectly valid. This is why methods that need *all* of a struct take `&self` or `self` — partial moves are a deliberate, visible act.

### Why methods and not free functions?

Both exist. Methods are chosen when the first parameter is "obviously the thing being operated on" — `book.summary()` reads better than `summary(book)` — and when the type should own its API surface. Free functions are used for conversions and utilities where no receiver makes sense. Rust's standard library uses both; so will you.

### Structs vs tuples vs unit structs: picking the right shape

You now have three data shapes, and the choice is mostly about *naming*:

- **Named struct** — fields have names; use it when the meaning of each field matters (almost always). `Book { title, author, pages }` is self-documenting; `("The Rust...", "Steve...", 500)` is not.
- **Tuple struct** — fields are positional; use it for small wrappers with one clear meaning per position (`Point(f64, f64)`, an `Rgb(u8, u8, u8)`). It still creates a distinct type, so you can't mix two different tuple structs by accident.
- **Unit struct** — no fields at all; use it as a marker or a "namespace" for associated functions. It costs nothing at runtime (zero-sized).

The rule of thumb: if you ever catch yourself wondering "which index was the phone number again?", it's time for a named struct.

### The exercise in a sentence each

- `Book::new(title, author, pages)` — constructor with field-init shorthand.
- `Book::summary(&self)` — read the fields, build a `String` with `format!`.
- `Book::is_long(&self)` — a pure `bool` computation.
- `Point::distance(&self, other: &Point)` — a tuple-struct method using `&self` and a borrowed argument.

The tests exercise construction, the summary string, the 400-page threshold, and distances (including the same-point case). All of this returns in Capstone 01, where `ContactBook` is a struct with methods.

## Common Pitfalls

- **Missing fields when constructing.** Rust requires *every* field at construction (unless `Default` is implemented). The compiler lists the missing ones — follow it.
- **Forgetting `.to_string()` for `&str` fields.** `title: &str` in a struct holding `String` is a type error; convert with `.to_string()` or `String::from`.
- **Writing `fn new` without `-> Self`.** `Self` is an alias for the type inside the `impl` block — `-> Self` is the idiomatic spelling.
- **Taking `&self` when you need mutation.** The compiler will tell you: change to `&mut self`. And taking `self` when `&self` would do is an ownership mistake — borrow, don't consume.
- **Using `.0` on a named struct.** Only tuple structs have positional access. Named structs always use field names.

## Key Terms

- **struct:** a named bundle of fields (`struct Book { ... }`).
- **method:** a function in an `impl` block taking `self`, `&self`, or `&mut self`.
- **associated function:** an `impl` function without `self` — called as `Type::fn(...)`, e.g. `Book::new`.
- **`Self`:** the type of the `impl` block you're inside.
- **tuple struct:** a struct with anonymous positional fields (`struct Point(f64, f64)`).
- **field-init shorthand:** `Book { title, author, pages }` when variable names match field names.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the four `// TODO(module-007)` comments (two in `impl Book`, two in `impl Point`).
2. Implement `Book::new` with the field-init shorthand and `.to_string()` conversions.
3. Implement `Book::summary` and `Book::is_long` using `&self`.
4. Implement `Point::distance` with `dx * dx + dy * dy` and `.sqrt()`.
5. Run `cargo test -p module-007-exercises` until all 6 tests pass.
6. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Book, Chapter 5: Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html) — the full struct chapter, including methods.
- [The Rust Book, Chapter 5: Method Syntax](https://doc.rust-lang.org/book/ch05-03-method-syntax.html) — `self`, `&self`, `&mut self`, associated functions.
- [The Rust Reference: Struct types](https://doc.rust-lang.org/reference/items/structs.html) — the formal definition of the three struct kinds.
