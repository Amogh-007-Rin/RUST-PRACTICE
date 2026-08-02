use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use module_034_solutions::{assert_thread_safe, run_atomic_increments, try_claim, AtomicCounter};

#[test]
fn counter_starts_at_zero() {
    assert_eq!(AtomicCounter::default().total(), 0);
}

#[test]
fn counter_increments_sequentially() {
    let counter = AtomicCounter::new();
    assert_eq!(counter.increment(), 1);
    assert_eq!(counter.increment(), 2);
    assert_eq!(counter.increment(), 3);
    assert_eq!(counter.total(), 3);
}

#[test]
fn increment_returns_the_new_value() {
    let counter = AtomicCounter::new();
    let value = counter.increment();
    assert_eq!(value, counter.total());
}

#[test]
fn threaded_increments_accumulate_exactly() {
    assert_eq!(run_atomic_increments(4, 1000), 4000);
}

#[test]
fn threaded_increments_odd_shapes() {
    assert_eq!(run_atomic_increments(8, 500), 4000);
    assert_eq!(run_atomic_increments(1, 10), 10);
    assert_eq!(run_atomic_increments(0, 1000), 0);
}

#[test]
fn claim_succeeds_exactly_once_across_threads() {
    let flag = Arc::new(AtomicBool::new(false));
    let mut handles = Vec::new();
    for _ in 0..8 {
        let flag = Arc::clone(&flag);
        handles.push(std::thread::spawn(move || try_claim(&flag)));
    }
    let winners: usize = handles
        .into_iter()
        .map(|handle| handle.join().unwrap())
        .filter(|&won| won)
        .count();
    assert_eq!(winners, 1);
}

#[test]
fn claim_fails_once_already_claimed() {
    let flag = AtomicBool::new(false);
    assert!(try_claim(&flag));
    assert!(!try_claim(&flag));
    assert_eq!(flag.load(Ordering::SeqCst), true);
}

#[test]
fn our_types_are_send_and_sync() {
    assert_thread_safe::<AtomicCounter>();
    assert_thread_safe::<AtomicBool>();
    assert_thread_safe::<u64>();
}
