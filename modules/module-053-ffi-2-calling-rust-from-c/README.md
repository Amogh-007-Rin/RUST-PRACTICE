# Module 053: FFI II — Calling Rust from C

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 60–90 min
**Prerequisites:** Module 052 (FFI I), Module 036 (Unsafe Rust II)

## Learning Objectives

- You will be able to export Rust functions with `#[no_mangle]` and `extern "C"` so C code can call them.
- You will be able to explain what `cbindgen` does and how it generates C headers from Rust code.
- You will be able to write a C caller that invokes Rust functions through a header file.
- You will be able to handle Rust types across the FFI boundary (converting to/from C-compatible types).
- You will be able to build a `cdylib` or `staticlib` crate for linking into C projects.

## Why This Matters

Sometimes the arrow of dependency points the other way: you have a C application that needs to call into Rust. Maybe you're replacing a performance-critical C module with Rust, or adding Rust's safety guarantees to a legacy C codebase. The technique is the mirror of Module 052: you export Rust functions with C ABI using `#[no_mangle]` and `extern "C"`, then generate a C header (manually or with `cbindgen`) so the C side knows the function signatures. This is how `librespot` (Spotify's Rust library) is called from C, how Firefox's Rust components integrate with the C++ codebase, and how you'd ship a Rust library to C users.

## Concept

### Exporting Rust functions with C ABI

To make a Rust function callable from C, you need two things:

1. **`#[no_mangle]`**: prevents the compiler from mangling the function name (C doesn't support overloading, so names must match exactly).
2. **`extern "C"`**: specifies the C calling convention (how arguments are passed, who cleans up the stack, etc.).

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}
```

This function can now be called from C as `rust_add(3, 4)`. The `pub` is required because the function is part of the crate's public ABI.

### The C header

C needs a header file declaring the function's signature:

```c
// rustlib.h
#ifndef RUSTLIB_H
#define RUSTLIB_H

#include <stdint.h>

int32_t rust_add(int32_t a, int32_t b);

#endif
```

The C code includes this header and calls the function:

```c
// main.c
#include "rustlib.h"
#include <stdio.h>

int main() {
    int32_t result = rust_add(3, 4);
    printf("Result: %d\n", result);
    return 0;
}
```

You compile the Rust crate as a C-compatible library (`cdylib` for dynamic, `staticlib` for static), then link it with the C code:

```bash
cargo build --release
gcc main.c -L target/release -lrustlib -o main
./main
```

### What `cbindgen` does

Writing C headers by hand is error-prone. `cbindgen` is a tool that parses your Rust code and generates the corresponding C header automatically. You run it as a build step or CLI tool:

```bash
cbindgen --config cbindgen.toml --crate my-rust-lib --output my-rust-lib.h
```

Given this Rust code:

```rust
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[repr(C)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

#[no_mangle]
pub extern "C" fn point_distance(p: *const Point) -> f64 {
    (p.x * p.x + p.y * p.y).sqrt()
}
```

`cbindgen` generates:

```c
#ifndef RUSTLIB_H
#define RUSTLIB_H

#include <stdint.h>
#include <stdbool.h>

typedef struct {
    double x;
    double y;
} Point;

int32_t rust_add(int32_t a, int32_t b);
double point_distance(const Point *p);

#endif
```

The `#[repr(C)]` on `Point` ensures the struct has C-compatible layout (Module 051). `cbindgen` handles the type mapping (`i32` → `int32_t`, `f64` → `double`, `*const T` → `const T*`).

### Simulating the C caller in Rust

For this module, we'll simulate the C side by declaring the exported Rust functions in an `extern "C"` block within the same crate, then calling them from a test. This teaches the export mechanism without requiring a C compiler.

```rust
// Export a Rust function with C ABI
#[no_mangle]
pub extern "C" fn rust_multiply(a: f64, b: f64) -> f64 {
    a * b
}

// Simulate the C caller (in real code, this would be a separate C file)
extern "C" {
    fn rust_multiply(a: f64, b: f64) -> f64;
}

#[test]
fn test_via_ffi() {
    let result = unsafe { rust_multiply(3.0, 4.0) };
    assert_eq!(result, 12.0);
}
```

The `extern "C"` block declares the function as if it were external, even though it's defined in the same crate. This mirrors the real FFI pattern: the Rust side exports with `#[no_mangle]`, and the caller (C or simulated) declares it in an `extern` block.

### Handling types across the FFI boundary

Not all Rust types have a direct C equivalent. You must convert at the boundary:

| Rust type | C equivalent | Notes |
|---|---|---|
| `i32`, `u32`, `i64`, `u64` | `int32_t`, `uint32_t`, etc. | Direct mapping |
| `f32`, `f64` | `float`, `double` | Direct mapping |
| `bool` | `bool` (from `<stdbool.h>`) | Direct mapping |
| `*const T`, `*mut T` | `const T*`, `T*` | Raw pointers |
| `&T`, `&mut T` | `const T*`, `T*` | Convert to raw pointers |
| `String`, `&str` | `const char*` | Must be null-terminated, UTF-8 |
| `Vec<T>` | `T*` + length | Pass as pointer + size |
| `Option<T>` | `T*` or nullable | `None` → null pointer |
| Structs | `struct` | Must be `#[repr(C)]` |

For strings, you typically accept a `*const c_char` (C string) and convert to `&CStr` or `&str`:

```rust
use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    let c_str = unsafe { CStr::from_ptr(s) };
    c_str.to_str().map(|s| s.len()).unwrap_or(0)
}
```

For structs, use `#[repr(C)]` to ensure C-compatible layout:

```rust
#[repr(C)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

#[no_mangle]
pub extern "C" fn rectangle_area(r: *const Rectangle) -> f64 {
    if r.is_null() {
        return 0.0;
    }
    let r = unsafe { &*r };
    r.width * r.height
}
```

### Building a C-compatible library

To link your Rust code into a C project, you need to build it as a C-compatible library. In `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]  # Dynamic library (.so/.dll/.dylib)
# or
crate-type = ["staticlib"]  # Static library (.a/.lib)
```

`cdylib` produces a shared library (like `libmylib.so` on Linux). `staticlib` produces a static archive (like `libmylib.a`). Use `cdylib` for dynamic linking, `staticlib` for static linking.

For this module, we'll keep the default `rlib` (Rust library) since we're simulating the C caller in Rust. But the README shows how to build a real `cdylib`.

### ASCII diagram: the FFI call stack (Rust → C)

```
C code                      C ABI boundary              Rust code
──────                      ───────────────              ─────────

int main() {                                          
  int32_t r = rust_add(3, 4);                         
       │                                              
       ▼                                              
  [C function call] ──────►  #[no_mangle]              
                             pub extern "C"            
                             fn rust_add(a, b) {       
                               a + b                   
                             }                         
                                    │                  
                                    ▼                  
                              returns i32              
                                    │                  
       ◄────────────────────────────┘                  
  r = 7                                                
}                                                      
```

The C code calls `rust_add` as if it were a C function. The Rust side exports it with `#[no_mangle]` and `extern "C"` to match the C ABI. The linker connects the two.

### The exercise in a sentence each

- `rust_add()` — export a Rust function that adds two integers.
- `rust_multiply()` — export a Rust function that multiplies two floats.
- `rust_strlen()` — export a Rust function that computes the length of a C string.
- `rust_rectangle_area()` — export a Rust function that computes the area of a `#[repr(C)]` struct.

The tests call your exported functions through an `extern "C"` block (simulating the C caller). You'll use `#[no_mangle]`, `extern "C"`, and handle raw pointers and `#[repr(C)]` structs.

## Common Pitfalls

- **Forgetting `#[no_mangle]`.** Without it, the compiler mangles the function name, and C can't find the symbol.
- **Forgetting `extern "C"`.** Without it, the function uses the Rust ABI, which is incompatible with C.
- **Not using `#[repr(C)]` on structs.** Rust's default layout can reorder fields. C expects declaration order.
- **Passing `&str` or `String` directly.** C doesn't understand Rust's string types. Convert to `*const c_char` (null-terminated).
- **Dereferencing raw pointers without `unsafe`.** Raw pointer dereference is always `unsafe`.
- **Not handling null pointers.** C strings and struct pointers can be null. Check for null before dereferencing.

## Key Terms

- **`#[no_mangle]`:** prevents name mangling, so the function's symbol matches its name.
- **`extern "C"`:** specifies the C calling convention for a function.
- **`cbindgen`:** a tool that generates C headers from Rust code.
- **`cdylib`:** a crate type that produces a C-compatible dynamic library.
- **`staticlib`:** a crate type that produces a C-compatible static library.
- **`#[repr(C)]`:** forces C-compatible struct layout (field order and padding).
- **`CStr`:** a borrowed reference to a null-terminated C string.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-053)` comments.
2. Implement `rust_add()` with `#[no_mangle]` and `extern "C"`.
3. Implement `rust_multiply()` with `#[no_mangle]` and `extern "C"`.
4. Implement `rust_strlen()` to compute the length of a C string (handle null pointers).
5. Implement `rust_rectangle_area()` to compute the area of a `#[repr(C)]` `Rectangle` struct (handle null pointers).
6. Run `cargo test -p module-053-exercises` until all tests pass.
7. Compare with `solutions/` afterwards.

### What the real C code would look like

For context, here's what the C caller would look like:

```c
// rustlib.h (generated by cbindgen or written by hand)
#ifndef RUSTLIB_H
#define RUSTLIB_H

#include <stdint.h>
#include <stddef.h>

int32_t rust_add(int32_t a, int32_t b);
double rust_multiply(double a, double b);
size_t rust_strlen(const char *s);

typedef struct {
    double width;
    double height;
} Rectangle;

double rust_rectangle_area(const Rectangle *r);

#endif
```

```c
// main.c
#include "rustlib.h"
#include <stdio.h>

int main() {
    printf("add: %d\n", rust_add(3, 4));
    printf("multiply: %.2f\n", rust_multiply(3.0, 4.0));
    printf("strlen: %zu\n", rust_strlen("hello"));
    
    Rectangle r = {5.0, 3.0};
    printf("area: %.2f\n", rust_rectangle_area(&r));
    
    return 0;
}
```

Compile the Rust crate as a `cdylib`:

```bash
# In Cargo.toml: crate-type = ["cdylib"]
cargo build --release
```

Then compile and link the C code:

```bash
gcc main.c -L target/release -lmodule_053_exercises -o main
./main
```

## Further Reading

- [The Rustonomicon: FFI](https://doc.rust-lang.org/nomicon/ffi.html) — the full guide to foreign function interfaces.
- [The `cbindgen` guide](https://github.com/eqrion/cbindgen) — generating C headers from Rust.
- [Rust FFI Omnibus](http://jvns.ca/blog/2015/01/11/rust-ffi-omnibus/) — a collection of FFI examples (Rust calling C, C calling Rust).
