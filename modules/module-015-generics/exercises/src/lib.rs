//! Module 015: Generics — generic functions, structs, and enums.
//!
//! Fill in the `TODO(module-015)` bodies below so the integration tests in
//! `tests/module_015.rs` pass.

/// Returns a reference to the largest element of `items`, or `None` if empty.
///
/// The `T: PartialOrd` bound lets `<` and `>` work for any comparable type.
pub fn largest<T: PartialOrd>(items: &[T]) -> Option<&T> {
    // TODO(module-015): walk the slice and keep the biggest element you've
    // seen — `items.iter().reduce(...)` is a tidy fit: keep `acc` or `item`,
    // whichever is larger.
    let _len = items.len();
    None
}

/// Returns a reference to the first element of `items`, or to `fallback`
/// when the slice is empty.
///
/// The `'a` is a lifetime annotation — Module 018 explains it fully. For now:
/// all three references live as long as `'a`, and the function may return a
/// borrow of either input.
pub fn first_or<'a, T>(items: &'a [T], fallback: &'a T) -> &'a T {
    // TODO(module-015): `items.first()` gives `Option<&T>`; `.unwrap_or(fallback)`
    // turns the `None` case into the fallback.
    let _count = items.len();
    fallback
}

/// A generic pair of values — possibly of two different types.
pub struct Pair<T, U> {
    pub first: T,
    pub second: U,
}

impl<T, U> Pair<T, U> {
    /// Returns a reference to the first value.
    pub fn first(&self) -> &T {
        // TODO(module-015): return `&self.first`.
        panic!("not implemented")
    }

    /// Returns a reference to the second value.
    pub fn second(&self) -> &U {
        // TODO(module-015): return `&self.second`.
        panic!("not implemented")
    }

    /// Consumes the pair and returns it with the values swapped.
    pub fn swap(self) -> Pair<U, T> {
        // TODO(module-015): construct `Pair { first: self.second, second: self.first }`.
        panic!("not implemented")
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
        // TODO(module-015): match on `self` and return `true` for `Just(_)`.
        // `matches!(self, Maybe::Just(_))` does the same in one expression.
        let _value = self;
        false
    }

    /// Returns the contained value, or `default` when it is `Nothing`.
    pub fn unwrap_or(self, default: T) -> T {
        // TODO(module-015): match on `self`: `Just(value) => value`,
        // `Nothing => default`.
        let _default = default;
        panic!("not implemented")
    }
}

/// Returns a new vector with all of `a` followed by all of `b`.
pub fn combine<T>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    // TODO(module-015): `a.extend(b)` appends `b`'s elements onto `a`
    // (consume both), then return `a`.
    let _count = b.len();
    a
}
