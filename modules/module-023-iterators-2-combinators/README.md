# Module 023: Iterators II — Combinators

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Module 022 (the `Iterator` trait and `next()`)

## Learning Objectives

- Build pipelines with `map`, `filter`, `collect`, and `fold`.
- Use `any`/`all`, `zip`, `count`, `sum`, and `max_by_key` on iterators.
- Read and reason about chained iterator expressions left to right.
- Know when a chain compiles to zero overhead (lazy adapters).

## Why This Matters

Iterator chains are Rust's default answer to "transform this collection." In real codebases you'll see them everywhere — parsing log lines, folding request metrics, mapping API responses — because they're declarative (you say *what*, not *how*), allocation-conscious, and almost always the fastest way to express the transformation. Being fluent here is also the prerequisite for the async streams of Block E, which reuse the same combinator vocabulary.

## Concept

Module 022 taught you that an iterator is a lazy state machine behind `next()`. The key word is **lazy**: combinators like `map` and `filter` do *not* run when you chain them — they just build up a description. The work happens only when something *drives* the iterator: `collect`, `sum`, `for`, or a manual `next()`.

### The big four: `map`, `filter`, `collect`, `fold`

`map` transforms each item; `filter` keeps only the items a predicate accepts; `collect` drives the iterator and turns the results into a collection; `fold` reduces the whole sequence to a single value by carrying state along:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let squares = v.iter().map(|&x| x * x).collect::<Vec<_>>();
    assert_eq!(squares, vec![1, 4, 9, 16, 25]);

    let evens = v.iter().filter(|&&x| x % 2 == 0).collect::<Vec<_>>();
    assert_eq!(evens, vec![2, 4]);

    let total = v.iter().fold(0, |acc, &x| acc + x);
    assert_eq!(total, 15);
}
```

Read a chain left to right: "take `v`'s elements, keep the even ones, square them, gather the results." Note `filter` receives `&&i32` — iterating a slice yields `&i32`, and `filter` hands its predicate a *reference to the item*, so the closure pattern is `|&&x|`. `fold` is the general-purpose state-carrier: the first argument is the initial state (`0`), the closure receives `(state, item)` and returns the new state.

### Terminal methods: they drive the iterator

`collect` is one of several **terminal** operations — the ones that actually consume the chain. Others you'll meet constantly:

```rust
fn main() {
    let words = "the quickest brown fox";

    assert_eq!(words.split_whitespace().count(), 4);
    assert_eq!(words.split_whitespace().max_by_key(|w| w.len()), Some("quickest"));

    assert!(words.split_whitespace().any(|w| w.contains('b')));
    assert!(!words.split_whitespace().all(|w| w.len() > 3));
    assert_eq!(words.split_whitespace().sum::<usize>(), 16);
}
```

- `count()` — how many items the iterator yields.
- `max_by_key(f)` / `min_by_key(f)` — the item whose key is largest/smallest. Note it returns the **last** maximum; it's not a stable-first tiebreaker.
- `any(pred)` — does *at least one* item satisfy `pred`? Short-circuits.
- `all(pred)` — does *every* item satisfy `pred`? Short-circuits. Vacuously true on an empty iterator.
- `sum()` / `product()` — requires the item type to implement the corresponding `Sum`/`Product` trait; the accumulator type comes from context, which is why `sum::<usize>()` spells it out above.

### Combining iterators: `zip`

`zip` walks two iterators in lockstep, yielding pairs, and stops when the shorter one ends. This makes `dot_product` a one-liner:

```rust
fn main() {
    let a = [1, 2, 3];
    let b = [4, 5, 6];
    let dot: i64 = a.iter().zip(b).map(|(&x, &y)| x as i64 * y as i64).sum();
    assert_eq!(dot, 32);
}
```

A step-by-step trace of what `zip` + `map` produce:

```
 a.iter()    b        zip yields        map yields
 -------------------------------------------------
   &1         4       (&1, &4)          1*4 = 4
   &2         5       (&2, &5)          2*5 = 10
   &3         6       (&3, &6)          3*6 = 18
   (done)    (done)   None              stop
                                          sum = 32
```

`zip` silently truncates to the shorter input — that's a feature ("stop at the end of the shared prefix") and a hazard (unexpected truncation) depending on your goal, which is why `dot_product` in the exercise checks lengths explicitly.

### Why laziness is a feature, not a delay

Each adapter in a chain is a small struct wrapping the previous iterator; nothing materializes intermediate vectors. This chain — *filter even, then square* — produces zero allocations and walks the slice exactly once:

```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5, 6];
    let result: Vec<i64> = v
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x as i64 * x as i64)
        .collect();
    assert_eq!(result, vec![4, 16, 36]);
}
```

There is no `Vec` of evens sitting in memory between `filter` and `map` — that's the same "zero-cost abstraction" idea you'll formalize in Module 056. The discipline it creates: reach for `collect` only when you genuinely need an owned collection at the end.

### Where combinators meet `fold`

`fold` is the escape hatch for anything `map`/`filter`/`sum` can't express in one operation — counting by predicate, building a string, tracking two pieces of state at once:

```rust
fn main() {
    let readings = [3, -5, 7, -2];
    let (positives, negatives) =
        readings.iter().fold((0, 0), |(p, n), &x| if x > 0 { (p + 1, n) } else { (p, n + 1) });
    assert_eq!((positives, negatives), (2, 2));
}
```

If your fold's closure is literally `|acc, x| acc + x`, clippy will (correctly) suggest `sum()` — reach for `fold` when the state transformation is doing real work.

### A broken chain

This will not compile: `map` is lazy, so the closure's result type is fixed when the chain is *defined*, and the `x * x` closure produces `i32` while the annotated `collect` wants `Vec<i64>`:

```rust,ignore
let v = vec![1, 2, 3];
let squares: Vec<i64> = v.iter().map(|x| x * x).collect(); // mismatched types: i32 vs i64
```

The fix is either an explicit cast (`|&x| x as i64 * x as i64`) or matching the annotation to the actual type (`Vec<i32>`). The compiler will never invent the cast for you — type mismatches in chains are a steady source of beginner confusion, and the fix is to be explicit about the accumulator/output type.

## Common Pitfalls

- **Forgetting `collect::<Vec<_>>()` (or a type annotation).** `collect` needs to know *what* to build; give it a turbofish or annotate the binding.
- **Using `collect` mid-chain.** You don't need intermediate `Vec`s — chains are lazy; let one terminal `collect` end the pipeline.
- **`max_by_key` returns the *last* maximum on ties.** If you need the first, iterate in reverse or use `fold` with an explicit tie rule.
- **`all` is vacuously true on an empty iterator.** `[].iter().all(|_| false)` is `true` — a classic off-by-one in validation logic.
- **Writing plain `|acc, x| acc + x` folds.** Clippy flags them (`unnecessary_fold`); `sum()`/`product()`/`count()` express intent better and are what the standard library does internally anyway.
- **`zip` truncating silently.** When lengths must match, check `len()` first (as `dot_product` does) instead of discovering truncation in your output.

## Key Terms

- **adapter / combinator:** a lazy method on an iterator that returns a new iterator (`map`, `filter`, `zip`, `take`).
- **terminal method:** a method that consumes the iterator and produces a concrete result (`collect`, `sum`, `count`, `fold`, `any`, `all`).
- **lazy:** an adapter does no work until something drives the iterator; chains are descriptions, not computations.
- **predicate:** a closure returning `bool`, used by `filter`, `any`, `all`.
- **`collect`:** drives the iterator and builds an owned collection; the target type must be annotatable or inferable.
- **turbofish:** the `::<T>` syntax that names a type parameter at a call site, e.g. `collect::<Vec<_>>()`.

## Exercise

In `exercises/`, nine functions are stubbed out, each exercising one or two combinators. Fill in each `TODO(module-023)`:

1. `squares_of_evens` — `filter` + `map` + `collect`.
2. `sum_of_squares` — `map` + `sum` (mind the `i64` cast).
3. `count_words` / `count_short_words` — `split_whitespace` + `count`, with a `filter` in the second.
4. `longest_word` — `max_by_key` (watch the `Option`).
5. `count_positive` — a `fold` whose closure does real work.
6. `dot_product` — length check, then `zip` + `map` + `sum`.
7. `contains_any` / `is_all_even` — `any` and `all`.

Run `cargo test -p module-023-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 13.2: Processing a Series of Items with Iterators](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [std docs: `std::iter::Iterator` (method list)](https://doc.rust-lang.org/std/iter/trait.Iterator.html#implemented-methods)
- [Rust by Example: Iterator::fold](https://doc.rust-lang.org/rust-by-example/fn/closures/closure_examples.html)
