//! Module 091 — Data Structures in Rust I: linked lists, stacks & queues.
//!
//! This scaffold compiles but every method you need to implement panics
//! (or returns a placeholder). Fill in the `// TODO(module-091)` markers
//! until the integration tests in `tests/module_091.rs` pass, then compare
//! your work with `solutions/`.

/// A single node in the linked list: a value plus an owned link to the next
/// node. The `Box` puts each node on the heap and the list owns it — when a
/// node is dropped, the chain of `Box`es drops recursively.
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

/// A singly-linked list of `T` values.
///
/// Ownership is a straight chain: `head` owns the first node, which owns the
/// second, and so on. No cycles, so the borrow checker is happy and `Drop`
/// is free.
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    /// Creates an empty list.
    pub fn new() -> Self {
        Self { head: None, len: 0 }
    }

    /// Prepends `value` to the list, O(1).
    ///
    /// The classic move: `take()` the old head out of the list (leaving
    /// `None` in its place), then build the new node pointing at it.
    pub fn push_front(&mut self, value: T) {
        // TODO(module-091): take the current head out of the list, box up a
        // new node holding `value` whose `next` points at the old head, and
        // make it the new head. Bump `self.len`.
        drop(value); // placeholder — remove once implemented
        panic!("stub: push_front is not implemented yet");
    }

    /// Appends `value` to the end, O(n).
    pub fn push_back(&mut self, value: T) {
        // TODO(module-091): walk the chain to the last node (the one whose
        // `next` is `None`) and set its `next` to a new node holding `value`.
        // Tip: walk with `&mut Option<Box<Node<T>>>` and `as_mut()` so the
        // borrow checker can see you only hold one mutable borrow at a time.
        // Bump `self.len`.
        drop(value); // placeholder — remove once implemented
        panic!("stub: push_back is not implemented yet");
    }

    /// Removes and returns the first element, O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        // TODO(module-091): `self.head.take()` moves the head node out of
        // the list (leaving `None` in its place). Decrement `self.len` and
        // return the value of the node you took.
        panic!("stub: pop_front is not implemented yet");
    }

    /// Removes and returns the last element, O(n).
    ///
    /// The famously awkward operation: to unlink the tail you must reach the
    /// *second-to-last* node. Walk with a mutable reference and stop when
    /// the node after `cur` is the last one.
    pub fn pop_back(&mut self) -> Option<T> {
        // TODO(module-091): handle the empty list and the single-element
        // list as special cases, then walk to the second-to-last node and
        // `take()` its `next`. Decrement `self.len`.
        panic!("stub: pop_back is not implemented yet");
    }

    /// Borrows the first element, O(1).
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Borrows the last element, O(n).
    pub fn peek_back(&self) -> Option<&T> {
        // TODO(module-091): walk the chain with `as_deref()` and return the
        // value of the last node, or `None` for an empty list.
        panic!("stub: peek_back is not implemented yet");
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        // TODO(module-091): return the cached length.
        0
    }

    /// Returns `true` when the list is empty, O(1).
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Immutable iterator over the values, front to back.
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            current: self.head.as_deref(),
        }
    }

    /// Mutable iterator over the values, front to back.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            current: self.head.as_deref_mut(),
        }
    }

    /// Removes and returns the element at `index`, O(index).
    ///
    /// Out-of-range indexes return `None` without touching the list.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        // TODO(module-091): guard against `index >= self.len` first, then
        // walk a `&mut Option<Box<Node<T>>>` forward `index` times with
        // `as_mut()` and take the node. Decrement `self.len`.
        let _ = index; // placeholder — remove once implemented
        panic!("stub: remove is not implemented yet");
    }
}

/// Immutable iterator for [`LinkedList`].
pub struct Iter<'a, T> {
    current: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.map(|node| {
            self.current = node.next.as_deref();
            &node.value
        })
    }
}

/// Mutable iterator for [`LinkedList`].
pub struct IterMut<'a, T> {
    current: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current.take().map(|node| {
            self.current = node.next.as_deref_mut();
            &mut node.value
        })
    }
}

/// Consuming iterator: yields each value and frees the list.
pub struct IntoIter<T> {
    list: LinkedList<T>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.list.pop_front()
    }
}

impl<T> IntoIterator for LinkedList<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { list: self }
    }
}

/// A LIFO stack built on top of [`LinkedList`].
///
/// Both operations are O(1) because they happen at the front of the list.
pub struct Stack<T> {
    inner: LinkedList<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Stack<T> {
    /// Creates an empty stack.
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }

    /// Pushes `value` onto the top of the stack.
    pub fn push(&mut self, value: T) {
        // TODO(module-091): delegate to the underlying list — the front of
        // the list is the top of the stack.
        drop(value); // placeholder — remove once implemented
        panic!("stub: Stack::push is not implemented yet");
    }

    /// Pops the top value off the stack, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        // TODO(module-091): delegate to the underlying list.
        panic!("stub: Stack::pop is not implemented yet");
    }

    /// Borrows the top value without removing it.
    pub fn peek(&self) -> Option<&T> {
        // TODO(module-091): delegate to the underlying list.
        panic!("stub: Stack::peek is not implemented yet");
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        // TODO(module-091): delegate to the underlying list.
        0
    }

    /// Returns `true` when the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// A FIFO queue built on top of [`LinkedList`].
///
/// `enqueue` appends at the back (O(n) for a plain linked list — see the
/// README for why a ring buffer like `VecDeque` is the production answer).
pub struct Queue<T> {
    inner: LinkedList<T>,
}

impl<T> Default for Queue<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Queue<T> {
    /// Creates an empty queue.
    pub fn new() -> Self {
        Self {
            inner: LinkedList::new(),
        }
    }

    /// Adds `value` to the back of the queue.
    pub fn enqueue(&mut self, value: T) {
        // TODO(module-091): delegate to `push_back` on the underlying list.
        drop(value); // placeholder — remove once implemented
        panic!("stub: Queue::enqueue is not implemented yet");
    }

    /// Removes and returns the front value, or `None` if empty.
    pub fn dequeue(&mut self) -> Option<T> {
        // TODO(module-091): delegate to `pop_front` on the underlying list.
        panic!("stub: Queue::dequeue is not implemented yet");
    }

    /// Borrows the front value without removing it.
    pub fn peek(&self) -> Option<&T> {
        // TODO(module-091): delegate to `peek_front` on the underlying list.
        panic!("stub: Queue::peek is not implemented yet");
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        // TODO(module-091): delegate to the underlying list.
        0
    }

    /// Returns `true` when the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
