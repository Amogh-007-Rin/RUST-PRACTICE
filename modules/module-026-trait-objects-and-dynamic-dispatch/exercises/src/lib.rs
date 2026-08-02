//! Module 026: Trait Objects & Dynamic Dispatch — exercise scaffold.
//!
//! Fill in every `TODO(module-026)` below so the integration tests in
//! `tests/module_026.rs` pass. The tests define "done".

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
        // TODO(module-026): `std::f64::consts::PI * radius * radius`.
        panic!("not implemented")
    }

    fn name(&self) -> &'static str {
        // TODO(module-026): `"circle"`.
        panic!("not implemented")
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        // TODO(module-026): `side * side`.
        panic!("not implemented")
    }

    fn name(&self) -> &'static str {
        // TODO(module-026): `"square"`.
        panic!("not implemented")
    }
}

/// Sums the areas of mixed shapes via *dynamic* dispatch: each `&dyn Shape`
/// carries a vtable, and `area` is found through it at runtime.
pub fn total_area(_shapes: &[&dyn Shape]) -> f64 {
    // TODO(module-026): `shapes.iter().map(|s| s.area()).sum()`.
    panic!("not implemented")
}

/// Sums the areas of `T` values via *static* dispatch: the compiler
/// monomorphizes this function for each concrete `T`.
pub fn total_area_generic<T: Shape>(_shapes: &[T]) -> f64 {
    // TODO(module-026): `shapes.iter().map(Shape::area).sum()`.
    panic!("not implemented")
}

/// Returns the shape with the largest area, or `None` for an empty slice.
pub fn largest_shape<'a>(_shapes: &[&'a dyn Shape]) -> Option<&'a dyn Shape> {
    // TODO(module-026): `shapes.iter().copied().max_by(...)` comparing
    // `area()` with `f64::total_cmp` (f64 is not `Ord`).
    panic!("not implemented")
}
