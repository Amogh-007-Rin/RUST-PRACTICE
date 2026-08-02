//! Module 038: reference solution.
//!
//! The impls a `#[derive(Describe)]` macro would generate — written by hand.

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
    fn describe(&self) -> String {
        format!("Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl Describe for Book {
    fn describe(&self) -> String {
        format!("Book {{ title: {}, pages: {} }}", self.title, self.pages)
    }
}

impl Describe for Shape {
    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("Shape::Circle {{ radius: {} }}", radius),
            Shape::Rectangle { width, height } => {
                format!(
                    "Shape::Rectangle {{ width: {}, height: {} }}",
                    width, height
                )
            }
        }
    }
}

/// Maps `Describe` over a slice — the generic harness the tests use.
pub fn describe_all<T: Describe>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.describe()).collect()
}
