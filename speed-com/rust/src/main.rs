use std::time::Instant;

fn main() {
    let start = Instant::now();
    
    // Using a 64-bit integer to prevent overflow
    // 0..=1_000_000_000 is an inclusive range
    let sum: u64 = (0..=1_000_000_000).sum();
    
    let duration = start.elapsed();
    
    println!("Sum: {}", sum);
    println!("Time taken: {:?}", duration);
}