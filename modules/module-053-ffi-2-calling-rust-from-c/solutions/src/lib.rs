//! Module 053: solution — the reference implementation.

use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn rust_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn rust_multiply(a: f64, b: f64) -> f64 {
    a * b
}

/// # Safety
///
/// `s` must point to a valid null-terminated C string, or be null.
#[no_mangle]
pub unsafe extern "C" fn rust_strlen(s: *const c_char) -> usize {
    if s.is_null() {
        return 0;
    }
    let c_str = unsafe { CStr::from_ptr(s) };
    c_str.to_bytes().len()
}

#[repr(C)]
#[derive(Debug)]
pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

/// # Safety
///
/// `r` must point to a valid `Rectangle`, or be null.
#[no_mangle]
pub unsafe extern "C" fn rust_rectangle_area(r: *const Rectangle) -> f64 {
    if r.is_null() {
        return 0.0;
    }
    let r = unsafe { &*r };
    r.width * r.height
}

/// # Safety
///
/// `data` must point to `len` valid `i32` values, or be null.
#[no_mangle]
pub unsafe extern "C" fn rust_sum_array(data: *const i32, len: usize) -> i32 {
    if data.is_null() || len == 0 {
        return 0;
    }
    let slice = unsafe { std::slice::from_raw_parts(data, len) };
    slice.iter().sum()
}
