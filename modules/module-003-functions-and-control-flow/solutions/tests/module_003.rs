use module_003_solutions::{classify, collatz_steps, is_even, sum_to};

#[test]
fn is_even_handles_positives() {
    assert!(is_even(4));
    assert!(!is_even(7));
}

#[test]
fn is_even_handles_zero_and_negatives() {
    assert!(is_even(0));
    assert!(!is_even(-3));
    assert!(is_even(-8));
}

#[test]
fn classify_negative() {
    assert_eq!(classify(-1), "negative");
}

#[test]
fn classify_zero() {
    assert_eq!(classify(0), "zero");
}

#[test]
fn classify_positive() {
    assert_eq!(classify(5), "positive");
}

#[test]
fn sum_to_one_is_one() {
    assert_eq!(sum_to(1), 1);
}

#[test]
fn sum_to_five_is_fifteen() {
    assert_eq!(sum_to(5), 15);
}

#[test]
fn sum_to_hundred() {
    assert_eq!(sum_to(100), 5050);
}

#[test]
fn collatz_steps_one_is_zero() {
    assert_eq!(collatz_steps(1), 0);
}

#[test]
fn collatz_steps_eight() {
    assert_eq!(collatz_steps(8), 3);
}

#[test]
fn collatz_steps_six() {
    assert_eq!(collatz_steps(6), 8);
}

#[test]
fn collatz_steps_twenty_seven() {
    assert_eq!(collatz_steps(27), 111);
}
