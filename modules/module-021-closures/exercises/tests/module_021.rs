use module_021_exercises::{apply_twice, call_counter, make_adder, run_once};

#[test]
fn apply_twice_increments() {
    assert_eq!(apply_twice(|x| x + 1, 10), 12);
}

#[test]
fn apply_twice_squares_twice() {
    assert_eq!(apply_twice(|x| x * x, 3), 81);
}

#[test]
fn make_adder_returns_a_reusable_closure() {
    let add5 = make_adder(5);
    assert_eq!(add5(10), 15);
    assert_eq!(add5(20), 25);
}

#[test]
fn run_once_can_consume_a_moved_value() {
    let greeting = String::from("hello");
    let length = run_once(move || greeting.len() + 1);
    assert_eq!(length, 6);
}

#[test]
fn run_once_accepts_a_stateful_fn_once_closure() {
    let mut total = 0usize;
    let result = run_once(move || {
        total += 3;
        total
    });
    assert_eq!(result, 3);
}

#[test]
fn call_counter_reports_how_many_times_the_closure_ran() {
    let mut total = 0;
    let calls = call_counter(|x| total += x, &[1, 2, 3]);
    assert_eq!(calls, 3);
    assert_eq!(total, 6);
}

#[test]
fn call_counter_closure_mutates_captured_state() {
    let mut visited = Vec::new();
    let calls = call_counter(|x| visited.push(x), &[4, 5]);
    assert_eq!(calls, 2);
    assert_eq!(visited, vec![4, 5]);
}
