//! Module 025: Advanced Traits — reference solution.

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

    fn get(&self, index: usize) -> Option<&T> {
        self.as_slice().get(index)
    }

    fn len(&self) -> usize {
        self.as_slice().len()
    }
}

impl Container for String {
    type Item = u8;

    fn get(&self, index: usize) -> Option<&u8> {
        self.as_bytes().get(index)
    }

    fn len(&self) -> usize {
        self.len()
    }
}

/// Returns the first item of any `Container`, or `None` when it is empty.
pub fn first<C: Container>(c: &C) -> Option<&C::Item> {
    c.get(0)
}

/// A 2D vector, demonstrating the newtype pattern: a distinct type wrapping
/// two `f64` values, with its own trait implementations.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector(pub f64, pub f64);

impl std::ops::Add for Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Mul<f64> for Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector(self.0 * rhs, self.1 * rhs)
    }
}

/// A temperature in degrees Celsius — another newtype.
pub struct Celsius(pub f64);

/// A temperature in degrees Fahrenheit — yet another newtype.
pub struct Fahrenheit(pub f64);

impl Celsius {
    /// Converts this temperature to Fahrenheit.
    pub fn to_fahrenheit(&self) -> f64 {
        self.0 * 9.0 / 5.0 + 32.0
    }
}

impl Fahrenheit {
    /// Converts this temperature to Celsius.
    pub fn to_celsius(&self) -> f64 {
        (self.0 - 32.0) * 5.0 / 9.0
    }
}

impl From<Celsius> for Fahrenheit {
    fn from(celsius: Celsius) -> Self {
        Fahrenheit(celsius.to_fahrenheit())
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} by {}", self.title, self.author)
    }
}

impl Summarizable for Book {
    fn summary(&self) -> String {
        format!("{} by {} ({} pages)", self.title, self.author, self.pages)
    }
}

/// Prints a `Summarizable` as `summary | display`. Because of the
/// supertrait bound, the function can call both `summary()` and `Display`.
pub fn print_summary<S: Summarizable>(item: &S) -> String {
    format!("{} | {}", item.summary(), item)
}
