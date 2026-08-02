//! Module 036: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//!
//! All `unsafe` code must be sound. The sound pattern here: every access to
//! `GLOBAL_COUNT` happens while `GLOBAL_LOCK` is held.

use std::sync::Mutex;

/// Serializes every access to `GLOBAL_COUNT`. It lives in a *safe* static —
/// `Mutex<()>` is not mutable, so no `unsafe` is needed to use it.
#[allow(dead_code)] // used by the functions you are about to implement
static GLOBAL_LOCK: Mutex<()> = Mutex::new(());

/// The shared counter. Mutable statics may only be touched inside `unsafe`.
static mut GLOBAL_COUNT: usize = 0;

/// Adds `by` to the global counter and returns the new value.
pub fn global_increment(_by: usize) -> usize {
    // TODO(module-036): lock `GLOBAL_LOCK`, then inside an `unsafe` block
    // do `GLOBAL_COUNT += by` and return the new value. Add a SAFETY
    // comment explaining why the `unsafe` block is sound.
    panic!("TODO(module-036): implement global_increment")
}

/// Returns the current value of the global counter.
pub fn global_value() -> usize {
    // TODO(module-036): lock `GLOBAL_LOCK`, then read `GLOBAL_COUNT` inside
    // an `unsafe` block with a SAFETY comment.
    panic!("TODO(module-036): implement global_value")
}

/// Resets the global counter to zero.
pub fn global_reset() {
    // TODO(module-036): lock `GLOBAL_LOCK`, then set `GLOBAL_COUNT = 0`
    // inside an `unsafe` block with a SAFETY comment.
    panic!("TODO(module-036): implement global_reset")
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
        // TODO(module-036): lock `GLOBAL_LOCK`, then inside an `unsafe`
        // block add `by` to the value behind `self.ptr` and return the new
        // value, with a SAFETY comment.
        let _ = (&self.ptr, &by);
        panic!("TODO(module-036): implement GlobalCounter::increment")
    }

    /// Returns the current value of the global counter.
    pub fn total(&self) -> usize {
        // TODO(module-036): lock `GLOBAL_LOCK`, then read the value behind
        // `self.ptr` inside an `unsafe` block, with a SAFETY comment.
        let _ = &self.ptr;
        panic!("TODO(module-036): implement GlobalCounter::total")
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
        // TODO(module-036): return `Self { i: value }` — writing a union
        // field is safe.
        let _ = &value;
        panic!("TODO(module-036): implement IntOrFloat::from_int")
    }

    /// Builds a union holding `value` in the float field.
    pub fn from_float(value: f32) -> Self {
        // TODO(module-036): return `Self { f: value }`.
        let _ = &value;
        panic!("TODO(module-036): implement IntOrFloat::from_float")
    }

    /// Reads the integer interpretation of the union's bits.
    pub fn as_int(&self) -> i32 {
        // TODO(module-036): return `unsafe { self.i }` — reading a union
        // field is unsafe.
        panic!("TODO(module-036): implement IntOrFloat::as_int")
    }

    /// Reads the float interpretation of the union's bits.
    pub fn as_float(&self) -> f32 {
        // TODO(module-036): return `unsafe { self.f }`.
        panic!("TODO(module-036): implement IntOrFloat::as_float")
    }
}

/// Reads the first 4 bytes of `bytes` as a `u32`, tolerating any alignment.
pub fn read_u32_unaligned(_bytes: &[u8]) -> u32 {
    // TODO(module-036): `assert!(bytes.len() >= 4)`, then return
    // `unsafe { bytes.as_ptr().cast::<u32>().read_unaligned() }` with a
    // SAFETY comment explaining why the read stays in bounds.
    panic!("TODO(module-036): implement read_u32_unaligned")
}
