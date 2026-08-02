# Module 084: WASM Performance Use Cases

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 60–90 min
**Prerequisites:** Module 081 (Introduction to WebAssembly), Module 082 (wasm-bindgen & JS Interop)

## Learning Objectives
- You will be able to implement compute-heavy image processing algorithms (grayscale conversion, box blur) operating on raw pixel buffers
- You will be able to write both naive and optimized versions of the same algorithm and verify they produce identical output
- You will understand why cache-friendly access patterns and reduced bounds checks matter especially in WASM's linear memory model
- You will be able to reason about when WASM outperforms JavaScript for compute-bound tasks

## Why This Matters
When you ship Rust to the browser via WASM, you're competing against JavaScript in a shared environment. For DOM manipulation, JS wins — it has direct access. But for pixel-level image processing, audio DSP, physics simulations, or cryptographic operations, WASM's predictable linear memory and lack of GC pauses give Rust a structural advantage. Companies like Figma, AutoCAD (web), and Google Earth (KML rendering) rely on exactly this pattern: heavy compute in WASM, thin JS glue for DOM events. Understanding how to write cache-friendly, bounds-check-minimized code is what separates a WASM port that's merely correct from one that's meaningfully faster than the JS equivalent.

## Concept

### The WASM Performance Story

When Rust compiles to `wasm32-unknown-unknown`, it produces a single linear memory buffer — essentially a `Vec<u8>` that the WASM runtime manages. Every pointer is an offset into this buffer. There's no garbage collector, no object headers, no indirection through vtables. This is both a superpower and a trap.

The superpower: your data layout is exactly what you designed. A flat `Vec<u8>` of RGB pixels is contiguous bytes in linear memory. When you iterate them sequentially, the CPU prefetcher (even in WASM's virtual environment) sees a predictable stride.

The trap: every array access in safe Rust includes a bounds check. In a tight loop over millions of pixels, those checks add up. The WASM `i32.load` instruction doesn't trap on out-of-bounds — it's the Rust runtime wrapper that panics. So the compiler inserts a check before every access.

### Image Processing Pipeline

Your task is a classic compute-heavy workload: convert an image to grayscale, then apply a box blur. The input is a flat `Vec<u8>` in RGBRGBRGB... order — the same format a browser's `ImageData` uses. Grayscale uses the ITU-R BT.601 luminance formula:

```
gray = 0.299 * R + 0.587 * G + 0.114 * B
```

The box blur averages all pixels within a radius around each output pixel. At the edges, you clamp rather than wrap.

```
Input (3x3, radius=1):       Output pixel at center:
+---+---+---+                avg of all 9 pixels
| 10| 20| 30|
+---+---+---+                Edge pixel (corner, radius=1):
| 40| 50| 60|                avg of 4 pixels (clamped)
+---+---+---+
| 70| 80| 90|
+---+---+---+
```

### Naive vs Optimized

The **naive** version is straightforward: nested loops, index into the buffer, compute. It's correct and clear.

The **optimized** version uses two techniques:

1. **`chunks_exact(3)`** for grayscale: instead of computing `i * 3` and indexing three times, you iterate over non-overlapping 3-byte chunks. The compiler sees a fixed-size slice and can often eliminate the bounds check entirely.

2. **Summed-Area Table (SAT)** for box blur: instead of summing a radius-sized window for every pixel (O(radius²) per pixel), you precompute a 2D prefix sum in O(n). Then each output pixel is four lookups and three arithmetic operations — O(1) regardless of radius.

```
Summed-Area Table concept:
  SAT[y][x] = sum of all pixels in rectangle (0,0)..=(x-1, y-1)

  To query sum in rectangle (x1,y1)..=(x2,y2):
  sum = SAT[y2+1][x2+1] - SAT[y1][x2+1] - SAT[y2+1][x1] + SAT[y1][x1]

  +---+---+---+---+
  | 0 | 0 | 0 | 0 |  <- extra row/col of zeros for indexing
  +---+---+---+---+
  | 0 | a | a+b | ...
  +---+---+---+---+
  | 0 | a+c | a+b+c+d | ...
  +---+---+---+---+
```

### Why This Matters for WASM Specifically

In native Rust, the compiler's auto-vectorizer and LLVM's optimizer can often eliminate bounds checks by proving the indices are in range. In WASM, the same optimizations apply — but the payoff is larger because:

- WASM memory accesses go through a linear buffer with no TLB benefits from page-level caching (it's all one allocation)
- Sequential access patterns are more predictable for the WASM runtime's internal optimizations
- There's no branch predictor benefit from OS-level page fault handling — every check is pure overhead

This is why `chunks_exact`, `array_chunks`, and explicit `get_unchecked` (in `unsafe` blocks when you've proven safety) are common in performance-sensitive WASM code.

### Benchmarking WASM vs JS

You can't run a browser in this module's exercise, but the methodology is the same as [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark):

1. Implement the same algorithm in Rust (compiled to WASM) and JavaScript
2. Use the same input data (same pixel buffer, same dimensions)
3. Run each version many times (hundreds of iterations) to amortize startup cost
4. Measure wall-clock time with `performance.now()` in JS and `std::time::Instant` in Rust (via `web-sys` or host-side)
5. Report median, p95, and p99 — not just mean — because GC pauses in JS and compilation overhead in WASM create bimodal distributions

The `examples/bench_filters.rs` binary in this module runs the native Rust comparison. To compare against JS, you'd compile to WASM, write a thin JS wrapper that calls the exported functions, and measure in the browser console.

## Common Pitfalls
- **Forgetting that `chunks_exact` drops the remainder.** If your buffer length isn't divisible by 3, the last partial pixel is silently skipped. For RGB images this shouldn't happen, but validate your input length.
- **Integer overflow in the SAT.** A 4K image has 12M pixels; summing even `u8` values across a large rectangle can exceed `u32::MAX`. Use `u64` for the SAT accumulators.
- **Off-by-one in SAT queries.** The SAT is indexed with a +1 offset (row 0 / col 0 are zeros). Forgetting this gives wrong results at the top-left edge.
- **Confusing radius with diameter.** A box blur of radius 1 covers a 3×3 window. Radius 2 covers 5×5. The SAT query rectangle is `[x-r, x+r] × [y-r, y+r]`.
- **Not clamping at edges.** The blur window extends past the image boundary. You must clamp `x_min`, `x_max`, `y_min`, `y_max` to `[0, width-1]` and `[0, height-1]`.

## Key Terms
- **Linear memory:** WASM's single contiguous byte buffer; all pointers are offsets into it.
- **Summed-Area Table (SAT):** A 2D prefix-sum array enabling O(1) rectangular sum queries.
- **Bounds check elimination:** Compiler optimization that removes runtime index validation when it can prove safety statically.
- **`chunks_exact(n)`:** Iterator adapter that yields non-overlapping slices of length `n`, dropping any remainder.
- **Luminance:** Perceived brightness of a pixel; the grayscale conversion weights R/G/B by human eye sensitivity.

## Exercise

In `exercises/src/lib.rs`, implement the six functions:

1. `grayscale_naive` — pixel-by-pixel loop, compute luminance, write R=G=B=gray
2. `grayscale_optimized` — same result, using `chunks_exact(3)`
3. `box_blur_naive` — nested loops, average the window, clamp at edges
4. `box_blur_optimized` — same result, using a summed-area table
5. `pipeline_naive` — compose grayscale_naive + box_blur_naive(radius=1)
6. `pipeline_optimized` — compose grayscale_optimized + box_blur_optimized(radius=1)

The tests verify:
- Correct luminance values for known inputs
- Naive and optimized versions produce identical output
- Pipeline output is grayscale (R=G=B for every pixel)
- Edge cases (single-pixel image, radius=0)

Run the example to see timing:
```bash
cargo run --example bench_filters --release
```

Sample output on this machine (256×256 image, 10 iterations):
```
Image: 256x256 (65536 pixels)
Iterations: 10
Naive pipeline:     34.28ms
Optimized pipeline: 20.28ms
Speedup: 1.69x
```

The optimized version is ~1.7× faster even in native Rust. In WASM, the gap typically widens because linear memory access patterns and bounds-check elimination matter more without OS-level memory management.

## Running This Module's Tests

```bash
cargo test -p module-084-exercises    # must fail (TODOs not filled)
cargo test -p module-084-solutions    # must pass
```

## Further Reading
- [The Rust Performance Book — WASM](https://nnethercote.github.io/perf-book/wasm.html)
- [MDN — WebAssembly performance](https://developer.mozilla.org/en-US/docs/WebAssembly/Understanding_the_text_format)
- [js-framework-benchmark](https://github.com/krausest/js-framework-benchmark) — methodology for comparing JS framework performance
- [Rust Book §15 — Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html) — `Vec<T>` memory layout is relevant to understanding linear memory
