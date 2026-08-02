//! Module 052: solution — the reference implementation.

use std::ffi::CStr;
use std::os::raw::c_char;

/// Wraps the simulated C `add` function.
pub fn add(a: i32, b: i32) -> i32 {
    ffi_add(a, b)
}

/// Wraps the simulated C `multiply` function.
pub fn multiply(a: f64, b: f64) -> f64 {
    ffi_multiply(a, b)
}

/// Wraps the simulated C `abs` function.
pub fn safe_abs(x: i32) -> i32 {
    ffi_abs(x)
}

/// Returns the length of a null-terminated C string.
///
/// # Safety
///
/// The caller must ensure `s` points to a valid null-terminated C string.
pub fn c_string_length(s: &CStr) -> usize {
    ffi_strlen(s.as_ptr())
}

// ————————————————————————————————————————————
// Simulated C side (same crate, C ABI)
// ————————————————————————————————————————————
// These functions use `extern "C"` to specify the C calling convention.
// In a real scenario, these would be in a separate C library linked via
// `#[link(name = "...")]`. Here we simulate them in the same crate.
//
// Note: when calling `extern "C"` functions defined in the same crate,
// `unsafe` is not required. In real FFI (linking an external C library),
// you would need `unsafe` blocks because the compiler cannot verify the
// C code upholds Rust's safety invariants.

#[no_mangle]
pub extern "C" fn ffi_add(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn ffi_multiply(a: f64, b: f64) -> f64 {
    a * b
}

#[no_mangle]
pub extern "C" fn ffi_abs(x: i32) -> i32 {
    if x < 0 {
        -x
    } else {
        x
    }
}

#[no_mangle]
pub extern "C" fn ffi_strlen(s: *const c_char) -> usize {
    let mut len = 0;
    let mut p = s;
    while unsafe { *p } != 0 {
        len += 1;
        p = unsafe { p.add(1) };
    }
    len
}
