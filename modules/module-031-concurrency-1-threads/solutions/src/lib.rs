//! Module 031: reference solution.
//!
//! `spawn` one thread per unit of work, `join` everything, collect results.

/// Spawns one thread per element of `inputs`. Each thread squares its element
/// and the function returns the results in the same order as the inputs.
pub fn compute_in_parallel(inputs: Vec<u32>) -> Vec<u32> {
    let mut handles = Vec::new();
    for input in inputs {
        handles.push(std::thread::spawn(move || input * input));
    }
    handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .collect()
}

/// Sums the squares of `1..=n` using `threads` worker threads. Thread `t`
/// handles the strided slice `t+1, t+1+threads, t+1+2*threads, ...`. Returns
/// the total, which equals the sequential sum for any `n` and `threads`.
pub fn sum_squares_parallel(n: u32, threads: usize) -> u64 {
    let mut handles = Vec::new();
    for t in 0..threads {
        handles.push(std::thread::spawn(move || {
            let mut sum: u64 = 0;
            let mut value = t as u32 + 1;
            while value <= n {
                sum += (value as u64) * (value as u64);
                value += threads as u32;
            }
            sum
        }));
    }
    let mut total: u64 = 0;
    for handle in handles {
        total += handle.join().unwrap();
    }
    total
}
