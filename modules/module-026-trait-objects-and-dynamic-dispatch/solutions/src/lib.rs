//! Module 026: Trait Objects & Dynamic Dispatch — reference solution.

/// A geometric shape that can report its area and name.
/// Object-safe: no generics, no `Self` by value, no associated functions.
pub trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &'static str;
}

/// A circle, defined by its radius.
pub struct Circle {
    pub radius: f64,
}

/// A square, defined by its side length.
pub struct Square {
    pub side: f64,
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    fn name(&self) -> &'static str {
        "circle"
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }

    fn name(&self) -> &'static str {
        "square"
    }
}

/// Sums the areas of mixed shapes via *dynamic* dispatch: each `&dyn Shape`
/// carries a vtable, and `area` is found through it at runtime.
pub fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

/// Sums the areas of `T` values via *static* dispatch: the compiler
/// monomorphizes this function for each concrete `T`.
pub fn total_area_generic<T: Shape>(shapes: &[T]) -> f64 {
    shapes.iter().map(Shape::area).sum()
}

/// Returns the shape with the largest area, or `None` for an empty slice.
pub fn largest_shape<'a>(shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
    shapes
        .iter()
        .copied()
        .max_by(|a, b| a.area().total_cmp(&b.area()))
}
