//! Module 007: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// A book in the library.
pub struct Book {
    pub title: String,
    pub author: String,
    pub pages: u32,
}

impl Book {
    /// Creates a new `Book` from its fields.
    pub fn new(title: &str, author: &str, pages: u32) -> Book {
        // TODO(module-007): build the struct with field-init shorthand.
        // `title` and `author` need `.to_string()` to become `String`s.
        let _ = (title, author, pages);
        panic!("TODO(module-007): implement Book::new")
    }

    /// Returns a one-line description like `"Title" by Author (NNN pages)`.
    pub fn summary(&self) -> String {
        // TODO(module-007): borrow `self` and format its fields, e.g.
        // `format!("\"{}\" by {} ({} pages)", self.title, self.author, self.pages)`.
        panic!("TODO(module-007): implement Book::summary")
    }

    /// Returns `true` if the book is longer than 400 pages.
    pub fn is_long(&self) -> bool {
        // TODO(module-007): compare `self.pages > 400`.
        let _ = &self;
        panic!("TODO(module-007): implement Book::is_long")
    }
}

/// A point in 2D space (a tuple struct with two `f64`s).
pub struct Point(pub f64, pub f64);

impl Point {
    /// Returns the Euclidean distance between this point and `other`.
    pub fn distance(&self, other: &Point) -> f64 {
        // TODO(module-007): `((dx).powi(2) + (dy).powi(2)).sqrt()` where
        // `dx = other.0 - self.0` and `dy = other.1 - self.1`.
        let _ = (self, other);
        panic!("TODO(module-007): implement Point::distance")
    }
}
