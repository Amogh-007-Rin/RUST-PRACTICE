//! Module 053: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

use std::os::raw::c_char;

/// Adds two 32-bit integers. Exported with C ABI.
#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    let _ = (a, b);
    panic!("TODO(module-053): implement rust_add")
}

/// Multiplies two 64-bit floats. Exported with C ABI.
#[no_mangle]
pub extern "C" fn rust_multiply(a: f64, b: f64) -> f64 {
    let _ = (a, b);
    panic!("TODO(module-053): implement rust_multiply")
}

/// Returns the length of a null-terminated C string.
///
/// Returns 0 if `s` is null.
#[no_mangle]
pub extern "C" fn rust_strlen(s: *const c_char) -> usize {
    let _ = s;
    panic!("TODO(module-053): implement rust_strlen")
}

/// A rectangle with C-compatible layout.
#[repr(C)]
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// Returns the area of a `Rectangle`.
///
/// Returns 0.0 if `r` is null.
#[no_mangle]
pub extern "C" fn rust_rectangle_area(r: *const Rectangle) -> f64 {
    let _ = r;
    panic!("TODO(module-053): implement rust_rectangle_area")
}

/// Returns the sum of all elements in a C-style array.
///
/// `data` points to `len` `i32` values. Returns 0 if `data` is null
/// or `len` is 0.
#[no_mangle]
pub extern "C" fn rust_sum_array(data: *const i32, len: usize) -> i32 {
    let _ = (data, len);
    panic!("TODO(module-053): implement rust_sum_array")
}
