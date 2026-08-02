# Module 021: Closures

**Block:** Block C — Intermediate Rust I
**Estimated time:** 45–90 min
**Prerequisites:** Modules 004–005 (ownership and borrowing), 015 (generics), 016–017 (traits)

## Learning Objectives

- Write closures (`|args| body`) and use `move` to force ownership of captures.
- Explain how a closure's *capture kind* determines whether it implements `Fn`, `FnMut`, or `FnOnce`.
- Accept closures as function parameters with the right trait bound, and return them from functions.
- Distinguish "call once" (`FnOnce`), "call repeatedly, mutating captures" (`FnMut`), and "call repeatedly, read-only" (`Fn`).

## Why This Matters

Closures are how Rust passes behavior around: iterator adapters (`map`, `filter`, `fold` — next module), `thread::spawn(move || ...)` (Block D), callback-driven APIs, and every web framework's handlers capture state through closures. The `Fn`/`FnMut`/`FnOnce` hierarchy is the single most common trait-bound pattern in real Rust code, and getting it wrong is the #1 source of "expected a closure, found..." compile errors in production codebases.

## Concept

A **closure** is a value that bundles a piece of code together with the *environment* it was created in. Where a function `fn` can only see its parameters, a closure can also see (and hold onto) variables from the surrounding scope:

```rust
fn main() {
    let factor = 3;
    let multiply = |x: i32| x * factor;
    assert_eq!(multiply(2), 6);
}
```

The syntax is a parameter list in pipes followed by a body: `|x| x * factor`. The parameter types can usually be inferred from the call site, but you can annotate them (`|x: i32|`) when inference needs help.

### What a closure captures

When you write a closure, the compiler analyzes *which* variables it uses and *how*, and captures them accordingly. This is the single most important idea in this module, so here is the decision tree:

```
                          What does the closure do with the variable?
                                      |
              +-----------------------+--------------------+
              |                       |                    |
     reads it by shared          mutates it or          moves it / can't
     reference                   takes a &mut           borrow for its
     (e.g. s.len())              (e.g. s.push(..))      whole life (e.g. drop(s))
              |                       |                    |
              v                       v                    v
          captures &s               captures &mut s       captures s (by value)
              |                       |                    |
              v                       v                    v
        implements Fn             implements FnMut      implements FnOnce
   (callable many times,     (callable many times,   (callable exactly once,
    through &self only)       through &mut self)       because calling it
                                                       consumes its captures)

   Every Fn closure is also FnMut, and every FnMut is also FnOnce:

                     FnOnce   (least powerful bound, most general)
                       ^
                       |
                     FnMut
                       ^
                       |
                      Fn      (most powerful bound, most restrictive)
```

The three traits are a *ladder*: `Fn: FnMut: FnOnce`. A closure that only reads its captures can be called through a shared reference any number of times — that's `Fn`. A closure that mutates a captured variable needs exclusive access, so it's `FnMut`. A closure that *moves* something out of its environment (or consumes a capture by value) can only ever run once — `FnOnce`.

```rust
fn main() {
    let count = 0;
    let read_only = || count;            // captures &count          -> Fn
    let _ = read_only();

    let mut counter = 0;
    let mut bump = || counter += 1;      // captures &mut counter    -> FnMut
    bump();
    bump();

    let text = String::from("hello");
    let consume = move || text.len();    // captures text by value   -> FnOnce
    assert_eq!(consume(), 5);
}
```

Note `move`: it forces the closure to take ownership of its captures instead of borrowing. Here `text` is a `String`, and the closure only calls `.len()` — without `move` it would *borrow* `text`, and `text` would still be usable afterwards. With `move`, `text` is owned by the closure.

### Closures as parameters

Because closures are just values implementing one of the three traits, you accept them with generic bounds. `impl Fn` in argument position is shorthand for a generic parameter:

```rust
fn apply_twice(f: impl Fn(i32) -> i32, x: i32) -> i32 {
    f(f(x))
}

fn main() {
    assert_eq!(apply_twice(|x| x + 1, 10), 12);
    assert_eq!(apply_twice(|x| x * x, 3), 81);
}
```

Rule of thumb: use the *least powerful* bound you need. If you call the closure in a loop, it's `FnMut`:

```rust
fn call_counter(mut f: impl FnMut(i32), xs: &[i32]) -> usize {
    let mut calls = 0;
    for &x in xs {
        f(x);
        calls += 1;
    }
    calls
}

fn main() {
    let mut total = 0;
    let calls = call_counter(|x| total += x, &[1, 2, 3]);
    assert_eq!(calls, 3);
    assert_eq!(total, 6);
}
```

Notice `f` itself must be declared `mut` here: calling an `FnMut` closure requires a mutable reference to it, so the binding has to be mutable. If you only call the closure once — perhaps because calling it consumes a moved capture — take `FnOnce`:

```rust
fn run_once(f: impl FnOnce() -> i32) -> i32 {
    f()
}

fn main() {
    let s = String::from("moved");
    assert_eq!(run_once(move || s.len() as i32), 5);
}
```

### Closures as return values

To return a closure, name its type with `impl Fn` in return position. A closure that doesn't capture anything could be a plain `fn` pointer (`fn(i32) -> i32`), but as soon as it captures, it needs `impl Fn` (or a boxed `Box<dyn Fn...>`, Module 026):

```rust
fn make_adder(amount: i32) -> impl Fn(i32) -> i32 {
    move |x| x + amount
}

fn main() {
    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    assert_eq!(add5(20), 25);
}
```

The `move` here is mandatory in spirit, not just style: the closure outlives `amount`'s scope, so it must *own* its capture. Without `move`, the compiler reports that the closure's borrow of `amount` would dangle.

### One thing closures can't be: generic

A closure is a single concrete type, so a given closure can't be generic over its argument. If you need `f(x)` to work for multiple types, use a generic *function* (Module 015) or a generic trait instead:

```rust
fn identity<T>(x: T) -> T {
    x
}

fn main() {
    assert_eq!(identity(42), 42);
    assert_eq!(identity("hi"), "hi");
}
```

### Broken: moving out of a captured variable

This example will not compile — the closure `|| drop(x)` needs to own `x` to drop it, but the closure isn't declared `move`, so the compiler refuses to move `x` out of the shared borrow it captured:

```rust,ignore
let x = String::from("hi");
let c = || drop(x); // error: cannot move out of `x` because it is borrowed
c();
```

The fix is to state the intent with `move` — and to call the closure at most once, because it becomes `FnOnce`:

```rust
fn main() {
    let x = String::from("hi");
    let c = move || drop(x);
    c();
}
```

## Common Pitfalls

- **Assuming a closure can mutate a capture with `Fn`.** A closure that does `counter += 1` is `FnMut`, not `Fn`. If a function requires `impl Fn`, pass a closure that doesn't mutate — or change the bound.
- **Forgetting `move` when returning a closure.** The closure outlives the captured variable; without `move` you get a lifetime error ("borrow of moved value" / "captured variable cannot escape"). Just add `move`.
- **Calling an `FnOnce` closure twice.** Calling consumes the closure's captures; the compiler rejects the second call. If you need repeated calls, your closure shouldn't move its captures — change it to borrow instead.
- **Using `return` inside a closure expecting it to exit the outer function.** `return` only exits the closure. Use an early-returning loop or restructure.
- **Overspecifying with `FnOnce` when you need `FnMut`.** It compiles, but callers can only invoke the closure once. Use the least-powerful bound you actually need — it makes your API more permissive, not less.

## Key Terms

- **closure:** a value pairing code with its captured environment; written `|args| body`.
- **capture:** the state a closure brings in from its surrounding scope, either by reference or by value.
- **`Fn`:** a closure callable any number of times through a shared reference (read-only captures).
- **`FnMut`:** a closure callable any number of times, allowed to mutate its captures.
- **`FnOnce`:** a closure callable exactly once; calling it consumes it.
- **`move`:** a keyword forcing a closure to take ownership of its captures.
- **`impl Fn(i32) -> i32`:** an opaque type name meaning "some closure with this signature" — usable in argument and return positions.

## Exercise

In `exercises/`, four functions are stubbed out. Make the tests in `tests/module_021.rs` pass by filling in each `TODO(module-021)`:

1. `apply_twice` — call an `Fn` closure twice. This is where the "least powerful bound" idea is easiest to see.
2. `make_adder` — return a closure capturing `amount`. Remember `move`.
3. `run_once` — invoke an `FnOnce` closure exactly once.
4. `call_counter` — call an `FnMut` closure once per element and count the calls. Declare the parameter `mut`.

Work from the tests: each test names the behavior, and the panic inside each stub is your cue that a function body is missing. Run `cargo test -p module-021-exercises` until everything is green, then compare with `solutions/`.

## Further Reading

- [The Rust Book, Chapter 13: Closures](https://doc.rust-lang.org/book/ch13-01-closures.html)
- [std docs: `std::ops::Fn`](https://doc.rust-lang.org/std/ops/trait.Fn.html)
- [Rust by Example: Closures](https://doc.rust-lang.org/rust-by-example/fn/closures.html)
