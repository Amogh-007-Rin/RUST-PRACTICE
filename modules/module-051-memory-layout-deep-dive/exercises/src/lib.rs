//! Module 051: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

use std::mem::{align_of, size_of};

/// A struct with fields in "worst" alignment order.
///
/// ```text
/// u8  + 7 pad + u64 + u8 + 7 pad = 24 bytes on 64-bit
/// ```
#[repr(C)]
#[derive(Debug)]
pub struct Messy {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

/// Returns the size in bytes of `Messy`.
pub fn size_of_messy() -> usize {
    // TODO(module-051): return `size_of::<Messy>()`.
    let _ = size_of::<Messy>;
    panic!("TODO(module-051): implement size_of_messy")
}

/// Returns the alignment in bytes of `Messy`.
pub fn align_of_messy() -> usize {
    // TODO(module-051): return `align_of::<Messy>()`.
    let _ = align_of::<Messy>;
    panic!("TODO(module-051): implement align_of_messy")
}

/// A sample struct whose field-offset you will compute manually.
#[repr(C)]
#[derive(Debug)]
pub struct Sample {
    pub x: u8,
    pub y: u64,
    pub z: u32,
}

/// Returns the byte offset of the `y` field within `Sample`.
///
/// Use `MaybeUninit::<Sample>::uninit()`, `as_ptr()`, and
/// `core::ptr::addr_of!` to compute the offset without constructing
/// a real `Sample`.
pub fn offset_of_y() -> usize {
    // TODO(module-051): compute the offset of `y` via pointer arithmetic.
    let _ = Sample { x: 0, y: 0, z: 0 };
    panic!("TODO(module-051): implement offset_of_y")
}

/// A C-repr struct — verify its size matches the documented layout.
#[repr(C)]
#[derive(Debug)]
pub struct ReprC {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

/// Returns the size in bytes of `ReprC`.
pub fn size_of_repr_c() -> usize {
    // TODO(module-051): return `size_of::<ReprC>()`.
    let _ = size_of::<ReprC>;
    panic!("TODO(module-051): implement size_of_repr_c")
}

/// Reorder the fields of this struct to minimize its size.
///
/// The current order (u8, u64, u8) wastes 14 bytes of padding on 64-bit.
/// Rearrange them in the type definition below so the struct is as small
/// as possible — then return the new size from this function.
#[derive(Debug)]
pub struct Optimized {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

/// Returns the size in bytes of the *optimized* `Optimized` struct.
///
/// After you reorder the fields in `Optimized` to minimize padding,
/// return the new size here.
pub fn optimized_size() -> usize {
    // TODO(module-051): reorder the fields of `Optimized` above, then
    // return `size_of::<Optimized>()`.
    let _ = size_of::<Optimized>;
    panic!("TODO(module-051): implement optimized_size")
}
