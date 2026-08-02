//! Module 051: solution — the reference implementation.

use std::mem::{align_of, size_of};

#[repr(C)]
#[derive(Debug)]
pub struct Messy {
    pub a: u8,
    pub b: u64,
    pub c: u8,
}

pub fn size_of_messy() -> usize {
    size_of::<Messy>()
}

pub fn align_of_messy() -> usize {
    align_of::<Messy>()
}

#[repr(C)]
#[derive(Debug)]
pub struct Sample {
    pub x: u8,
    pub y: u64,
    pub z: u32,
}

pub fn offset_of_y() -> usize {
    let uninit = std::mem::MaybeUninit::<Sample>::uninit();
    let base = uninit.as_ptr();
    let field = unsafe { std::ptr::addr_of!((*base).y) };
    (field as usize) - (base as usize)
}

#[repr(C)]
#[derive(Debug)]
pub struct ReprC {
    pub a: u8,
    pub b: u32,
    pub c: u16,
}

pub fn size_of_repr_c() -> usize {
    size_of::<ReprC>()
}

#[derive(Debug)]
pub struct Optimized {
    pub b: u64,
    pub a: u8,
    pub c: u8,
}

pub fn optimized_size() -> usize {
    size_of::<Optimized>()
}
