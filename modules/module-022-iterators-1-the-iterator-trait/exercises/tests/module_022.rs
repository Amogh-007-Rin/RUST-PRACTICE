use module_022_exercises::{first_greater, sum_evens, Fibonacci, Step};

#[test]
fn step_yields_inclusive_range() {
    assert_eq!(Step::new(1, 5).collect::<Vec<_>>(), vec![1, 2, 3, 4, 5]);
}

#[test]
fn step_handles_inverted_bounds_as_empty() {
    assert!(Step::new(5, 1).collect::<Vec<_>>().is_empty());
}

#[test]
fn step_next_driven_by_hand() {
    let mut it = Step::new(1, 3);
    assert_eq!(it.next(), Some(1));
    assert_eq!(it.next(), Some(2));
    assert_eq!(it.next(), Some(3));
    assert_eq!(it.next(), None);
    assert_eq!(it.next(), None);
}

#[test]
fn fibonacci_first_eight() {
    assert_eq!(
        Fibonacci::new().take(8).collect::<Vec<_>>(),
        vec![1, 1, 2, 3, 5, 8, 13, 21]
    );
}

#[test]
fn fibonacci_is_infinite_but_take_limits_it() {
    let sum: u64 = Fibonacci::new().take(90).sum();
    assert!(sum > 0);
}

#[test]
fn sum_evens_totals_only_even_numbers() {
    assert_eq!(sum_evens(&[1, 2, 3, 4, 5, 6]), 12);
    assert_eq!(sum_evens(&[]), 0);
    assert_eq!(sum_evens(&[1, 3, 5]), 0);
}

#[test]
fn first_greater_finds_the_first_match() {
    assert_eq!(first_greater(&[1, 3, 5, 7], 4), Some(5));
    assert_eq!(first_greater(&[1, 2, 3], 10), None);
    assert_eq!(first_greater(&[1, 2, 3], 0), Some(1));
}
