//! Module 046: Pinning & Async Internals — exercise scaffold.
//!
//! `Pin<T>` prevents a value from being moved. Most types are `Unpin` and
//! can be moved freely even when pinned. Types that are `!Unpin` (like
//! self-referential structs or async futures holding references across
//! `.await`) must stay at the same memory address. You will work with
//! `Pin<Box<T>>`, check `Unpin` bounds, and observe how async functions
//! generate state machines.

use std::pin::Pin;

/// Return `true` if `T` is `Unpin`. This function exists only to be
/// called — the compiler resolves the bound at compile time.
pub fn type_is_unpin<T: Unpin>() -> bool {
    // TODO(module-046): simply return `true`. The fact that this
    // compiles at all is the point: the bound `T: Unpin` is checked
    // by the compiler when the caller instantiates this function.
    panic!("TODO(module-046): implement type_is_unpin")
}

/// Pin `value` into a `Box` and return the pinned box. The caller can
/// then use `Pin<Box<T>>` to guarantee the value will not be moved.
pub fn pin_in_box<T>(value: T) -> Pin<Box<T>> {
    // TODO(module-046): use `Box::pin(value)` to allocate the value on
    // the heap and return it pinned.
    let _ = value;
    panic!("TODO(module-046): implement pin_in_box")
}

/// Given a pinned mutable reference to a `u64`, write `new_value` into
/// it. This demonstrates that pinning does not prevent mutation — only
/// movement.
pub fn write_through_pin(pinned: Pin<&mut u64>, new_value: u64) {
    // TODO(module-046): use `unsafe { pinned.get_unchecked_mut() }` to
    // get `&mut u64`, then assign `new_value`. The pin guarantees the
    // value has not been moved, so this is safe for `Unpin` types.
    let _ = (pinned, new_value);
    panic!("TODO(module-046): implement write_through_pin")
}

/// A self-referential struct: `owner` holds a `String`, and `ref_to_self`
/// is a pointer into that `String`'s buffer. Moving this struct without
/// pinning would invalidate the pointer.
pub struct SelfRef {
    pub owner: String,
    pub ref_to_self: *const String,
}

impl SelfRef {
    /// Build a new pinned `SelfRef` where `ref_to_self` points to `owner`.
    /// Returns `Pin<Box<Self>>` because the struct must be pinned before
    /// the self-reference is valid.
    pub fn new(owner: String) -> Pin<Box<Self>> {
        // TODO(module-046): create a `Box::pin(SelfRef { owner, ref_to_self: null })`,
        // then set `ref_to_self` to point at `owner` using `Pin::get_unchecked_mut`
        // inside an `unsafe` block. Return the pinned box.
        let _ = owner;
        panic!("TODO(module-046): implement SelfRef::new")
    }

    /// Dereference `ref_to_self` and return the `String` it points to.
    /// This is only safe if the struct has not been moved since
    /// construction — which is exactly what `Pin` guarantees.
    ///
    /// # Safety
    /// The caller must ensure `self` is pinned.
    pub fn read_self_ref(self: Pin<&Self>) -> &String {
        // TODO(module-046): use `self.ref_to_self` (dereference the raw
        // pointer) inside an `unsafe` block. The pin guarantees the
        // struct has not moved, so the pointer is still valid.
        panic!("TODO(module-046): implement SelfRef::read_self_ref")
    }
}

/// Return `true` if the future returned by `async_fn_example` is `Unpin`.
/// This function demonstrates that async functions holding references
/// across `.await` points generate `!Unpin` futures.
pub fn async_future_is_unpin() -> bool {
    // TODO(module-046): call `async_fn_example()` and check if the
    // returned future satisfies `Unpin`. You can do this by trying to
    // use it where `Unpin` is required, or by checking the type. The
    // future from `async_fn_example` is `!Unpin` because it holds a
    // reference across `.await`. Return `false`.
    panic!("TODO(module-046): implement async_future_is_unpin")
}

/// An async function that holds a reference across an `.await` point.
/// The future this generates is `!Unpin`.
pub async fn async_fn_example(data: &str) -> String {
    tokio::time::sleep(std::time::Duration::from_millis(1)).await;
    data.to_uppercase()
}
