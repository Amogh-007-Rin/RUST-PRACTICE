# Module 011: Common Collections I — `Vec<T>`

**Block:** Block B — Foundations II
**Estimated time:** 45–90 min
**Prerequisites:** Module 004 (ownership), Module 005 (borrowing), Module 006 (slices & strings)

## Learning Objectives

- Create, grow, and shrink a `Vec<T>` with `new`, `push`, `pop`, `insert`, and `remove`.
- Read elements safely with indexing vs. `.get()`, and explain when each panics.
- Iterate over a `Vec` by reference, by mutable reference, and by value, and predict which one each of `&v`, `&mut v`, and `v` gives you.
- Explain how a `Vec` stores its data on the heap and what `capacity` means, including why repeated pushes are amortized O(1).
- Use slices (`&[T]`) as function parameters so your functions accept both `Vec<T>` and arrays.
- Apply common collection operations: `len`, `is_empty`, `contains`, `sort`, `retain`, and iterator-style accumulation (`sum`, `map`, `collect`).

## Why This Matters

`Vec<T>` is the single most-used collection in Rust — it's the default "growable array" behind almost every real codebase, from `axum` request body buffers to `serde_json`'s JSON arrays. If you're reading a `&[T]` in a function signature or collecting iterator results with `.collect()`, you are working with `Vec`'s slice vocabulary. Understanding how it grows, when it copies, and how borrowing rules apply to it is a prerequisite for every other collection in this block, for `HashMap` (Module 012), and for ownership questions in every interview.

## Concept

### What a `Vec` is

A `Vec<T>` is a growable, heap-allocated array of `T` values. The values live in a contiguous block of memory on the heap (contiguous means "one after the other, no gaps"), which is why indexing and iteration are so fast. `Vec` itself is a small three-field struct that lives wherever the binding is, and it *points* at that heap block:

```
binding: v               heap buffer (capacity 4)
+--------------+         +----------------------------+
| ptr  |----------->     | [ 10 ][ 20 ][ 30 ]  [    ] |
| len  | 3               +----------------------------+
| cap  | 4
+--------------+
```

- `ptr` is a pointer to the first element on the heap.
- `len` is how many elements are *live* right now.
- `cap` is how many elements the heap buffer could hold without reallocating.

This is why `Vec` is a "smart pointer": it owns its heap data and frees it automatically when the `Vec` is dropped (remember Module 004 — ownership means cleanup is deterministic).

### Creating a `Vec`

```rust
let mut numbers: Vec<i32> = Vec::new();   // empty, capacity 0
numbers.push(1);

let explicit = vec![1, 2, 3];             // the vec! macro, most common
let repeated = vec![0; 5];                // five zeroes
let from_array = Vec::from([10, 20, 30]);
```

`vec![...]` is just a macro that calls `Vec::new()` plus `push`es for you — nothing magical.

### Reading elements

Indexing with `[]` panics when the index is out of bounds; `.get()` returns an `Option` instead:

```rust
let v = vec![10, 20, 30];

assert_eq!(v[1], 20);                 // panics if index >= len
assert_eq!(v.get(1), Some(&20));      // returns Option<&T>, never panics
assert_eq!(v.get(99), None);
```

This snippet **will not compile** — you cannot write through an index that borrows the same `Vec` immutably:

```rust,ignore
let mut v = vec![1, 2, 3];
let first = &v[0];       // immutable borrow of v
v.push(4);               // ERROR: cannot borrow v as mutable while borrowed
println!("{first}");
```

The fix: take the value you need *before* mutating, or make the borrow very short-lived:

```rust
let mut v = vec![1, 2, 3];
let first = v[0];
v.push(4);
println!("{first}");
```

Copying out an `i32` is cheap; if the element were a `String`, you'd clone it (or better, hold the borrow until you're done mutating).

### Growing: capacity and reallocation

When you `push` and `len == cap`, the `Vec` must grow. It does so by allocating a *new, larger* heap buffer, copying every element into it, and freeing the old buffer:

```
push(10)      push(20)      push(30)      push(40)      push(50)
cap 4, len 0  cap 4, len 1  cap 4, len 2  cap 4, len 3  buffer FULL!

      heap: [10|  |  |  ]  [10|20|  |  ]  [10|20|30|  ]  [10|20|30|40]

push(50) -> allocate cap 8 buffer, copy 4 elements, free old buffer:
      heap: [10|20|30|40|50|  |  |  ]
```

Growth is not +1 each time — Rust doubles the capacity (the exact policy is an implementation detail, but it's multiplicative). Because each element is copied only O(log n) times over the lifetime of the `Vec`, *amortized* push cost is O(1): most pushes are a single write, and the rare expensive one pays for all the cheap ones that followed. That's the standard "dynamic array" strategy you've seen in any language — Rust's `Vec` is exactly that, with the layout control and safety guarantees Rust provides.

You can inspect and steer this:

```rust
let mut v: Vec<i32> = Vec::with_capacity(100); // one allocation up front
assert!(v.capacity() >= 100);
v.push(1);
assert_eq!(v.len(), 1);
```

### Iterating

There are three iteration modes, and the syntax mirrors borrowing (Module 005):

```rust
let mut v = vec![1, 2, 3];

for n in &v {
    // n: &i32 — immutable borrow, v still usable afterwards
}

for n in &mut v {
    *n *= 10; // n: &mut i32 — mutate elements in place
}

for n in v {
    // n: i32 — v is MOVED here, unusable afterwards
}
```

`for` loops over a `&Vec` desugar to `iter()`, and `iter()` yields `&T`. If you need an index too, use `v.iter().enumerate()`.

### Slices: the borrow of a `Vec`

A slice `&[T]` is a view into a contiguous run of elements — a pointer plus a length. You get one by borrowing a `Vec` (or array) and optionally taking a subrange:

```rust
fn sum(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

let v = vec![1, 2, 3, 4, 5];
assert_eq!(sum(&v), 15);          // whole Vec
assert_eq!(sum(&v[1..4]), 9);     // elements 1,2,3
assert_eq!(sum(&[7, 8]), 15);     // even a temporary array works
```

This is why you'll see `&[T]` in signatures everywhere: it accepts `Vec<T>`, arrays, and sub-ranges, and it doesn't move or clone anything.

### Common operations

```rust
let mut v = vec![3, 1, 2];
assert_eq!(v.len(), 3);
assert!(!v.is_empty());
assert!(v.contains(&2));

v.sort();                          // in-place
assert_eq!(v, vec![1, 2, 3]);

v.retain(|&n| !n.is_multiple_of(2)); // keep only odd numbers
assert_eq!(v, vec![1, 3]);

let last = v.pop();                // remove and return the end
assert_eq!(last, Some(3));

v.push(9);
v.insert(0, 7);                    // shift everything right
assert_eq!(v, vec![7, 1, 9]);
v.remove(1);                       // shift everything left
assert_eq!(v, vec![7, 9]);
```

`.retain` removes every element failing a predicate — the idiomatic "filter in place." `remove` shifts all later elements, so it's O(n); prefer `swap_remove` when order doesn't matter, and `pop` when you only need the end.

## Common Pitfalls

- **Indexing out of bounds panics.** `v[i]` on an index ≥ `len` crashes the program. Use `v.get(i)` when the index might be invalid, or check `len` first.
- **Iterating by value when you meant by reference.** `for n in v` moves the `Vec`; if you use `v` afterwards the compiler stops you. Write `for n in &v` to keep the `Vec` alive.
- **Calling `sort` on a borrow you don't own.** `v.sort()` needs `&mut v`; if you have `&Vec` (e.g. a function parameter), take a `&mut` or clone.
- **Expecting `len` to be capacity.** `len` is the live elements; pushing past `len` grows capacity silently. `Vec::with_capacity` doesn't change `len`.
- **Mutating while holding a borrow.** Taking `&v[i]` and then `v.push(...)` is a borrow-check error, not a runtime problem — copy the value out or scope the borrow.

## Key Terms

- **Collection:** a data structure that owns multiple values (in `std::collections` or `Vec`).
- **Heap buffer:** the contiguous block of memory a `Vec` points to and owns.
- **Capacity:** how many elements the buffer could hold; differs from `len`.
- **Reallocation / growth:** allocating a new buffer and copying elements when `len == cap`.
- **Amortized O(1):** the average cost of `push` over many pushes, despite rare expensive growth.
- **Slice (`&[T]`):** an immutable view into contiguous elements; borrows, doesn't own.
- **`vec!` macro:** shorthand for creating a `Vec` from literals.

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-011)` bodies:

1. `sum_even` — iterate and sum only even numbers.
2. `push_many` — push every element of a slice onto a `Vec`.
3. `median` — sort in place, then handle the odd/even length cases.
4. `remove_value` — find an element's position and `remove` it.
5. `word_lengths` — build a `Vec` with `map` + `collect`.
6. `mean` — return `None` for empty input, the average otherwise.

The tests in `tests/module_011.rs` define "done":

```bash
cargo test -p module-011-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 8.1 — Storing Lists of Values with Vectors](https://doc.rust-lang.org/book/ch08-01-vectors.html)
- [std::vec::Vec — the full standard library reference](https://doc.rust-lang.org/std/vec/struct.Vec.html)
- [std::slice — slicing and slice methods](https://doc.rust-lang.org/std/slice/index.html)
- [Rust Performance Book — Writing Faster Rust: vec growth and allocation tips](https://nnethercote.github.io/perf-book/collecting.html)
