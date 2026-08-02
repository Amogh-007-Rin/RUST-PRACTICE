# Module 046: Pinning & Async Internals

**Block:** Block E — Async Rust
**Estimated time:** 75–120 min
**Prerequisites:** Module 041 (async/await), Module 042 (Tokio runtime), Module 028 (`Box<T>`)

## Learning Objectives

- Explain what `Pin<P>` does and why it exists.
- Distinguish `Unpin` types (can move when pinned) from `!Unpin` types (cannot).
- Use `Box::pin` to pin self-referential and async values on the heap.
- Describe what the compiler generates for an `async fn` (a state machine enum).
- Explain why holding a reference across `.await` makes the future `!Unpin`.

## Why This Matters

You have been using `async fn` since Module 041. Every `async fn` call returns a future — an anonymous type the compiler generates for you. Most of the time that future is `Unpin` and you never think about it. But the moment you hold a reference across an `.await` point, or build a self-referential struct, the future becomes `!Unpin` and you must pin it. Tokio's `spawn` requires `Send + 'static`, which pins the future internally. Understanding `Pin` is the difference between "it compiles" and "I know why it compiles."

## Concept

### What problem does `Pin` solve?

Consider a self-referential struct:

```rust
struct SelfRef {
    data: String,
    pointer_to_data: *const String,
}
```

The `pointer_to_data` field points into `data`'s heap buffer. If you move the struct (by assigning it, passing it to a function, returning it), the `data` field moves to a new address — but `pointer_to_data` still holds the old address. Dereferencing it is undefined behavior.

```
Before move:
┌─────────────────────────────────────┐
│ SelfRef @ 0x1000                    │
│   data: String ──────────┐          │
│   pointer_to_data: 0x2000│          │
└──────────────────────────│──────────┘
                           ▼
                     ┌──────────┐
                     │ "hello"  │  @ 0x2000 (heap)
                     └──────────┘

After move (to 0x3000):
┌─────────────────────────────────────┐
│ SelfRef @ 0x3000                    │
│   data: String ──────────┐          │
│   pointer_to_data: 0x2000│          │  ← STILL POINTS TO 0x2000
└──────────────────────────│──────────┘
                           ▼
                     ┌──────────┐
                     │ "hello"  │  @ 0x2000 (heap, still valid)
                     └──────────┘
```

In this specific case the heap buffer did not move (String's buffer is stable), so the pointer is still valid. But if `data` were a type whose address changed on move (like a small `String` stored inline via small-string optimization, or a `Vec` with inline storage), the pointer would dangle.

`Pin<P>` solves this by wrapping a pointer `P` (like `Box<T>` or `&mut T`) and making it a compile error to obtain `&mut T` from a `Pin<&mut T>` unless `T: Unpin`. Without `&mut T`, you cannot use `std::mem::swap` or `std::mem::replace` to move the value out.

### `Unpin`: the escape hatch

Most Rust types are `Unpin`. `Unpin` means "I am safe to move even when pinned." `i32`, `String`, `Vec<T>`, `Box<T>` — all `Unpin`. For these types, `Pin<&mut T>` still lets you get `&mut T` (via `get_mut`), because moving them is harmless.

```rust
fn example() {
    let mut s = String::from("hello");
    let mut pinned: Pin<&mut String> = Pin::new(&mut s);
    // String: Unpin, so we can get &mut String back:
    pinned.get_mut().push_str(" world");
}
```

`Pin::new` only accepts `Unpin` types. For `!Unpin` types, you must use `Box::pin` (which allocates and pins on the heap) or `unsafe { Pin::new_unchecked }` (which you promise not to move the value).

### What the compiler generates for `async fn`

When you write:

```rust
async fn fetch_and_process(url: &str) -> String {
    let response = fetch(url).await;
    process(response)
}
```

The compiler generates an anonymous enum representing the state machine:

```
State 0: waiting to start
  - holds: url: &str

State 1: waiting on fetch().await
  - holds: url: &str
  - holds: the future returned by fetch(url)

State 2: done
  - holds: the final String
```

```
┌────────────────────────────────────────────────────────────┐
│ enum FetchAndProcessFuture<'a> {                           │
│   State0 { url: &'a str },                                 │
│   State1 { url: &'a str, fetch_future: FetchFuture<'a> }, │
│   State2 { result: String },                               │
│ }                                                          │
└────────────────────────────────────────────────────────────┘
```

Because `State1` holds `url: &'a str` (a reference) across the `.await`, the future borrows from its environment. That borrow makes the future `!Unpin` — you cannot move it while it is in `State1`, because the reference inside would be invalidated.

This is why `tokio::spawn` requires `Send + 'static`: the future must own all its data (no borrowed references), and it must be movable to another thread. If your async fn holds a reference across `.await`, you cannot spawn it directly — you must either make it own the data or use a local executor (like `tokio::task::spawn_local` with `!Send` futures, or `futures::executor::block_on`).

### Pinning in practice: `Box::pin`

When you have a `!Unpin` future and need to store it (in a struct, a `Vec`, or pass it to a function that expects `Pin<&mut F>`), you use `Box::pin`:

```rust
use std::pin::Pin;
use std::future::Future;

async fn my_async_fn() -> i32 { 42 }

fn store_future() -> Pin<Box<dyn Future<Output = i32>>> {
    Box::pin(my_async_fn())
}
```

`Box::pin` allocates the future on the heap and returns `Pin<Box<F>>`. The future lives at a stable address, so it is safe for it to be `!Unpin`.

### The exercise's `SelfRef`

The exercise asks you to build a self-referential struct and pin it before reading. The struct holds a `String` and a raw pointer into that `String`. After construction, you must `Box::pin` the struct before calling `read_self_ref` — the pin guarantees the struct will not move, so the pointer remains valid.

```rust
let s = SelfRef::new("hello".to_string());
let pinned = Box::pin(s);
let read_back = pinned.read_self_ref();  // safe: pinned guarantees no move
```

The `read_self_ref` method takes `self: Pin<&Self>`, which is the pinned-reference form. Inside, you dereference the raw pointer in an `unsafe` block — the pin is what makes this safe.

## Common Pitfalls

- **Assuming all futures are `Unpin`.** Async fns holding references across `.await` are `!Unpin`. Use `Box::pin` to store them.
- **Trying `Pin::new` on a `!Unpin` type.** It will not compile. Use `Box::pin` instead.
- **Thinking `Pin` prevents mutation.** It prevents *movement*. You can still mutate through `get_mut` (for `Unpin`) or `get_unchecked_mut` (unsafe, for `!Unpin`).
- **Spawning an async fn that borrows.** `tokio::spawn` requires `'static`. Make the async fn own its data, or use a local task.
- **Forgetting that `Box::pin` allocates.** For hot paths, consider `std::pin::pin!` (stack pinning) or restructuring to avoid `!Unpin`.

## Key Terms

- **`Pin<P>`:** a wrapper around a pointer `P` that prevents moving the pointee.
- **`Unpin`:** auto-trait marking types safe to move even when pinned. Most types are `Unpin`.
- **`!Unpin`:** types that must not move when pinned (self-referential structs, async futures holding references).
- **`Box::pin`:** allocate on the heap and return `Pin<Box<T>>`.
- **State machine enum:** what the compiler generates for `async fn`; each variant is a suspension point.

## Exercise

Work in `exercises/` and make `cargo test -p module-046-exercises` pass. Five TODOs in `src/lib.rs`:

1. `type_is_unpin` — return `true`; the bound `T: Unpin` is the check.
2. `pin_in_box` — use `Box::pin(value)` to pin a value on the heap.
3. `write_through_pin` — mutate through a `Pin<&mut u64>` using `get_mut` (since `u64: Unpin`).
4. `SelfRef::new` — build a self-referential struct; set `ref_to_self` to point at `owner`.
5. `SelfRef::read_self_ref` — dereference the raw pointer inside `unsafe`; the pin guarantees safety.
6. `async_future_is_unpin` — return `false`; `async_fn_example` holds a `&str` across `.await`.

Tests check `Unpin` for common types, that `pin_in_box` preserves values, that `write_through_pin` mutates, that `SelfRef` reads correctly when pinned, and that the async future is `!Unpin`. Compare with `solutions/` when done.

## Further Reading

- [The Rustonomicon: Pin](https://doc.rust-lang.org/nomicon/pin.html)
- [Tokio tutorial: Pin](https://tokio.rs/tokio/tutorial/pin)
- [`std::pin` docs](https://doc.rust-lang.org/std/pin/index.html)
- [Async-await on stable Rust (blog post)](https://blog.rust-lang.org/2019/11/07/Async-await-stable.html) — the original announcement explaining the state machine transformation
