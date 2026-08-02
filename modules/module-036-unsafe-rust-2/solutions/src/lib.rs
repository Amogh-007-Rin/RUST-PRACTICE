//! Module 036: reference solution.
//!
//! Sound mutable statics behind a lock, a `Sync` handle over a raw pointer,
//! union reinterpretation, and unaligned reads. Every `unsafe` block carries
//! its SAFETY argument.

use std::sync::Mutex;

/// Serializes every access to `GLOBAL_COUNT`. It lives in a *safe* static —
/// `Mutex<()>` is not mutable, so no `unsafe` is needed to use it.
static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

/// The shared counter. Mutable statics may only be touched inside `unsafe`.
static mut GLOBAL_COUNT: usize = 0;

/// Adds `by` to the global counter and returns the new value.
pub fn global_increment(by: usize) -> usize {
    let _guard = GLOBAL_LOCK.lock().unwrap();
    // SAFETY: every access to `GLOBAL_COUNT` happens while `GLOBAL_LOCK`
    // is held, so the read-modify-write is exclusive: no data race, and no
    // aliasing is possible within the critical section.
    unsafe {
        GLOBAL_COUNT += by;
        GLOBAL_COUNT
    }
}

/// Returns the current value of the global counter.
pub fn global_value() -> usize {
    let _guard = GLOBAL_LOCK.lock().unwrap();
    // SAFETY: same argument as `global_increment` — the lock serializes all
    // reads of `GLOBAL_COUNT`.
    unsafe { GLOBAL_COUNT }
}

/// Resets the global counter to zero.
pub fn global_reset() {
    let _guard = GLOBAL_LOCK.lock().unwrap();
    // SAFETY: the write is exclusive because `GLOBAL_LOCK` is held.
    unsafe {
        GLOBAL_COUNT = 0;
    }
}

/// A shareable handle to the global counter. Its raw-pointer field is what
/// makes the manual `Send`/`Sync` impls necessary: a `*mut usize` is neither
/// `Send` nor `Sync` on its own.
#[derive(Clone, Copy)]
pub struct GlobalCounter {
    /// Points at `GLOBAL_COUNT`. Every method locks `GLOBAL_LOCK` before
    /// dereferencing, which is what makes sharing this handle sound.
    ptr: *mut usize,
}

// SAFETY: `GlobalCounter` has no data of its own, and every method
// serializes its access through `GLOBAL_LOCK`, so moving it between threads
// cannot create a race.
unsafe impl Send for GlobalCounter {}

// SAFETY: all dereferences of the wrapped pointer happen under
// `GLOBAL_LOCK`, so concurrent sharing cannot race or alias.
unsafe impl Sync for GlobalCounter {}

impl GlobalCounter {
    /// Creates a handle pointing at `GLOBAL_COUNT`.
    pub fn new() -> Self {
        Self {
            ptr: &raw mut GLOBAL_COUNT,
        }
    }

    /// Adds `by` to the global counter and returns the new value.
    pub fn increment(&self, by: usize) -> usize {
        let _guard = GLOBAL_LOCK.lock().unwrap();
        // SAFETY: `self.ptr` always points at `GLOBAL_COUNT` (see `new`),
        // and this dereference happens under `GLOBAL_LOCK`, so it is
        // exclusive, in-bounds, and race-free.
        unsafe {
            *self.ptr += by;
            *self.ptr
        }
    }

    /// Returns the current value of the global counter.
    pub fn total(&self) -> usize {
        let _guard = GLOBAL_LOCK.lock().unwrap();
        // SAFETY: `self.ptr` points at `GLOBAL_COUNT`, which is valid and
        // initialized, and this read is serialized by `GLOBAL_LOCK`.
        unsafe { *self.ptr }
    }
}

impl Default for GlobalCounter {
    fn default() -> Self {
        Self::new()
    }
}

/// A union: `i` and `f` share the same 4 bytes. Writing a field is safe;
/// reading one reinterprets the bits and is `unsafe`.
#[repr(C)]
pub union IntOrFloat {
    pub i: i32,
    pub f: f32,
}

impl IntOrFloat {
    /// Builds a union holding `value` in the integer field.
    pub fn from_int(value: i32) -> Self {
        Self { i: value }
    }

    /// Builds a union holding `value` in the float field.
    pub fn from_float(value: f32) -> Self {
        Self { f: value }
    }

    /// Reads the integer interpretation of the union's bits.
    pub fn as_int(&self) -> i32 {
        // SAFETY: the caller last wrote `i` (or accepts whatever the bits
        // are), and `i32` is valid for any bit pattern.
        unsafe { self.i }
    }

    /// Reads the float interpretation of the union's bits.
    pub fn as_float(&self) -> f32 {
        // SAFETY: the caller last wrote `f`, and the read is a pure bit
        // reinterpretation; `f32` is valid for any bit pattern.
        unsafe { self.f }
    }
}

/// Reads the first 4 bytes of `bytes` as a `u32`, tolerating any alignment.
pub fn read_u32_unaligned(bytes: &[u8]) -> u32 {
    assert!(bytes.len() >= 4, "need at least 4 bytes");
    // SAFETY: `bytes.len() >= 4` keeps the read inside the slice, the byte
    // pointer is non-null, and `read_unaligned` does not require alignment.
    unsafe { bytes.as_ptr().cast::<u32>().read_unaligned() }
}
