//! Module 056: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// A simplified Clone-on-Write string wrapper.
///
/// Internally it holds `Option<String>`:
/// - `None` means no allocation has happened yet (conceptually "borrowed").
/// - `Some(s)` means we own the string.
///
/// The "clone on write" step happens at `to_mut()` time:
/// if the string hasn't been materialized yet, it clones the provided source.
#[derive(Debug, Clone, PartialEq)]
pub struct CowStr(Option<String>);

impl CowStr {
    /// Create an empty CowStr with no allocation.
    pub fn new() -> Self {
        panic!("TODO(module-056): implement CowStr::new")
    }

    /// Create a CowStr that immediately owns a copy of `s`.
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Self {
        let _ = s;
        panic!("TODO(module-056): implement CowStr::from_str")
    }

    /// Return a string slice.
    ///
    /// Returns the inner string if `Some`, or `""` if `None`.
    pub fn as_str(&self) -> &str {
        match &self.0 {
            Some(s) => s.as_str(),
            None => panic!("TODO(module-056): return empty string for None case"),
        }
    }

    /// Ensure the inner string is materialized, then return a mutable reference.
    ///
    /// If `None`, materializes by cloning `source`. If already `Some`, ignores
    /// `source` and returns the existing string.
    pub fn to_mut(&mut self, source: &str) -> &mut String {
        let _ = source;
        match self.0 {
            Some(ref mut s) => s,
            None => panic!("TODO(module-056): materialize string from source"),
        }
    }

    /// Consume the CowStr and return the owned string (or an empty `String` if `None`).
    pub fn into_string(self) -> String {
        panic!("TODO(module-056): implement into_string")
    }
}

impl Default for CowStr {
    fn default() -> Self {
        Self::new()
    }
}

/// A fixed-capacity buffer backed by a stack array, using const generics.
///
/// `N` is the compile-time capacity.
#[derive(Debug)]
#[allow(dead_code)]
pub struct ArrayBuffer<T, const N: usize> {
    data: [T; N],
    len: usize,
}

impl<T: Default + Copy, const N: usize> ArrayBuffer<T, N> {
    /// Create an empty buffer.
    pub fn new() -> Self {
        panic!("TODO(module-056): implement ArrayBuffer::new")
    }

    /// Append a value. Returns `Ok(())` on success, or `Err("full")` if
    /// the buffer is already at capacity.
    pub fn push(&mut self, value: T) -> Result<(), &'static str> {
        let _ = value;
        panic!("TODO(module-056): implement ArrayBuffer::push")
    }

    /// Get a reference to the value at `index`, or `None` if out of bounds.
    pub fn get(&self, index: usize) -> Option<&T> {
        let _ = index;
        panic!("TODO(module-056): implement ArrayBuffer::get")
    }

    /// Return the number of elements currently stored.
    pub fn len(&self) -> usize {
        panic!("TODO(module-056): implement ArrayBuffer::len")
    }

    /// Return `true` if the buffer contains no elements.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T: Default + Copy, const N: usize> Default for ArrayBuffer<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

/// Return a `CowStr` containing whichever of `a` (a shared reference) or
/// `b` (an owned `CowStr`) is longer, owning the longer string.
///
/// If they're equal length, return `b` to avoid unnecessary allocation.
pub fn longest_str(a: &str, b: CowStr) -> CowStr {
    let _ = (a, b);
    panic!("TODO(module-056): implement longest_str")
}
