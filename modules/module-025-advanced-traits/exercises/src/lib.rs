//! Module 025: Advanced Traits — exercise scaffold.
//!
//! Fill in every `TODO(module-025)` below so the integration tests in
//! `tests/module_025.rs` pass. The tests define "done".

/// A container that can be indexed and measured. The associated type
/// `Item` names the kind of value it holds.
pub trait Container {
    type Item;

    /// Returns the item at `index`, or `None` if out of bounds.
    fn get(&self, index: usize) -> Option<&Self::Item>;

    /// Returns the number of items.
    fn len(&self) -> usize;

    /// Returns `true` when the container holds no items.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T> Container for Vec<T> {
    type Item = T;

    fn get(&self, _index: usize) -> Option<&T> {
        // TODO(module-025): use `self.as_slice().get(index)`.
        panic!("not implemented")
    }

    fn len(&self) -> usize {
        // TODO(module-025): use `self.as_slice().len()`.
        panic!("not implemented")
    }
}

impl Container for String {
    type Item = u8;

    fn get(&self, _index: usize) -> Option<&u8> {
        // TODO(module-025): use `self.as_bytes().get(index)`.
        panic!("not implemented")
    }

    fn len(&self) -> usize {
        // TODO(module-025): `self.len()` resolves to the inherent
        // `String::len` here (inherent methods win over trait methods).
        panic!("not implemented")
    }
}

/// Returns the first item of any `Container`, or `None` when it is empty.
pub fn first<C: Container>(_c: &C) -> Option<&C::Item> {
    // TODO(module-025): return `c.get(0)`.
    panic!("not implemented")
}

/// A 2D vector, demonstrating the newtype pattern: a distinct type wrapping
/// two `f64` values, with its own trait implementations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(pub f64, pub f64);

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, _rhs: Self) -> Self::Output {
        // TODO(module-025): add component-wise:
        // `Vector(self.0 + rhs.0, self.1 + rhs.1)`.
        panic!("not implemented")
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, _rhs: f64) -> Self::Output {
        // TODO(module-025): scale both components by `rhs`.
        panic!("not implemented")
    }
}

/// A temperature in degrees Celsius — another newtype.
pub struct Celsius(pub f64);

/// A temperature in degrees Fahrenheit — yet another newtype.
pub struct Fahrenheit(pub f64);

impl Celsius {
    /// Converts this temperature to Fahrenheit.
    pub fn to_fahrenheit(&self) -> f64 {
        // TODO(module-025): `self.0 * 9.0 / 5.0 + 32.0`.
        panic!("not implemented")
    }
}

impl Fahrenheit {
    /// Converts this temperature to Celsius.
    pub fn to_celsius(&self) -> f64 {
        // TODO(module-025): `(self.0 - 32.0) * 5.0 / 9.0`.
        panic!("not implemented")
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(_celsius: Celsius) -> Self {
        // TODO(module-025): build a `Fahrenheit` by converting.
        panic!("not implemented")
    }
}

/// Something printable that also knows how to summarize itself.
/// The supertrait bound `std::fmt::Display` means: to implement
/// `Summarizable` you must also implement `Display`.
pub trait Summarizable: std::fmt::Display {
    fn summary(&self) -> String;
}

/// A book, the concrete example of `Summarizable`.
#[derive(Debug, PartialEq)]
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

impl std::fmt::Display for Book {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO(module-025): `write!(f, "{} by {}", self.title, self.author)`.
        panic!("not implemented")
    }
}

impl Summarizable for Book {
    fn summary(&self) -> String {
        // TODO(module-025): `format!("{} by {} ({} pages)", self.title,
        // self.author, self.pages)`.
        panic!("not implemented")
    }
}

/// Prints a `Summarizable` as `summary | display`. Because of the
/// supertrait bound, the function can call both `summary()` and `Display`.
pub fn print_summary<S: Summarizable>(_item: &S) -> String {
    // TODO(module-025): `format!("{} | {}", item.summary(), item)`.
    panic!("not implemented")
}
