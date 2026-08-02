//! Module 046: Pinning & Async Internals — integration tests.

use module_046_exercises::{
    async_fn_example, async_future_is_unpin, pin_in_box, type_is_unpin, write_through_pin, SelfRef,
};
use std::pin::Pin;

#[test]
fn u32_is_unpin() {
    assert!(type_is_unpin::<u32>());
}

#[test]
fn string_is_unpin() {
    assert!(type_is_unpin::<String>());
}

#[test]
fn vec_is_unpin() {
    assert!(type_is_unpin::<Vec<i32>>());
}

#[test]
fn pin_in_box_preserves_value() {
    let pinned = pin_in_box(42u64);
    assert_eq!(*pinned, 42);
}

#[test]
fn write_through_pin_mutates() {
    let mut value = 10u64;
    let pinned = Pin::new(&mut value);
    write_through_pin(pinned, 99);
    assert_eq!(value, 99);
}

#[test]
fn self_ref_reads_correctly_when_pinned() {
    let pinned = SelfRef::new("hello".to_string());
    let read_back = pinned.as_ref().read_self_ref();
    assert_eq!(read_back, "hello");
}

#[test]
fn async_future_is_not_unpin() {
    assert!(!async_future_is_unpin());
}

#[tokio::test]
async fn async_fn_example_works() {
    let result = async_fn_example("test").await;
    assert_eq!(result, "TEST");
}
