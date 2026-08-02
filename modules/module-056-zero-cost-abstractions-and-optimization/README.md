# Module 056: Zero-Cost Abstractions & Optimization

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 60–90 min
**Prerequisites:** Module 015 (Generics), Module 016 (Traits I), Module 028 (Smart Pointers I — Box)

## Learning Objectives

- You will be able to implement a simplified Clone-on-Write (Cow) string wrapper.
- You will be able to write a generic fixed-capacity buffer using const generics (`const N: usize`).
- You will be able to explain when to use `&str` vs `String` and how Cow defers allocation.
- You will understand how zero-cost abstractions let Rust express high-level patterns without runtime overhead.

## Why This Matters

Rust's signature promise is "zero-cost abstractions" — patterns that read like high-level code but compile to the same machine code you'd write by hand. `Cow<'_, str>` is a perfect example: it lets you write functions that accept either `&str` or `String` without always cloning, deferring allocation until mutation happens. Const generics (`ArrayBuffer<T, 4>`) let you write generic code parameterized by *values*, not just types, with the buffer size compiled into the code at monomorphization time. These patterns aren't academic — `Cow` is used throughout the stdlib and popular crates like `serde`, and const generics power everything from fixed-size arrays to SIMD lane counts.

## Concept

### Clone-on-Write (Cow)

`std::borrow::Cow<'_, B>` is an enum:

```rust
pub enum Cow<'a, B: ToOwned + ?Sized> {
    Borrowed(&'a B),
    Owned(<B as ToOwned>::Owned),
}
```

For strings, `Cow<'_, str>` is either `Cow::Borrowed(&str)` or `Cow::Owned(String)`. When you have a `Cow::Borrowed`, reading is free (just a pointer dereference). When you call `.to_mut()`, it clones into `Cow::Owned` — but only if it was previously borrowed. If it was already owned, the mutation is in-place.

This pattern shines in functions that mostly read but occasionally need to modify:

```rust
use std::borrow::Cow;

fn sanitize(input: &str) -> Cow<'_, str> {
    if input.contains('<') {
        Cow::Owned(input.replace('<', "&lt;"))
    } else {
        Cow::Borrowed(input) // zero allocation
    }
}
```

Only when `<` is present does this allocate. The rest of the time it's a zero-cost reference.

### Implementing a simplified CowStr

For this module you'll implement a simplified `CowStr` wrapper:

```rust
pub struct CowStr(Option<String>);
```

It represents either a not-yet-materialized string (no allocation — conceptually "borrowed") or an owned `String`. The "clone" happens lazily:

- `CowStr::from_str(s)` stores `Some(s.to_string())` — own it immediately.
- `CowStr::new()` returns `CowStr(None)` — no allocation yet.
- `.as_str()` returns `&str`: either the inner string, or `""` if `None`.
- `.to_mut(source)` is the write trigger: if `None`, it clones `source` into `Some(source.to_string())`. If already `Some`, just returns the inner string.
- `.into_string()` extracts the owned string (or returns an empty `String`).

### Const generics: `ArrayBuffer<T, const N: usize>`

Const generics let generic code be parameterized by compile-time integer values. Before const generics stabilized, you had to use macros or type-level tricks (e.g., `typenum`) to write `[T; N]` generically. Now you can write:

```rust
pub struct ArrayBuffer<T, const N: usize> {
    data: [T; N],
    len: usize,
}
```

Here `N` is a compile-time constant — it's monomorphized just like a type parameter. `ArrayBuffer<i32, 4>` and `ArrayBuffer<i32, 8>` are different types with different sizes known at compile time.

The exercise requires:
- `new()` — initializes the buffer (needs `T: Default + Copy` to fill the array).
- `push(value)` — appends if there's room.
- `get(index)` — returns `Option<&T>`.
- `len()` and `is_empty()`.

### `&str` vs `String` tradeoffs

A key Rust optimization skill is knowing when to borrow and when to own:

| Pattern | Cost | When to use |
|---|---|---|
| `&str` parameter | Zero copy, borrow-checked | Read-only access, caller owns the data |
| `String` parameter | Moves ownership, allocates | You need to store, modify, or return the data |
| `Cow<'_, str>` | Zero copy if unchanged | Mostly reading, occasionally modifying |

The exercise includes a function that uses `CowStr` to avoid allocation when a string is already owned — returning the owned version without cloning.

### ASCII diagram: CowStr states

```
CowStr(None)                     CowStr(Some("hello"))
┌──────┐                        ┌──────────────────┐
│ None │  no allocation         │ Some("hello")    │ owns the string
│      │                        │                  │
│ as_str() → ""                 │ as_str() → "hello"
│ to_mut(src) → Some(src)       │ to_mut(_) → "hello" (no clone)
│ (clones at write time)        │                  │
└──────┘                        └──────────────────┘
```

### Monomorphization and zero cost

Both const generics and the Cow pattern produce identical machine code to a hand-written version. When Rust compiles `ArrayBuffer<i32, 4>::push()`, it generates a push function specialized for a 4-element `i32` buffer — no dynamic dispatch, no runtime size checks beyond the `len < N` guard. A `CowStr` that's always `Some` compiles to the same code as `String` access. The abstraction costs nothing at runtime.

## Common Pitfalls

- **Cow: forgetting `.to_mut()` before mutation.** Writing to a `Cow::Borrowed` directly won't compile — you need to call `.to_mut()` to get a `&mut String`.
- **Const generics: `[T; N]` requires `Default` or `MaybeUninit`.** You can't initialize `[T; N]` with uninitialized memory in safe Rust without `Default`. Use `[T::default(); N]` or `MaybeUninit` for no-default types.
- **Over-allocating with Cow.** If your input is always modified, Cow adds branch overhead without saving any allocations — just take `String`.
- **Const generics: forgetting the `usize` bound.** Older Rust editions required `const N: usize` to also satisfy `ArrayLength<T>` or similar. Edition 2021 with `min_const_generics` stabilized this.

## Key Terms

- **Zero-cost abstraction:** a high-level pattern that compiles to code no worse than a hand-written low-level equivalent.
- **Clone-on-Write (Cow):** a smart pointer that borrows data until mutation, then clones into an owned copy.
- **Const generics:** generic parameters that are compile-time constant values (e.g., array sizes) rather than types.
- **Monomorphization:** the compiler's process of generating a specialized copy of generic code for each concrete type/value.
- **`ToOwned`:** the trait that converts a borrowed type to its owned equivalent (`&str` → `String`).

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-056)` comments.
2. Implement `CowStr` methods: `new()`, `from_str()`, `as_str()`, `to_mut()`, `into_string()`.
3. Implement `ArrayBuffer<T, const N: usize>` with `new()`, `push()`, `get()`, `len()`, `is_empty()`.
4. Implement `longest_str()` — returns the longer of a `&str` and a `CowStr`, as a `CowStr`.
5. Run `cargo test -p module-056-exercises` until all tests pass.
6. Compare with `solutions/` afterwards.

## Further Reading

- [`std::borrow::Cow` documentation](https://doc.rust-lang.org/std/borrow/enum.Cow.html) — the real Clone-on-Write type.
- [The Rust Reference: Const Generics](https://doc.rust-lang.org/reference/items/generics.html#const-generics) — formal specification of const generic parameters.
- [Rust by Example: Cow](https://doc.rust-lang.org/rust-by-example/trait/cow.html) — practical Cow examples.
- [The C++ `std::string_view` / Rust `Cow` comparison](https://nnethercote.github.io/perf-book/strings.html) — from the Rust Performance Book.
