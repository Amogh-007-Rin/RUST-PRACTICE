//! Module 079: integration tests.

use module_079_exercises::{publish, PubSub};
use std::cell::RefCell;
use std::rc::Rc;

#[test]
fn test_subscribe_and_publish_single_handler() {
    let mut ps: PubSub<String> = PubSub::new();
    let received: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let r = received.clone();

    ps.subscribe("orders", move |event| {
        r.borrow_mut().push(event.clone());
    });

    publish(&ps, "orders", &"order-1".to_string());
    publish(&ps, "orders", &"order-2".to_string());

    let events = received.borrow();
    assert_eq!(*events, vec!["order-1".to_string(), "order-2".to_string()]);
}

#[test]
fn test_multiple_subscribers_same_topic() {
    let mut ps: PubSub<i32> = PubSub::new();
    let a: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
    let b: Rc<RefCell<Vec<i32>>> = Rc::new(RefCell::new(vec![]));
    let a1 = a.clone();
    let b1 = b.clone();

    ps.subscribe("numbers", move |n| a1.borrow_mut().push(*n));
    ps.subscribe("numbers", move |n| b1.borrow_mut().push(*n));

    publish(&ps, "numbers", &42);

    assert_eq!(*a.borrow(), vec![42]);
    assert_eq!(*b.borrow(), vec![42]);
}

#[test]
fn test_publish_to_nonexistent_topic_is_noop() {
    let ps: PubSub<String> = PubSub::new();
    // Should not panic
    publish(&ps, "nonexistent", &"test".to_string());
}

#[test]
fn test_different_topics_isolated() {
    let mut ps: PubSub<String> = PubSub::new();
    let topic_a: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let topic_b: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
    let a1 = topic_a.clone();
    let b1 = topic_b.clone();

    ps.subscribe("a", move |e| a1.borrow_mut().push(e.clone()));
    ps.subscribe("b", move |e| b1.borrow_mut().push(e.clone()));

    publish(&ps, "a", &"hello".to_string());
    publish(&ps, "b", &"world".to_string());

    assert_eq!(*topic_a.borrow(), vec!["hello".to_string()]);
    assert_eq!(*topic_b.borrow(), vec!["world".to_string()]);
}
