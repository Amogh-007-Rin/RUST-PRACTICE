//! Module 013: Error Handling I — `panic!` and `Result<T, E>`
//! (reference solution).

use std::path::Path;

/// Returns `Ok("pass")` if `score` is at least 50, otherwise `Err("fail")`.
pub fn check_grade(score: u8) -> Result<&'static str, &'static str> {
    if score >= 50 {
        Ok("pass")
    } else {
        Err("fail")
    }
}

/// Divides `a` by `b`, returning an error instead of crashing on zero.
pub fn safe_divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

/// Parses `s` as a quantity, turning the parse failure into a `String`
/// message with `map_err`.
pub fn parse_stock_quantity(s: &str) -> Result<u32, String> {
    s.parse::<u32>()
        .map_err(|_| format!("not a valid quantity: {s}"))
}

/// Returns the element at `index`, or an error if the index is out of bounds.
pub fn nth_item(items: &[i32], index: usize) -> Result<&i32, &'static str> {
    items.get(index).ok_or("index out of bounds")
}

/// Reads the first line of the file at `path`.
///
/// The empty file has no lines, so its "first line" is the empty string.
/// Any I/O problem is returned as the error.
pub fn read_first_line(path: &Path) -> Result<String, std::io::Error> {
    std::fs::read_to_string(path).map(|contents| contents.lines().next().unwrap_or("").to_string())
}
