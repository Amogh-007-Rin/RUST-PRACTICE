//! Module 046: Pinning & Async Internals — reference solution.
//!
//! `Pin<T>` prevents movement. `Unpin` types can be moved even when
//! pinned. Self-referential structs and async futures holding references
//! across `.await` are `!Unpin` and require pinning to be safe.

use std::pin::Pin;

/// Return `true` if `T` is `Unpin`.
pub fn type_is_unpin<T: Unpin>() -> bool {
    true
}

/// Pin `value` into a `Box` and return the pinned box.
pub fn pin_in_box<T>(value: T) -> Pin<Box<T>> {
    Box::pin(value)
}

/// Given a pinned mutable reference to a `u64`, write `new_value` into
/// it. Since `u64: Unpin`, we can use `get_mut`.
pub fn write_through_pin(pinned: Pin<&mut u64>, new_value: u64) {
    unsafe { *pinned.get_unchecked_mut() = new_value };
}

/// A self-referential struct: `owner` holds a `String`, and `ref_to_self`
/// is a pointer into that `String`'s buffer.
pub struct SelfRef {
    pub owner: String,
    pub ref_to_self: *const String,
}

impl SelfRef {
    /// Build a new pinned `SelfRef` where `ref_to_self` points to `owner`.
    /// Returns `Pin<Box<Self>>` because the struct must be pinned before
    /// the self-reference is valid.
    pub fn new(owner: String) -> Pin<Box<Self>> {
        let mut s = Box::pin(SelfRef {
            owner,
            ref_to_self: std::ptr::null(),
        });
        // SAFETY: We are about to return the pinned box, and the caller
        // cannot move the value out of it. Setting the pointer now is
        // safe because the Box pins the value in place.
        let ptr = &s.owner as *const String;
        unsafe {
            let inner = Pin::get_unchecked_mut(s.as_mut());
            inner.ref_to_self = ptr;
        }
        s
    }

    /// Dereference `ref_to_self` and return the `String` it points to.
    ///
    /// # Safety
    /// The caller must ensure `self` is pinned.
    pub fn read_self_ref(self: Pin<&Self>) -> &String {
        // SAFETY: The caller guarantees the struct is pinned and has not
        // been moved since construction. The pointer was set to point at
        // `owner` in `new`, and pinning ensures `owner` has not moved.
        unsafe { &*self.ref_to_self }
    }
}

/// Return `true` if the future returned by `async_fn_example` is `Unpin`.
/// The future is `!Unpin` because it holds a reference across `.await`.
pub fn async_future_is_unpin() -> bool {
    false
}

/// An async function that holds a reference across an `.await` point.
/// The future this generates is `!Unpin`.
pub async fn async_fn_example(data: &str) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    data.to_uppercase()
}
