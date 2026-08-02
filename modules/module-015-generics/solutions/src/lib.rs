//! Module 015: Generics — generic functions, structs, and enums
//! (reference solution).

/// Returns a reference to the largest element of `items`, or `None` if empty.
///
/// The `T: PartialOrd` bound lets `<` and `>` work for any comparable type.
pub fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    items
        .iter()
        .reduce(|acc, item| if item > acc { item } else { acc })
}

/// Returns a reference to the first element of `items`, or to `fallback`
/// when the slice is empty.
///
/// The `'a` is a lifetime annotation — Module 018 explains it fully. For now:
/// all three references live as long as `'a`, and the function may return a
/// borrow of either input.
pub fn first_or<'a, T>(items: &'a [T], fallback: &'a T) -> &'a T {
    items.first().unwrap_or(fallback)
}

/// A generic pair of values — possibly of two different types.
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Returns a reference to the first value.
    pub fn first(&self) -> &T {
        &self.first
    }

    /// Returns a reference to the second value.
    pub fn second(&self) -> &U {
        &self.second
    }

    /// Consumes the pair and returns it with the values swapped.
    pub fn swap(self) -> Pair<U, T> {
        Pair {
            first: self.second,
            second: self.first,
        }
    }
}

/// A generic `Option`-like type: a value, or nothing.
pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    /// Returns `true` when the value is `Just`.
    pub fn is_just(&self) -> bool {
        matches!(self, Maybe::Just(_))
    }

    /// Returns the contained value, or `default` when it is `Nothing`.
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            Maybe::Just(value) => value,
            Maybe::Nothing => default,
        }
    }
}

/// Returns a new vector with all of `a` followed by all of `b`.
pub fn combine<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    let mut combined = a;
    combined.extend(b);
    combined
}
