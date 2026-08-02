# Module 012: Common Collections II — `HashMap<K, V>` & `HashSet<T>`

**Block:** Block B — Foundations II
**Estimated time:** 45–90 min
**Prerequisites:** Module 011 (`Vec<T>`), Module 008 (`Option<T>`)

## Learning Objectives

- Build and read `HashMap<K, V>` with `insert`, `get`, `entry`, and iteration.
- Explain why lookups are O(1) on average and why the map requires `K: Hash + Eq`.
- Use the `entry` API (`or_insert`, `or_default`) for the "increment a counter" pattern.
- Use `HashSet<T>` for membership tests, deduplication, and set operations (`&`, `|`, `-`).
- Choose between `Vec`, `HashMap`, and `HashSet` for a given problem, and know the memory/layout tradeoffs.

## Why This Matters

`HashMap` and `HashSet` are the workhorses of real programs: counting word frequencies in a log analyzer, grouping users by country, caching lookup results, deduplicating request IDs. Almost every web framework you'll meet later (axum, Actix, sqlx's row maps) surfaces hashed data structures in its request context and configuration handling. And because Rust's `HashMap` uses a *hash-builder* (randomized per-process), it resists denial-of-service attacks aimed at predictable hash maps — a real production concern that interviewers like to probe.

## Concept

### Hash tables in one paragraph

A `HashMap<K, V>` stores `(K, V)` pairs so that looking up `K` is O(1) on average. It works by hashing: a `Hash` function turns the key into a big number, and that number picks which *bucket* (slot) the pair lives in. When you `get("bob")`, the map re-hashes `"bob"`, jumps straight to the bucket, and finds the pair — no scanning. `HashSet<T>` is exactly the same machinery with only a key and no value: a `HashMap<T, ()>`.

```
keys                          buckets (array of slots)
"alice"  --hash--> 0x9f...  -> [ alice -> 3 ]
"bob"    --hash--> 0x21...  -> [ carol -> 7 ]
"carol"  --hash--> 0x21...  -> [ bob -> 9 ]   <- collision! resolved
                                              by chaining within the bucket
```

Collisions (two keys hashing to the same bucket) are handled transparently — the map keeps multiple entries per bucket and compares actual keys with `==` (that's the `Eq` requirement). Rust's `HashMap` uses SipHash by default, which is fast *and* cryptographically seeded per process, so attackers can't precompute colliding keys to degrade you to O(n).

### Creating and reading

```rust
use std::collections::HashMap;

let mut scores: HashMap<String, u32> = HashMap::new();
scores.insert("alice".to_string(), 3);
scores.insert("bob".to_string(), 9);

// get returns Option<&V> — never panics on a missing key
assert_eq!(scores.get("bob"), Some(&9));
assert_eq!(scores.get("nobody"), None);

// get_mut lets you modify in place
if let Some(score) = scores.get_mut("bob") {
    *score += 1;
}
assert_eq!(scores.get("bob"), Some(&10));

// iteration yields (&K, &V) pairs; ORDER IS NOT GUARANTEED
let total: u32 = scores.values().sum();
assert_eq!(total, 13);
```

`get` never panics — it returns `Option<&V>`. That's the hash map's version of `Vec::get` from Module 011, and it composes with `if let` and `match` from Module 008. `insert` overwrites and returns the *old* value as `Option<V>`.

### The entry API: the idiomatic counter

The most common real-world pattern is "increment a counter for a key". The naive way is awkward:

```rust
let mut counts = HashMap::new();
for word in ["a", "b", "a"] {
    let count = counts.entry(word.to_string()).or_insert(0);
    *count += 1;
}
assert_eq!(counts.get("a"), Some(&2));
```

`entry(key)` hands you a *vacant or occupied* slot; `or_insert(0)` returns a `&mut V`, inserting `0` first if the key was missing. One lookup, no double-hashing, and the `*count += 1` works because `count` is a mutable reference into the map. `or_default()` is the same with `V::default()`.

### Keys need `Hash + Eq`

Any type can be a key if it implements `Hash` and `Eq`:

```rust
use std::collections::HashMap;

let mut by_year: HashMap<u32, String> = HashMap::new();
by_year.insert(2015, "Rust 1.0".to_string());

let mut by_name: HashMap<String, u32> = HashMap::new();
by_name.insert("Ada".to_string(), 36);

let counts: HashMap<char, usize> = "hello".chars().fold(HashMap::new(), |mut m, c| {
    *m.entry(c).or_insert(0) += 1;
    m
});
assert_eq!(counts.get(&'l'), Some(&2));
```

All the primitive types, `String`, and any struct/enum with `#[derive(Hash, PartialEq, Eq)]` qualify — you'll meet `derive` for real in Module 016. Note that `f32`/`f64` are `Hash` but *not* `Eq` (NaN != NaN), so they can't be keys.

### `HashSet<T>`: membership and deduplication

```rust
use std::collections::HashSet;

let mut seen: HashSet<String> = HashSet::new();
assert!(seen.insert("bob".to_string()));   // true: was not present
assert!(!seen.insert("bob".to_string()));  // false: already present

assert!(seen.contains("bob"));
assert!(!seen.contains("alice"));
```

Set operations between two sets are supported with operators or methods:

```rust
use std::collections::HashSet;

let a: HashSet<i32> = [1, 2, 3, 4].into_iter().collect();
let b: HashSet<i32> = [3, 4, 5].into_iter().collect();

let common: HashSet<i32> = &a & &b;   // intersection: {3, 4}
let union: HashSet<i32> = &a | &b;    // union: {1,2,3,4,5}
let diff: HashSet<i32> = &a - &b;     // difference: {1, 2}

assert!(common.contains(&3) && !common.contains(&1));
assert_eq!(union.len(), 5);
assert_eq!(diff.len(), 2);
```

`&a & &b` reads as "intersection": the `&` before each name borrows the set so you don't move it, and the resulting `HashSet` is a new owned collection. This is the idiomatic Rust way to deduplicate, filter against a blocklist, or find the overlap between two groups.

### Choosing a collection

| Need | Use |
|---|---|
| Ordered, indexable sequence | `Vec<T>` (Module 011) |
| Look up a value by an arbitrary key | `HashMap<K, V>` |
| "Have I seen this before?" / dedupe | `HashSet<T>` |
| Count occurrences | `HashMap<K, usize>` + `entry` |
| Preserve insertion order | `Vec`, or `std::collections::BTreeMap` (ordered keys) |

Tradeoffs: `Vec` is contiguous, cache-friendly, and preserves order — but lookups are O(n). `HashMap` gives O(1) lookups at the cost of hashing, no ordering, and more memory per element (buckets + hash). `HashSet` is a `HashMap` with the values stripped out. For small n (say, under ~50 elements), a `Vec` scan is often faster than hashing in practice — but the *code* for `HashMap` stays O(1) at any scale.

## Common Pitfalls

- **Forgetting `.to_string()` on keys.** `insert("bob", 9)` with a `HashMap<String, u32>` doesn't compile — the key must be an owned `String`. Fix: `"bob".to_string()`, or use `HashMap<&str, u32>` when the strings live elsewhere.
- **Iterating and mutating the map at the same time.** `for (k, v) in &map` borrows the whole map; calling `insert` inside the loop is a borrow-check error. Fix: collect the changes into a `Vec` first, then apply.
- **Relying on iteration order.** HashMap iteration order is arbitrary and per-process randomized. If output must be deterministic, sort keys first or use `BTreeMap`.
- **`get` returning `&V` when you wanted to change it.** `get` gives an immutable borrow; use `get_mut` or the `entry` API to modify values.
- **Using `f32`/`f64` as keys.** They're not `Eq` (NaN != NaN); the compiler will reject it. Fix: use integers, strings, or a wrapper type with defined equality.

## Key Terms

- **Hash function:** maps a key to a large number that selects a bucket.
- **Bucket:** a slot in the map's internal array; collisions pile up here.
- **Collision:** two distinct keys landing in the same bucket, resolved by comparing keys with `Eq`.
- **Hash builder:** the per-process seeded hasher that makes Rust's map DoS-resistant.
- **Entry API:** `map.entry(k)` plus `or_insert`/`or_default`, the idiomatic "get or create" pattern.
- **Set operation:** `&` (intersection), `|` (union), `-` (difference) between two sets.

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-012)` bodies:

1. `build_scoreboard` — `zip` two slices and `.collect()` into a `HashMap`.
2. `word_frequencies` — count words with the `entry` API.
3. `top_scorer` — `max_by_key` over the map's entries.
4. `unique_words` — merge words from many texts into a `HashSet`.
5. `intersection` — keep only elements present in both sets.
6. `count_above` — count entries passing a predicate with `values()`.

The tests in `tests/module_012.rs` define "done":

```bash
cargo test -p module-012-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 8.3 — Storing Keys with Associated Values in Hash Maps](https://doc.rust-lang.org/book/ch08-03-hash-maps.html)
- [std::collections::HashMap — reference](https://doc.rust-lang.org/std/collections/struct.HashMap.html)
- [std::collections::HashSet — reference](https://doc.rust-lang.org/std/collections/struct.HashSet.html)
- [The Rust Book, Appendix B — the `Hash` and `Eq` trait requirements](https://doc.rust-lang.org/book/appendix-03-derivable-traits.html)
