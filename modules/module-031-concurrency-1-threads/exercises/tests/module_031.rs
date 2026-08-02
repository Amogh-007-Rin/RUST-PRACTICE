use module_031_exercises::{compute_in_parallel, sum_squares_parallel};

#[test]
fn parallel_squares_preserve_input_order() {
    assert_eq!(
        compute_in_parallel(vec![1, 2, 3, 4, 5]),
        vec![1, 4, 9, 16, 25]
    );
}

#[test]
fn parallel_squares_empty_input() {
    assert_eq!(compute_in_parallel(Vec::new()), Vec::new());
}

#[test]
fn parallel_squares_single_element() {
    assert_eq!(compute_in_parallel(vec![7]), vec![49]);
}

#[test]
fn sum_of_squares_single_thread_matches_sequential() {
    let expected: u64 = (1..=100).map(|x| (x as u64) * (x as u64)).sum();
    assert_eq!(sum_squares_parallel(100, 1), expected);
}

#[test]
fn sum_of_squares_multiple_threads_match_sequential() {
    let expected: u64 = (1..=100).map(|x| (x as u64) * (x as u64)).sum();
    assert_eq!(sum_squares_parallel(100, 4), expected);
    assert_eq!(sum_squares_parallel(100, 8), expected);
}

#[test]
fn sum_of_squares_more_threads_than_values() {
    let expected: u64 = (1..=5).map(|x| (x as u64) * (x as u64)).sum();
    assert_eq!(sum_squares_parallel(5, 16), expected);
}

#[test]
fn sum_of_squares_zero() {
    assert_eq!(sum_squares_parallel(0, 4), 0);
}
