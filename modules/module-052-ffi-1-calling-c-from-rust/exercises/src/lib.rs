//! Module 052: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Wraps the simulated C `add` function.
pub fn add(a: i32, b: i32) -> i32 {
    // TODO(module-052): call the `ffi_add` function.
    let _ = (a, b);
    panic!("TODO(module-052): implement add")
}

/// Wraps the simulated C `multiply` function.
pub fn multiply(a: f64, b: f64) -> f64 {
    // TODO(module-052): call the `ffi_multiply` function.
    let _ = (a, b);
    panic!("TODO(module-052): implement multiply")
}

/// Wraps the simulated C `abs` function.
pub fn safe_abs(x: i32) -> i32 {
    // TODO(module-052): call `ffi_abs`.
    let _ = x;
    panic!("TODO(module-052): implement safe_abs")
}

/// Returns the length of a null-terminated C string.
///
/// # Safety
///
/// The caller must ensure `s` points to a valid null-terminated C string.
pub fn c_string_length(s: &std::ffi::CStr) -> usize {
    // TODO(module-052): call `ffi_strlen`.
    let _ = s;
    panic!("TODO(module-052): implement c_string_length")
}

// ————————————————————————————————————————————
// Simulated C side (same crate, C ABI)
// ————————————————————————————————————————————
// Define the following as `#[no_mangle] pub extern "C" fn`:
// - `ffi_add(a: i32, b: i32) -> i32`
// - `ffi_multiply(a: f64, b: f64) -> f64`
// - `ffi_abs(x: i32) -> i32`
// - `ffi_strlen(s: *const std::os::raw::c_char) -> usize`
//
// TODO(module-052): implement these four functions below.
