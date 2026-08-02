//! Module 035: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//!
//! Every `unsafe` block you write must be *sound*: prove the pointer is
//! non-null, aligned, in-bounds, and points to initialized data.

/// Reads the `u32` behind `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn read_via_raw(ptr: *const u32) -> u32 {
    // TODO(module-035): dereference `ptr` inside an `unsafe` block and
    // return the value. Soundness is the caller's responsibility here.
    let _ = &ptr;
    panic!("TODO(module-035): implement read_via_raw")
}

/// Writes `value` through `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to writable, initialized
/// `u32` storage with no other aliasing reference.
pub unsafe fn write_via_raw(ptr: *mut u32, value: u32) {
    // TODO(module-035): dereference `ptr` as a mutable target inside an
    // `unsafe` block and store `value`.
    let _ = (&ptr, &value);
    panic!("TODO(module-035): implement write_via_raw")
}

/// Sums the elements of `slice` by walking a raw pointer with `add`.
pub fn sum_slice_via_raw(slice: &[u32]) -> u32 {
    // TODO(module-035): take `slice.as_ptr()`, loop `i` from 0 to
    // `slice.len()`, and add `*ptr.add(i)` (wrapping) to a running sum.
    // `ptr.add(i)` must stay inside the allocation, so bound the loop by
    // `slice.len()` and document why the dereference is sound.
    let _ = &slice;
    panic!("TODO(module-035): implement sum_slice_via_raw")
}

/// Swaps two values in place using raw pointers (no `std::mem::swap`).
///
/// The manual swap is deliberate: this exercise is about raw pointers, so
/// `clippy::manual_swap` is allowed instead of replacing the pointer dance.
#[allow(clippy::manual_swap)]
pub fn swap_via_raw(a: &mut i32, b: &mut i32) {
    // TODO(module-035): cast both references to `*mut i32`, then swap the
    // values behind the pointers with a temporary, using `unsafe` blocks.
    // Handle the `a == b` case correctly (read into the temporary first).
    let _ = (&a, &b);
    panic!("TODO(module-035): implement swap_via_raw")
}

/// Doubles the value behind `ptr`.
///
/// # Safety
///
/// `ptr` must be non-null, aligned, and point to a valid, initialized `u32`.
pub unsafe fn unsafe_double(ptr: *const u32) -> u32 {
    // TODO(module-035): dereference `ptr` in an `unsafe` block and return
    // the doubled value. Remember: the `unsafe fn` marker alone does not
    // license the dereference inside the body.
    let _ = &ptr;
    panic!("TODO(module-035): implement unsafe_double")
}
