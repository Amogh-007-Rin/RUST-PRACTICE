# Module 091: Data Structures in Rust I — Linked Lists, Stacks & Queues

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 60–90 min
**Prerequisites:** Module 028 (Smart Pointers I — `Box`), Module 029 (Smart Pointers II — `Rc`/`RefCell`), Module 011 (`Vec`), Module 019 (testing)

## Learning Objectives

- You will be able to implement a singly-linked list in Rust using `Box`, including the operations interviewers actually ask for: `push_front`, `push_back`, `pop_front`, `pop_back`, `peek`, iteration, and removal by index.
- You will be able to explain *why* linked lists are famously awkward in Rust, and articulate the three standard designs (owned `Box` chain, `Rc<RefCell>` chain, index-based arena) with their tradeoffs.
- You will be able to build a `Stack<T>` and a `Queue<T>` on top of your list and reason about where each is and isn't O(1).
- You will be able to predict what the borrow checker will reject when you write a naive list mutation, and apply the `take()`-then-rewire pattern to satisfy it.

## Why This Matters

Linked lists are the single most common "you claim to know Rust" trap in interviews: a C programmer's instinctive implementation doesn't typecheck, and the compiler errors teach you more about ownership than a month of reading about it. The pattern you'll internalize here — *move a node out of the structure with `Option::take()`, then rewire* — is the same pattern used everywhere in real code, from `std::collections::VecDeque` internals to lock-free queue implementations. Stacks and queues are also the backbone of almost every coding interview question you'll face in the next module block, so building them once by hand makes every later exercise faster.

## Concept

### The ownership problem, stated precisely

In C, a linked list node looks like this: `struct node { int value; struct node *next; }`. The pointer is a raw address — two nodes can point at each other, nodes can be freed and leave dangling pointers, and there is no guarantee the list is even acyclic. Rust's memory-safety rules forbid raw pointers by default, so you have to *encode the structure of your list in its types*. A singly-linked list is a linear ownership chain — each node owns the next one — and Rust's `Box<T>` expresses exactly that: a heap allocation with a single owner.

```
    stack                          heap
    ─────                          ────
    list: LinkedList<i32>
    ├─ head: Option<Box<Node>> ──► ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
    ├─ len: 3                      │ value: 10       │    │ value: 20       │    │ value: 30       │
                                   │ next: Some ─────┼───►│ next: Some ─────┼───►│ next: None      │
                                   └─────────────────┘    └─────────────────┘    └─────────────────┘
```

`head` is an `Option<Box<Node<T>>>`: `Option` because the list can be empty, `Box` because the node must live on the heap (the list can grow), and no pointer is ever dangling because when `head` is dropped, the first node is dropped, which drops its `next` box, and so on — a recursive teardown you get for free. There are no cycles here, so `Drop` terminates.

### Building the chain: the `take()`-then-rewire pattern

`push_front` is the operation that makes everything else click. The naive instinct — `self.head = Some(Box::new(Node { value, next: self.head }))` — actually compiles in this one case, because Rust reads the field before assigning it. But the *reliable* version is the pattern you'll use everywhere:

```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> Self {
        List { head: None }
    }

    fn push_front(&mut self, value: T) {
        let old_head = self.head.take(); // Option::take: swap in None, hand you the old value
        self.head = Some(Box::new(Node {
            value,
            next: old_head,
        }));
    }
}
```

`Option::take()` is the Swiss-army knife of data structures in Rust: it lets you move a value *out of* a structure you only have a mutable reference to, leaving `None` behind. Almost every "impossible" borrow-checker error in list code is solved by taking first and rewiring second.

### Why the borrow checker fights you: the deadlock

Removing a node from the middle requires touching `cur.next` twice at once — reading the node after it while repointing the link past it:

```rust,ignore
// This will NOT compile — E0499, "cannot borrow `*cur` as mutable more
// than once". The right-hand side needs `&mut cur.next` to reach the
// following node; the left-hand side needs it again to rewire the link.
fn unlink_after<T>(mut cur: &mut Node<T>) {
    cur.next = cur.next.as_mut().unwrap().next.take();
}
```

```
                  ┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
   cur ──────────►│ value: 20       │    │ value: 30       │    │ value: 40       │
                  │ next: Some ─────┼───►│ next: Some ─────┼───►│ next: None      │
                  └─────────────────┘    └─────────────────┘    └─────────────────┘
                                              ▲                        ▲
                                              │                        │
                         write: cur.next      │        read: cur.next (borrow #1)
                         (borrow #2)          └─────► next (borrow #2, through #1)
```

The compiler sees two overlapping mutable borrows of the same `Option` and refuses — not because the operation is unsound (the C version "works"), but because your intent is ambiguous: if the read and the write overlap, which one wins? The fix is to sequence them:

```rust
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

fn unlink_after<T>(cur: &mut Node<T>) -> Option<T> {
    let mut victim = cur.next.take()?; // borrow ends as soon as the value is moved out
    cur.next = victim.next.take();     // a fresh, non-overlapping borrow — fine
    Some(victim.value)
}
```

This is precisely what `pop_back` and `remove` do in the exercise: walk to the node *before* the one you want, `take()` the victim out, and splice its `next` back into the list. When you're implementing `remove(i)`, walk with a `&mut Option<Box<Node<T>>>` and use `as_mut()` at every step — the type of the walk variable itself keeps the borrows non-overlapping.

### The other two designs: `Rc<RefCell>` and the arena

`Box` gives you a strictly linear list. If you need *shared* ownership — two lists that share a tail, or nodes that point at their parents — you switch from "one owner" to "shared reference counts plus interior mutability":

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
struct Node {
    value: i32,
    next: Option<Rc<RefCell<Node>>>,
}

fn main() {
    let tail = Rc::new(RefCell::new(Node { value: 2, next: None }));
    let head = Rc::new(RefCell::new(Node {
        value: 1,
        next: Some(tail.clone()), // clone of the Rc: both "own" the tail
    }));

    // read through either handle
    assert_eq!(head.borrow().next.as_ref().unwrap().borrow().value, 2);
    assert_eq!(tail.borrow().value, 2);
}
```

This is the design you saw in Module 029, and it comes with two costs: every access goes through a runtime `borrow()` check, and reference cycles leak memory (an `Rc<RefCell<Node>>` graph with a cycle never drops). Interviewers will ask you to name exactly those two downsides.

The third design inverts the problem — instead of pointers, use indices:

```
arena: Vec<Node>
index:  0                  1                  2
        ┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
        │ value: 10       │ │ value: 20       │ │ value: 30       │
        │ next: Some(1)   │ │ next: Some(2)   │ │ next: None      │
        └─────────────────┘ └─────────────────┘ └─────────────────┘
                                                       ▲
   head = 0              no pointers at all —          │
   free list: 3, 4, ...  removal is just re-indexing ──┘
```

```rust
struct Arena {
    nodes: Vec<Node>,
}

struct Node {
    value: i32,
    next: Option<usize>,
}

fn main() {
    let mut arena = Arena { nodes: Vec::new() };
    arena.nodes.push(Node { value: 1, next: None });
    let tail = 0;
    arena.nodes.push(Node { value: 2, next: Some(tail) });
    assert_eq!(arena.nodes[1].next, Some(0));
}
```

An arena (also called an index-based list or "slot map") is one `Vec` plus a free list. There are no borrow-checker conflicts because there are no pointers — a "link" is just a number, and `arena.nodes[i].next` reads and writes freely. This is exactly how real systems implement intrusive structures: games store entity relations as index lists, databases store row ids, and Rust's own `slab` crate is an arena. The cost is that indices don't know if they're dangling — you must validate them, which is why `slab` returns `Option<&T>` on lookup.

### Stacks and queues

A **stack** is LIFO: push and pop at the same end. A **queue** is FIFO: push at one end, pop at the other. On a singly-linked list, the stack is free (both operations at the front are O(1)), while the queue costs O(n) for `enqueue` because appending at the back means walking the whole chain. That's why production Rust almost never uses a linked list for these — `VecDeque` is a ring buffer, so *both* ends are O(1):

```rust
use std::collections::VecDeque;

fn main() {
    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    assert_eq!(queue.pop_front(), Some(1)); // FIFO, O(1) on a ring buffer
    queue.push_front(0);                    // cheap at the front, too
    assert_eq!(queue.pop_front(), Some(0));
}
```

The rule of thumb: build the linked list by hand in exercises (it teaches ownership), but reach for `VecDeque` the moment you need a real stack or queue — a linked list's O(1) insertions are only a win when you're also splitting and splicing arbitrary positions, and that's exactly the case Rust's aliasing rules make hardest.

When an interviewer asks "implement a stack or a queue in Rust," the expected answer is: *implement the behavior and the API, then back it with `VecDeque` — and be able to say why.*

## Common Pitfalls

- **Storing `Option<Box<Node>>` where you meant a value.** `cur.next` moves the box, not a reference. If the compiler says "cannot move out of borrowed content," you forgot `as_ref()`/`as_deref()` or `as_mut()`.
- **Unwrapping while walking.** `cur.next.as_ref().unwrap()` panics the moment the chain is shorter than you think. Walk with `while let Some(next) = cur.next.as_deref()` and you never panic.
- **Forgetting `self.len -= 1`.** Every pop/remove must keep the cached length in sync — the length tests in this module catch exactly this.
- **Building a cycle with `Rc<RefCell>` and wondering why memory grows.** `Rc` cycles leak: nobody ever reaches refcount zero. If you need a doubly-linked list, ask yourself whether an arena or a `Vec` of pairs is actually simpler.
- **Reaching for `LinkedList` when you want a queue.** `std::collections::LinkedList` exists but is rarely the right tool; `VecDeque` is a ring buffer with O(1) on both ends and cache-friendly storage.

## Key Terms

- **`Option::take()`:** swaps `None` into the option and returns the old value — the standard way to move a value out of a structure through a `&mut` reference.
- **Arena:** a flat `Vec` of nodes where "links" are indices, not pointers; no aliasing rules, but no dangling-pointer detection either.
- **Ring buffer:** the internal layout of `VecDeque` — a fixed circular backing array where front and back push/pop are both O(1).
- **Free list:** the arena's index chain of deallocated slots, reused on allocation.
- **Cycle:** a chain of `Rc` links that points back on itself; refcounts never reach zero, so the memory is never freed.

## Exercise

In `exercises/`, the `LinkedList<T>` type is fully defined, but every interesting method is a stub. Fill in the `// TODO(module-091)` markers:

1. `push_front`, `push_back`, `pop_front`, `pop_back` — the four mutations; note which are O(1) and which walk the chain.
2. `peek_back`, `len`, `is_empty` — the read side.
3. `remove(index)` — the hardest one: walk to the node *before* the victim with `as_mut()`, then `take()`.
4. `Stack<T>` and `Queue<T>` — thin wrappers that delegate to the right end of your list.

Run `cargo test -p module-091-exercises` until green, then compare with `solutions/` and run `cargo clippy -p module-091-exercises -- -D warnings` on your finished code.

## Further Reading

- [The Rust Book, Chapter 15.1 — Using `Box<T>` to Point to Data on the Heap](https://doc.rust-lang.org/book/ch15-01-box.html)
- [The Rust Book, Chapter 15.4 — `Rc<T>` and the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [Learn Rust With Entirely Too Many Linked Lists — the definitive deep dive into exactly why this module's errors happen](https://rust-unofficial.github.io/too-many-lists/)
- [`std::collections::VecDeque` documentation](https://doc.rust-lang.org/std/collections/struct.VecDeque.html)
