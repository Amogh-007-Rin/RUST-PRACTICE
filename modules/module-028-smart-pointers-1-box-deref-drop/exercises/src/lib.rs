//! Module 028: Smart Pointers I — exercise scaffold.
//!
//! Fill in every `TODO(module-028)` below so the integration tests in
//! `tests/module_028.rs` pass. The tests define "done".

/// An arithmetic expression tree. `Box<Expr>` breaks the recursion:
/// without the indirection, `Add` would contain two `Expr` values of
/// unbounded size.
pub enum Expr {
    Num(i64),
    Add(Box<Expr>, Box<Expr>),
    Mul(Box<Expr>, Box<Expr>),
}

/// Evaluates `expr` to a number.
pub fn eval(_expr: &Expr) -> i64 {
    // TODO(module-028): match on `expr`: `Num(n) => *n`, and recurse
    // through `Add`/`Mul` with `eval(lhs) + eval(rhs)` etc.
    panic!("not implemented")
}

/// A minimal `Box`-like type holding a value of type `T` on the stack
/// inside the struct.
///
/// The inner field is not read until you implement `Deref`/`DerefMut`.
#[allow(dead_code)]
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
        // TODO(module-028): return `&self.0`.
        panic!("not implemented")
    }
}

impl<T> std::ops::DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        // TODO(module-028): return `&mut self.0`.
        panic!("not implemented")
    }
}

use std::sync::atomic::AtomicU32;

/// A counter of `Gadget`s dropped so far. `Gadget::drop` increments it,
/// which is how the tests observe the `Drop` impl running.
pub static DROPPED_GADGETS: AtomicU32 = AtomicU32::new(0);

/// A unit struct whose whole point is being dropped.
pub struct Gadget;

impl Drop for Gadget {
    fn drop(&mut self) {
        // TODO(module-028): `DROPPED_GADGETS.fetch_add(1, Ordering::Relaxed);`
    }
}
