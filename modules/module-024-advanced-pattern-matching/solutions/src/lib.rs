//! Module 024: Advanced Pattern Matching — reference solution.

/// Returns a description of where `point` sits in the plane:
/// origin, one of the four axes, or one of the four quadrants.
pub fn describe_point(point: (i32, i32)) -> &'static str {
    match point {
        (0, 0) => "origin",
        (x, 0) if x > 0 => "positive x-axis",
        (_, 0) => "negative x-axis",
        (0, y) if y > 0 => "positive y-axis",
        (0, _) => "negative y-axis",
        (x, y) if x > 0 && y > 0 => "quadrant I",
        (x, y) if x < 0 && y > 0 => "quadrant II",
        (x, y) if x < 0 && y < 0 => "quadrant III",
        (_, _) => "quadrant IV",
    }
}

/// A geometric shape.
#[derive(Debug, PartialEq)]
pub enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
}

/// Describes a shape, calling out squares and invalid (negative-radius)
/// circles. Use an `@` binding and a match guard.
pub fn describe_shape(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius: r @ 0.0.. } => format!("circle of radius {r}"),
        Shape::Circle { radius: r } => format!("invalid circle with radius {r}"),
        Shape::Rectangle { width, height } if width == height => {
            format!("square of side {width}")
        }
        Shape::Rectangle { width, height } => format!("rectangle {width} x {height}"),
    }
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
pub fn greeting(user: &User) -> String {
    match user {
        User {
            name,
            role: Role::Admin,
        } => format!("Welcome back, admin {name}"),
        User {
            name,
            role: Role::Member { joined_year: 2024 },
        } => format!("{name}, welcome aboard"),
        User {
            name,
            role: Role::Member { .. },
        } => format!("Hi {name}"),
    }
}

/// Parses `"x, y"` into a pair of integers, returning `None` if there
/// aren't exactly two comma-separated parts or either fails to parse.
pub fn parse_i32_pair(input: &str) -> Option<(i32, i32)> {
    let parts: Vec<&str> = input.split(',').map(str::trim).collect();
    match parts.as_slice() {
        [a, b] => Some((a.parse().ok()?, b.parse().ok()?)),
        _ => None,
    }
}
