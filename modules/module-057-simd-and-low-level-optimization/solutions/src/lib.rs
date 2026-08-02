//! Module 057: solution — the reference implementation.

pub fn sum_vectorized(data: &[i64]) -> i64 {
    let chunks = data.chunks_exact(4);
    let remainder = chunks.remainder();

    let mut sum: i64 = 0;
    for chunk in chunks {
        sum += chunk[0] + chunk[1] + chunk[2] + chunk[3];
    }
    for &val in remainder {
        sum += val;
    }
    sum
}

pub fn compare_vectorized(a: &[i64], b: &[i64]) -> Vec<bool> {
    assert_eq!(a.len(), b.len(), "slices have different lengths");

    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let remainder_a = chunks_a.remainder();
    let remainder_b = chunks_b.remainder();

    let mut result = Vec::with_capacity(a.len());
    for (ca, cb) in chunks_a.zip(chunks_b) {
        result.push(ca[0] == cb[0]);
        result.push(ca[1] == cb[1]);
        result.push(ca[2] == cb[2]);
        result.push(ca[3] == cb[3]);
    }
    for (ra, rb) in remainder_a.iter().zip(remainder_b.iter()) {
        result.push(ra == rb);
    }
    result
}

pub fn dot_product_chunked(a: &[f64], b: &[f64]) -> f64 {
    assert_eq!(a.len(), b.len(), "slices have different lengths");

    let chunks_a = a.chunks_exact(4);
    let chunks_b = b.chunks_exact(4);
    let remainder_a = chunks_a.remainder();
    let remainder_b = chunks_b.remainder();

    let mut sum = 0.0;
    for (ca, cb) in chunks_a.zip(chunks_b) {
        sum += ca[0] * cb[0] + ca[1] * cb[1] + ca[2] * cb[2] + ca[3] * cb[3];
    }
    for (ra, rb) in remainder_a.iter().zip(remainder_b.iter()) {
        sum += ra * rb;
    }
    sum
}
