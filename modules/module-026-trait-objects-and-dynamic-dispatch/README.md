# Module 026: Trait Objects & Dynamic Dispatch

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Module 025 (advanced traits), Module 017 (trait bounds), Module 021 (closures)

## Learning Objectives

- Write `dyn Trait` types and explain what a vtable is and where it lives.
- Distinguish static dispatch (generics, monomorphized) from dynamic dispatch (vtables, runtime lookup) and state the tradeoffs.
- Know the object-safety rules well enough to predict why a trait can't be `dyn`.
- Choose between `&dyn Trait`, `Box<dyn Trait>`, and generic bounds for a given design.

## Why This Matters

Trait objects are the "interface type" of Rust: a single type that can hold *any* implementor, which is exactly what plugin systems, event handlers, middleware layers (Block G), and UI widget trees need. But unlike Go interfaces or Java's `Object`, Rust makes you pay for polymorphism explicitly — `dyn` is a first-class concept with real costs and real rules. Understanding those rules is what separates "I made it compile" from "I can design APIs", and the object-safety restrictions you'll learn here are also the reason Rust has alternative patterns (closures, generics, trait hierarchies) that other languages don't need.

## Concept

### Two ways to be polymorphic

Rust has two dispatch mechanisms. **Static dispatch** happens at compile time: a generic function is *monomorphized* — duplicated for every concrete type it's called with. **Dynamic dispatch** happens at runtime: a pointer to the value is paired with a **vtable**, a table of function pointers for that type's implementation of the trait, and every trait method call goes through it.

```rust
trait Sound {
    fn make(&self) -> String;
}

struct Dog;
struct Cat;

impl Sound for Dog {
    fn make(&self) -> String {
        "woof".into()
    }
}

impl Sound for Cat {
    fn make(&self) -> String {
        "meow".into()
    }
}

fn main() {
    let animals: Vec<&dyn Sound> = vec![&Dog, &Cat];
    let sounds: Vec<String> = animals.iter().map(|a| a.make()).collect();
    assert_eq!(sounds, vec!["woof", "meow"]);
}
```

The `Vec<&dyn Sound>` is the tell: `Dog` and `Cat` have different sizes, so the vector stores *fat pointers* — each element is two words: the address of the value, plus the address of that type's `Sound` vtable. Let's look at the layout:

```
Memory behind Vec<&dyn Sound> — each element is a fat pointer (2 words):

  element 0                        element 1
  +----------------+--------+      +----------------+--------+
  | data ptr ----->| Dog    |      | data ptr ----->| Cat    |
  +----------------+--------+      +----------------+--------+
  | vtable ptr --->| make   |      | vtable ptr --->| make   |
                   | (fn ptr)|                     | (fn ptr)|
                   +--------+                     +--------+

  a.make()  ->  load vtable ptr  ->  indirect call through it
```

Without `dyn` you cannot mix types in one collection. A generic function `fn all_sounds<T: Sound>(xs: &[T])` requires all elements to be the same `T` — the compiler must know every type at compile time to monomorphize. `dyn` trades that guarantee for flexibility.

### The tradeoff table

```
                              static dispatch (generics)        dynamic dispatch (dyn Trait)
---------------------------------------------------------------------------------------------
when it happens               compile time (monomorphization)    runtime (vtable lookup)
code size                     one copy per concrete type         one copy of the code
collection types              homogeneous only                   heterogeneous allowed
method calls                  direct, inlinable                  indirect call, not inlinable
errors                        caught at compile time             caught at compile time too
```

In practice the performance difference is tiny (one indirect jump vs. a direct one), which is why real codebases mix both freely: generics in hot inner loops, `dyn` at API boundaries where heterogeneity is the point.

### `&dyn`, `Box<dyn>`, and ownership

`&dyn Trait` borrows an existing value. If the collection should *own* its elements, box them — `Box<dyn Trait>` is a fat pointer owning the heap-allocated value:

```rust
trait Shape {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(Circle { radius: 2.0 })];
    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    assert!((total - std::f64::consts::PI * 4.0).abs() < 1e-9);
}
```

Three ways to hold a trait object, by ownership flavor:

```
  &dyn Trait      borrows an existing value      "let me use this shape"
  Box<dyn Trait>  owns a heap value              "let me keep this shape"
  Rc<dyn Trait>   shares a heap value            "let us all share this shape"  (Module 029)
```

### Object safety: the rules that make `dyn` legal

Not every trait can be turned into a `dyn` type. The compiler enforces **object safety**; a trait is object-safe if and only if:

1. **No generic methods.** A method like `fn convert<T>(&self)` can't be monomorphized — the vtable would need a function pointer for every possible `T`.
2. **No returning `Self` by value.** `fn clone(&self) -> Self` is the classic offender: the vtable would have to know the concrete type's size, which the caller of `dyn Clone` doesn't have.
3. **No `Self` in parameter position** (except `self`/`&self`/`&mut self`).

This will not compile — `from_side` returns `Self`:

```rust,ignore
trait BadShape {
    fn area(&self) -> f64;
    fn from_side(side: f64) -> Self; // error: the trait `BadShape` cannot
}                                    // be made into an object
```

The fix is to keep constructors out of the trait (put `from_side` on the concrete types) or in an associated type-free trait:

```rust
trait GoodShape {
    fn area(&self) -> f64;
}

struct Triangle {
    base: f64,
    height: f64,
}

impl GoodShape for Triangle {
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

fn main() {
    let shapes: Vec<&dyn GoodShape> = vec![&Triangle { base: 2.0, height: 3.0 }];
    assert!((shapes[0].area() - 3.0).abs() < 1e-9);
}
```

The error message names the offending method, and the compiler's suggestion (`consider removing the method or adding `where Self: Sized`) is the standard workaround: `fn from_side(side: f64) -> Self where Self: Sized;` makes the method unavailable on `dyn` types but keeps the trait object-safe.

### Why `dyn` needs no lifetime annotations here

`&dyn Shape` — one lifetime, elided, tied to the data pointer, exactly like `&Shape`. `Box<dyn Shape>` owns its data, so it's `'static` by default. If you write a function returning `&dyn Shape` from a slice of them, you'll write the lifetime explicitly — as in the exercise's `largest_shape` — which is just ordinary reference lifetime elision rules from Module 018, with the vtable pointer traveling along.

## Common Pitfalls

- **Using generics when you mean heterogeneity.** If a collection must hold several implementors, generics won't compile — switch to `&dyn` or `Box<dyn>`. If you don't need heterogeneity, generics are cheaper and more flexible.
- **Forgetting `dyn`.** `&Shape` (bare trait name) is an old pre-2018 spelling and now an error ("trait objects must include the `dyn` keyword").
- **`Clone`-ing a `dyn` type.** `Box<dyn Clone>` is not object-safe; reach for `Box<dyn Any>` + downcasting or redesign (this is why `Rc<RefCell<...>>` and trait hierarchies exist).
- **Writing a generic method inside an object-safe trait by accident.** Every generic method makes the whole trait non-object-safe; move it to a helper trait or use `where Self: Sized`.
- **Measuring `dyn` as a performance problem before measuring.** One indirect call is rarely the bottleneck; choose `dyn` for the design, not the fear.

## Key Terms

- **trait object:** a value of type `dyn Trait` — a fat pointer pairing data with a vtable.
- **vtable:** the per-type table of function pointers backing a trait object.
- **fat pointer:** a two-word pointer (data + vtable) used for `dyn Trait`.
- **static dispatch:** compile-time resolution via monomorphization of generics.
- **dynamic dispatch:** runtime resolution through a vtable.
- **object safety:** the property (no generic methods, no `Self` by value/parameter) that makes a trait usable as `dyn`.
- **monomorphization:** the compiler generating one specialized copy of a generic function per concrete type.

## Exercise

In `exercises/`, the `Shape` trait and the `Circle`/`Square` structs are defined. Fill in each `TODO(module-026)`:

1. `Shape` impls for `Circle` and `Square` — the area formulas and names.
2. `total_area` — dynamic dispatch over `&[&dyn Shape]`.
3. `total_area_generic` — static dispatch with a `T: Shape` bound; compare the signatures.
4. `largest_shape` — return the biggest; `f64` isn't `Ord`, so compare with `total_cmp` inside `max_by`.

Run `cargo test -p module-026-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 17.2: Using Trait Objects That Allow for Values of Different Types](https://doc.rust-lang.org/book/ch17-02-trait-objects.html)
- [Rust Reference: Trait objects](https://doc.rust-lang.org/reference/types/trait-object.html)
- [Rust Blog: Dynamic dispatch in Rust (inside the fat pointer)](https://doc.rust-lang.org/std/keyword.dyn.html)
