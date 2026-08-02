# Module 057: SIMD & Low-Level Optimization

**Block:** Block F — Systems Programming & Performance
**Estimated time:** 45–75 min
**Prerequisites:** Module 022 (Iterators I), Module 056 (Zero-Cost Abstractions & Optimization)

## Learning Objectives

- You will be able to explain what SIMD (Single Instruction, Multiple Data) means at a conceptual level.
- You will be able to write chunked (unrolled) loops that emulate SIMD processing on multiple elements per iteration.
- You will be able to handle remainder elements when the data size is not a multiple of the chunk size.
- You will understand when manual loop unrolling helps (and when the compiler already does it for you).

## Why This Matters

SIMD instructions (like SSE, AVX, and NEON) let the CPU perform the same operation on multiple values simultaneously — e.g., adding 4 `i64` values in one instruction instead of 4 separate `add` instructions. While Rust has `std::simd` (nightly only) and crates like `wide` for explicit SIMD, the *pattern* of chunked processing applies everywhere: processing data in fixed-size batches reduces loop overhead and improves cache behavior, even without SIMD intrinsics. This module teaches the chunked-processing pattern using plain Rust — the same mental model you'll use when you work with real SIMD, GPU compute shaders, or data-parallel iterators in `rayon`.

## Concept

### What SIMD means

Imagine you have two arrays and you want to add them element-wise:

```
a = [1, 2, 3, 4, 5, 6, 7, 8]
b = [10, 20, 30, 40, 50, 60, 70, 80]
```

Without SIMD, the CPU executes 8 separate `add` instructions:

```
add r1 + r10 → r1'
add r2 + r20 → r2'
... (6 more)
```

With SIMD (e.g., 4-lane integer vectors), the CPU packs 4 values into a wide register and adds them in one instruction:

```
[1, 2, 3, 4] + [10, 20, 30, 40] = [11, 22, 33, 44]   // one SIMD add
[5, 6, 7, 8] + [50, 60, 70, 80] = [55, 66, 77, 88]   // one SIMD add
```

### Emulating SIMD with chunked loops

Since actual SIMD requires either nightly Rust or platform-specific intrinsics, we emulate the pattern with plain Rust loops that process 4 elements per iteration:

```rust
fn sum_vectorized(data: &[i64]) -> i64 {
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();

    let mut sum = 0i64;
    for chunk in chunks {
        // Process 4 elements at once — emulate SIMD lane computation
        sum += chunk[0] + chunk[1] + chunk[2] + chunk[3];
    }
    // Handle any remaining elements
    for &val in remainder {
        sum += val;
    }
    sum
}
```

This doesn't actually use SIMD instructions — the compiler might auto-vectorize it, or it might not. But the **pattern** (fixed-size chunking + remainder handling) is exactly what SIMD code looks like. When you graduate to real SIMD, the only change is that `chunk[0] + chunk[1] + chunk[2] + chunk[3]` becomes a single intrinsic call like `_mm_add_epi64(chunk_a, chunk_b)`.

### The remainder problem

The hardest part of SIMD-style processing is handling data that isn't a multiple of the lane count. If you have 10 elements and process in chunks of 4:

```
[0  1  2  3]  [4  5  6  7]  [8  9]
 └──chunk 0──┘  └──chunk 1──┘  └─remainder─┘
```

The standard pattern uses `chunks_exact()` (which skips incomplete trailing chunks) and `remainder()` (which gives you the leftover elements as a flat slice). You process full chunks in the SIMD-style loop, then handle the remaining 0–3 elements with a scalar fallback.

### Chunked dot product

Dot product is an even better example because it combines multiplication and accumulation — the inner loop mirrors what SIMD fused multiply-add does:

```rust
fn dot_product_chunked(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len());

    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let remainder_a = chunks_a.remainder();
    let remainder_b = chunks_b.remainder();

    let mut sum = 0.0;
    for (chunk_a, chunk_b) in chunks_a.zip(chunks_b) {
        // Emulate 4-lane SIMD multiply-add
        sum += chunk_a[0] * chunk_b[0]
             + chunk_a[1] * chunk_b[1]
             + chunk_a[2] * chunk_b[2]
             + chunk_a[3] * chunk_b[3];
    }
    for (a, b) in remainder_a.iter().zip(remainder_b.iter()) {
        sum += a * b;
    }
    sum
}
```

### Chunked comparison

Element-wise comparison (`a[0] == b[0]`, `a[1] == b[1]`, ...) also maps naturally to chunked processing:

```rust
fn compare_vectorized(a: &[i64], b: &[i64]) -> Vec<bool> {
    assert_eq!(a.len(), b.len());

    let mut result = Vec::with_capacity(a.len());
    for (va, vb) in a.iter().zip(b.iter()) {
        // Compare chunks of 4 at a time
        result.push(*va == *vb);
    }
    result
}
```

While this example processes one pair at a time, the exercise asks you to structure the loop with `chunks_exact(4)` — emulating the access pattern that SIMD code uses. The test verifies correctness regardless of stride.

### ASCII diagram: chucked processing flow

```
Input: [a0, a1, a2, a3, a4, a5, a6, a7, a8, a9]
                     │
        ┌────────────┴────────────┐
        ▼                         ▼
chunks_exact(4)              remainder()
[a0, a1, a2, a3]             [a8, a9]
[a4, a5, a6, a7]
        │                         │
        ▼                         ▼
Process 4-wide:              Scalar fallback:
sum += c[0]+c[1]+c[2]+c[3]   sum += a8 + a9
        │                         │
        └────────────┬────────────┘
                     ▼
                Final result
```

### Does the compiler auto-vectorize plain loops?

Often yes — LLVM (Rust's backend) is good at auto-vectorizing simple counted loops. But complex loops, loops with control flow, or loops with non-contiguous access may not auto-vectorize. The chunked pattern gives you explicit control: you decide the chunk size, you handle the remainder, and you can drop in actual SIMD intrinsics later without restructuring the code.

### When to reach for SIMD

- **Hot numeric loops** in data processing, image/video codecs, scientific computing.
- **When the data is dense and contiguous** — SIMD works on packed registers; scattered access kills its advantage.
- **When the operation is uniform** — same instruction on all lanes. Conditional logic per element usually requires masking or scalar fallback.

For most Rust code, the standard library and compiler optimizations are sufficient. You reach for explicit SIMD only when profiling (Module 054) shows a hot loop that manual vectorization can improve.

## Common Pitfalls

- **Off-by-one in remainder handling.** After `chunks_exact(N)`, the remainder has 0 to N-1 elements. A common mistake is assuming the input length is always a multiple of N.
- **Indexing past the chunk bound.** If you hardcode `chunk[3]` on a 4-element chunk, be sure you're calling `chunks_exact(4)` (which guarantees full chunks) — not `chunks(4)` (which gives a shorter final chunk).
- **Comparing slices of different lengths.** Element-wise comparison requires equal lengths. Assert or return an error.
- **Trying to use nightly-only `std::simd` in stable code.** This module uses plain Rust chunked loops. Real SIMD requires nightly Rust or external crates like `wide`.

## Key Terms

- **SIMD (Single Instruction, Multiple Data):** a CPU feature that performs the same operation on multiple values in one instruction.
- **Vectorization:** transforming scalar operations into SIMD operations (either manually or by the compiler).
- **Lane:** one element position within a SIMD vector (e.g., a 4-lane vector holds 4 values).
- **Chunked processing:** processing data in fixed-size groups, typically matching SIMD lane count.
- **Remainder / tail:** leftover elements that don't fill a full chunk, processed separately with scalar code.
- **Auto-vectorization:** the compiler's automatic transformation of loops into SIMD instructions.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the `// TODO(module-057)` comments.
2. Implement `sum_vectorized()` — sum all elements, processing in chunks of 4.
3. Implement `compare_vectorized()` — compare two slices element-wise, processing in chunks of 4. Panic if lengths differ.
4. Implement `dot_product_chunked()` — compute the dot product, processing in chunks of 4.
5. Run `cargo test -p module-057-exercises` until all tests pass.
6. Compare with `solutions/` afterwards.

## Further Reading

- [The Rust Performance Book — SIMD](https://nnethercote.github.io/perf-book/simd.html) — SIMD guidance for Rust.
- [`std::slice::chunks_exact` documentation](https://doc.rust-lang.org/std/primitive.slice.html#method.chunks_exact) — the core method for chunked access.
- [The `wide` crate](https://crates.io/crates/wide) — portable SIMD types for stable Rust.
- [`std::simd` (nightly-only)](https://doc.rust-lang.org/std/simd/index.html) — the future of portable SIMD in Rust.
