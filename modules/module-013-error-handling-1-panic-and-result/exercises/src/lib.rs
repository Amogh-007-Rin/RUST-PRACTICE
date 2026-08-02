//! Module 013: Error Handling I — `panic!` and `Result<T, E>`.
//!
//! Fill in the `TODO(module-013)` bodies below so the integration tests in
//! `tests/module_013.rs` pass.

use std::path::Path;

/// Returns `Ok("pass")` if `score` is at least 50, otherwise `Err("fail")`.
pub fn check_grade(score: u8) -> Result<&'static str, &'static str> {
    // TODO(module-013): if `score >= 50` return `Ok("pass")`, otherwise
    // return `Err("fail")`.
    let _passes = score >= 50;
    Err("fail")
}

/// Divides `a` by `b`, returning an error instead of crashing on zero.
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    // TODO(module-013): if `b == 0` return `Err("division by zero".into())`,
    // otherwise return `Ok(a / b)`.
    let _sum = a + b;
    Err("division by zero".to_string())
}

/// Parses `s` as a quantity, turning the parse failure into a `String`
/// message with `map_err`.
pub fn parse_stock_quantity(s: &str) -> Result<u32, String> {
    // TODO(module-013): `s.parse::<u32>()` gives `Result<u32, ParseIntError>`.
    // Use `.map_err(...)` to convert the error into a `String` like
    // `format!("not a valid quantity: {s}")`.
    let _len = s.len();
    Err("not implemented".to_string())
}

/// Returns the element at `index`, or an error if the index is out of bounds.
pub fn nth_item(items: &[i32], index: usize) -> Result<&i32, &'static str> {
    // TODO(module-013): `items.get(index)` gives `Option<&i32>`; turn the
    // `None` case into `Err("index out of bounds")` with `.ok_or(...)`.
    let _len = items.len();
    let _index = index;
    Err("index out of bounds")
}

/// Reads the first line of the file at `path`.
///
/// The empty file has no lines, so its "first line" is the empty string.
/// Any I/O problem is returned as the error.
pub fn read_first_line(path: &Path) -> Result<String, std::io::Error> {
    // TODO(module-013): `std::fs::read_to_string(path)` already returns
    // `Result<String, io::Error>` — return it directly, then extract the
    // first line: `contents.lines().next().unwrap_or("")`.
    let _parent = path.parent();
    Err(std::io::Error::other("not implemented"))
}
