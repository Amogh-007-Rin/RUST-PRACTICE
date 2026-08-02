//! Module 031: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Spawns one thread per element of `inputs`. Each thread squares its element
/// and the function returns the results in the same order as the inputs.
pub fn compute_in_parallel(_inputs: Vec<u32>) -> Vec<u32> {
    // TODO(module-031): rename `_inputs` to `inputs`, then spawn one thread
    // per element with a `move` closure that squares the element; collect
    // every `JoinHandle` first. Then join every handle (unwrapping the
    // result) and collect the values in input order.
    panic!("TODO(module-031): implement compute_in_parallel")
}

/// Sums the squares of `1..=n` using `threads` worker threads. Thread `t`
/// handles the strided slice `t+1, t+1+threads, t+1+2*threads, ...`. Returns
/// the total, which must equal the sequential sum for any `n` and `threads`.
pub fn sum_squares_parallel(_n: u32, _threads: usize) -> u64 {
    // TODO(module-031): rename `_n` and `_threads` (dropping the underscores),
    // then spawn one thread per worker in 0..threads; each thread sums the
    // squares of its strided slice and returns the partial sum. Join all
    // handles in a second loop and add the partials together.
    panic!("TODO(module-031): implement sum_squares_parallel")
}
