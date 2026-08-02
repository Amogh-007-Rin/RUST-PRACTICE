# Module 029: Smart Pointers II — `Rc<T>`, `RefCell<T>`, Interior Mutability

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Module 028 (`Box`, `Deref`, `Drop`), Modules 004–005 (ownership, borrowing), 018 (lifetimes)

## Learning Objectives

- Use `Rc<T>` for shared, read-only ownership and explain how the reference count works.
- Use `RefCell<T>` to mutate through an immutable reference (interior mutability) and state the cost.
- Explain the runtime borrow rules that make `RefCell` panic instead of failing to compile.
- Combine `Rc<RefCell<T>>` for shared, mutable state in single-threaded code.

## Why This Matters

Graph structures, observer registries, UI models, and caches all share one shape: *multiple owners of the same data, some of which need to mutate it*. `Rc<RefCell<T>>` is the single-threaded answer, and it's everywhere: it's the backing of most tree/graph crates, it appears in every `cell`-based cache, and its thread-safe cousins `Arc<Mutex<T>>` (Module 032) follow the exact same pattern with a different runtime mechanism. Understanding `Rc` + `RefCell` means understanding shared state itself — the pattern *and* its sharp edges.

## Concept

### `Rc<T>`: shared ownership, read-only

`Box<T>` gives you one owner. `Rc<T>` ("reference counting") gives you many. Each `Rc::clone(&rc)` creates a new handle onto the *same* allocation and bumps a counter; when a handle is dropped, the counter decrements; when it hits zero, the value is freed. It's read-only from the outside — `Rc` does not implement `DerefMut` — so aliasing is safe by construction:

```rust
use std::rc::Rc;

fn main() {
    let chat = Rc::new(String::from("rust"));
    let a = Rc::clone(&chat);
    let b = Rc::clone(&chat);
    assert_eq!(Rc::strong_count(&chat), 3);

    drop(a);
    assert_eq!(Rc::strong_count(&chat), 2);

    drop(b);
    assert_eq!(Rc::strong_count(&chat), 1);
}
```

The mental model — a shared heap allocation with a live count:

```
  Rc::new(...)        Rc::clone(&chat)        Rc::clone(&chat)
  ┌────────────┐      ┌────────────┐          ┌────────────┐
  │ chat       │      │ a          │          │ b          │
  └─────┬──────┘      └─────┬──────┘          └─────┬──────┘
        │                   │                      │
        └───────────┬───────┴──────────────────────┘
                    ▼
        ┌──────────────────────────┐
        │ Rc: refcount = 3         │   <-- one allocation,
        │ data: String "rust"      │       three handles
        └──────────────────────────┘

        drop(a): refcount -> 2     (allocation still alive)
        drop(b): refcount -> 1
        drop(chat): refcount -> 0  (data freed, last owner gone)
```

Two conventions matter. First, always write `Rc::clone(&x)` rather than `x.clone()` — the former makes the "I'm bumping a refcount, not deep-copying" intent visible. Second, `Rc` is single-threaded: it is neither `Send` nor `Sync` (details in Module 034), so the compiler refuses to share it across threads — which is exactly what you want, since the count isn't atomic.

### `RefCell<T>`: interior mutability with runtime checks

The borrow checker is compile-time; `RefCell` re-implements the same rules at runtime. `RefCell<T>` wraps a `T` and lets you borrow it *mutably even through an immutable reference* — the mutation happens "inside" the cell, hence **interior mutability**:

```rust
use std::cell::RefCell;

fn main() {
    let counter = RefCell::new(0);
    *counter.borrow_mut() += 1;
    *counter.borrow_mut() += 1;
    assert_eq!(*counter.borrow(), 2);
}
```

`counter` is not declared `mut`, yet its contents change. `borrow()` gives you a shared reference (`Ref<T>`), `borrow_mut()` an exclusive one (`RefMut<T>`). The rules are the same as the borrow checker — one `&mut` XOR any number of `&` — but enforced *at runtime*, with panics instead of compile errors:

```
  RefCell borrow state machine (runtime, not compile time):

  ┌────────────────┐    borrow()    ┌───────────────────┐
  │ 0 active       │───────────────>│ 1..N shared       │
  │ borrows        │<───────────────│ borrows active    │
  │ anything goes  │  drop Ref      └─────────┬─────────┘
  └───────┬────────┘                          │ borrow_mut()
          │ borrow_mut()                      ▼
          ▼                          PANIC: "already borrowed"
  ┌────────────────┐
  │ 1 exclusive    │    borrow() or borrow_mut()
  │ borrow active  │──────────────────────────────►  PANIC:
  └────────────────┘              "already mutably borrowed"
```

Step by step, when the panic happens:

```
  code                               state            result
  -------------------------------------------------------------
  let c = RefCell::new(42);          0 borrows        ok
  let r1 = c.borrow();               1 shared borrow  ok
  let r2 = c.borrow();               2 shared borrows ok (readers may coexist)
  let r3 = c.borrow_mut();           2 shared borrows PANIC: already borrowed
  drop(r1); drop(r2);                0 borrows        ok
  let r4 = c.borrow_mut();           1 exclusive      ok
  let r5 = c.borrow();               1 exclusive      PANIC: already mutably borrowed
```

This will not compile *as a unit*, but the failure mode you're training for is the runtime panic. Don't write this:

```rust,ignore
let cell = RefCell::new(42);
let borrow = cell.borrow();
let mut borrow_mut = cell.borrow_mut(); // panics: already borrowed
```

The fix is to keep borrows short — scope them so they end before the mutable one starts:

```rust
use std::cell::RefCell;

fn main() {
    let cell = RefCell::new(42);
    let value = { *cell.borrow() }; // borrow ends at the block's end
    assert_eq!(value, 42);

    *cell.borrow_mut() += 1; // fine now
    assert_eq!(*cell.borrow(), 43);
}
```

`RefCell` is a trade: it makes *legal* patterns compile that the borrow checker rejects (mutation through `&self`), at the cost of moving the check from compile time to runtime and adding a tiny per-access bookkeeping overhead. That's the correct trade for self-contained data structures, and the wrong one for hot paths where borrows would be held across long spans.

### `Rc<RefCell<T>>`: shared AND mutable

Together they're the workhorse of single-threaded shared state: `Rc` provides the shared ownership, `RefCell` provides the mutation, and `Deref` makes the combination transparent:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared = Rc::new(RefCell::new(10));
    let clone = Rc::clone(&shared);

    *shared.borrow_mut() += 5; // mutate through one handle...
    assert_eq!(*clone.borrow(), 15); // ...visible through the other
}
```

A layered look at the combination:

```
  Rc<RefCell<i32>>
    │
    │  Rc: owns the RefCell (shared ownership)
    ▼
  RefCell<i32>
    │
    │  RefCell: owns the i32 (runtime borrow checks)
    ▼
  i32: the actual value

  one handle mutates via borrow_mut()  ->  the other handle sees it,
  because they are two handles onto the SAME RefCell allocation.
```

This is the pattern behind the exercise's `Wallet`: one balance, many handles, all mutations visible to all. (The capstone and the graph/UI crates you'll meet later use exactly this shape — and when you reach Module 032, `Arc<Mutex<T>>` will be the same diagram with atomics instead of a plain count and a lock instead of borrow flags.)

## Common Pitfalls

- **Deep-copying with `Rc::clone` vs. deep-copying.** `Rc::clone` is O(1) — it only bumps a counter. `Rc` has no `DerefMut`, so you can't mutate through it; reach for `RefCell` (or redesign).
- **Panicking instead of borrowing.** `borrow_mut()` while any borrow is live panics at runtime. Keep borrows scoped short, and prefer `try_borrow()` if you need graceful handling.
- **Forgetting `*` around `borrow_mut()` results.** `*cell.borrow_mut() += 1` — the `*` is the deref that reaches the value; forgetting it assigns through the `RefMut` wrapper or fails to compile.
- **Holding a `Ref` across a `borrow_mut` in another scope.** The borrow lives as long as the `Ref`/`RefMut` value — a `let` in a long function is a long borrow. Scope it or drop it.
- **Using `Rc` where threads are involved.** `Rc` is not `Send`; the compiler will refuse. That's `Arc`'s job (Module 032), and the error message will say so.
- **Cycles leak.** Two `Rc`s pointing at each other never reach refcount zero. If you need cycles, `Weak` is the answer — for now, design acyclic (the capstone's index-based graph sidesteps this entirely).

## Key Terms

- **`Rc<T>`:** reference-counted shared ownership for single-threaded use; `Rc::clone` bumps the count.
- **interior mutability:** mutating data through an immutable reference, via a type that moves the borrow check.
- **`RefCell<T>`:** the interior-mutability cell that checks borrow rules at runtime.
- **`Ref` / `RefMut`:** the guard types returned by `borrow()` / `borrow_mut()`; dropping them releases the borrow.
- **strong count:** the number of live `Rc` handles; at zero, the value is dropped.
- **`Rc<RefCell<T>>`:** the "shared mutable state" pattern for one thread.

## Exercise

In `exercises/`, three pieces of shared state are scaffolded. Fill in each `TODO(module-029)`:

1. `shared_members` and `Member::chat_name` — shared ownership with `Rc`; the tests assert the strong count before and after dropping the members.
2. `Counter` — interior mutability with `RefCell`: `increment` uses `borrow_mut`, `value` uses `borrow`.
3. `Wallet` — the full `Rc<RefCell<T>>` pattern: `share`, `deposit`, `withdraw` (borrow once, check, then mutate), and `balance`.

Run `cargo test -p module-029-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 15.4: Rc<T>, the Reference Counted Smart Pointer](https://doc.rust-lang.org/book/ch15-04-rc.html)
- [The Rust Book, Chapter 15.5: RefCell<T> and the Interior Mutability Pattern](https://doc.rust-lang.org/book/ch15-05-interior-mutability.html)
- [std docs: `std::cell::RefCell`](https://doc.rust-lang.org/std/cell/struct.RefCell.html)
