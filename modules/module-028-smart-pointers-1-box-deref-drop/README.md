# Module 028: Smart Pointers I — `Box<T>`, `Deref`, `Drop`

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Modules 004–005 (ownership, borrowing), 018 (lifetimes), 026 (trait objects)

## Learning Objectives

- Explain what `Box<T>` buys you: heap allocation, indirection, and fixed size for recursive types.
- Implement `Deref`/`DerefMut` and use deref coercion in your own smart-pointer types.
- Write `Drop` impls and reason about when and how often they run.
- Read the layout of pointer-wrapped values in memory.

## Why This Matters

`Box<T>`, `Deref`, and `Drop` are the substrate under every other smart pointer in the language — `Rc`, `Arc`, `MutexGuard` (Modules 029, 032) are all "a wrapper with `Deref` to the inner value and a `Drop` that does the cleanup." `Box` itself is everywhere: recursive data structures, trait objects (`Box<dyn Trait>`), and the "too big for the stack" escape hatch. Understanding these three traits once means understanding every pointer type in the standard library from then on.

## Concept

### `Box<T>`: a pointer, owned

A `Box<T>` is a single-owner pointer to a `T` allocated on the heap. You construct it with `Box::new(value)`, dereference it with `*`, and it *owns* its target: when the `Box` is dropped, the heap value is freed. There is no manual `free` — the "smart" part is that drop does it automatically (that's the `Drop` half of this module, below):

```rust
fn main() {
    let b = Box::new(42);
    assert_eq!(*b, 42);
    assert_eq!(*b + 1, 43);
}
```

Why heap-allocate a number the stack could hold? The point is **indirection**: a `Box` is a fixed-size pointer regardless of what it points at. That's what makes recursive types legal. This will not compile — `Expr::Add` would need to contain two complete `Expr` values, and their total size is unbounded (each `Add` nests two more `Add`s...):

```rust,ignore
enum Expr {
    Num(i64),
    Add(Expr, Expr), // error: recursive type `Expr` has infinite size
}
```

The fix is exactly one word: wrap the children in `Box`, so `Add` holds two *pointers* to heap-allocated subtrees:

```rust
enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
    }
}

fn main() {
    let expr = Expr::Add(
        Box::new(Expr::Num(2)),
        Box::new(Expr::Mul(Box::new(Expr::Num(3)), Box::new(Expr::Num(4)))),
    );
    assert_eq!(eval(&expr), 14);
}
```

Here is the layout that makes the recursion finite — the enum's size is fixed at "largest variant" (`Add` = two pointers + a tag), and every child lives on the heap:

```
Stack (expr is 24 bytes, fixed size)          Heap (variable depth lives here)

  ┌──────────────────────────┐
  │ tag: Add                 │
  │ lhs: •──────────┐        │             ┌──────────────┐
  │ rhs: •───────┐  │        │             │ tag: Num     │
  └──────────────┼──┼────────┘             │ value: 2     │
                 │  │                      └──────────────┘
                 │  └───────────────────┐
                 │                      │  ┌──────────────┐
                 └───────────────────┐  │  │ tag: Mul     │
                                    │  │  │ lhs: •─────┐ │    ┌──────────────┐
                                    │  │  │ rhs: •───┐ │ │    │ tag: Num     │
                                    │  │  └──────────┼─┼─┼───>│ value: 3     │
                                    │  │             │ │ │    └──────────────┘
                                    │  │             │ └─┼────┐
                                    │  │             │   │    └────>  tag: Num(4)
                                    │  │             │   │          (heap, not drawn)
                                    └──┼─────────────┘   │
                                       └─────────────────┘
  Each Box<Expr> is a single pointer; the recursion is bounded because
  the enum stores pointers, never nested values.
```

The recursive descent in `eval` follows the pointers: `Add` evaluates its left and right subtrees and combines them — this is the same pattern you'll use for every recursive data structure.

### `Deref`: making `*` work on your type

`Deref` is the trait behind the `*` operator for your own types. `Box<T>` implements it so `*boxed` gives you the `T` inside. `String` implements `Deref<Target = str>`, `Vec<T>` implements `Deref<Target = [T]>` — which is why `&String` can be used where `&str` is expected. Here's a minimal `Box`-like type implementing it:

```rust
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn main() {
    let b = MyBox::new(String::from("hello"));
    assert_eq!(b.len(), 5);   // b.len() resolves through deref coercion
    assert_eq!(*b, "hello");  // *b gives the String
}
```

Two behaviors come free with `Deref`. First, `*b` desugars to `*(b.deref())`, so `*b` yields a `&T` → `T` for `Copy` types, or a reference for everything else. Second — the star of the show — **deref coercion**: when the compiler finds a method on `T` that `MyBox<T>` doesn't have (`len`, `starts_with`), it silently dereferences until it finds one. `b.len()` works because `&MyBox<String>` coerces to `&String`, then to `&str`. This is why `&String` works where `&str` is expected, why `&Vec<T>` works where `&[T]` is expected, and why your smart pointers feel transparent.

### `DerefMut`: making `*` mutable

`DerefMut` extends the same idea to mutable contexts. Implement it and `*ptr += ...`, `*ptr.push(...)`, and mutable coercion all work:

```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(v: T) -> Self {
        MyBox(v)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

fn main() {
    let mut b = MyBox::new(10);
    *b += 5;
    assert_eq!(*b, 15);
}
```

The rule of thumb: implement `Deref` when your type is "a wrapper around one thing"; implement `DerefMut` only if mutating through it is semantically sound. (`Deref` to a shared inner value, then `DerefMut`, gives you interior-mutability-flavored bugs if you aren't careful — see Module 029.)

### `Drop`: cleanup on the way out

`Drop` is where the "smart" in smart pointer lives. The trait has one method, `drop(&mut self)`, and Rust calls it automatically when the value goes out of scope — you never call it yourself (`std::mem::drop` is just "take ownership and drop immediately"). Here's the lifecycle observed with a counter:

```rust
use std::sync::atomic::{AtomicU32, Ordering};

static GADGETS_ALIVE: AtomicU32 = AtomicU32::new(0);

struct Gadget;

impl Gadget {
    fn new() -> Self {
        GADGETS_ALIVE.fetch_add(1, Ordering::Relaxed);
        Gadget
    }
}

impl Drop for Gadget {
    fn drop(&mut self) {
        GADGETS_ALIVE.fetch_sub(1, Ordering::Relaxed);
    }
}

fn main() {
    assert_eq!(GADGETS_ALIVE.load(Ordering::Relaxed), 0);
    {
        let _g = Gadget::new();
        assert_eq!(GADGETS_ALIVE.load(Ordering::Relaxed), 1);
    }
    assert_eq!(GADGETS_ALIVE.load(Ordering::Relaxed), 0);
}
```

Step by step:

```
  line                          GADGETS_ALIVE   what happens
  -------------------------------------------------------------
  before the block              0
  Gadget::new()                 1               constructor runs
  end of block                  0               Drop runs automatically:
                                                _g leaves scope -> drop(_g)
```

`Box<T>`'s own `Drop` frees the heap allocation; `MutexGuard`'s `Drop` (Module 032) unlocks the mutex; your `LogTimer` from Module 027 printed its elapsed time. In every case the guarantee is the same: **cleanup runs exactly once, at the end of the owner's scope — even when a panic unwinds.**

## Common Pitfalls

- **Moving out of a `Box`.** `let s = *b;` where the content isn't `Copy` is a compile error ("cannot move out of dereference of `Box<String>`") — you can't extract ownership through a pointer. Clone, destructure, or redesign.
- **Forgetting `*` and fighting reference-vs-value confusion.** `*b` yields the inner value; `b` itself is the pointer. Method calls mostly hide this via deref coercion, but arithmetic and `match` don't.
- **Implementing `Deref` without `DerefMut` and wondering why mutation fails.** `*b += 1` needs `DerefMut`. Implement both when mutating through the wrapper is intended.
- **Overflowing the stack with recursive types.** If the compiler says "recursive type has infinite size," you forgot the `Box` indirection somewhere in the cycle.
- **Expecting `Drop` to be called more than once.** It runs exactly once per value, at scope end. Calling `mem::drop` early just moves that moment earlier; the value can't be dropped twice (the compiler enforces it via ownership).

## Key Terms

- **smart pointer:** an owned wrapper around a pointer (`Box`, `Rc`, ...) that manages its target's lifetime.
- **indirection:** storing a pointer instead of the value, breaking size recursion and sharing layouts.
- **`Deref` / `DerefMut`:** the traits behind `*` and deref coercion; `Target` names the pointed-to type.
- **deref coercion:** the compiler auto-dereferencing `&Wrapper<T>` to `&T` to find methods/values.
- **`Drop`:** the trait whose `drop(&mut self)` runs automatically at scope exit.

## Exercise

In `exercises/`, three things are stubbed out. Fill in each `TODO(module-028)`:

1. `eval` — recursive evaluation of the `Box`-based `Expr` tree.
2. `Deref`/`DerefMut` for `MyBox` — return `&self.0` / `&mut self.0`; watch `*b` and `s.len()` start working.
3. `Gadget::drop` — increment `DROPPED_GADGETS` with `fetch_add(1, Ordering::Relaxed)`. (You'll also need to import `Ordering`.)

Run `cargo test -p module-028-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 15: Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [std docs: `std::boxed::Box`](https://doc.rust-lang.org/std/boxed/struct.Box.html)
- [std docs: `std::ops::Deref`](https://doc.rust-lang.org/std/ops/trait.Deref.html)
