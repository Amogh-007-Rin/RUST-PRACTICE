//! Integration tests for Module 092 — trees, heaps & hash maps.
//!
//! Run with: `cargo test -p module-092-exercises`

use module_092_exercises::{BinarySearchTree, HashMap, MinHeap};

// ---------------------------------------------------------------------------
// Binary search tree
// ---------------------------------------------------------------------------

#[test]
fn insert_then_in_order_is_sorted() {
    let mut tree = BinarySearchTree::new();
    for value in [5, 3, 8, 1, 4, 7, 9, 6, 2] {
        assert!(tree.insert(value));
    }
    assert_eq!(tree.in_order(), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

#[test]
fn duplicate_insert_is_rejected() {
    let mut tree = BinarySearchTree::new();
    assert!(tree.insert(7));
    assert!(!tree.insert(7));
    assert!(!tree.insert(7));
    assert_eq!(tree.len(), 1);
    assert_eq!(tree.in_order(), vec![7]);
}

#[test]
fn contains_finds_inserted_values() {
    let mut tree = BinarySearchTree::new();
    for value in [10, 4, 17, 2, 8] {
        tree.insert(value);
    }
    for value in [10, 4, 17, 2, 8] {
        assert!(tree.contains(&value));
    }
    assert!(!tree.contains(&0));
    assert!(!tree.contains(&5));
    assert!(!tree.contains(&100));
}

#[test]
fn contains_on_empty_tree_is_false() {
    let tree: BinarySearchTree<i32> = BinarySearchTree::new();
    assert!(!tree.contains(&42));
    assert!(tree.is_empty());
}

#[test]
fn min_and_max_find_extremes() {
    let mut tree = BinarySearchTree::new();
    assert_eq!(tree.min(), None);
    assert_eq!(tree.max(), None);
    for value in [50, 30, 80, 10, 90, 40] {
        tree.insert(value);
    }
    assert_eq!(tree.min(), Some(&10));
    assert_eq!(tree.max(), Some(&90));
}

#[test]
fn len_tracks_insert_and_delete() {
    let mut tree = BinarySearchTree::new();
    for value in [5, 3, 8, 1, 4, 7, 9] {
        tree.insert(value);
    }
    assert_eq!(tree.len(), 7);
    assert!(tree.delete(&5));
    assert_eq!(tree.len(), 6);
    assert!(!tree.delete(&5));
    assert_eq!(tree.len(), 6);
    assert!(tree.delete(&9));
    assert!(tree.delete(&1));
    assert_eq!(tree.len(), 4);
    assert!(!tree.is_empty());
}

#[test]
fn delete_leaf_node() {
    let mut tree = BinarySearchTree::new();
    for value in [5, 3, 8, 1, 4] {
        tree.insert(value);
    }
    assert!(tree.delete(&1));
    assert_eq!(tree.in_order(), vec![3, 4, 5, 8]);
    assert!(!tree.contains(&1));
}

#[test]
fn delete_node_with_one_child() {
    let mut tree = BinarySearchTree::new();
    for value in [5, 3, 8, 1] {
        tree.insert(value);
    }
    assert!(tree.delete(&3));
    assert_eq!(tree.in_order(), vec![1, 5, 8]);
}

#[test]
fn delete_node_with_two_children_uses_successor() {
    let mut tree = BinarySearchTree::new();
    for value in [50, 30, 70, 20, 40, 60, 80, 35] {
        tree.insert(value);
    }
    assert!(tree.delete(&30));
    assert_eq!(tree.in_order(), vec![20, 35, 40, 50, 60, 70, 80]);
    assert!(tree.delete(&50));
    assert_eq!(tree.in_order(), vec![20, 35, 40, 60, 70, 80]);
}

#[test]
fn delete_root_only_node() {
    let mut tree = BinarySearchTree::new();
    tree.insert(42);
    assert!(tree.delete(&42));
    assert!(tree.is_empty());
    assert_eq!(tree.in_order(), Vec::<i32>::new());
}

#[test]
fn delete_nonexistent_returns_false() {
    let mut tree = BinarySearchTree::new();
    for value in [5, 3, 8] {
        tree.insert(value);
    }
    assert!(!tree.delete(&99));
    assert_eq!(tree.len(), 3);
}

#[test]
fn from_sorted_builds_working_tree() {
    let values: Vec<i32> = (1..=15).collect();
    let tree = BinarySearchTree::from_sorted(&values);
    assert_eq!(tree.len(), 15);
    assert_eq!(tree.in_order(), values);
    assert_eq!(tree.min(), Some(&1));
    assert_eq!(tree.max(), Some(&15));
}

#[test]
fn from_sorted_empty_slice() {
    let tree: BinarySearchTree<i32> = BinarySearchTree::from_sorted(&[]);
    assert!(tree.is_empty());
    assert_eq!(tree.in_order(), Vec::<i32>::new());
}

// ---------------------------------------------------------------------------
// Min-heap
// ---------------------------------------------------------------------------

#[test]
fn push_then_pop_returns_sorted_values() {
    let mut heap = MinHeap::new();
    for value in [5, 1, 4, 2, 3, 7, 6, 0] {
        heap.push(value);
    }
    let mut out = Vec::new();
    while let Some(v) = heap.pop() {
        out.push(v);
    }
    assert_eq!(out, vec![0, 1, 2, 3, 4, 5, 6, 7]);
}

#[test]
fn peek_returns_smallest_without_removing() {
    let mut heap = MinHeap::new();
    assert_eq!(heap.peek(), None);
    heap.push(10);
    heap.push(1);
    heap.push(5);
    assert_eq!(heap.peek(), Some(&1));
    assert_eq!(heap.len(), 3);
    assert_eq!(heap.pop(), Some(1));
    assert_eq!(heap.peek(), Some(&5));
}

#[test]
fn pop_empty_heap_is_none() {
    let mut heap: MinHeap<i32> = MinHeap::new();
    assert_eq!(heap.pop(), None);
    assert_eq!(heap.peek(), None);
}

#[test]
fn len_and_is_empty_track_operations() {
    let mut heap = MinHeap::new();
    assert!(heap.is_empty());
    assert_eq!(heap.len(), 0);
    for value in [3, 1, 2] {
        heap.push(value);
    }
    assert_eq!(heap.len(), 3);
    assert!(!heap.is_empty());
    heap.pop();
    heap.pop();
    assert_eq!(heap.len(), 1);
    heap.pop();
    assert!(heap.is_empty());
}

#[test]
fn single_element_heap_pops_itself() {
    let mut heap = MinHeap::new();
    heap.push(99);
    assert_eq!(heap.pop(), Some(99));
    assert_eq!(heap.pop(), None);
}

#[test]
fn duplicate_values_are_handled() {
    let mut heap = MinHeap::new();
    for value in [2, 2, 1, 2, 1] {
        heap.push(value);
    }
    let mut out = Vec::new();
    while let Some(v) = heap.pop() {
        out.push(v);
    }
    assert_eq!(out, vec![1, 1, 2, 2, 2]);
}

#[test]
fn from_vec_heapifies_correctly() {
    let heap = MinHeap::from_vec(vec![9, 4, 7, 2, 8, 5, 6, 1, 3]);
    assert_eq!(heap.len(), 9);
    assert_eq!(heap.peek(), Some(&1));
    let mut heap = heap;
    let mut out = Vec::new();
    while let Some(v) = heap.pop() {
        out.push(v);
    }
    assert_eq!(out, (1..=9).collect::<Vec<i32>>());
}

#[test]
fn heap_order_invariant_holds_after_each_pop() {
    let mut heap = MinHeap::new();
    for value in [42, 7, 19, 3, 88, 12, 55, 1, 33] {
        heap.push(value);
    }
    let mut prev = None;
    while let Some(v) = heap.pop() {
        if let Some(p) = prev {
            assert!(p <= v, "heap popped {p} then {v}, violating order");
        }
        prev = Some(v);
    }
}

// ---------------------------------------------------------------------------
// Hash map
// ---------------------------------------------------------------------------

#[test]
fn insert_and_get_round_trip() {
    let mut map = HashMap::new();
    assert_eq!(map.get(&5), None);
    map.insert(5, "five");
    map.insert(9, "nine");
    assert_eq!(map.get(&5), Some(&"five"));
    assert_eq!(map.get(&9), Some(&"nine"));
    assert_eq!(map.get(&4), None);
}

#[test]
fn insert_returns_previous_value() {
    let mut map = HashMap::new();
    assert_eq!(map.insert("a", 1), None);
    assert_eq!(map.insert("a", 2), Some(1));
    assert_eq!(map.get(&"a"), Some(&2));
}

#[test]
fn contains_key_reflects_inserts_and_removes() {
    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert!(map.contains_key(&1));
    assert!(map.contains_key(&2));
    assert!(!map.contains_key(&3));
    map.remove(&1);
    assert!(!map.contains_key(&1));
}

#[test]
fn remove_returns_value_and_updates_len() {
    let mut map = HashMap::new();
    map.insert(1, 10);
    map.insert(2, 20);
    assert_eq!(map.len(), 2);
    assert_eq!(map.remove(&1), Some(10));
    assert_eq!(map.len(), 1);
    assert_eq!(map.remove(&1), None);
    assert_eq!(map.remove(&99), None);
}

#[test]
fn get_mut_allows_update() {
    let mut map = HashMap::new();
    map.insert("counter", 0);
    for _ in 0..5 {
        *map.get_mut(&"counter").unwrap() += 1;
    }
    assert_eq!(map.get(&"counter"), Some(&5));
}

#[test]
fn many_inserts_force_resize_and_round_trip() {
    let mut map = HashMap::new();
    for i in 0..100 {
        map.insert(i, i * i);
    }
    assert!(map.capacity() > 8, "table should have grown");
    assert_eq!(map.len(), 100);
    for i in 0..100 {
        assert_eq!(map.get(&i), Some(&(i * i)));
    }
    assert_eq!(map.get(&200), None);
}

#[test]
fn remove_then_reinsert_same_key() {
    let mut map = HashMap::new();
    map.insert(7, "old");
    assert_eq!(map.remove(&7), Some("old"));
    assert!(map.is_empty());
    map.insert(7, "new");
    assert_eq!(map.get(&7), Some(&"new"));
    assert_eq!(map.len(), 1);
}

#[test]
fn entries_matches_len_and_values() {
    let mut map = HashMap::new();
    for i in 0..20 {
        map.insert(i, i * 10);
    }
    let entries = map.entries();
    assert_eq!(entries.len(), 20);
    for (k, v) in entries {
        assert_eq!(*v, *k * 10);
    }
}

#[test]
fn string_keys_work() {
    let mut map = HashMap::new();
    map.insert(String::from("rust"), 1);
    map.insert(String::from("stack"), 2);
    assert_eq!(map.get("rust"), Some(&1));
    assert_eq!(map.get("stack"), Some(&2));
    assert_eq!(map.get("missing"), None);
    assert_eq!(map.remove("rust"), Some(1));
    assert_eq!(map.len(), 1);
}
