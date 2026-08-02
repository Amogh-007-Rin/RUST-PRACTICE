//! Module 017: Traits II — trait bounds, `where` clauses, and `dyn Trait`
//! (reference solution).

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
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl Area for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Area for Triangle {
    fn area(&self) -> f64 {
        self.base * self.height / 2.0
    }
}

/// Returns the largest area among `shapes` (0.0 for an empty slice).
///
/// Written with a `where` clause instead of an inline bound — same meaning.
pub fn largest_area<T>(shapes: &[T]) -> f64
where
    T: Area,
{
    shapes.iter().map(Area::area).fold(0.0, f64::max)
}

/// Returns the sum of the areas of `shapes`.
pub fn total_area<T: Area>(shapes: &[T]) -> f64 {
    shapes.iter().map(Area::area).sum()
}

/// Returns the shape with the largest area, or `None` if empty.
pub fn biggest<T>(shapes: &[T]) -> Option<&T>
where
    T: Area,
{
    shapes
        .iter()
        .max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap())
}

/// Sums the areas of a heterogeneous collection of `dyn Area` shapes.
pub fn total_area_mixed(shapes: &[&dyn Area]) -> f64 {
    shapes.iter().map(|shape| shape.area()).sum()
}

/// Describes every shape in a heterogeneous `dyn Area` collection.
pub fn describe_shapes(shapes: &[&dyn Area]) -> Vec<String> {
    shapes.iter().map(|shape| shape.describe()).collect()
}
