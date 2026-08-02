//! Module 057: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Sum all elements in `data`, processing in chunks of 4 to emulate SIMD lanes.
///
/// The algorithm:
/// - Use `chunks_exact(4)` to get full 4-element chunks.
/// - Sum the 4 elements of each chunk in the inner loop.
/// - Add any remaining elements (the `remainder()`) with a scalar fallback.
pub fn sum_vectorized(data: &[i64]) -> i64 {
    let _ = data;
    panic!("TODO(module-057): implement sum_vectorized")
}

/// Compare `a` and `b` element-wise, processing in chunks of 4.
///
/// Returns a `Vec<bool>` where each entry is `a[i] == b[i]`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
pub fn compare_vectorized(a: &[i64], b: &[i64]) -> Vec<bool> {
    let _ = (a, b);
    panic!("TODO(module-057): implement compare_vectorized")
}

/// Compute the dot product of `a` and `b`, processing in chunks of 4.
///
/// The dot product is `a[0]*b[0] + a[1]*b[1] + ... + a[n-1]*b[n-1]`.
///
/// # Panics
///
/// Panics if `a` and `b` have different lengths.
pub fn dot_product_chunked(a: &[f64], b: &[f64]) -> f64 {
    let _ = (a, b);
    panic!("TODO(module-057): implement dot_product_chunked")
}
