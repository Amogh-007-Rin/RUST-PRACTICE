# Module 022: Iterators I — The Iterator Trait

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Module 021 (closures), Modules 015–016 (generics, traits)

## Learning Objectives

- Read and implement the `Iterator` trait (`type Item` + `next()`).
- Drive an iterator by hand with `next()` and explain why it returns `Option<Item>`.
- Explain what a `for` loop desugars to (`IntoIterator` + `next()` until `None`).
- Write your own bounded and infinite iterators.

## Why This Matters

Iterators are the backbone of every serious Rust codebase: collections, file reads, `std::io::BufRead::lines()`, `tokio::fs::read_dir`, serde's streaming deserialization — all of them expose `Iterator` or a close cousin. More importantly, *you* will write iterators: any time you have a custom collection and want `for x in my_collection` to work, you implement the trait. And a clear mental model of `next()` is the prerequisite for the combinator toolbox in Module 023.

## Concept

### The trait

`Iterator` is a trait with one required method and one associated type:

```rust
pub trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}
```

That's the whole contract. An iterator is a *stateful* value: every call to `next()` advances it by one step and returns the current item, or `None` once it is exhausted. `type Item` (associated types are covered properly in Module 025 — for now, read it as "this iterator's output type").

`Option` is the key design decision: it lets you express both "here's the next value" (`Some(x)`) and "there are no more values" (`None`) in one return type. The caller can *never* call `next()` on a finished iterator and get a surprise.

### Driving an iterator by hand

An array's `.into_iter()` gives you an iterator over its elements:

```rust
fn main() {
    let mut numbers = vec![1, 2, 3].into_iter();
    assert_eq!(numbers.next(), Some(1));
    assert_eq!(numbers.next(), Some(2));
    assert_eq!(numbers.next(), Some(3));
    assert_eq!(numbers.next(), None);
    assert_eq!(numbers.next(), None);
}
```

Note the last two lines: once `None` is returned, the iterator is exhausted and keeps returning `None`. Also note the type changes: `next()` takes `&mut self`, so the iterator must be declared `mut`.

### What `for` actually does

The `for` loop is sugar. This:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let mut total = 0;
    for x in &v {
        total += x;
    }
    assert_eq!(total, 60);
}
```

is expanded by the compiler into roughly this:

```rust
fn main() {
    let v = vec![10, 20, 30];
    let mut it = v.iter();
    let mut total = 0;
    while let Some(x) = it.next() {
        total += x;
    }
    assert_eq!(total, 60);
}
```

Two details worth noticing. First, `for x in &v` calls `.iter()` behind the scenes: the `for` loop actually works on `IntoIterator` — anything that can be converted into an iterator. `&Vec<T>`, `&[T]`, `&String`, `HashMap`, ranges, all implement it. Second, `x` here is `&i32`, not `i32` — iterating over a slice *yields references*, because yielding owned values would require copying them out of the slice. That's why the loop body does `total += x` (adding through the reference) rather than `total += *x` — although if you write `for &x in v` you destructure the reference and get an `i32`.

### Writing your own iterator

Implementing `Iterator` for your own type is the core skill of this module. Here's a bounded one — it counts from `start` to `end` inclusive:

```rust
pub struct Step {
    current: i64,
    end: i64,
}

impl Step {
    pub fn new(start: i64, end: i64) -> Self {
        Self { current: start, end }
    }
}

impl Iterator for Step {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        if self.current > self.end {
            return None;
        }
        let value = self.current;
        self.current += 1;
        Some(value)
    }
}

fn main() {
    assert_eq!(Step::new(1, 3).collect::<Vec<_>>(), vec![1, 2, 3]);
}
```

Trace the state across calls — this is the anatomy of *every* iterator:

```
State of Step::new(1, 3) across calls to next():

  call          current    end     action                          return
  --------------------------------------------------------------------------
  before        1          3       (initial state)                 --
  1st next()    1          3       return 1, then current -> 2     Some(1)
  2nd next()    2          3       return 2, then current -> 3     Some(2)
  3rd next()    3          3       return 3, then current -> 4     Some(3)
  4th next()    4          3       current > end, stop             None
  5th next()    4          3       already stopped                 None
```

`collect::<Vec<_>>()` — you'll meet it in earnest in Module 023 — just calls `next()` repeatedly and gathers the results. Because your type implements `Iterator`, it gets every standard method for free: `take`, `sum`, `filter`, `map`, and a hundred more.

### Infinite iterators

`next()` returns `None` only when *you* decide the sequence is over. If you never return `None`, you have an infinite iterator — which is perfectly legal, as long as consumers bound it somehow:

```rust
fn main() {
    let first_five = (0..).take(5).collect::<Vec<_>>();
    assert_eq!(first_five, vec![0, 1, 2, 3, 4]);
}
```

`(0..)` is an endless range; `.take(5)` wraps it in an iterator that passes through the first five items and then reports `None`. This is the pattern behind the `Fibonacci` type you'll implement in the exercise: a math sequence with no end, made safe purely by how it's *consumed*.

### A broken-looking but instructive snippet

This will not compile: `Step` has no way to know its own length, so calling `.len()` on it fails. `len()` is a method of the `ExactSizeIterator` trait, which `Step` deliberately does not implement:

```rust,ignore
let steps = Step::new(1, 100);
let _ = steps.len(); // error: no method named `len` found
```

The fix is to consume it (`steps.count()` walks to the end) or implement `ExactSizeIterator` — which is almost never what you want for a custom iterator, precisely because most sequences aren't pre-computable.

## Common Pitfalls

- **Forgetting that `for x in &v` yields `&T`, not `T`.** If the body needs an owned value, write `for &x in v` (works for `Copy` types) or `for x in v.into_iter()` if you own the collection.
- **Calling `.collect()` on an infinite iterator.** It loops forever. Always bound with `.take(n)` first.
- **Returning `None` before the sequence is truly done.** Once your `next()` returns `None`, callers are allowed to assume the iterator is exhausted; a `Step` that yielded everything except the final element breaks that contract.
- **Declaring `let mut it` but never mutating.** `next()` takes `&mut self`; without `mut`, the compiler refuses. Conversely, if you never call `next()` directly, drop the `mut`.
- **Implementing `Iterator` when you just need a method named `next`.** Only implement the trait when you want `for` loops and the combinator methods; clippy will flag a method called `next` on a type that isn't an iterator (`should_implement_trait`).

## Key Terms

- **iterator:** a stateful value that produces items one at a time via `next()`.
- **`next()`:** the one required method; returns `Some(item)` while items remain, `None` once exhausted.
- **`type Item`:** the associated type declaring what kind of value the iterator yields.
- **`IntoIterator`:** the trait `for` loops consume; `.iter()`/`.into_iter()` convert collections into iterators.
- **exhausted:** the state of an iterator that has returned `None`; it stays that way forever.
- **infinite iterator:** an iterator whose `next()` never returns `None`, safe only when consumers bound it with `take`/`step_by`/etc.

## Exercise

In `exercises/`, two custom iterators and two functions are stubbed out. Make the tests in `tests/module_022.rs` pass by filling in each `TODO(module-022)`:

1. `Step::next` — a bounded counter. Check the `current > end` condition first, then yield and advance. The test `step_next_driven_by_hand` drives it manually.
2. `Fibonacci::next` — an infinite sequence. Update the `(a, b)` pair with `(a, b) = (b, a + b)` after yielding `a`.
3. `sum_evens` — practice the `for` loop form over a slice.
4. `first_greater` — practice the manual `next()` form in a loop.

The two iterator structs already have `new()` implemented — only `next()` is missing. Run `cargo test -p module-022-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 13.2: Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [std docs: `std::iter::Iterator`](https://doc.rust-lang.org/std/iter/trait.Iterator.html)
- [std docs: `std::iter::IntoIterator`](https://doc.rust-lang.org/std/iter/trait.IntoIterator.html)
