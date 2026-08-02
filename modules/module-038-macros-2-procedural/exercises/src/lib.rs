//! Module 038: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//!
//! A real `#[derive(Describe)]` macro would generate these impls from the
//! type definitions — your job is to write what it would emit, by hand.

/// The trait a `#[derive(Describe)]` macro would implement for any type.
pub trait Describe {
    /// Returns a `TypeName { field: value, ... }` style description.
    fn describe(&self) -> String;
}

/// A two-dimensional point.
#[derive(Debug, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

/// A book with a title and a page count.
#[derive(Debug, Clone, PartialEq)]
pub struct Book {
    pub title: String,
    pub pages: u32,
}

/// A geometric shape, either a circle or a rectangle.
#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: u32, height: u32 },
}

impl Describe for Point {
    // TODO(module-038): replace this placeholder with the impl a derive
    // would emit, e.g. `Point { x: 1, y: 2 }` → `"Point { x: 1, y: 2 }"`.
    // Remember `{{` and `}}` are the literal-brace escapes in `format!`.
    fn describe(&self) -> String {
        String::new()
    }
}

impl Describe for Book {
    // TODO(module-038): emit e.g. `"Book { title: The Rust Book, pages: 400 }"`.
    fn describe(&self) -> String {
        String::new()
    }
}

impl Describe for Shape {
    // TODO(module-038): `match` on `self` and emit e.g.
    // `"Shape::Circle { radius: 1.5 }"` and
    // `"Shape::Rectangle { width: 3, height: 4 }"`.
    fn describe(&self) -> String {
        String::new()
    }
}

/// Maps `Describe` over a slice — the generic harness the tests use.
pub fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}
