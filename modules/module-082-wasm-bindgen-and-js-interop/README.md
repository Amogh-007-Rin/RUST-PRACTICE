# Module 082: `wasm-bindgen` & JS Interop

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 60–90 min
**Prerequisites:** Module 081 (WebAssembly fundamentals). Modules 001–080.

## Learning Objectives

- You will be able to explain what `wasm-bindgen` does and why raw WASM/JS interop needs glue code at all.
- You will be able to read and write `#[wasm_bindgen]`-annotated functions and know which Rust types cross the boundary and which don't.
- You will be able to keep "application logic" separate from "DOM glue" so that the logic is testable without a browser.
- You will be able to verify the wasm-facing API of a crate from the host using a stub module, and know how to run the real wasm path with `wasm-pack test --headless`.

## Why This Matters

Module 081 gave you the raw runtime model: JS and wasm only share linear memory plus a callable table. Nobody wants to hand-write `TextEncoder` + pointer arithmetic to pass a string across that boundary — that's what `wasm-bindgen` (and its DOM companion `web-sys`) automate. Every production Rust-wasm project — Figma-style renderers, `wasm-bindgen`-based games, data-heavy web apps — is built on this layer. Understanding the boundary deeply is also what lets you design crates that are testable on the host, which is the difference between a Rust-wasm developer and someone who blindly runs `wasm-pack` and hopes.

## Concept

### The interop problem

In Module 081 you saw the mechanics: a wasm module exports functions and a linear memory; JS calls the functions and reads/writes the memory through a typed array. To call `wasm.add(2, 3)` from JS, the host has to:

1. marshal each argument into the wasm ABI (numbers are fine — they're just registers/stack values),
2. call the exported function,
3. marshal the return value back.

Numbers pass trivially. But what about a `String`? A string is a pointer + length into linear memory. So "passing a string" actually means: allocate bytes in linear memory, write the UTF-8 there, pass the pointer and length as two numbers, and have the other side read them back — then free or reuse the buffer. And objects? They don't exist across the boundary at all unless you represent them as bytes in memory or as opaque handles.

Hand-rolling this for every function is exactly the kind of repetitive, error-prone work Rust exists to eliminate. `wasm-bindgen` generates the glue:

```rust,ignore
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("hello, {name}!")
}
```

```js
import init, { greet } from './pkg/demo.js';
await init();
console.log(greet("world")); // "hello, world!"
```

The generated JS glue does the memory dance for you. On the Rust side you write ordinary Rust; on the JS side you get ordinary JS functions.

### What crosses the boundary

| Rust type | JS type | Notes |
|---|---|---|
| `u8..u64`, `i8..i64`, `f32`, `f64` | `Number` (`BigInt` only with opt-in) | prefer `u32`/`f64` for ids and math |
| `bool` | `Boolean` | |
| `&str` / `String` | `String` | copied through linear memory |
| `Vec<u8>` | `Uint8Array` | copied, not shared |
| `Option<T>` | `T \| undefined` | |
| `Result<T, JsValue>` | thrown exception on `Err` | the idiomatic error channel |
| `()`, `JsValue` | `undefined`, arbitrary | `JsValue` is an opaque handle to anything |
| arbitrary structs | *nothing by default* | derive `#[wasm_bindgen]` or `#[wasm_bindgen(getter/setter)]` to expose fields |

Two rules follow. First: **return `Result<_, JsValue>` from fallible functions** — `Err` becomes a thrown JS exception, `Ok` becomes the value. Second: **keep your data model on the Rust side and pass primitives across**. This is the design this module's exercise is built around: a plain `TodoList` struct in pure Rust, and a thin `bindings` layer that converts primitives into method calls.

### DOM access: `web-sys`

Calling the DOM from wasm-bindgen is done through `web-sys`, a crate that pre-generates Rust bindings for the entire web platform API. A DOM manipulation example:

```rust,ignore
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

#[wasm_bindgen]
pub fn make_red() -> Result<(), JsValue> {
    let document = web_sys::window()
        .ok_or("no window")?
        .document()
        .ok_or("no document")?;
    let heading = document
        .get_element_by_id("title")
        .ok_or("no #title element")?
        .dyn_into::<web_sys::HtmlHeadingElement>()?;
    heading.style().set_property("color", "red")?;
    Ok(())
}
```

> **Note:** `window()` and `document()` are only available when the page runs the module — that's why every step is a `Result`/`Option`. The `?` chain above is idiomatic: one error path for "not in a browser".

And the classic create-and-append pattern:

```rust,ignore
#[wasm_bindgen]
pub fn add_li(text: &str) -> Result<(), JsValue> {
    let document = web_sys::window().ok_or("no window")?.document().ok_or("no document")?;
    let li = document.create_element("li")?;
    li.set_text_content(Some(text));
    let list = document.get_element_by_id("list").ok_or("no list")?;
    list.append_child(&li)?;
    Ok(())
}
```

### The architecture this module teaches

The pattern that keeps Rust-wasm apps sane — and testable — is three layers:

1. **Core logic** — plain Rust types and functions with no knowledge of the web (your `TodoList`).
2. **Host-stubbed API** — the exact functions you'll export to JS, implemented twice: once against `web-sys` behind `#[cfg(target_arch = "wasm32")]`, once against an in-memory fake DOM behind `#[cfg(not(target_arch = "wasm32"))]`.
3. **Sync/render logic** — the code that decides *what* the DOM should contain, written against the smallest possible DOM interface so it runs against both the fake and the real thing.

Because layers 1 and 3 are pure host-side Rust, your tests run with plain `cargo test` — no browser, no wasm target — and the *same test file* exercises the exact API surface that `#[wasm_bindgen]` will expose in the browser.

### A note on global state

`#[wasm_bindgen]` exports are free functions — there's no way to pass `&mut TodoList` in from JS. So the wasm layer typically owns the app state in a global (a `static Mutex<Option<TodoList>>`), and the exported functions lock it. That is exactly what this module's `bindings` module does — and the host stub uses the *same* global structure, so tests exercise identical behavior on both sides.

## Common Pitfalls

- **Exporting a struct directly and expecting it to survive.** Without `#[wasm_bindgen]` on the struct and its fields, nothing crosses the boundary. Pass primitives; keep objects on the Rust side.
- **`u64` across the boundary.** It maps to `BigInt` only with opt-in, and old glue silently truncated. Use `u32` ids.
- **Returning `Result<(), MyError>` without a `JsValue`.** The error type must be convertible; the idiomatic choice is `Result<T, JsValue>`, which turns `Err` into a thrown exception.
- **Forgetting `wasm-bindgen` is target-gated.** `#[wasm_bindgen]` compiles only on `wasm32`. Gate that module with `#[cfg(target_arch = "wasm32")]` and keep a host stub so CI never breaks.
- **Touching `window`/`document` at module load time.** Browsers parse the whole module before running it; browser APIs exist only inside functions called from JS. Always fail gracefully (`?`) rather than panic.

## Key Terms

- **`wasm-bindgen`:** the crate + CLI that generates JS glue for Rust functions compiled to wasm, marshalling types across the boundary.
- **`web-sys`:** generated Rust bindings for the browser's DOM and web-platform APIs.
- **Boundary:** the wasm ↔ JS interface — only primitives and bytes cross it directly.
- **Glue:** the generated JS (and Rust-side ABI code) that converts arguments and return values.
- **Host stub:** a `#[cfg(not(target_arch = "wasm32"))]` implementation of the same API as the wasm bindings, used to test without a browser.
- **`wasm-pack test --headless`:** runs your test suite in a headless browser against the real wasm build.

## Exercise

In `exercises/` the architecture above is pre-wired. You implement the `// TODO(module-082)` stubs:

1. **`todos::TodoList`** — `add`, `toggle`, `remove`. Sequential ids, never reused.
2. **`dom::Dom`** — `create_element`, `set_text`, `set_class`, `remove_element`. The in-memory DOM stub.
3. **`render_todos`** — sync logic: create the `ul#todo-list` container on first render, one `li#todo-<id>` per item, class `done` on completed items, remove stale `li`s. Must be idempotent.

The `bindings` module (both the wasm and host variants) is given to you in full — it shows exactly what the interop layer looks like on both sides. When the core stubs are implemented, all tests pass.

## Running This Module's Tests

The default path needs nothing special:

```bash
cargo test -p module-082-exercises     # host tests: core logic + stub bindings
cargo test -p module-082-solutions
```

The `#[wasm_bindgen]` code is gated behind `cfg(target_arch = "wasm32")` and is **not** compiled or run by `cargo test` on a host machine — CI covers the host path only. If you want to exercise the *real* wasm path, install a wasm target and run the suite in a headless browser:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack test --headless --chrome exercises
```

This compiles the `#[wasm_bindgen]` module, runs the same integration tests against the real `web-sys` DOM, and requires the wasm target plus Chrome — both optional for this module.

## Further Reading

- [The Rust and WebAssembly Book — "wasm-bindgen" chapter](https://rustwasm.github.io/docs/book/wasm-bindgen.html) — the official intro to the bindgen model.
- [wasm-bindgen reference: supported types](https://rustwasm.github.io/wasm-bindgen/reference/types.html) — the authoritative table of what crosses the boundary.
- [web-sys docs](https://docs.rs/web-sys/latest/web_sys/) — the generated DOM bindings.
- [Module 081 — Introduction to WebAssembly](../module-081-introduction-to-webassembly/README.md) — the runtime model these bindings sit on top of.
