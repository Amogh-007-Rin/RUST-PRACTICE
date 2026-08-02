# Module 017: Traits II — Bounds, `where` Clauses & `dyn Trait`

**Block:** Block B — Foundations II
**Estimated time:** 60–90 min
**Prerequisites:** Module 015 (generics), Module 016 (defining and implementing traits)

## Learning Objectives

- Constrain generic parameters with trait bounds, both inline (`T: Area`) and with `where` clauses.
- Write functions that compute over any type implementing a trait, using bound methods inside the body.
- Compare static dispatch (generics — one monomorphized copy per type) with dynamic dispatch (`dyn Trait` — one function, runtime dispatch).
- Build heterogeneous collections (`Vec<&dyn Area>`) that generics cannot express.
- Explain why `dyn Trait` requires a reference (or `Box`) and a known "object-safe" trait.

## Why This Matters

Bounds are how you tell the compiler "I don't care what the type is, as long as it can do *this*" — the single most common pattern in library code, from `Vec::sort` (`T: Ord`) to axum's extractors (`FromRequest`) to serde's serializers (`Serialize`). And `dyn Trait` is the escape hatch when generics can't express what you need: a collection holding *different* types, or a trait object stored behind `Box` or `Arc` (Module 028). Every real codebase mixes both; knowing which one a signature uses — and why — is a skill interviewers probe directly.

## Concept

### Revisiting bounds

Module 016 ended with `fn announce<T: Greeter>(greeter: &T)`. The bound `T: Greeter` does three jobs at once:

1. It *permits* the body to call `Greeter` methods on `T`.
2. It *restricts* the function to types that implement `Greeter`.
3. It *monomorphizes*: one compiled copy per concrete type (Module 015).

```rust
trait Area {
    fn area(&self) -> f64;

    fn describe(&self) -> String {
        format!("area = {:.2}", self.area())
    }
}

fn largest_area<T: Area>(shapes: &[T]) -> f64 {
    shapes.iter().map(Area::area).fold(0.0, f64::max)
}
```

`shapes: &[T]` — note this is a slice of *the same* `T`. All elements must be the same concrete type (`[Rectangle; 3]`, `[Circle; 3]`). This is the price of static dispatch, and it's a feature: the compiler can lay out the slice contiguously and call `area` without any indirection.

### `where` clauses

When bounds pile up (or get long), Rust offers the `where` clause form — identical meaning, better readability:

```rust
fn summarize_shapes<T, U>(shapes: &[T], labels: &[U]) -> Vec<String>
where
    T: Area,
    U: Area + std::fmt::Display,
{
    shapes
        .iter()
        .zip(labels)
        .map(|(shape, label)| format!("{label}: {}", shape.describe()))
        .collect()
}
```

The bound `U: Area + std::fmt::Display` is a **bound combination** — `U` must implement both traits. `where` is especially useful with multiple parameters and complicated return types, and it's what the standard library uses in all its complex signatures. For one simple bound, the inline form is idiomatic; for several, `where`.

### Inside the body: bounds unlock methods

The bound isn't just a filter — it's a *capability grant*. Inside `largest_area`, the compiler knows `T: Area`, so `shape.area()` compiles. Remove the bound and the body breaks:

```rust,ignore
fn largest_area_no_bound<T>(shapes: &[T]) -> f64 {
    // ERROR: the method `area` exists for type `T`, but its trait bounds
    // were not satisfied: `T: Area` — the compiler cannot call `area()`
    // without the bound.
    shapes.iter().map(|shape| shape.area()).sum()
}
```

The fix is the bound. This is the exact error you'll meet constantly with generic code: "the method exists but the trait bound is not satisfied" — the compiler's way of saying "add the bound."

### `dyn Trait`: when the type is not known

Generics require the concrete type at compile time. But some problems genuinely need a *runtime* choice of type: a list of shapes where each element may be a different kind. The type is unknown when the function is compiled, so monomorphization is impossible. Rust's answer is a **trait object**: `dyn Area`, always behind a reference (or `Box`/`Arc`).

```rust
fn total_area_mixed(shapes: &[&dyn Area]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

let circle = Circle { radius: 1.0 };
let rect = Rectangle { width: 4.0, height: 2.0 };
let mixed: Vec<&dyn Area> = vec![&circle, &rect];
assert_eq!(total_area_mixed(&mixed), std::f64::consts::PI + 8.0);
```

A `&dyn Area` is a **fat pointer**: two words — a pointer to the data plus a pointer to a *vtable* (a table of the trait's method addresses for that concrete type). When you call `shape.area()`, the compiler emits an indirect call through the vtable — **dynamic dispatch**:

```
static dispatch (generics)               dynamic dispatch (dyn Trait)
largest_area::<Rectangle>                total_area_mixed
+--------------------------------+       +--------------------------------+
| copy specialized for Rectangle |       |  shapes: [ &dyn Area, &dyn Area ]
| .area()  ->  direct call       |       |              |           |
| Rectangle::area at compile time|       |              v           v
+--------------------------------+       |        +--------+    +--------+
largest_area::<Circle>                   |        | Circle |    | Rect   |
+--------------------------------+       |        | vtable |    | vtable |
| copy specialized for Circle    |       |        +--------+    +--------+
| .area()  ->  direct call       |       |  .area() -> indirect call via
+--------------------------------+       |              vtable (runtime)
```

- **Static dispatch:** N concrete types → N specialized copies → zero runtime overhead (Module 015's monomorphization).
- **Dynamic dispatch:** 1 function → runtime vtable lookup per call → small, predictable overhead, plus a pointer-width cost per `dyn` value.

Why the reference? A `dyn Area` has no known size at compile time (a `Circle` and a `Rectangle` differ in size), so it can only live behind a pointer: `&dyn Area`, `&mut dyn Area`, or an owning `Box<dyn Area>` (Module 028). The "no known size" point is exactly why `fn f(shape: dyn Area)` doesn't compile, and why `dyn Trait` doesn't work for every trait — only **object-safe** ones: no generic methods, no `Self` in arguments/return (you'll see the details in Module 026).

### Choosing between them

| Need | Tool |
|---|---|
| One function, one concrete type per call | Generic bound `T: Area` (static dispatch) |
| Performance-critical, few types | Generic bound |
| A collection of *different* types with a shared trait | `&dyn Area` / `Vec<&dyn Area>` |
| Store an unknown type in a struct field | `Box<dyn Trait>` (Module 028) |
| "Any error type" return values | `Box<dyn Error>` (Module 014) |

The default is generics. Reach for `dyn` when the concrete type is genuinely unknown at compile time — heterogeneous collections, plugin-style APIs, storing handlers.

## Common Pitfalls

- **Writing a heterogeneous slice with generics.** `[Circle, Rectangle]` as one array doesn't compile. Fix: `Vec<&dyn Area>` — or keep the slice homogeneous.
- **Forgetting the reference on `dyn`.** `fn f(s: &[dyn Area])` errors ("the size for values of type `dyn Area` cannot be known"). Fix: `&[&dyn Area]` (or `&[Box<dyn Area>]` later).
- **Method-call error: "trait bounds were not satisfied".** The body calls `area()` but `T` has no bound. Fix: add `T: Area` (inline or `where`).
- **Assuming `&dyn Area` implements `Area`.** It doesn't, directly — but you can call methods *through* it. If a generic bound needs it, `dyn Area` (unsized) works where sized types do not; keep trait objects behind pointers.
- **`where` vs inline confusion.** They're the same thing — pick inline for one short bound, `where` for several or long ones.

## Key Terms

- **Trait bound (`T: Trait`):** a generic constraint permitting trait methods and restricting concrete types.
- **Bound combination (`T: TraitA + TraitB`):** multiple required traits, separated by `+`.
- **`where` clause:** the after-signature form of bounds; identical semantics, better formatting for complex cases.
- **Static dispatch:** compile-time specialization of generic code per concrete type (monomorphization).
- **Dynamic dispatch:** runtime method selection through a vtable, used by `dyn Trait`.
- **Trait object (`dyn Trait`):** an unsized value implementing a trait, usable only behind a pointer.
- **Fat pointer:** a reference to a `dyn` value: data pointer + vtable pointer.
- **Vtable:** the per-type table of method pointers a trait object dispatches through.
- **Object safety:** the set of restrictions a trait must satisfy to be usable as `dyn Trait`.

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-017)` bodies:

1. `Area` impls for `Circle`, `Rectangle`, `Triangle` (default `describe()` inherited).
2. `largest_area` — a `where`-clause bound, map + `fold(0.0, f64::max)`.
3. `total_area` — inline bound, map + `sum`.
4. `biggest` — `max_by` comparing two areas.
5. `total_area_mixed` / `describe_shapes` — iterate `&dyn Area` collections.

The tests in `tests/module_017.rs` define "done":

```bash
cargo test -p module-017-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 10.2 — Trait Bounds; 17.2 — Trait Objects](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rust Book, Chapter 17.2 — Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Rust Reference — trait and lifetime bounds](https://doc.rust-lang.org/reference/trait-bounds.html)
- [Rust Performance Book — trait objects and dynamic dispatch costs](https://nnethercote.github.io/perf-book/trait-objects.html)
