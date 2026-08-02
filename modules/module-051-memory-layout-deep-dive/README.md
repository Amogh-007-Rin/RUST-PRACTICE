# Module 051: Memory Layout Deep Dive

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 60–90 min
**Prerequisites:** Module 035 (Unsafe Rust I), Module 007 (Structs)

## Learning Objectives

- You will be able to compute the size and alignment of a struct using `std::mem::size_of` and `std::mem::align_of`.
- You will be able to predict the padding bytes the compiler inserts between struct fields.
- You will be able to use `#[repr(C)]` to force C-compatible layout and explain when it matters.
- You will be able to reorder fields to minimize padding and shrink a struct's footprint.
- You will be able to manually compute field offsets using pointer arithmetic (the `offset_of` pattern).

## Why This Matters

When you call a C library, serialize a struct to disk, or write a driver that talks to hardware registers, the *exact byte layout* of your data matters. Rust's default `#[repr(Rust)]` lets the compiler reorder and pad fields for performance — which is great until you need a stable ABI. Knowing how layout works lets you write FFI bindings that don't silently corrupt memory, design cache-friendly data structures, and understand why `size_of::<(u8, u64)>()` is 16, not 9. This is the knowledge that separates "it compiles" from "it works on every platform."

## Concept

### Size and alignment: the basics

Every type in Rust has two layout properties:

- **Size** (`size_of::<T>()`): how many bytes an instance occupies.
- **Alignment** (`align_of::<T>()`): the address must be a multiple of this (powers of two: 1, 2, 4, 8, 16...).

```rust
use std::mem::{size_of, align_of};

fn main() {
    println!("u8:  size={}, align={}", size_of::<u8>(), align_of::<u8>());
    println!("u32: size={}, align={}", size_of::<u32>(), align_of::<u32>());
    println!("u64: size={}, align={}", size_of::<u64>(), align_of::<u64>());
}
```

Output on a 64-bit machine:

```
u8:  size=1, align=1
u32: size=4, align=4
u64: size=8, align=8
```

Alignment is a *constraint*: a `u64` must live at an address divisible by 8. The CPU enforces this — misaligned access is slower or (on some architectures) a hardware fault.

### Struct layout and padding

A struct's size is *not* the sum of its fields. The compiler inserts **padding** to satisfy each field's alignment. Consider:

```rust
use std::mem::size_of;

struct Messy {
    a: u8,   // 1 byte
    b: u64,  // 8 bytes, needs align=8
    c: u8,   // 1 byte
}

fn main() {
    println!("{}", size_of::<Messy>()); // 24, not 10
}
```

Here's what happens:

```
Offset  Field   Size   Padding
0       a       1      7 bytes (to align b to 8)
8       b       8      0
16      c       1      7 bytes (to make struct size a multiple of 8)
```

The struct's alignment is the *maximum* alignment of its fields (8 here), so its total size must be a multiple of 8. The compiler pads the end to satisfy this.

### Reordering fields to shrink a struct

You can eliminate padding by grouping fields by alignment:

```rust
use std::mem::size_of;

struct Tidy {
    b: u64,  // 8 bytes, align=8
    a: u8,   // 1 byte
    c: u8,   // 1 byte
}

fn main() {
    println!("{}", size_of::<Tidy>()); // 16, not 24
}
```

Layout:

```
Offset  Field   Size   Padding
0       b       8      0
8       a       1      0
9       c       1      6 bytes (to make size a multiple of 8)
```

Six bytes of trailing padding remain (to make the size a multiple of the struct's alignment), but you saved 8 bytes compared to `Messy`. For arrays of structs, this adds up.

### `#[repr(C)]`: forcing C-compatible layout

Rust's default `#[repr(Rust)]` lets the compiler reorder fields however it likes (it won't change the *observable* behavior of safe code, but it *will* change the byte layout). When you need a stable layout — for FFI, for memory-mapped hardware, for casting a byte slice to a struct — use `#[repr(C)]`:

```rust
#[repr(C)]
struct CStyle {
    a: u8,
    b: u64,
    c: u8,
}
```

`#[repr(C)]` guarantees fields appear in declaration order, with C's padding rules (which happen to match what we described above). This is what `bindgen` generates when it wraps a C header.

### Computing field offsets

The offset of a field is its byte position within the struct. You can compute it manually using pointer arithmetic:

```rust
use std::mem::{size_of, align_of};

#[repr(C)]
struct Sample {
    x: u8,
    y: u64,
    z: u32,
}

fn offset_of_y() -> usize {
    let uninit = std::mem::MaybeUninit::<Sample>::uninit();
    let base = uninit.as_ptr();
    let field = unsafe { std::ptr::addr_of!((*base).y) };
    (field as usize) - (base as usize)
}

fn main() {
    println!("size_of::<Sample>() = {}", size_of::<Sample>());
    println!("offset of y = {}", offset_of_y());
}
```

This uses `MaybeUninit` to avoid constructing a real value, and `addr_of!` to get the field's address without dereferencing. The offset is the difference between the field's address and the struct's base address.

For `Sample` above (on 64-bit):

```
Offset  Field   Size   Padding
0       x       1      7 bytes
8       y       8      0
16      z       4      4 bytes (trailing padding to align to 8)
```

So `size_of::<Sample>()` is 24, and `offset_of_y()` is 8.

### Why `repr(Rust)` reorders

The compiler is free to reorder fields in `#[repr(Rust)]` to minimize padding. It won't change the *behavior* of safe code (you can't observe field order through safe Rust), but it can shrink the struct. If you need a specific order, use `#[repr(C)]`.

### ASCII diagram: struct layout with padding

```
struct Messy { a: u8, b: u64, c: u8 }

Byte offset:  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15 16 17 18 19 20 21 22 23
              [a][pad pad pad pad pad pad pad][ b  b  b  b  b  b  b  b ][c][pad pad pad pad pad pad pad]
              ^                                 ^                                 ^
              offset 0                          offset 8                          offset 16

struct Tidy { b: u64, a: u8, c: u8 }

Byte offset:  0  1  2  3  4  5  6  7  8  9 10 11 12 13 14 15
              [ b  b  b  b  b  b  b  b ][a][c][pad pad pad pad pad pad]
              ^                          ^  ^
              offset 0                   offset 8  offset 9
```

### When layout matters

- **FFI**: C expects fields in declaration order. Use `#[repr(C)]`.
- **Hardware registers**: memory-mapped I/O requires exact offsets. Use `#[repr(C)]` and `volatile` reads.
- **Serialization**: if you cast a `&[u8]` to a `&T`, the bytes must match `T`'s layout. Prefer explicit parsing (Module 075) unless you control the format.
- **Performance**: reordering fields can shrink arrays of structs, improving cache usage.

### The exercise in a sentence each

- `size_of_struct()` — compute the size of a struct with mixed-alignment fields.
- `align_of_struct()` — return the alignment of a struct.
- `offset_of_field()` — manually compute a field's byte offset.
- `repr_c_layout()` — verify that `#[repr(C)]` produces the expected layout.
- `optimized_layout()` — reorder fields to minimize padding.

The tests assert exact sizes and offsets. You'll use `size_of`, `align_of`, and pointer arithmetic. All of this returns in Module 058 (embedded), where hardware registers demand exact layout.

## Common Pitfalls

- **Assuming `size_of` is the sum of field sizes.** Padding exists. Measure with `size_of`, don't guess.
- **Forgetting trailing padding.** A struct's size is padded to a multiple of its alignment, even if the last field is small.
- **Using `#[repr(Rust)]` for FFI.** The compiler can reorder fields. Always use `#[repr(C)]` for C-compatible layout.
- **Dereferencing uninitialized memory to compute offsets.** Use `MaybeUninit` and `addr_of!` to avoid undefined behavior.
- **Assuming pointer width is always 64-bit.** Use `#[cfg(target_pointer_width = "64")]` for pointer-sized assertions.

## Key Terms

- **size_of::<T>():** the number of bytes a `T` occupies.
- **align_of::<T>():** the alignment constraint of `T` (address must be a multiple of this).
- **padding:** bytes inserted between fields (or at the end) to satisfy alignment.
- **`#[repr(C)]`:** forces C-compatible field order and padding.
- **`#[repr(Rust)]`:** the default; lets the compiler reorder fields for optimization.
- **offset_of:** the byte position of a field within a struct.
- **`MaybeUninit<T>`:** a wrapper for potentially uninitialized data, used for low-level memory manipulation.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-051)` comments.
2. Implement `size_of_messy()` to return the size of `Messy` (use `size_of`).
3. Implement `align_of_messy()` to return the alignment of `Messy`.
4. Implement `offset_of_y()` to compute the byte offset of field `y` in `Sample` using pointer arithmetic.
5. Implement `size_of_repr_c()` to verify the size of a `#[repr(C)]` struct.
6. Implement `optimized_size()` by reordering fields in `Optimized` to minimize padding.
7. Run `cargo test -p module-051-exercises` until all tests pass.
8. Compare with `solutions/` afterwards.

## Further Reading

- [The Rustonomicon: Representations](https://doc.rust-lang.org/nomicon/other-reprs.html) — `#[repr(C)]`, `#[repr(transparent)]`, and more.
- [The Rust Reference: Type layout](https://doc.rust-lang.org/reference/type-layout.html) — the formal rules for size, alignment, and padding.
- [Rust Blog: std::mem::offset_of!](https://blog.rust-lang.org/2022/08/05/nll.html) — the stabilization of `offset_of!` (nightly at time of writing, but the manual pattern works on stable).
