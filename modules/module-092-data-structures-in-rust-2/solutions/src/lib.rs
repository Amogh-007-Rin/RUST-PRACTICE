//! Module 092 — Data Structures in Rust II: trees, heaps & hash maps.
//!
//! Reference solution. Compare against your `exercises/` implementation
//! after you have made a genuine attempt.

// ---------------------------------------------------------------------------
// Binary search tree
// ---------------------------------------------------------------------------

/// A node in the tree: the value plus optional left/right children.
struct TreeNode<T: Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

/// A binary search tree: `left` subtree values are strictly smaller than the
/// node, `right` subtree values are strictly larger. Duplicates are ignored
/// by `insert` (returns `false`).
pub struct BinarySearchTree<T: Ord> {
    root: Option<Box<TreeNode<T>>>,
    len: usize,
}

impl<T: Ord> Default for BinarySearchTree<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> BinarySearchTree<T> {
    /// Creates an empty tree.
    pub fn new() -> Self {
        Self { root: None, len: 0 }
    }

    /// Inserts `value`, returning `true` if a new node was added and `false`
    /// if the value was already present. O(log n) on a balanced tree.
    pub fn insert(&mut self, value: T) -> bool {
        let inserted = insert_rec(&mut self.root, value);
        if inserted {
            self.len += 1;
        }
        inserted
    }

    /// Returns `true` when `value` is in the tree.
    pub fn contains(&self, value: &T) -> bool {
        let mut cur = self.root.as_deref();
        while let Some(node) = cur {
            if value < &node.value {
                cur = node.left.as_deref();
            } else if value > &node.value {
                cur = node.right.as_deref();
            } else {
                return true;
            }
        }
        false
    }

    /// Removes `value`, returning `true` if it was present. O(log n).
    pub fn delete(&mut self, value: &T) -> bool {
        let removed = delete_rec(&mut self.root, value);
        if removed {
            self.len -= 1;
        }
        removed
    }

    /// Smallest value in the tree, or `None` when empty.
    pub fn min(&self) -> Option<&T> {
        let mut cur = self.root.as_deref()?;
        while let Some(left) = cur.left.as_deref() {
            cur = left;
        }
        Some(&cur.value)
    }

    /// Largest value in the tree, or `None` when empty.
    pub fn max(&self) -> Option<&T> {
        let mut cur = self.root.as_deref()?;
        while let Some(right) = cur.right.as_deref() {
            cur = right;
        }
        Some(&cur.value)
    }

    /// All values in ascending order (in-order traversal).
    pub fn in_order(&self) -> Vec<T>
    where
        T: Clone,
    {
        let mut out = Vec::with_capacity(self.len);
        in_order_rec(&self.root, &mut out);
        out
    }

    /// Builds a balanced tree from a slice that is already sorted in
    /// ascending order. The middle element becomes the root, and each half
    /// is built recursively the same way.
    pub fn from_sorted(values: &[T]) -> Self
    where
        T: Clone,
    {
        let mut tree = Self::new();
        tree.root = build_rec(values);
        tree.len = values.len();
        tree
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when the tree is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

/// Recursive insert: returns `true` if a node was added.
fn insert_rec<T: Ord>(node: &mut Option<Box<TreeNode<T>>>, value: T) -> bool {
    match node {
        None => {
            *node = Some(Box::new(TreeNode {
                value,
                left: None,
                right: None,
            }));
            true
        }
        Some(n) => {
            if value < n.value {
                insert_rec(&mut n.left, value)
            } else if value > n.value {
                insert_rec(&mut n.right, value)
            } else {
                false
            }
        }
    }
}

/// Recursive delete: removes `value` from the subtree `node`.
fn delete_rec<T: Ord>(node: &mut Option<Box<TreeNode<T>>>, value: &T) -> bool {
    match node {
        None => false,
        Some(n) if value < &n.value => delete_rec(&mut n.left, value),
        Some(n) if value > &n.value => delete_rec(&mut n.right, value),
        Some(_) => {
            // Found it. Pop the node out of the tree, then splice a
            // replacement subtree back in.
            let mut taken = node.take().unwrap();
            let replacement = match (taken.left.take(), taken.right.take()) {
                (None, None) => None,
                (Some(left), None) => Some(left),
                (None, Some(right)) => Some(right),
                (Some(left), Some(right)) => {
                    // Two children: replace the node with its in-order
                    // successor (the leftmost value of the right subtree).
                    let (successor_value, rest) = pop_leftmost(right);
                    Some(Box::new(TreeNode {
                        value: successor_value,
                        left: Some(left),
                        right: rest,
                    }))
                }
            };
            *node = replacement;
            true
        }
    }
}

/// Removes and returns the smallest value of a non-empty subtree, returning
/// the tree with that node pruned as the second element.
fn pop_leftmost<T: Ord>(mut root: Box<TreeNode<T>>) -> (T, Option<Box<TreeNode<T>>>) {
    if root.left.is_none() {
        (root.value, root.right)
    } else {
        let (value, new_left) = pop_leftmost(root.left.take().unwrap());
        root.left = new_left;
        (value, Some(root))
    }
}

/// Recursive in-order collector: left subtree, node, right subtree.
fn in_order_rec<T: Ord + Clone>(node: &Option<Box<TreeNode<T>>>, out: &mut Vec<T>) {
    if let Some(n) = node {
        in_order_rec(&n.left, out);
        out.push(n.value.clone());
        in_order_rec(&n.right, out);
    }
}

/// Recursive balanced build from a sorted slice.
fn build_rec<T: Ord + Clone>(values: &[T]) -> Option<Box<TreeNode<T>>> {
    if values.is_empty() {
        return None;
    }
    let mid = values.len() / 2;
    Some(Box::new(TreeNode {
        value: values[mid].clone(),
        left: build_rec(&values[..mid]),
        right: build_rec(&values[mid + 1..]),
    }))
}

// ---------------------------------------------------------------------------
// Min-heap (binary heap from scratch, array-backed)
// ---------------------------------------------------------------------------

/// A min-heap: the smallest value is always at the front (`data[0]`).
///
/// The heap is stored in a plain `Vec` where index `i`'s children live at
/// `2*i + 1` and `2*i + 2` — no pointers needed, the *positions* encode the
/// tree structure.
pub struct MinHeap<T: Ord> {
    data: Vec<T>,
}

impl<T: Ord> Default for MinHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> MinHeap<T> {
    /// Creates an empty heap.
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    /// Builds a heap from a vector of values, O(n).
    pub fn from_vec(mut values: Vec<T>) -> Self {
        for i in (0..values.len() / 2).rev() {
            sift_down(&mut values, i);
        }
        Self { data: values }
    }

    /// Pushes `value` and restores the heap invariant, O(log n).
    pub fn push(&mut self, value: T) {
        self.data.push(value);
        let last = self.data.len() - 1;
        sift_up(&mut self.data, last);
    }

    /// Removes and returns the smallest value, O(log n).
    pub fn pop(&mut self) -> Option<T> {
        if self.data.is_empty() {
            return None;
        }
        let last = self.data.len() - 1;
        self.data.swap(0, last);
        let smallest = self.data.pop();
        sift_down(&mut self.data, 0);
        smallest
    }

    /// Borrows the smallest value without removing it, O(1).
    pub fn peek(&self) -> Option<&T> {
        self.data.first()
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        self.data.len()
    }

    /// Returns `true` when the heap is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

/// Restores the heap invariant by moving `data[idx]` up toward the root.
fn sift_up<T: Ord>(data: &mut [T], mut idx: usize) {
    while idx > 0 {
        let parent = (idx - 1) / 2;
        if data[idx] >= data[parent] {
            break;
        }
        data.swap(idx, parent);
        idx = parent;
    }
}

/// Restores the heap invariant by moving `data[idx]` down toward the leaves.
fn sift_down<T: Ord>(data: &mut [T], mut idx: usize) {
    loop {
        let left = 2 * idx + 1;
        let right = 2 * idx + 2;
        let mut smallest = idx;
        if left < data.len() && data[left] < data[smallest] {
            smallest = left;
        }
        if right < data.len() && data[right] < data[smallest] {
            smallest = right;
        }
        if smallest == idx {
            break;
        }
        data.swap(idx, smallest);
        idx = smallest;
    }
}

// ---------------------------------------------------------------------------
// Hash map from scratch (open addressing, linear probing)
// ---------------------------------------------------------------------------

const INITIAL_CAPACITY: usize = 8;
const MAX_LOAD_FACTOR: f64 = 0.7;

/// An entry as stored in the table.
struct Entry<K, V> {
    key: K,
    value: V,
}

/// One slot of the table. `Deleted` is a tombstone: the slot once held a
/// value that was removed, and probing must keep walking past it.
enum Slot<K, V> {
    Empty,
    Occupied(Entry<K, V>),
    Deleted,
}

/// A hash map with open addressing (linear probing).
pub struct HashMap<K: Eq + std::hash::Hash, V> {
    slots: Vec<Slot<K, V>>,
    len: usize,
}

impl<K: Eq + std::hash::Hash, V> Default for HashMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}

impl<K: Eq + std::hash::Hash, V> HashMap<K, V> {
    /// Creates an empty map with `INITIAL_CAPACITY` slots.
    pub fn new() -> Self {
        let slots = (0..INITIAL_CAPACITY).map(|_| Slot::Empty).collect();
        Self { slots, len: 0 }
    }

    /// Inserts `key` → `value`, returning the previous value if the key was
    /// already present. Grows the table when the load factor is exceeded.
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if (self.len + 1) as f64 / self.slots.len() as f64 > MAX_LOAD_FACTOR {
            self.resize();
        }
        let start = probe_start(&key, self.slots.len());
        let mut first_writable = None;
        for offset in 0..self.slots.len() {
            let idx = (start + offset) % self.slots.len();
            match &mut self.slots[idx] {
                Slot::Empty => {
                    let idx = first_writable.unwrap_or(idx);
                    self.slots[idx] = Slot::Occupied(Entry { key, value });
                    self.len += 1;
                    return None;
                }
                Slot::Deleted => {
                    if first_writable.is_none() {
                        first_writable = Some(idx);
                    }
                }
                Slot::Occupied(entry) if entry.key == key => {
                    let old = std::mem::replace(&mut entry.value, value);
                    return Some(old);
                }
                Slot::Occupied(_) => {}
            }
        }
        // Full of tombstones we could not write into: resize and retry.
        self.resize();
        self.insert(key, value)
    }

    /// Borrows the value for `key`, or `None` when missing.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        let idx = find_slot(&self.slots, key)?;
        match &self.slots[idx] {
            Slot::Occupied(entry) => Some(&entry.value),
            _ => unreachable!("find_slot only reports occupied slots"),
        }
    }

    /// Borrows the value for `key` mutably, or `None` when missing.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        let idx = find_slot(&self.slots, key)?;
        match &mut self.slots[idx] {
            Slot::Occupied(entry) => Some(&mut entry.value),
            _ => unreachable!("find_slot only reports occupied slots"),
        }
    }

    /// Removes `key` and returns its value, or `None` when missing.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        let idx = find_slot(&self.slots, key)?;
        let entry = std::mem::replace(&mut self.slots[idx], Slot::Deleted);
        self.len -= 1;
        match entry {
            Slot::Occupied(entry) => Some(entry.value),
            _ => unreachable!("find_slot only reports occupied slots"),
        }
    }

    /// Returns `true` when `key` is present.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        self.get(key).is_some()
    }

    /// Number of live entries, O(1).
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `true` when the map is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Number of slots in the backing table.
    pub fn capacity(&self) -> usize {
        self.slots.len()
    }

    /// Doubles the table and rehashes every live entry. Tombstones are
    /// dropped in the process.
    pub fn resize(&mut self) {
        let old_slots = std::mem::take(&mut self.slots);
        let new_capacity = old_slots.len() * 2;
        self.slots = (0..new_capacity).map(|_| Slot::Empty).collect();
        for slot in old_slots {
            if let Slot::Occupied(entry) = slot {
                let start = probe_start(&entry.key, self.slots.len());
                for offset in 0..self.slots.len() {
                    let idx = (start + offset) % self.slots.len();
                    if let Slot::Empty = self.slots[idx] {
                        self.slots[idx] = Slot::Occupied(entry);
                        break;
                    }
                }
            }
        }
    }

    /// All live (key, value) pairs, in no particular order.
    pub fn entries(&self) -> Vec<(&K, &V)> {
        let mut out = Vec::with_capacity(self.len);
        for slot in &self.slots {
            if let Slot::Occupied(entry) = slot {
                out.push((&entry.key, &entry.value));
            }
        }
        out
    }
}

/// Finds the index of the slot holding `key` in an open-addressed table,
/// probing linearly from the hashed start slot. Returns `None` when an
/// `Empty` slot is hit first (the key cannot be present).
fn find_slot<K, V, Q>(slots: &[Slot<K, V>], key: &Q) -> Option<usize>
where
    K: std::borrow::Borrow<Q>,
    Q: Eq + std::hash::Hash + ?Sized,
{
    let start = probe_start(key, slots.len());
    for offset in 0..slots.len() {
        let idx = (start + offset) % slots.len();
        match &slots[idx] {
            Slot::Empty => return None,
            Slot::Deleted => continue,
            Slot::Occupied(entry) if entry.key.borrow() == key => return Some(idx),
            Slot::Occupied(_) => continue,
        }
    }
    None
}

/// Hashes `key` into a starting slot index.
fn probe_start<Q: Eq + std::hash::Hash + ?Sized>(key: &Q, capacity: usize) -> usize {
    use std::hash::{DefaultHasher, Hasher};

    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    (hasher.finish() as usize) % capacity
}
