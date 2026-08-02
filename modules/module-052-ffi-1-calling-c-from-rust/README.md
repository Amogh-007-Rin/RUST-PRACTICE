# Module 052: FFI I — Calling C from Rust

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 60–90 min
**Prerequisites:** Module 036 (Unsafe Rust II), Module 051 (Memory Layout)

## Learning Objectives

- You will be able to declare an `extern "C"` block and call C functions from Rust.
- You will be able to use `#[no_mangle]` and `extern "C"` to export Rust functions with C ABI.
- You will be able to wrap unsafe FFI calls in safe Rust abstractions.
- You will be able to explain what `bindgen` does and when you'd use it.
- You will be able to write `# Safety` documentation for unsafe FFI wrappers.

## Why This Matters

Rust doesn't live in a vacuum. You'll call C libraries (OpenSSL, SQLite, zlib), use system APIs (POSIX, Win32), and integrate with existing C++ codebases. The `extern "C"` block is how you tell Rust "this function exists somewhere else, and it uses the C calling convention." Wrapping those unsafe calls in safe Rust is what makes FFI palatable in production code — you contain the `unsafe` to a thin layer and expose a safe API to the rest of your crate. This is the pattern every `sys` crate (`openssl-sys`, `sqlite3-sys`) uses.

## Concept

### The C ABI and `extern "C"`

Rust functions use the *Rust ABI*, which is unstable and compiler-specific. C functions use the *C ABI* (also called the *C calling convention*), which is stable and platform-defined. To call a C function from Rust, you declare it in an `extern "C"` block:

```rust
extern "C" {
    fn abs(x: i32) -> i32;
}

fn main() {
    let x = unsafe { abs(-42) };
    println!("{}", x); // 42
}
```

The `extern "C"` block tells the compiler: "these functions exist somewhere (in a linked C library), and they use the C calling convention." You must link the library (via `#[link(name = "c")]` or a build script) for the linker to find the symbols.

Calling an `extern` function is `unsafe` because the compiler can't verify the C code upholds Rust's safety invariants (no null pointers, no buffer overflows, etc.). You must wrap the call in an `unsafe` block.

### Simulating C functions in pure Rust

For learning purposes, you can *simulate* the C side by defining `#[no_mangle] pub extern "C" fn` functions in the same crate, then declaring them in an `extern "C"` block:

```rust
// The "C" implementation (really Rust, but with C ABI)
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

// Declare it as if it were a C function
extern "C" {
    fn rust_add(a: i32, b: i32) -> i32;
}

fn main() {
    let result = unsafe { rust_add(3, 4) };
    println!("{}", result); // 7
}
```

`#[no_mangle]` prevents the compiler from renaming the function (Rust mangles names to support overloading; C doesn't). `extern "C"` makes it use the C calling convention. The `extern "C"` block declares the function's signature so you can call it.

This technique lets you practice FFI without a C compiler. In real code, you'd link an actual C library.

### The real thing: linking a C library

Here's what it looks like when you call a real C library (e.g., `libm`, the C math library):

```rust
#[link(name = "m")]
extern "C" {
    fn sqrt(x: f64) -> f64;
}

fn main() {
    let x = 16.0;
    let result = unsafe { sqrt(x) };
    println!("sqrt({}) = {}", x, result); // 4
}
```

`#[link(name = "m")]` tells the linker to link `libm` (on Unix) or `m.lib` (on Windows). The `extern "C"` block declares `sqrt`'s signature. You call it in an `unsafe` block.

### Wrapping unsafe FFI in safe Rust

Exposing raw `extern "C"` functions forces every caller to use `unsafe`. Instead, wrap them in safe Rust functions that uphold the safety invariants:

```rust
extern "C" {
    fn strlen(s: *const i8) -> usize;
}

/// Returns the length of a null-terminated C string.
///
/// # Safety
///
/// The caller must ensure `s` points to a valid null-terminated C string.
pub fn safe_strlen(s: &std::ffi::CStr) -> usize {
    unsafe { strlen(s.as_ptr()) }
}
```

The wrapper takes a safe type (`&CStr`) and calls the unsafe FFI function internally. The `# Safety` doc comment explains what the caller must uphold (even though in this case, the wrapper makes it safe — the comment is for the internal `unsafe` block).

### What `bindgen` does

Writing `extern "C"` blocks by hand is tedious and error-prone. `bindgen` is a tool that parses C headers and generates the corresponding Rust `extern "C"` declarations automatically. You give it a `.h` file, and it produces a `.rs` file with all the function signatures, struct definitions, and constants.

For example, given this C header:

```c
// mathlib.h
int add(int a, int b);
double multiply(double x, double y);
```

`bindgen` generates:

```rust
extern "C" {
    pub fn add(a: i32, b: i32) -> i32;
}
extern "C" {
    pub fn multiply(x: f64, y: f64) -> f64;
}
```

You'd typically run `bindgen` in a `build.rs` build script, so the bindings are regenerated whenever the C header changes. The `bindgen` crate is a dev-dependency; the generated code is committed or generated at build time.

For this module, we'll simulate the C side in Rust (no C compiler required), but the README shows what the real C code would look like so you can see the full picture.

### ASCII diagram: the FFI call stack

```
Rust code                 C ABI boundary              C code (or simulated)
─────────                 ───────────────              ─────────────────────
                          
fn main() {                                       
  let r = safe_add(3, 4);                         
       │                                          
       ▼                                          
  safe_add(a, b) {                                
    unsafe { extern_add(a, b) } ──────►  extern "C" fn extern_add(a, b)
       │                                    │      
       │                                    ▼      
       │                                  a + b    
       │                                    │      
       ◄────────────────────────────────────┘      
      returns i32                                  
  }                                                
}                                                  
```

The safe wrapper (`safe_add`) contains the `unsafe` block. The caller of `safe_add` doesn't need to know about the FFI — they just call a normal Rust function.

### The exercise in a sentence each

- `add()` — wrap a simulated C `add` function in a safe Rust wrapper.
- `multiply()` — wrap a simulated C `multiply` function.
- `safe_abs()` — wrap the C `abs` function (from `libc`) in a safe wrapper.
- `c_string_length()` — wrap `strlen` and return the length of a `&CStr`.

The tests call your safe wrappers. You'll declare the `extern "C"` functions, implement the simulated C side with `#[no_mangle]`, and write safe wrappers with `# Safety` docs.

## Common Pitfalls

- **Forgetting `unsafe` when calling `extern` functions.** All `extern` calls are unsafe — the compiler can't verify the C code's safety.
- **Mismatched function signatures.** If the Rust declaration doesn't match the C definition, you get undefined behavior (silent corruption, crashes). Double-check types (`i32` vs `i64`, `*const i8` vs `*mut i8`).
- **Not linking the C library.** `extern "C"` declares the function, but the linker needs to find it. Use `#[link(name = "...")]` or a build script.
- **Missing `# Safety` docs on unsafe functions.** Clippy requires `# Safety` sections for public unsafe functions. Document what the caller must uphold.
- **Using `#[no_mangle]` on non-`pub` functions.** `#[no_mangle]` exports the symbol globally. If you're simulating C in the same crate, the function must be `pub` (or at least visible to the `extern` block).

## Key Terms

- **`extern "C"`:** declares a function uses the C calling convention (for calling C or exporting Rust functions with C ABI).
- **`#[no_mangle]`:** prevents name mangling, so the function's symbol matches its name (required for C interop).
- **C ABI:** the C calling convention — a stable, platform-defined way to call functions.
- **`bindgen`:** a tool that generates Rust FFI bindings from C headers.
- **`# Safety`:** a doc comment section explaining what invariants an unsafe function requires.
- **`CStr`:** a borrowed reference to a null-terminated C string (the C equivalent of `&str`).

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-052)` comments.
2. Implement the simulated C functions (`simulated_add`, `simulated_multiply`) with `#[no_mangle]` and `extern "C"`.
3. Declare them in an `extern "C"` block.
4. Implement `add()` and `multiply()` as safe wrappers that call the `extern` functions.
5. Implement `safe_abs()` to wrap `libc::abs` (or simulate it).
6. Implement `c_string_length()` to wrap `strlen` and return the length of a `&CStr`.
7. Add `# Safety` doc comments to all unsafe wrappers.
8. Run `cargo test -p module-052-exercises` until all tests pass.
9. Compare with `solutions/` afterwards.

### What the real C code would look like

For context, here's what the C side would look like if you were linking a real library:

```c
// mathlib.c
#include <stdlib.h>

int add(int a, int b) {
    return a + b;
}

double multiply(double x, double y) {
    return x * y;
}
```

```c
// mathlib.h
#ifndef MATHLIB_H
#define MATHLIB_H

int add(int a, int b);
double multiply(double x, double y);

#endif
```

You'd compile this with `gcc -c mathlib.c` to get `mathlib.o`, then link it into your Rust binary with `#[link(name = "mathlib")]` (or via a build script). `bindgen` would generate the `extern "C"` declarations from `mathlib.h`.

## Further Reading

- [The Rust Book: FFI](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html#using-extern-functions-to-call-external-code) — calling external code and the C ABI.
- [The Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html) — the full guide to foreign function interfaces.
- [The `bindgen` book](https://rust-lang.github.io/rust-bindgen/) — generating Rust bindings from C headers.
