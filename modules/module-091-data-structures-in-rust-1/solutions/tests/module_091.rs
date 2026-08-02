//! Integration tests for Module 091 — linked lists, stacks & queues.
//!
//! Run with: `cargo test -p module-091-exercises`

use module_091_solutions::{LinkedList, Queue, Stack};

#[test]
fn push_front_reverses_insertion_order() {
    let mut list = LinkedList::new();
    list.push_front(1);
    list.push_front(2);
    list.push_front(3);
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values, vec![3, 2, 1]);
}

#[test]
fn push_back_preserves_insertion_order() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values, vec![1, 2, 3]);
}

#[test]
fn pop_front_returns_front_first() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.pop_front(), Some(1));
    assert_eq!(list.pop_front(), Some(2));
    assert_eq!(list.pop_front(), None);
}

#[test]
fn pop_back_removes_tail() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    assert_eq!(list.pop_back(), Some(3));
    assert_eq!(list.pop_back(), Some(2));
    assert_eq!(list.pop_back(), Some(1));
    assert_eq!(list.pop_back(), None);
}

#[test]
fn pop_on_single_element_list() {
    let mut list = LinkedList::new();
    list.push_front(42);
    assert_eq!(list.pop_front(), Some(42));
    assert!(list.is_empty());
    list.push_front(1);
    assert_eq!(list.pop_back(), Some(1));
    assert!(list.is_empty());
}

#[test]
fn peek_returns_borrowed_values() {
    let mut list = LinkedList::new();
    assert_eq!(list.peek_front(), None);
    assert_eq!(list.peek_back(), None);
    list.push_back(1);
    list.push_back(2);
    assert_eq!(list.peek_front(), Some(&1));
    assert_eq!(list.peek_back(), Some(&2));
    list.pop_front();
    assert_eq!(list.peek_front(), Some(&2));
}

#[test]
fn len_and_is_empty_track_operations() {
    let mut list = LinkedList::new();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    list.push_front(1);
    list.push_front(2);
    list.push_back(3);
    assert_eq!(list.len(), 3);
    assert!(!list.is_empty());
    list.pop_front();
    list.pop_back();
    assert_eq!(list.len(), 1);
    list.pop_front();
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
}

#[test]
fn iter_yields_front_to_back() {
    let mut list = LinkedList::new();
    for i in 0..5 {
        list.push_back(i);
    }
    let collected: Vec<i32> = list.iter().copied().collect();
    assert_eq!(collected, vec![0, 1, 2, 3, 4]);
}

#[test]
fn iter_mut_can_modify_values() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    for value in list.iter_mut() {
        *value *= 10;
    }
    let collected: Vec<i32> = list.iter().copied().collect();
    assert_eq!(collected, vec![10, 20, 30]);
}

#[test]
fn into_iter_consumes_the_list() {
    let mut list = LinkedList::new();
    list.push_back(1);
    list.push_back(2);
    let collected: Vec<i32> = list.into_iter().collect();
    assert_eq!(collected, vec![1, 2]);
}

#[test]
fn remove_at_front_middle_and_back() {
    let mut list = LinkedList::new();
    for i in 0..5 {
        list.push_back(i);
    }
    assert_eq!(list.remove(0), Some(0));
    assert_eq!(list.remove(2), Some(3));
    assert_eq!(list.remove(1), Some(2));
    assert_eq!(list.remove(1), Some(4));
    assert_eq!(list.remove(0), Some(1));
    assert!(list.is_empty());
}

#[test]
fn remove_out_of_bounds_is_none() {
    let mut list = LinkedList::new();
    list.push_back(1);
    assert_eq!(list.remove(1), None);
    assert_eq!(list.remove(5), None);
    assert_eq!(list.remove(0), Some(1));
    assert_eq!(list.remove(0), None);
}

#[test]
fn remove_middle_updates_links() {
    let mut list = LinkedList::new();
    for i in 0..5 {
        list.push_back(i);
    }
    assert_eq!(list.remove(2), Some(2));
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values, vec![0, 1, 3, 4]);
    assert_eq!(list.len(), 4);
}

#[test]
fn stack_is_lifo() {
    let mut stack = Stack::new();
    assert!(stack.is_empty());
    stack.push(1);
    stack.push(2);
    stack.push(3);
    assert_eq!(stack.len(), 3);
    assert_eq!(stack.peek(), Some(&3));
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.peek(), Some(&1));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn queue_is_fifo() {
    let mut queue = Queue::new();
    assert!(queue.is_empty());
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.peek(), Some(&1));
    assert_eq!(queue.dequeue(), Some(1));
    assert_eq!(queue.dequeue(), Some(2));
    assert_eq!(queue.peek(), Some(&3));
    assert_eq!(queue.dequeue(), Some(3));
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}

#[test]
fn large_list_keeps_order_and_length() {
    let mut list = LinkedList::new();
    for i in 0..1000 {
        list.push_back(i);
    }
    assert_eq!(list.len(), 1000);
    let values: Vec<i32> = list.iter().copied().collect();
    assert_eq!(values.len(), 1000);
    assert_eq!(values.first(), Some(&0));
    assert_eq!(values.last(), Some(&999));
    assert_eq!(list.remove(500), Some(500));
    assert_eq!(list.len(), 999);
}
