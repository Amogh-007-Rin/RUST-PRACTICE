//! Module 092 — Data Structures in Rust II: trees, heaps & hash maps.
//!
//! This scaffold compiles but every method you need to implement panics
//! (or returns a placeholder). Fill in the `// TODO(module-092)` markers
//! until the integration tests in `tests/module_092.rs` pass, then compare
//! your work with `solutions/`.

// ---------------------------------------------------------------------------
// Binary search tree
// ---------------------------------------------------------------------------

/// A node in the tree: the value plus optional left/right children.
/// Children are owned (`Box`) and the tree is acyclic, so dropping the root
/// drops the whole tree.
#[allow(dead_code)] // read once you implement the methods below
struct TreeNode<T: Ord> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

/// A binary search tree: `left` subtree values are strictly smaller than the
/// node, `right` subtree values are strictly larger. Duplicates are ignored
/// by `insert` (returns `false`).
///
/// Note: `root` is marked `#[allow(dead_code)]` only because the scaffold's
/// methods are stubs; it becomes "read" as soon as you implement them.
pub struct BinarySearchTree<T: Ord> {
    #[allow(dead_code)] // read once the methods below are implemented
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
        // TODO(module-092): delegate to the private `insert_rec` helper, then
        // bump `self.len` only when a node was really added.
        drop(value); // placeholder — remove once implemented
        panic!("stub: insert is not implemented yet");
    }

    /// Returns `true` when `value` is in the tree.
    pub fn contains(&self, value: &T) -> bool {
        // TODO(module-092): walk from the root: go left if `value` is
        // smaller, right if larger, and report a hit when you find it.
        // A missing node means `false`.
        let _ = value; // placeholder — remove once implemented
        panic!("stub: contains is not implemented yet");
    }

    /// Removes `value`, returning `true` if it was present. O(log n).
    ///
    /// The classic tricky part: a node with two children must be replaced by
    /// its in-order successor (the smallest value of its right subtree).
    pub fn delete(&mut self, value: &T) -> bool {
        // TODO(module-092): delegate to the private `delete_rec` helper and
        // decrement `self.len` when a node was removed.
        let _ = value; // placeholder — remove once implemented
        panic!("stub: delete is not implemented yet");
    }

    /// Smallest value in the tree, or `None` when empty.
    pub fn min(&self) -> Option<&T> {
        // TODO(module-092): walk left until there is no left child.
        panic!("stub: min is not implemented yet");
    }

    /// Largest value in the tree, or `None` when empty.
    pub fn max(&self) -> Option<&T> {
        // TODO(module-092): walk right until there is no right child.
        panic!("stub: max is not implemented yet");
    }

    /// All values in ascending order (in-order traversal).
    pub fn in_order(&self) -> Vec<T>
    where
        T: Clone,
    {
        // TODO(module-092): recursively collect: left subtree, node, right
        // subtree. This is the property every BST test in this module relies
        // on: `in_order` of a BST is always sorted.
        panic!("stub: in_order is not implemented yet");
    }

    /// Builds a balanced tree from a slice that is already sorted in
    /// ascending order. Recursively pick the middle element as the root.
    pub fn from_sorted(values: &[T]) -> Self
    where
        T: Clone,
    {
        // TODO(module-092): implement `build_rec` as a private helper and
        // call it from here, then set `self.len = values.len()`.
        let _ = values; // placeholder — remove once implemented
        panic!("stub: from_sorted is not implemented yet");
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

/// Recursive insert: returns `true` if a node was added. When the `node`
/// slot is `None`, a new leaf is created there — that's the only place the
/// tree ever grows.
#[allow(dead_code)] // used once `insert` is implemented
fn insert_rec<T: Ord>(_node: &mut Option<Box<TreeNode<T>>>, value: T) -> bool {
    // TODO(module-092): match on `_node`: `None` → put a new leaf there and
    // return `true`; `Some(n)` → recurse into `n.left` or `n.right`
    // depending on the comparison, and return `false` for equality.
    drop(value); // placeholder — remove once implemented
    panic!("stub: insert_rec is not implemented yet");
}

/// Recursive delete: removes `value` from the subtree `node`, returning
/// `true` if anything was removed. On the two-children case, call the
/// `pop_leftmost` helper to extract the in-order successor and splice the
/// rest of the right subtree in its place.
#[allow(dead_code)] // used once `delete` is implemented
fn delete_rec<T: Ord>(_node: &mut Option<Box<TreeNode<T>>>, value: &T) -> bool {
    // TODO(module-092): three cases — not found (`None` or value missing),
    // found with 0 or 1 child (splice the child up), found with 2 children
    // (swap in the successor and prune it from the right subtree).
    let _ = value; // placeholder — remove once implemented
    panic!("stub: delete_rec is not implemented yet");
}

/// Removes and returns the smallest value of a non-empty subtree, returning
/// the tree with that node pruned as the second element.
#[allow(dead_code)] // used once `delete_rec` is implemented
fn pop_leftmost<T: Ord>(_root: Box<TreeNode<T>>) -> (T, Option<Box<TreeNode<T>>>) {
    // TODO(module-092): if `_root.left` is `None`, `_root` is the leftmost —
    // return its value and its right subtree as the rest. Otherwise recurse
    // into `_root.left`, reattach the pruned subtree, and return `_root`.
    drop(_root); // placeholder — remove once implemented
    panic!("stub: pop_leftmost is not implemented yet");
}

// ---------------------------------------------------------------------------
// Min-heap (binary heap from scratch, array-backed)
// ---------------------------------------------------------------------------

/// A min-heap: the smallest value is always at the front (`data[0]`).
///
/// The heap is stored in a plain `Vec` where index `i`'s children live at
/// `2*i + 1` and `2*i + 2` — no pointers needed, the *positions* encode the
/// tree structure. `push` restores the invariant by sifting up, `pop` by
/// sifting down.
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

    /// Builds a heap from a vector of values.
    pub fn from_vec(values: Vec<T>) -> Self {
        // TODO(module-092): push every element, then sift every non-leaf
        // element down (`for i in (0..len / 2).rev() { self.sift_down(i) }`).
        drop(values); // placeholder — remove once implemented
        panic!("stub: from_vec is not implemented yet");
    }

    /// Pushes `value` and restores the heap invariant, O(log n).
    pub fn push(&mut self, value: T) {
        // TODO(module-092): push onto the back, then sift it up toward the
        // root while it is smaller than its parent.
        drop(value); // placeholder — remove once implemented
        panic!("stub: push is not implemented yet");
    }

    /// Removes and returns the smallest value, O(log n).
    pub fn pop(&mut self) -> Option<T> {
        // TODO(module-092): swap the root with the last element, pop the
        // back, then sift the new root down to restore the invariant.
        panic!("stub: pop is not implemented yet");
    }

    /// Borrows the smallest value without removing it, O(1).
    pub fn peek(&self) -> Option<&T> {
        // TODO(module-092): the smallest value lives at `data[0]`.
        panic!("stub: peek is not implemented yet");
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

/// Restores the heap invariant by moving `data[idx]` up toward the root:
/// while the element is smaller than its parent, swap them.
#[allow(dead_code)] // used once `push`/`from_vec` are implemented
fn sift_up<T: Ord>(_data: &mut [T], _idx: usize) {
    // TODO(module-092): parent of `_idx` is `(_idx - 1) / 2`. While
    // `_idx > 0` and `_data[_idx] < _data[(_idx - 1) / 2]`, swap and move up.
    panic!("stub: sift_up is not implemented yet");
}

/// Restores the heap invariant by moving `data[idx]` down toward the leaves:
/// swap with the *smaller* child while the element is larger than it.
#[allow(dead_code)] // used once `pop`/`from_vec` are implemented
fn sift_down<T: Ord>(_data: &mut [T], _idx: usize) {
    // TODO(module-092): children of `_idx` are `2*_idx + 1` and `2*_idx + 2`.
    // Pick the smaller child that exists; if the element is larger than it,
    // swap and continue.
    panic!("stub: sift_down is not implemented yet");
}

// ---------------------------------------------------------------------------
// Hash map from scratch (open addressing, linear probing)
// ---------------------------------------------------------------------------

const INITIAL_CAPACITY: usize = 8;
#[allow(dead_code)] // used once `insert`/`resize` are implemented
const MAX_LOAD_FACTOR: f64 = 0.7;

/// An entry as stored in the table.
#[allow(dead_code)] // constructed once `insert` is implemented
struct Entry<K, V> {
    key: K,
    value: V,
}

/// One slot of the table. `Deleted` is a tombstone: the slot once held a
/// value that was removed, and probing must keep walking past it.
#[allow(dead_code)] // `Occupied`/`Deleted` are constructed once implemented
enum Slot<K, V> {
    Empty,
    Occupied(Entry<K, V>),
    Deleted,
}

/// A hash map with open addressing (linear probing).
///
/// Keys are mapped to a starting slot by hashing, then `get`/`insert`/`remove`
/// walk forward (wrapping around the end) until they find the key, an empty
/// slot, or a tombstone. When the load factor passes 70%, the table doubles
/// in size and every live entry is rehashed into the new table.
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
        // TODO(module-092): hash the key to a start slot, probe forward for
        // `key` or the first writable slot (Empty or Deleted). If the key
        // exists, replace the value. Otherwise write a new entry, bump
        // `self.len`, and resize when the load factor is exceeded.
        drop(key); // placeholder — remove once implemented
        drop(value); // placeholder — remove once implemented
        panic!("stub: insert is not implemented yet");
    }

    /// Borrows the value for `key`, or `None` when missing.
    pub fn get<Q>(&self, key: &Q) -> Option<&V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        // TODO(module-092): probe from the hashed slot; `Empty` means
        // "missing", `Deleted` means "keep probing". Compare with
        // `entry.key.borrow() == key`.
        let _ = key; // placeholder — remove once implemented
        panic!("stub: get is not implemented yet");
    }

    /// Borrows the value for `key` mutably, or `None` when missing.
    pub fn get_mut<Q>(&mut self, key: &Q) -> Option<&mut V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        // TODO(module-092): same probe as `get`, but return `&mut`.
        let _ = key; // placeholder — remove once implemented
        panic!("stub: get_mut is not implemented yet");
    }

    /// Removes `key` and returns its value, or `None` when missing.
    pub fn remove<Q>(&mut self, key: &Q) -> Option<V>
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        // TODO(module-092): probe; on a hit, replace the slot with a
        // `Deleted` tombstone and decrement `self.len`.
        let _ = key; // placeholder — remove once implemented
        panic!("stub: remove is not implemented yet");
    }

    /// Returns `true` when `key` is present.
    pub fn contains_key<Q>(&self, key: &Q) -> bool
    where
        K: std::borrow::Borrow<Q>,
        Q: Eq + std::hash::Hash + ?Sized,
    {
        // TODO(module-092): `get(key).is_some()`.
        let _ = key; // placeholder — remove once implemented
        panic!("stub: contains_key is not implemented yet");
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
        // TODO(module-092): build a new `Vec<Slot<K, V>>` twice as big, walk
        // the old table, and reinsert every `Occupied` entry using the same
        // probing logic as `insert` — without touching `self.len`.
        panic!("stub: resize is not implemented yet");
    }

    /// All live (key, value) pairs, in no particular order.
    pub fn entries(&self) -> Vec<(&K, &V)> {
        // TODO(module-092): walk the slots and collect every `Occupied`
        // entry. `Vec::new()` is a fine placeholder return.
        panic!("stub: entries is not implemented yet");
    }
}
