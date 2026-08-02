//! Module 091 — Data Structures in Rust I: linked lists, stacks & queues.
//!
//! Reference solution. Compare against your `exercises/` implementation
//! after you have made a genuine attempt.

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
    pub fn push_front(&mut self, value: T) {
        let old_head = self.head.take();
        self.head = Some(Box::new(Node {
            value,
            next: old_head,
        }));
        self.len += 1;
    }

    /// Appends `value` to the end, O(n).
    pub fn push_back(&mut self, value: T) {
        let mut cur: &mut Option<Box<Node<T>>> = &mut self.head;
        while cur.is_some() {
            cur = &mut cur.as_mut().expect("loop guard").next;
        }
        *cur = Some(Box::new(Node { value, next: None }));
        self.len += 1;
    }

    /// Removes and returns the first element, O(1).
    pub fn pop_front(&mut self) -> Option<T> {
        let node = self.head.take()?;
        self.head = node.next;
        self.len -= 1;
        Some(node.value)
    }

    /// Removes and returns the last element, O(n).
    pub fn pop_back(&mut self) -> Option<T> {
        self.head.as_ref()?;
        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }
        let mut cur = self.head.as_mut().unwrap();
        while cur.next.as_ref().unwrap().next.is_some() {
            cur = cur.next.as_mut().unwrap();
        }
        let tail = cur.next.take().unwrap();
        self.len -= 1;
        Some(tail.value)
    }

    /// Borrows the first element, O(1).
    pub fn peek_front(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    /// Borrows the last element, O(n).
    pub fn peek_back(&self) -> Option<&T> {
        let mut cur: &Node<T> = self.head.as_deref()?;
        while let Some(next) = cur.next.as_deref() {
            cur = next;
        }
        Some(&cur.value)
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        self.len
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
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if index >= self.len {
            return None;
        }
        let mut cur: &mut Option<Box<Node<T>>> = &mut self.head;
        for _ in 0..index {
            cur = &mut cur.as_mut()?.next;
        }
        let node = cur.take()?;
        *cur = node.next;
        self.len -= 1;
        Some(node.value)
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
        self.inner.push_front(value);
    }

    /// Pops the top value off the stack, or `None` if empty.
    pub fn pop(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Borrows the top value without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.inner.peek_front()
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` when the stack is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}

/// A FIFO queue built on top of [`LinkedList`].
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
        self.inner.push_back(value);
    }

    /// Removes and returns the front value, or `None` if empty.
    pub fn dequeue(&mut self) -> Option<T> {
        self.inner.pop_front()
    }

    /// Borrows the front value without removing it.
    pub fn peek(&self) -> Option<&T> {
        self.inner.peek_front()
    }

    /// Number of elements, O(1).
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    /// Returns `true` when the queue is empty.
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
