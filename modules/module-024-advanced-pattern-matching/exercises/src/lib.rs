//! Module 024: Advanced Pattern Matching — exercise scaffold.
//!
//! Fill in every `TODO(module-024)` below so the integration tests in
//! `tests/module_024.rs` pass. The tests define "done".

/// Returns a description of where `point` sits in the plane:
/// origin, one of the four axes, or one of the four quadrants.
pub fn describe_point(_point: (i32, i32)) -> &'static str {
    // TODO(module-024): match on the tuple. Use `if` guards for the axis
    // directions (positive/negative x/y) and the quadrants.
    panic!("not implemented")
}

/// A geometric shape.
#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

/// Describes a shape, calling out squares and invalid (negative-radius)
/// circles. Use an `@` binding and a match guard.
pub fn describe_shape(_shape: &Shape) -> String {
    // TODO(module-024): match with `radius: r @ 0.0..` for valid circles,
    // `radius: r` for negative radii, a `width == height` guard for
    // squares, and a plain rectangle arm.
    panic!("not implemented")
}

/// A role within a system.
#[derive(Debug, PartialEq)]
pub enum Role {
    Admin,
    Member { joined_year: u32 },
}

/// A registered user.
#[derive(Debug, PartialEq)]
pub struct User {
    pub name: String,
    pub role: Role,
}

/// Produces a personalized greeting. New members (joined in 2024) get a
/// special message. Use nested destructuring.
pub fn greeting(_user: &User) -> String {
    // TODO(module-024): destructure `User { name, role: ... }` and inside
    // it `Role::Member { joined_year: ... }`.
    panic!("not implemented")
}

/// Parses `"x, y"` into a pair of integers, returning `None` if there
/// aren't exactly two comma-separated parts or either fails to parse.
pub fn parse_i32_pair(_input: &str) -> Option<(i32, i32)> {
    // TODO(module-024): split on ',', trim, collect into a `Vec<&str>`,
    // then match on `parts.as_slice()` with a slice pattern `[a, b]`.
    // Use `.parse().ok()?` for the numbers.
    panic!("not implemented")
}
