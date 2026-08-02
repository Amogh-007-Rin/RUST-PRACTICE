//! Module 035: reference solution.
//!
//! Sound raw-pointer reads, writes, slice walking, swapping, and an
//! `unsafe fn`. Every dereference is proven non-null, aligned, in-bounds,
//! and initialized at the site of the dereference.

/// Reads the `u32` behind `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn read_via_raw(ptr: *const u32) -> u32 {
    // SAFETY: the caller provides a non-null, aligned pointer to a valid,
    // initialized `u32` — that is the contract of this function.
    unsafe { *ptr }
}

/// Writes `value` through `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to writable, initialized
/// `u32` storage with no other aliasing reference.
pub unsafe fn write_via_raw(ptr: *mut u32, value: u32) {
    // SAFETY: the caller provides a non-null, aligned pointer to writable,
    // initialized `u32` storage with no other aliasing reference.
    unsafe {
        *ptr = value;
    }
}

/// Sums the elements of `slice` by walking a raw pointer with `add`.
pub fn sum_slice_via_raw(slice: &[u32]) -> u32 {
    let ptr = slice.as_ptr();
    let mut sum = 0u32;
    for i in 0..slice.len() {
        // SAFETY: `i` is in `0..slice.len()`, so `ptr.add(i)` stays inside
        // the slice's allocation, is aligned to `u32`, and points to the
        // initialized `i`-th element.
        unsafe {
            sum = sum.wrapping_add(*ptr.add(i));
        }
    }
    sum
}

/// Swaps two values in place using raw pointers (no `std::mem::swap`).
///
/// The manual swap is deliberate: this exercise is about raw pointers, so
/// `clippy::manual_swap` is allowed instead of replacing the pointer dance.
#[allow(clippy::manual_swap)]
pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    let a_ptr: *mut i32 = a;
    let b_ptr: *mut i32 = b;
    // SAFETY: both pointers derive from `&mut` references the caller owns,
    // so they are non-null, aligned, initialized, and (unless `a` and `b`
    // alias, which safe code can only do as the same location) exclusive.
    // Reading into the temporary first keeps the `a == b` case sound too.
    unsafe {
        let temp = *a_ptr;
        *a_ptr = *b_ptr;
        *b_ptr = temp;
    }
}

/// Doubles the value behind `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn unsafe_double(ptr: *const u32) -> u32 {
    // SAFETY: upheld by the caller per the `# Safety` contract above.
    unsafe { *ptr * 2 }
}
