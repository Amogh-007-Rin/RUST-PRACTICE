# Module 081: Introduction to WebAssembly

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 45–90 min
**Prerequisites:** Modules 001–080. Helpful: Module 039 (Cargo deep dive) and a passing familiarity with how `cargo` builds libraries vs binaries.

## Learning Objectives

- You will be able to explain what WebAssembly is, what problem it solves, and why Rust is one of the best languages for targeting it.
- You will be able to describe the structure of a `.wasm` module: sections, linear memory, imports, and exports.
- You will be able to implement a byte-addressed linear memory with bounds checking and growth, simulating what every wasm instance gets at runtime.
- You will be able to model a wasm module's import/export table in code.
- You will be able to set up a `wasm-pack` project and know the commands that turn a Rust crate into a `.wasm` module.

## Why This Matters

WebAssembly is the only bytecode format that runs in every major browser, and Rust is its first-class citizen: Rust's `no_std` output maps almost one-to-one onto wasm's model, and the `wasm-bindgen` ecosystem gives Rust code real access to the DOM. Companies ship heavy compute (image/video processing, cryptography, games, Figma's entire rendering engine) in Rust→wasm precisely because the browser needs native-speed code that is still memory-safe. Any job posting that mentions "Rust and WebAssembly" is looking for someone who understands the runtime model this module teaches — not just how to run `wasm-pack`.

## Concept

### What WebAssembly actually is

WebAssembly (WASM) is a **portable, sandboxed binary instruction format**. It is not assembly for your CPU; it is assembly for an abstract stack machine that every host (browser, Node.js, Wasmtime, ...) implements. A `.wasm` file is compiled once from a language like Rust or C, and then executed by a host-side *engine* (V8, SpiderMonkey, Wasmtime) that compiles the bytecode down to native code at load time. The sandbox matters: a wasm module cannot touch the file system, the network, or the host's memory unless the host explicitly hands it capabilities. That is the property that lets untrusted code run in your browser tab.

Three properties make this combination powerful:

1. **Performance.** The engine compiles wasm to native code with the same JIT machinery it uses for JS, but wasm's static types mean it can skip most of the guesswork JS requires. Roughly: wasm runs at near-native speed, JS at 5–50x slower than native for compute-heavy code.
2. **Safety.** Wasm is memory-safe by construction. There is no heap pointer arithmetic; the only memory a module can touch is its own **linear memory**, and every load/store is bounds-checked by the engine.
3. **Universality.** Any language that can lower to wasm (Rust, C/C++, Go, Zig, ...) runs in any browser, forever — wasm is a W3C standard, not a vendor's format.

### Why Rust targets it so well

Rust was designed for "systems programming without a GC", and wasm has no GC either. A Rust program that doesn't allocate at runtime is nearly a 1:1 translation to wasm. Rust's `no_std` support (no standard library, no allocator) compiles to tiny modules, and Rust's ownership model is respected by wasm's sandbox: safe Rust code can't create the memory corruption wasm forbids, so there is no trust mismatch. Compare this with garbage-collected languages that must drag a runtime into the browser just to run. The Rust/WASM workflow is: write a `cdylib` crate, compile with the `wasm32-unknown-unknown` target, and let `wasm-bindgen` generate the JS glue that exposes your functions to the page.

### Anatomy of a `.wasm` module

A wasm binary is a sequence of **sections**. The ones you'll meet constantly:

| Section | What it declares |
|---|---|
| Type | function signatures used by the module |
| Import | external things the module needs from the host (functions, memory) |
| Function | the module's own function signatures (indexing into Type) |
| Memory | how much linear memory the module wants (in 64 KiB pages) |
| Export | which functions/memories the host may call/read |
| Code | the actual function bodies, as stack-machine bytecode |

A minimal module in text format (`.wat`) — this is what the binary encodes:

```text
(module
  (func $add (export "add") (param i32 i32) (result i32)
    local.get 0
    local.get 1
    i32.add))
```

> **Note:** `.wat` is WebAssembly's human-readable text format, not Rust. The `(export "add")` line is what makes `wasm.add(2, 3)` callable from JS. The `$add` function is a stack machine: push `local 0`, push `local 1`, pop both and push their sum.

The execution model is a **value stack**: instructions pop operands and push results. There are no registers, no loops over memory, and no branches that can leave the sandbox. Control flow uses structured `block`/`loop`/`if` instructions — deliberately the same shape as high-level-language control flow, which is why wasm compiles so cleanly from Rust.

### Linear memory

The heart of the wasm data model is **linear memory**: one flat, byte-addressable array, allocated by the module (a default of 1 page = 64 KiB), growable in pages, and *shared* with the host. The wasm module's `i32.load`/`i32.store` instructions and the host's JavaScript `Uint8Array` view are two windows onto the same buffer. Because there is no pointer arithmetic beyond the buffer itself, every access is a bounds check away from a trap — the engine refuses, it doesn't corrupt.

```
        wasm module                        host (browser JS)
+------------------------+          +-----------------------------+
|  func add, sub, ...    |  exports |  wasm.add(2,3)  -> 5        |
|  exports: "add", "mem" |--------->|  Uint8Array view            |
|                        |          |  /\                         |
|  LINEAR MEMORY:        |  memory  |  | write bytes              |
|  +-----------------+   |  shared  |  v                          |
|  | [0] [1] ... [N] |<------------>|  wasm.instance.exports.mem  |
|  +-----------------+   |          |                             |
|  byte-addressed        |  imports |  env.log(msg)  <------------+
|  bounds-checked        |<---------|  JS function provided by    |
+------------------------+          |  the host at instantiation  |
                                     +-----------------------------+
```

The direction of arrows matters: **imports** are what the module receives from the host (callbacks, memory, globals); **exports** are what the module gives back (functions the host calls, memory the host reads). Nothing crosses that boundary except through the import/export table and the shared linear memory.

> **Practical consequence:** "passing a string" between JS and Rust means writing its UTF-8 bytes into linear memory, telling the other side where it starts and how long it is, and letting it read back — which is exactly why `wasm-bindgen` generates glue code for you instead of making you do this by hand.

### The setup: `wasm-pack`

The standard toolchain is `wasm-pack`, which wraps `cargo` + `wasm-bindgen`. A Rust crate becomes a wasm module like this:

```bash
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack new hello-wasm && cd hello-wasm
wasm-pack build --target web   # produces pkg/ with .wasm + JS glue
```

A library that exports to JS looks like this (`cdylib` is what tells cargo to produce an importable artifact):

```rust,ignore
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}
```

> **Note:** `#[wasm_bindgen]` only compiles on the `wasm32` target; this module's exercise does not require the wasm target, so the "real" snippets above are for reading, not building. You'll build the same concepts in pure Rust on your own machine.

On the JS side:

```html
<script type="module">
  import init, { add } from './pkg/hello_wasm.js';
  await init();
  console.log(add(2, 3)); // 5
</script>
```

### What this module's exercise simulates

You can't easily run a `.wasm` module without installing a target and a browser host — so this exercise builds the *runtime model* in pure, testable Rust:

- `LinearMemory` — a `Vec<u8>` with `load_u8`/`store_u8`/`load_u32`/`store_u32`, explicit bounds-checking errors, and `grow` honoring a maximum. This is exactly the semantics of wasm's linear memory, minus the engine enforcing it.
- `WasmModule` — a name, an import list, an export table (name → function), and its own linear memory. This is the decoded content of the type/import/function/memory/export/code sections.

Everything is deterministic and host-testable, and the concepts transfer directly to real wasm code later.

## Common Pitfalls

- **Forgetting bounds checks in `load_u32`/`store_u32`.** `load_u32(offset)` reads 4 bytes; at offset `size - 2` it must fail, not read garbage. Always check `offset + 4 <= size` (watch for `offset + 4` overflowing `usize` in real code).
- **Confusing imports and exports.** Imports come *from* the host (JS provides them at instantiation); exports go *to* the host (JS calls them). The direction is from the module's point of view.
- **Assuming JS and Rust share objects directly.** They don't — they share linear memory plus a callable table. "Passing data" is really "writing bytes and agreeing on a layout".
- **Rejecting duplicate exports silently.** Real engines trap on duplicate export names; your table should error, not overwrite.
- **Treating `grow` like `resize`.** Wasm's `memory.grow` returns the *previous* size, not the new one, and fails if the result exceeds the declared maximum.

## Key Terms

- **WebAssembly (WASM):** a portable, sandboxed, stack-machine bytecode format executed by browsers and other hosts.
- **Linear memory:** the single flat, bounds-checked byte array a wasm module allocates and the host can share.
- **Section:** a named chunk of a `.wasm` binary (type, import, function, memory, export, code, ...).
- **Import:** a capability (function, memory, global) the host provides to the module at instantiation.
- **Export:** a capability (function, memory) the module exposes to the host.
- **Trap:** a runtime failure in wasm (e.g. an out-of-bounds memory access) that stops the module deterministically instead of corrupting memory.
- **`wasm-pack`:** the CLI that builds a Rust crate to `wasm32-unknown-unknown` and generates JS glue via `wasm-bindgen`.

## Exercise

In `exercises/` you'll find the skeleton of a mini wasm runtime. Implement the `// TODO(module-081)` stubs:

1. **`LinearMemory`**: `grow`, `load_u8`, `store_u8`, `load_u32`, `store_u32` — correct bounds checks, little-endian `u32` handling, and growth honoring `max_size`.
2. **`WasmModule`**: `add_import` (deduplicate), `add_export` (reject duplicates with `AlreadyExported`), and `export` (resolve names, `NotFound` for missing).

Run `cargo test -p module-081-exercises` until all tests pass, then compare with `solutions/`. The tests check byte-level behavior (e.g. `0xDEADBEEF` stored little-endian), boundary cases at exactly `size`, and the export/import semantics — no browser required.

## Further Reading

- [MDN: WebAssembly concepts](https://developer.mozilla.org/en-US/docs/WebAssembly/Concepts) — the mental model, from memory to sandboxing.
- [The Rust and WebAssembly Book](https://rustwasm.github.io/docs/book/) — the official Rust↔wasm workflow with `wasm-bindgen`.
- [The `wasm-pack` book](https://rustwasm.github.io/wasm-pack/book/) — project setup and build targets.
- [WebAssembly Specification](https://webassembly.github.io/spec/) — the authoritative section-by-section module structure.
