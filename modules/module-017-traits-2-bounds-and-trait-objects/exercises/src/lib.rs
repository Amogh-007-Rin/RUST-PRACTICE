//! Module 017: Traits II — trait bounds, `where` clauses, and `dyn Trait`.
//!
//! Fill in the `TODO(module-017)` bodies below so the integration tests in
//! `tests/module_017.rs` pass.

/// A two-dimensional shape with an area.
pub trait Area {
    /// The area of the shape in square units.
    fn area(&self) -> f64;

    /// A human-readable description; defaults to the area.
    fn describe(&self) -> String {
        format!("area = {:.2}", self.area())
    }
}

pub struct Circle {
    pub radius: f64,
}

pub struct Rectangle {
    pub width: f64,
    pub height: f64,
}

pub struct Triangle {
    pub base: f64,
    pub height: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        // TODO(module-017): `std::f64::consts::PI * self.radius * self.radius`.
        panic!("not implemented")
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        // TODO(module-017): `self.width * self.height`.
        panic!("not implemented")
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        // TODO(module-017): `self.base * self.height / 2.0`.
        panic!("not implemented")
    }
}

/// Returns the largest area among `shapes` (0.0 for an empty slice).
///
/// Written with a `where` clause instead of an inline bound — same meaning.
pub fn largest_area<T>(shapes: &[T]) -> f64
where
    T: Area,
{
    // TODO(module-017): map each shape to its area, then take the maximum.
    // `shapes.iter().map(Area::area)` gives an iterator of f64; `.fold(0.0, f64::max)`
    // keeps the largest value seen (or 0.0 for empty input).
    let _count = shapes.len();
    0.0
}

/// Returns the sum of the areas of `shapes`.
pub fn total_area<T: Area>(shapes: &[T]) -> f64 {
    // TODO(module-017): `shapes.iter().map(Area::area).sum()`.
    let _count = shapes.len();
    0.0
}

/// Returns the shape with the largest area, or `None` if empty.
pub fn biggest<T>(shapes: &[T]) -> Option<&T>
where
    T: Area,
{
    // TODO(module-017): `shapes.iter().max_by(|a, b| ...)` comparing the two
    // areas with `partial_cmp`. Return its result directly.
    let _count = shapes.len();
    None
}

/// Sums the areas of a heterogeneous collection of `dyn Area` shapes.
pub fn total_area_mixed(shapes: &[&dyn Area]) -> f64 {
    // TODO(module-017): iterate and call `.area()` on each `&dyn Area`.
    let _count = shapes.len();
    0.0
}

/// Describes every shape in a heterogeneous `dyn Area` collection.
pub fn describe_shapes(shapes: &[&dyn Area]) -> Vec<String> {
    // TODO(module-017): map each shape to `shape.describe()` and collect.
    let _count = shapes.len();
    Vec::new()
}
