# Module 015: Generics

**Block:** Block B — Foundations II
**Estimated time:** 60–90 min
**Prerequisites:** Module 004 (ownership), Module 007 (structs), Module 008 (enums)

## Learning Objectives

- Write generic functions (`fn largest<T>(...)`) that work for any type satisfying a bound.
- Define generic structs and enums (`Pair<T, U>`, `Maybe<T>`) and implement methods on them with `impl<T, U> Pair<T, U>`.
- Explain monomorphization: how the compiler generates a specialized copy of generic code per concrete type, and why Rust generics are zero-cost.
- Use multiple type parameters and the `T: Trait` bound syntax.
- Recognize `Option<T>` and `Result<T, E>` as generic types you've already been using.

## Why This Matters

Generic code is the difference between copy-pasting a function five times and writing it once. The entire standard library is built on generics — `Vec<T>`, `Option<T>`, `Result<T, E>`, `HashMap<K, V>` — and every framework you'll use later (axum's extractors, serde's `Serialize`) is defined in terms of them. But Rust's generics are special: unlike Java or TypeScript, where generic code boxes values behind references, Rust *specializes* the code at compile time. Understanding that (monomorphization) explains a huge amount of Rust behavior you'll see in the wild — why generics don't cost you performance, why "trait objects" exist (Module 017), and why compile times grow with generic usage.

## Concept

### The problem generics solve

Three functions, identical except the type:

```rust
fn largest_i32(items: &[i32]) -> Option<&i32> {
    items.iter().reduce(|acc, item| if item > acc { item } else { acc })
}

fn largest_f64(items: &[f64]) -> Option<&f64> {
    items.iter().reduce(|acc, item| if item > acc { item } else { acc })
}

fn largest_str(items: &[&str]) -> Option<&&str> {
    items.iter().reduce(|acc, item| if item > acc { item } else { acc })
}
```

Copy-paste at this scale is error-prone and unmaintainable. Generics replace the *type* with a placeholder — a type parameter — and write the body once:

```rust
fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    items.iter().reduce(|acc, item| if item > acc { item } else { acc })
}

assert_eq!(largest(&[3, 7, 1]), Some(&7));
assert_eq!(largest(&[2.5, 1.0]), Some(&2.5));
assert_eq!(largest(&["apple", "kiwi"]), Some(&"kiwi"));
```

Convention: single uppercase letters — `T` for "type", `U`, `V` for more. The same function now works for `i32`, `f64`, `&str`, and any other type.

### The `T: PartialOrd` bound

Not every type can do `>`. So `largest` declares: "`T` may be any type *as long as it implements `PartialOrd`*" — the trait that gives `<`, `>`, `<=`, `>=`. This is a **trait bound**. Without it the function doesn't compile:

```rust,ignore
fn largest_no_bound<T>(items: &[T]) -> Option<&T> {
    items.iter().reduce(|acc, item| if item > acc { item } else { acc })
    // ^^^^ ERROR: binary operation `>` cannot be applied to type `T`
}
```

The fix is the `T: PartialOrd` bound — the compiler literally cannot call this function with a type that lacks ordering, so the failure happens at the call site, not at runtime. Module 016 defines your own traits; Module 017 goes deep on bounds.

### Generic structs and enums

```rust
struct Pair<T, U> {
    first: T,
    second: U,
}

enum Maybe<T> {
    Just(T),
    Nothing,
}
```

`Maybe<T>` is `Option<T>`'s cousin — an enum with one generic parameter. Methods are written with their own impl block:

```rust
impl<T, U> Pair<T, U> {
    fn first(&self) -> &T {
        &self.first
    }

    fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

impl<T> Maybe<T> {
    fn is_just(&self) -> bool {
        matches!(self, Maybe::Just(_))
    }

    fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => default,
        }
    }
}
```

Notice `swap`: it consumes the pair and returns a *new* type, `Pair<U, T>` — the type itself changes as the values move. And `unwrap_or` moves both `self` and `default`, so neither can be used afterwards — ownership (Module 004) working together with generics.

`impl<T, U>` might look like it's "declaring" the generics — it is: an `impl` block for a generic type must repeat the type parameters, and inside it `T` and `U` are in scope for all methods.

### Monomorphization: the zero-cost magic

Here is the part that separates Rust from most languages. When the compiler sees three different call sites of `largest`, it does not emit one function taking a runtime type tag. It emits **three specialized copies**, one per concrete type:

```
source                        compiled machine code
largest::<i32>  ->  largest_i32:  compares i32s with i32 instructions
largest::<f64>  ->  largest_f64:  compares f64s with f64 instructions
largest::<&str> ->  largest_str:  compares &strs with str instructions
```

```
fn largest<T: PartialOrd>(...) { ... }          // one generic definition
        |
        +-- used with i32  ->  [ copy with T = i32  ]  largest_i32
        +-- used with f64  ->  [ copy with T = f64  ]  largest_f64
        +-- used with &str ->  [ copy with T = &str ]  largest_str
```

That's **monomorphization** — "one shape" → "many shapes". Each copy is fully type-checked and optimized with the concrete type baked in, so `largest(&[3, 7, 1])` runs with zero indirection, zero boxing, no dynamic dispatch. The performance is identical to writing `largest_i32` by hand. The costs are all at compile time: more code emitted (code bloat if you misuse generics with dozens of types) and longer compiles. At runtime, generic code is free.

This is the direct contrast to generics in Java/C# (type erasure + boxing) and dynamic languages — and it's why Rust's `Vec<T>` is as fast as a C array of structs.

### You've been using generics all along

`Option<T>` (Module 008), `Result<T, E>` (Module 013), `Vec<T>` (Module 011), `HashMap<K, V>` (Module 012) — all generic types. And the bound on `T` in `largest` is the same idea as the `Hash + Eq` requirement you met on `HashMap` keys: the type system checking, at compile time, that the operation you need is actually available.

## Common Pitfalls

- **Forgetting the bound.** Writing `fn largest<T>(...)` and comparing with `>` fails to compile. Fix: add `T: PartialOrd` (or the trait you actually need).
- **`impl<T> Foo<T>` with the wrong parameter set.** Methods on `Pair<T, U>` must be in `impl<T, U> Pair<T, U>`; writing `impl<T> Pair<T, U>` doesn't compile (unbound `U`).
- **Expecting a generic `Vec<T>` to be one type.** `Vec<i32>` and `Vec<String>` are different types; you can't push a `String` into a `Vec<i32>` — and a function taking `&Vec<i32>` won't accept `&Vec<String>`. Fix: make the function generic.
- **Assuming generics cost performance.** Monomorphized generics are as fast as hand-written concrete code. The "overhead" of generics is compile time, not runtime.
- **Type annotation in turbofish when inference fails.** `Maybe::Nothing` has no `T` to infer — the compiler needs `Maybe::<i32>::Nothing` or a type-annotated binding.

## Key Terms

- **Type parameter:** the placeholder (`T`, `U`, ...) a generic item is written against.
- **Trait bound:** `T: SomeTrait`, restricting which types the generic may be instantiated with.
- **Monomorphization:** the compiler generating one specialized copy of generic code per concrete type used.
- **Zero-cost abstraction:** a feature with no runtime overhead versus a hand-written equivalent; monomorphization is the canonical example.
- **Turbofish:** `::<T>` syntax for specifying a generic type argument explicitly (`Vec::<i32>::new()`).
- **Concrete type:** the result of filling in all type parameters (`Vec<i32>`, `Option<String>`).

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-015)` bodies:

1. `largest<T: PartialOrd>` — return the biggest element, or `None`.
2. `first_or<'a, T>` — first element or the fallback (the `'a` is a lifetime, Module 018).
3. `Pair::first` / `Pair::second` / `Pair::swap` — accessors and the type-swapping method.
4. `Maybe::is_just` / `Maybe::unwrap_or` — matching on the generic enum.
5. `combine<T>` — concatenate two `Vec<T>`s.

The tests in `tests/module_015.rs` define "done":

```bash
cargo test -p module-015-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 10.1 — Generic Data Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [Rust Reference — generic parameters](https://doc.rust-lang.org/reference/items/generics.html)
- [std::option::Option — a generic enum you already know](https://doc.rust-lang.org/std/option/enum.Option.html)
- [Rust Performance Book — generics and monomorphization](https://nnethercote.github.io/perf-book/)
