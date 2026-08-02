# Module 092: Data Structures in Rust II — Trees, Heaps & Hash Maps

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 90–120 min
**Prerequisites:** Module 091 (linked lists, stacks & queues), Module 011 (`Vec`), Module 015 (generics), Module 012 (`HashMap` in std)

## Learning Objectives

- You will be able to implement a binary search tree from scratch in Rust — `insert`, `contains`, `min`/`max`, in-order traversal, and the hard part, `delete` — using the same `Option<Box<T>>` ownership chain you built in Module 091.
- You will be able to implement a min-heap on top of a plain `Vec` using index arithmetic instead of pointers, and write the two invariant-restoring routines `sift_up` and `sift_down` by hand.
- You will be able to implement an open-addressed hash map with linear probing, tombstones, and load-factor-triggered resizing, and explain why tombstones are necessary.
- You will be able to reason about the O-notation of each operation and connect each structure to its standard-library counterpart (`BTreeMap`, `BinaryHeap`, `HashMap`).

## Why This Matters

"Build a binary search tree" and "implement a heap" are the two most common data-structure questions in systems interviews, and Rust's ownership model makes them genuinely *better* interview answers: a tree in Rust is a linear ownership chain with no `free()` bookkeeping, and a heap in Rust is just a `Vec` with a discipline. Hash maps are the daily bread of production Rust (every `HashMap` in `std` and every database index you'll meet in Capstones 07 and 10 does exactly this under the hood), and understanding probing versus chaining will let you talk about real design tradeoffs instead of memorizing answers.

## Concept

### Binary search trees: ownership, recursive descent

A binary search tree (BST) is the linked list from Module 091 with two `next`s instead of one: each node holds a value and a left and right subtree, and the ordering invariant is that everything in `left` is strictly smaller than the node's value, everything in `right` strictly larger. In Rust you write that as the same owned chain, branched:

```rust
struct Node<T: Ord> {
    value: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

struct Tree<T: Ord> {
    root: Option<Box<Node<T>>>,
}

impl<T: Ord> Tree<T> {
    fn insert(&mut self, value: T) {
        insert_rec(&mut self.root, value);
    }
}

fn insert_rec<T: Ord>(node: &mut Option<Box<Node<T>>>, value: T) {
    match node {
        None => {
            *node = Some(Box::new(Node {
                value,
                left: None,
                right: None,
            }));
        }
        Some(n) => {
            if value < n.value {
                insert_rec(&mut n.left, value);
            } else if value > n.value {
                insert_rec(&mut n.right, value);
            }
            // equal: no-op, we don't store duplicates
        }
    }
}
```

The whole tree is one ownership chain: the root owns its children, which own theirs. Dropping the root drops the entire tree recursively — no `free()` loop, no memory leaks, no double-frees. Recursion does the walk for you: "insert into this subtree" is a function call, not a loop, and the borrow checker is happy because at every step you only hold *one* `&mut` into the tree at a time (remember the Module 091 deadlock — recursion sidesteps it entirely).

```
                    ┌───────────┐
                    │ value: 50 │
                    │ len: 7    │
                    └──┬─────┬──┘
          left  ┌──────┘     └──────┐  right
        ┌───────┴─────┐      ┌──────┴─────┐
        │ value: 30   │      │ value: 70  │
        └───┬─────┬──┘      └───┬─────┬──┘
     ┌──────┘     └───┐    ┌────┘     └───┐
  ┌──┴─────┐      ┌───┴──┐  ┌──┴────┐  ┌───┴───┐
  │ value: │      │value:│  │value: │  │value: │
  │ 20     │      │ 40   │  │ 60    │  │ 80    │
  └────────┘      └──────┘  └───────┘  └───────┘

  in-order traversal: 20, 30, 40, 50, 60, 70, 80   ← always sorted
```

**In-order traversal** — left subtree, node, right subtree — is the magic property: for any BST it visits values in ascending order. Interviewers use this as the instant correctness check, and your tests in this module do exactly that.

**Deletion** is the hard case. Three situations: a leaf (just remove it), a node with one child (splice the child up into its place), and a node with two children (replace it with its **in-order successor** — the smallest value in the right subtree — then prune that successor from the right subtree). The solution implements this with a `pop_leftmost` helper that extracts the successor and returns the pruned subtree; doing it by *returning* values rather than by pointer-surgery is what keeps the borrow checker calm.

The hidden caveat every interviewer waits for: an *unbalanced* BST degrades to O(n). Inserting `1, 2, 3, 4, 5` in order produces a chain. That's why production Rust uses `BTreeMap` (a self-balancing B-tree) and why `from_sorted` in the exercise builds a *balanced* tree by picking the middle element as the root.

### Heaps: pointers are just arithmetic

A binary heap is a complete binary tree that you never build as a tree at all. It lives in an array, and the structure is implicit: the children of index `i` are at `2i + 1` and `2i + 2`. A **min-heap** only maintains one invariant — every parent is `<=` its children — which means the smallest element is always at index 0.

```
array:  [ 3 | 5 | 9 | 17 | 12 | 11 | 20 ]      ← this is the WHOLE heap

heap view:                    index map
            3 (0)              0
          /      \            / \
        5 (1)    9 (2)        1   2
       /  \     /  \        / \ / \
     17(3) 12(4) 11(5) 20(6)  3 4 5 6
```

To `push`, append to the array and **sift up**: while the new element is smaller than its parent `(i-1)/2`, swap them. To `pop`, swap the root with the last element, remove the last, and **sift down**: while the element is larger than its smaller child, swap with that child.

```rust
struct MinHeap {
    data: Vec<i32>,
}

impl MinHeap {
    fn push(&mut self, value: i32) {
        self.data.push(value);
        let mut i = self.data.len() - 1;
        while i > 0 && self.data[i] < self.data[(i - 1) / 2] {
            self.data.swap(i, (i - 1) / 2);
            i = (i - 1) / 2;
        }
    }
}
```

Both operations are O(log n), `peek` is O(1), and building from a vector is O(n) if you sift down every non-leaf from the bottom up (that's `from_vec` in the exercise). Rust's `BinaryHeap` in std is a *max*-heap; wrapping it with `Reverse` flips it into a min-heap — a trick you'll use in interview solutions more times than you can count.

### Hash maps: open addressing, probing, and tombstones

A hash map is an array of slots plus a rule: the slot for a key is `hash(key) % capacity`. Two different keys can hash to the same slot — that's a collision. **Open addressing** resolves collisions by walking forward to the next free slot:

```
hash("cat") % 8 = 7     → insert "cat" at slot 7
hash("dog") % 8 = 7     → slot 7 taken, try 8 → insert "dog" at slot 8
hash("cow") % 8 = 8     → slot 8 taken, try 9 → insert "cow" at slot 9

slots:  0   1   2   3   4   5   6     7      8      9
      [ ∅ | ∅ | ∅ | ∅ | ∅ | ∅ | ∅ | "cat" | "dog" | "cow" ]
```

A lookup for "cow" starts at 8, checks 8, misses, checks 9, hits. **Removal is the trap**: if you blank out slot 8, the lookup for "cow" stops at the empty slot and reports "missing". The fix is a **tombstone** — a `Deleted` marker that probes keep walking past but new inserts may overwrite. And the second trap is **load factor**: as the table fills, probes get longer and longer. When more than ~70% of slots are occupied, you resize (double the capacity and rehash every live entry, dropping tombstones on the way).

```rust
fn find_slot(slots: &[Option<&str>], key: usize) -> Option<usize> {
    let capacity = slots.len();
    let mut idx = key % capacity;
    for _ in 0..capacity {
        match &slots[idx] {
            Some(value) if *value == format!("k{key}") => return Some(idx),
            None => return None,
            _ => idx = (idx + 1) % capacity, // keep probing, wrapping around
        }
    }
    None
}
```

Rust's own `HashMap` uses a more sophisticated open-addressing scheme (Robin Hood hashing with SIMD lookup), but the concepts are identical: hashing, probing, tombstones, resizing. Interview talking points you can now back with real knowledge: why `load factor < 1` for open addressing but chaining tolerates more; why resizing is O(n) amortized; why you never iterate a hash map expecting order.

## Common Pitfalls

- **Recursing without passing `&mut` down.** A recursive `insert` must take `&mut Option<Box<Node<T>>>` — take a plain `&Node`, and you'll hit "cannot borrow as mutable" the moment you try to build the tree.
- **Treating the heap as a sorted array.** Only the min is guaranteed at `data[0]`; the rest is heap-ordered, not sorted. `heap.pop()` repeatedly is O(n log n) sorting — that's exactly why it's useful, but you can't index into a heap expecting order.
- **Forgetting to sift after `push`.** Appending to the backing `Vec` without sifting breaks the invariant silently — the value at `data[0]` may no longer be the min.
- **Deleting a BST node with two children by just removing it.** You'd orphan the right subtree. Always splice in the in-order successor (or predecessor — either is correct, but pick one and be consistent).
- **No tombstone, then wondering why lookups fail after `remove`.** Blanking a slot breaks the probe chain for every key that hashed to a collision behind it.

## Key Terms

- **In-order successor:** the smallest value in the node's right subtree; the value that replaces a two-child node during BST deletion.
- **Sift up / sift down:** the two O(log n) routines that restore heap order after `push` and `pop`.
- **Open addressing:** storing entries directly in the table's slots, resolving collisions by probing to other slots.
- **Tombstone:** a `Deleted` slot marker that keeps probe chains intact after removal.
- **Load factor:** live entries ÷ slots; the trigger for resizing.

## Exercise

In `exercises/`, three structures are scaffolded with the tricky signatures already in place:

1. `BinarySearchTree<T>` — implement `insert`, `contains`, `min`/`max`, `in_order`, `from_sorted`, and the star of the show, `delete`, plus its helpers `insert_rec`, `delete_rec`, `pop_leftmost`.
2. `MinHeap<T>` — implement `push`/`pop`/`peek` via `sift_up`/`sift_down`, plus `from_vec`.
3. `HashMap<K, V>` — implement `insert`/`get`/`get_mut`/`remove`/`contains_key`/`entries` and `resize`, using linear probing and tombstones.

Run `cargo test -p module-092-exercises` until green, compare with `solutions/`, and keep `cargo clippy -p module-092-exercises -- -D warnings` clean. The tests are "property-ish" on purpose: BST tests check sorted traversal after every mutation, heap tests check pop order, and map tests check round-trips — the same shape as real fuzz-lite test suites.

## Further Reading

- [The Rust Book, Chapter 15 — Smart Pointers (recall the ownership chain you're using everywhere here)](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [`std::collections::BTreeMap` — the production balanced tree](https://doc.rust-lang.org/std/collections/struct.BTreeMap.html)
- [`std::collections::BinaryHeap` — max-heap, plus the `Reverse` wrapper for min-heaps](https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html)
- [Open addressing in Wikipedia's hash table article — probing strategies and load factors](https://en.wikipedia.org/wiki/Hash_table#Open_addressing)
