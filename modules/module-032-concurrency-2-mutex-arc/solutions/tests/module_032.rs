use module_032_solutions::{run_threaded_increments, Counter};

#[test]
fn counter_starts_at_zero() {
    assert_eq!(Counter::default().total(), 0);
}

#[test]
fn counter_increments_sequentially() {
    let counter = Counter::new();
    assert_eq!(counter.increment(), 1);
    assert_eq!(counter.increment(), 2);
    assert_eq!(counter.increment(), 3);
    assert_eq!(counter.total(), 3);
}

#[test]
fn increment_returns_the_new_value() {
    let counter = Counter::new();
    let value = counter.increment();
    assert_eq!(value, counter.total());
}

#[test]
fn threaded_increments_accumulate_exactly() {
    assert_eq!(run_threaded_increments(4, 1000), 4000);
}

#[test]
fn threaded_increments_odd_shapes() {
    assert_eq!(run_threaded_increments(8, 500), 4000);
    assert_eq!(run_threaded_increments(1, 10), 10);
}

#[test]
fn threaded_increments_zero_threads() {
    assert_eq!(run_threaded_increments(0, 1000), 0);
}
