# Module 093: Algorithms in Rust — Sorting, Searching & Two-Pointer Patterns

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 90–120 min
**Prerequisites:** Module 091 (lists, stacks & queues), Module 092 (trees, heaps & hash maps), Module 022–023 (iterators), Module 015 (generics)

## Learning Objectives

- You will be able to implement merge sort and quick sort in Rust — one immutable and recursion-first (`merge_sort(&[T]) -> Vec<T>`), one in-place with a partition step (`quick_sort(&mut [T])`) — and derive their complexity from first principles.
- You will be able to write binary search by hand, including the `mid = lo + (hi - lo) / 2` form that never overflows.
- You will be able to recognize two-pointer patterns and apply them to `two-sum`-on-sorted-input and container-with-most-water in O(n).
- You will be able to explain when an idiomatic iterator chain beats a hand-rolled index loop in Rust, and when it can't.

## Why This Matters

Algorithms are the filter in most whiteboard interviews — and Rust changes how you answer them in a way that *helps* you: the compiler catches the off-by-ones your interviewer is silently testing for, slice bounds make "walk the middle half" a one-liner, and `T: Ord + Clone` makes you say out loud whether your sort is stable, in-place, or both. The two-pointer pattern you'll practice here is the same one behind `std::Vec::dedup`, HTTP range requests, and database merge joins — and the deterministic-testing habit (LCG instead of `rand`) is exactly how serious projects keep tests reproducible.

## Concept

### Complexity, before anything else

Interviewers do not ask you to recite Big-O; they ask you to *derive* it. Get comfortable with the reasoning, not the table:

| Algorithm | Best | Average | Worst | Space | Stable | In-place |
|---|---|---|---|---|---|---|
| merge sort | O(n log n) | O(n log n) | O(n log n) | O(n) | yes | no |
| quick sort | O(n log n) | O(n log n) | O(n²) | O(log n) stack | no | yes |
| binary search | O(1) | O(log n) | O(log n) | O(1) | — | — |
| two-pointer | O(n) | O(n) | O(n) | O(1) | — | — |

Merge sort's O(n log n) comes from the halving: each of the log₂ n levels does O(n) total work merging. Quick sort's O(n²) worst case comes from an adversarial pivot: partition on an already-sorted input with a last-element pivot gives one empty side every time, so you get n levels of O(n) each. Say that sentence in an interview and you've answered the "when does quicksort degrade?" question. (The standard mitigations — pick the middle element, or the median-of-three — are also worth knowing.)

### Merge sort: divide, recurse, merge

```
[ 38 | 27 | 43 | 3 | 9 | 82 | 10 ]
            split into halves
[ 38 | 27 | 43 | 3 ]        [ 9 | 82 | 10 ]
   split again        →        split again
[ 38 27 ] [ 43 3 ]          [ 9 82 ] [ 10 ]
   ... until single elements (trivially sorted) ...
[ 27 38 ] [ 3 43 ]          [ 9 82 ] [ 10 ]
        merge               merge
[ 3 27 38 43 ]              [ 9 10 82 ]
            merge
[ 3 | 9 | 10 | 27 | 38 | 43 | 82 ]     ← sorted
```

The *merge* step is the algorithm; the recursion is just bookkeeping. Two sorted halves, walk both with an index, always emit the smaller element:

```rust
fn merge<T: Ord + Clone>(left: &[T], right: &[T]) -> Vec<T> {
    let mut out = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            out.push(left[i].clone());
            i += 1;
        } else {
            out.push(right[j].clone());
            j += 1;
        }
    }
    out.extend_from_slice(&left[i..]); // one of these drains
    out.extend_from_slice(&right[j..]);
    out
}
```

Note the type signature: `&[T] -> Vec<T>` means the function *returns* a new vector instead of mutating — that's the Rust-flavored version of merge sort, and it composes beautifully with `clone_from_slice`, `split_at`, and the rest of the slice API. `<=` on the left makes the merge stable: equal elements keep their original order.

### Quick sort: partition, then recurse

Quick sort is the opposite deal: the work happens in `partition`, and the recursion is the payoff. Lomuto's scheme: take the last element as pivot, scan the rest, swapping anything smaller into the "store" region:

```rust
fn partition<T: Ord>(slice: &mut [T]) -> usize {
    let pivot = slice.len() - 1;
    let mut store = 0;
    for i in 0..pivot {
        if slice[i] < slice[pivot] {
            slice.swap(i, store);
            store += 1;
        }
    }
    slice.swap(store, pivot); // pivot is now in its final position
    store
}

fn quick_sort<T: Ord>(slice: &mut [T]) {
    if slice.len() <= 1 {
        return;
    }
    let pivot = partition(slice);
    quick_sort(&mut slice[..pivot]);
    quick_sort(&mut slice[pivot + 1..]);
}
```

```
pivot = 5 (last element)                    after partition
[ 3 | 9 | 1 | 4 | 5 ]                       [ 3 | 1 | 4 | 5 | 9 ]
        scan → store swaps                       ▲ pivot in place
  3 < 5 → swap(3,3)                          everything left  < 5
  9 ≥ 5 → skip                                everything right ≥ 5
  1 < 5 → swap(9,1)  →  [ 3 | 1 | 9 | 4 | 5 ]  recurse on both sides
  4 < 5 → swap(9,4)  →  [ 3 | 1 | 4 | 9 | 5 ]
  final: swap(pivot into store) → [ 3 | 1 | 4 | 5 | 9 ]
```

`&mut [T]` in, nothing out: slicing `&mut slice[..pivot]` gives the compiler disjoint halves for free, so the two recursive calls are legal borrows. This — "the type system proves my recursion doesn't alias" — is a genuinely good thing to point at in an interview.

### Binary search: the shape of correctness

Binary search is where off-by-ones live. Two rules keep it boring: make `hi` *exclusive*, and compute the middle without overflow:

```rust
fn binary_search<T: Ord>(slice: &[T], target: &T) -> Option<usize> {
    let mut lo = 0;
    let mut hi = slice.len(); // exclusive upper bound
    while lo < hi {
        let mid = lo + (hi - lo) / 2; // never overflows, unlike (lo+hi)/2
        match slice[mid].cmp(target) {
            std::cmp::Ordering::Less => lo = mid + 1,
            std::cmp::Ordering::Greater => hi = mid,
            std::cmp::Ordering::Equal => return Some(mid),
        }
    }
    None
}
```

The loop ends when `lo == hi`, and because `hi` is exclusive, every slice you examine is `slice[lo..hi]` — a half-open range, the same convention as Rust's own slicing. `match slice[mid].cmp(target)` is the idiomatic Rust form; the `Less`/`Greater`/`Equal` arms *are* the branch logic. (Production code calls `slice.binary_search()` instead, which returns `Result` and tells you the insertion point — a useful "this is the std version" detail to mention.)

### Two pointers: one pass, two indices

The pattern: a sorted slice, one index at each end, and a rule for which one moves. For `two_sum` on sorted input:

```rust
fn two_sum_sorted(slice: &[i64], target: i64) -> Option<(usize, usize)> {
    let mut lo = 0;
    let mut hi = slice.len().checked_sub(1)?;
    while lo < hi {
        let sum = slice[lo] + slice[hi];
        if sum == target {
            return Some((lo, hi));
        } else if sum < target {
            lo += 1; // sum too small: the only way up is a bigger left element
        } else {
            hi -= 1; // sum too big: the only way down is a smaller right element
        }
    }
    None
}
```

The invariant is the whole proof: because the slice is sorted, moving `lo` up can only increase the sum and moving `hi` down can only decrease it — so any pair that sums to `target` will be found, and each index visits each position once, O(n) total. Container-with-most-water uses the same dance with a twist: the area is `width × min(heights)`, so moving the *taller* bar can never help (the width shrinks and the min is still capped), so you always move the shorter one.

### The idiomatic-Rust layer: iterators vs. index loops

Hand-written index loops are the right tool for the two-pointer family — the whole point is that both ends move by *rules*. But where the algorithm is sequential, the iterator version is shorter and just as fast (iterators don't allocate, and they let the compiler optimize). Classic example: comparing every element with its neighbor used to mean an index loop with an off-by-one; today it's `windows(2)`:

```rust
// Idiomatic Rust: `windows(2)` turns "compare each element to its
// predecessor" into a one-liner — no indices, no off-by-ones.
fn is_sorted<T: Ord>(slice: &[T]) -> bool {
    slice.windows(2).all(|pair| pair[0] <= pair[1])
}

fn main() {
    assert!(is_sorted(&[1, 2, 2, 3]));
    assert!(!is_sorted(&[1, 3, 2]));
    assert!(is_sorted::<i32>(&[])); // vacuously true
}
```

The mental rule: **index loops when the indices move independently, iterators when you process a stream of elements**. If you ever write `for i in 0..slice.len() { slice[i] ... }`, ask yourself whether `slice.iter()` (or `.windows(2)`, `.chunks`, `.zip`) says it better. Interviewers who know Rust specifically listen for that.

### Testing without `rand`

The exercise tests use a linear congruential generator — a few lines of arithmetic that produce a deterministic pseudo-random-looking sequence:

```rust
fn lcg_values(seed: u64, count: usize) -> Vec<i64> {
    let mut state = seed;
    (0..count)
        .map(|_| {
            state = state
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (state >> 33) as i64
        })
        .collect()
}
```

Same seed → same sequence, on every machine, forever. A failing test with a printed seed is reproducible; `rand`-generated tests are not, which is why real projects pin their property tests or print seeds.

## Common Pitfalls

- **`(lo + hi) / 2` overflowing.** With large arrays the addition can wrap. Always `lo + (hi - lo) / 2`.
- **Inclusive `hi` bounds.** They produce the classic infinite-loop binary search. Use exclusive `hi` and `lo < hi` — the slice-style convention.
- **Quick sort recursion on the pivot.** Recurse into `[..pivot]` and `[pivot+1..]`, not into ranges that still contain the pivot — otherwise sorted input never terminates.
- **Cloning instead of borrowing in `merge`.** `Vec<T>` for a `T: Clone` is the right call here, but merge logic over `&T` references avoids the clone entirely when you only need to *compare*.
- **Forgetting that Rust's `sort` is the answer.** In production, `slice.sort()` (stable, adaptive) beats anything you hand-roll. The exercise is about understanding, not about replacing `std`.

## Key Terms

- **Stable sort:** equal elements keep their relative order (merge sort yes, Lomuto quick sort no).
- **In-place algorithm:** uses only O(1) extra space beyond the input (quick sort with slice recursion uses O(log n) stack, so it's "sort of").
- **Partition:** the step that puts the pivot in its final position with everything smaller to its left.
- **Half-open range:** `[lo, hi)` — `lo` included, `hi` excluded; the convention that makes binary search and slicing compose.
- **LCG:** linear congruential generator — a tiny deterministic pseudo-random source for reproducible tests.

## Exercise

In `exercises/`, implement the five public functions plus the two helpers:

1. `merge_sort` + `merge` — the recursion and the merge step.
2. `quick_sort` + `partition` — Lomuto partition, then recursion excluding the pivot.
3. `binary_search` — half-open bounds, `cmp`-based dispatch.
4. `two_sum_sorted` — the two-pointer classic.
5. `max_water` — two pointers where the *shorter* bar always moves.

Run `cargo test -p module-093-exercises` until green. The tests cross-check your implementations against `std` sorting and against brute-force reference solutions on deterministic LCG inputs — the same "check against a slow oracle" pattern used in property testing. Then compare with `solutions/` and keep clippy clean.

## Further Reading

- [The Rust Book, Chapter 13 — Iterator Adaptors and `collect` (the idiomatic half of this module)](https://doc.rust-lang.org/book/ch13-02-iterators.html)
- [The Rust Performance Book — iterators vs. loops and zero-cost abstraction](https://perf.rust-lang.org/)
- [Big-O Cheat Sheet — a reference for the complexity of every structure in Modules 091–093](https://www.bigocheatsheet.com/)
- [The classic "container with most water" — LeetCode #11, the canonical two-pointer problem](https://leetcode.com/problems/container-with-most-water/)
