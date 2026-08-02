use module_001_solutions::{greet, message_length};

#[test]
fn greet_contains_hello() {
    assert!(greet("Ada").to_lowercase().contains("hello"));
}

#[test]
fn greet_contains_name() {
    assert!(greet("Ada").contains("Ada"));
}

#[test]
fn greet_exact_format() {
    assert_eq!(greet("Grace"), "Hello, Grace!");
}

#[test]
fn message_length_counts_bytes() {
    assert_eq!(message_length("hello"), 5);
}

#[test]
fn message_length_zero_for_empty() {
    assert_eq!(message_length(""), 0);
}
