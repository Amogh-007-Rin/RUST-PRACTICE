//! Module 099 — Advanced Topics & Staying Current: reference solution.

/// Sum all elements of a fixed-size array. The array's length is known at
/// compile time via the const generic parameter `N`.
pub fn fixed_size_array_sum<const N: usize>(arr: [i32; N]) -> i32 {
    arr.into_iter().sum()
}

/// A trait with a Generic Associated Type (GAT).
///
/// Unlike a regular associated type, a GAT is parameterized by a lifetime
/// (or type parameter). This means different lifetimes can yield different
/// types without changing the implementing type.
pub trait Container {
    type Item<'a>
    where
        Self: 'a;
    fn get<'a>(&'a self, index: usize) -> Option<Self::Item<'a>>;
}

impl<T> Container for Vec<T> {
    type Item<'a>
        = &'a T
    where
        T: 'a;

    fn get<'a>(&'a self, index: usize) -> Option<Self::Item<'a>> {
        self.as_slice().get(index)
    }
}

/// Computes the factorial of `n` at compile time.
const fn factorial(n: usize) -> usize {
    match n {
        0 | 1 => 1,
        n => n * factorial(n - 1),
    }
}

/// Returns the factorial of 5, computed entirely at compile time via
/// `const fn`.
pub fn demonstrate_const_evaluation() -> usize {
    const RESULT: usize = factorial(5);
    RESULT
}

/// A compile-time assertion type.
///
/// Only `Assert<true>` has the `OK` constant. Attempting to use
/// `Assert::<{ false }>::OK` produces a compile error — that is the
/// assertion mechanism.
pub struct Assert<const COND: bool>;

impl Assert<true> {
    pub const OK: () = ();
}

/// Demonstrates a static assertion that passes.
pub fn demonstrate_static_assertion() -> &'static str {
    let _: () = Assert::<true>::OK;
    "static assertion passed"
}
