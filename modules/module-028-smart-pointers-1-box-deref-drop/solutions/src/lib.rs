//! Module 028: Smart Pointers I — reference solution.

/// An arithmetic expression tree. `Box<Expr>` breaks the recursion:
/// without the indirection, `Add` would contain two `Expr` values of
/// unbounded size.
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

/// Evaluates `expr` to a number.
pub fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Num(n) => *n,
        Expr::Add(lhs, rhs) => eval(lhs) + eval(rhs),
        Expr::Mul(lhs, rhs) => eval(lhs) * eval(rhs),
    }
}

/// A minimal `Box`-like type holding a value of type `T` on the stack
/// inside the struct.
pub struct MyBox<T>(T);

impl<T> MyBox<T> {
    /// Wraps `value` in a `MyBox`.
    pub fn new(value: T) -> Self {
        MyBox(value)
    }
}

impl<T> std::ops::Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}

use std::sync::atomic::{AtomicU32, Ordering};

/// A counter of `Gadget`s dropped so far. `Gadget::drop` increments it,
/// which is how the tests observe the `Drop` impl running.
pub static DROPPED_GADGETS: AtomicU32 = AtomicU32::new(0);

/// A unit struct whose whole point is being dropped.
pub struct Gadget;

impl Drop for Gadget {
    fn drop(&mut self) {
        DROPPED_GADGETS.fetch_add(1, Ordering::Relaxed);
    }
}
