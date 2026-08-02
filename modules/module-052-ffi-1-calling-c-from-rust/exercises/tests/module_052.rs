use module_052_exercises::{add, c_string_length, multiply, safe_abs};
use std::ffi::CStr;

#[test]
fn add_works() {
    assert_eq!(add(3, 4), 7);
    assert_eq!(add(-10, 5), -5);
    assert_eq!(add(0, 0), 0);
}

#[test]
fn multiply_works() {
    assert_eq!(multiply(2.0, 3.0), 6.0);
    assert_eq!(multiply(-2.5, 4.0), -10.0);
    assert_eq!(multiply(0.0, 100.0), 0.0);
}

#[test]
fn safe_abs_works() {
    assert_eq!(safe_abs(-42), 42);
    assert_eq!(safe_abs(42), 42);
    assert_eq!(safe_abs(0), 0);
}

#[test]
fn c_string_length_works() {
    let s = CStr::from_bytes_with_nul(b"hello\0").unwrap();
    assert_eq!(c_string_length(s), 5);

    let empty = CStr::from_bytes_with_nul(b"\0").unwrap();
    assert_eq!(c_string_length(empty), 0);

    let longer = CStr::from_bytes_with_nul(b"Rust.Stack\0").unwrap();
    assert_eq!(c_string_length(longer), 10);
}
