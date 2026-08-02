use std::time::Duration;

use module_055_exercises::{compare, time_execution};

#[test]
fn time_execution_returns_correct_result() {
    let (result, duration) = time_execution(|| 42, 10);
    assert_eq!(result, 42);
    assert!(duration > Duration::ZERO, "duration should be measurable");
}

#[test]
fn time_execution_runs_multiple_times() {
    let mut counter = 0u32;
    let (last, duration) = time_execution(
        || {
            counter += 1;
            counter
        },
        5,
    );
    assert_eq!(last, 5, "closure ran 5 times, last value should be 5");
    assert!(duration > Duration::ZERO);
}

#[test]
fn time_execution_single_iteration() {
    let (result, duration) = time_execution(|| "hello", 1);
    assert_eq!(result, "hello");
    assert!(duration > Duration::ZERO);
}

#[test]
fn compare_identifies_faster() {
    let slow = || {
        std::hint::black_box((0..10_000u64).sum::<u64>());
    };
    let fast = || {
        std::hint::black_box(42u64);
    };

    let result = compare("slow", slow, "fast", fast, 100);

    assert_eq!(result.name1, "slow");
    assert_eq!(result.name2, "fast");
    assert_eq!(result.faster, "fast");
    assert!(result.time1 > result.time2, "slow should take longer");
    assert!(
        result.speedup > 1.0,
        "speedup should be > 1 since fast is faster"
    );
}

#[test]
fn compare_handles_tie() {
    let a = || {
        std::hint::black_box(1u64);
    };
    let b = || {
        std::hint::black_box(1u64);
    };
    let result = compare("a", a, "b", b, 1000);
    assert!(result.faster == "a" || result.faster == "b" || result.faster == "tie");
    assert!(result.speedup >= 0.99);
}

#[test]
fn compare_fields_are_consistent() {
    let slow = || {
        let mut s: u64 = 0;
        for i in 0..1000u64 {
            s += i;
        }
        std::hint::black_box(s);
    };
    let fast = || {
        std::hint::black_box(0u64);
    };

    let result = compare("slow", slow, "fast", fast, 200);

    assert!(result.time1 > Duration::ZERO);
    assert!(result.time2 > Duration::ZERO);

    if result.faster == "slow" {
        assert!(result.time1 <= result.time2);
    } else if result.faster == "fast" {
        assert!(result.time2 <= result.time1);
    }
    assert!(result.speedup >= 1.0);
}
