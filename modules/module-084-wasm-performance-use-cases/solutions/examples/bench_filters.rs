use std::time::Instant;

fn main() {
    use module_084_solutions::*;

    let width = 256u32;
    let height = 256u32;
    let size = (width * height) as usize;
    let mut pixels = vec![0u8; size * 3];
    for (i, byte) in pixels.iter_mut().enumerate() {
        *byte = (i % 256) as u8;
    }

    let iterations = 10;

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = pipeline_naive(&pixels, width, height);
    }
    let naive_time = start.elapsed();

    let start = Instant::now();
    for _ in 0..iterations {
        let _ = pipeline_optimized(&pixels, width, height);
    }
    let opt_time = start.elapsed();

    println!("Image: {width}x{height} ({size} pixels)");
    println!("Iterations: {iterations}");
    println!("Naive pipeline:     {naive_time:?}");
    println!("Optimized pipeline: {opt_time:?}");
    if opt_time.as_nanos() > 0 {
        let speedup = naive_time.as_nanos() as f64 / opt_time.as_nanos() as f64;
        println!("Speedup: {speedup:.2}x");
    }
    println!();
    println!("When compiled to WASM, these same algorithms run in the browser.");
    println!("The optimized version's reduced bounds checks and cache-friendly");
    println!("access patterns matter even more in WASM due to linear memory model.");
}
