use module_002_solutions::{describe_length, double, fahrenheit, MAX_USERS};

#[test]
fn double_doubles() {
    assert_eq!(double(4), 8);
    assert_eq!(double(0), 0);
    assert_eq!(double(-3), -6);
}

#[test]
fn max_users_is_100() {
    assert_eq!(MAX_USERS, 100);
}

#[test]
fn fahrenheit_converts_freezing() {
    assert_eq!(fahrenheit(0.0), 32.0);
}

#[test]
fn fahrenheit_converts_boiling() {
    assert_eq!(fahrenheit(100.0), 212.0);
}

#[test]
fn describe_length_counts_bytes() {
    assert_eq!(describe_length("hello"), 5);
    assert_eq!(describe_length(""), 0);
}
