use module_057_exercises::{compare_vectorized, dot_product_chunked, sum_vectorized};

#[test]
fn sum_vec_empty() {
    assert_eq!(sum_vectorized(&[]), 0);
}

#[test]
fn sum_vec_less_than_chunk() {
    assert_eq!(sum_vectorized(&[1, 2, 3]), 6);
}

#[test]
fn sum_vec_exact_multiple() {
    assert_eq!(sum_vectorized(&[1, 2, 3, 4]), 10);
    assert_eq!(sum_vectorized(&[1, 2, 3, 4, 5, 6, 7, 8]), 36);
}

#[test]
fn sum_vec_with_remainder() {
    assert_eq!(sum_vectorized(&[1, 2, 3, 4, 5]), 15); // 1 chunk + 1 remainder
    assert_eq!(sum_vectorized(&[10, 20, 30, 40, 50, 60, 70]), 280); // 1 chunk + 3 remainder
    assert_eq!(sum_vectorized(&[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55); // 2 chunks + 2 remainder
}

// ---------------------------------------------------------------------------
// compare_vectorized
// ---------------------------------------------------------------------------

#[test]
fn compare_vec_empty() {
    assert_eq!(compare_vectorized(&[], &[]), Vec::<bool>::new());
}

#[test]
fn compare_vec_simple() {
    let a = [1, 2, 3, 4];
    let b = [1, 2, 5, 4];
    assert_eq!(compare_vectorized(&a, &b), vec![true, true, false, true]);
}

#[test]
fn compare_vec_with_remainder() {
    let a = [1, 2, 3, 4, 5, 6];
    let b = [1, 2, 3, 0, 5, 0];
    assert_eq!(
        compare_vectorized(&a, &b),
        vec![true, true, true, false, true, false]
    );
}

#[test]
fn compare_vec_all_match() {
    let a = [10, 20, 30, 40, 50];
    let b = [10, 20, 30, 40, 50];
    assert_eq!(
        compare_vectorized(&a, &b),
        vec![true, true, true, true, true]
    );
}

#[test]
#[should_panic(expected = "different lengths")]
fn compare_vec_length_mismatch() {
    compare_vectorized(&[1, 2], &[1, 2, 3]);
}

// ---------------------------------------------------------------------------
// dot_product_chunked
// ---------------------------------------------------------------------------

#[test]
fn dot_product_empty() {
    assert_eq!(dot_product_chunked(&[], &[]), 0.0);
}

#[test]
fn dot_product_simple() {
    let a = [1.0, 2.0, 3.0, 4.0];
    let b = [2.0, 3.0, 4.0, 5.0];
    // 1*2 + 2*3 + 3*4 + 4*5 = 2 + 6 + 12 + 20 = 40
    assert_eq!(dot_product_chunked(&a, &b), 40.0);
}

#[test]
fn dot_product_with_remainder() {
    let a = [1.0, 2.0, 3.0, 4.0, 5.0];
    let b = [2.0, 3.0, 4.0, 5.0, 6.0];
    // 1*2 + 2*3 + 3*4 + 4*5 + 5*6 = 2+6+12+20+30 = 70
    assert_eq!(dot_product_chunked(&a, &b), 70.0);
}

#[test]
fn dot_product_large() {
    let a: Vec<f64> = (0..100).map(|i| i as f64).collect();
    let b: Vec<f64> = (0..100).map(|i| (i * 2) as f64).collect();
    let expected: f64 = (0..100).map(|i| i as f64 * (i * 2) as f64).sum();
    assert!((dot_product_chunked(&a, &b) - expected).abs() < 1e-10);
}

#[test]
#[should_panic(expected = "different lengths")]
fn dot_product_length_mismatch() {
    dot_product_chunked(&[1.0], &[1.0, 2.0]);
}
