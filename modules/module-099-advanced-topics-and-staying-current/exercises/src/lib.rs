//! Module 099 — Advanced Topics & Staying Current: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// Sum all elements of a fixed-size array. The array's length is known at
/// compile time via the const generic parameter `N`.
///
/// This function works for *any* array size, including zero-length arrays.
pub fn fixed_size_array_sum<const N: usize>(_arr: [i32; N]) -> i32 {
    todo!("module-099: implement fixed_size_array_sum - iterate over the array and sum its elements. Use `arr.into_iter().sum()`")
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

// TODO(module-099): implement `Container` for `Vec<T>` correctly.
// The stub below compiles but panics — replace the todo!() body with a real
// implementation using `self.as_slice().get(index)`.
impl<T> Container for Vec<T> {
    type Item<'a>
        = &'a T
    where
        T: 'a;

    fn get<'a>(&'a self, _index: usize) -> Option<Self::Item<'a>> {
        todo!("module-099: implement Container::get for Vec<T>")
    }
}

/// Compute the factorial of a number at compile time using a `const fn`.
///
/// You must write a `const fn factorial(n: usize) -> usize` free function,
/// then call it in a `const` context inside this function and return the
/// result.
pub fn demonstrate_const_evaluation() -> usize {
    todo!("module-099: define `const fn factorial(n: usize) -> usize`, then return `factorial(5)` as a const value")
}

/// A compile-time assertion type.
///
/// `Assert<COND>` only lets you access `OK` when `COND` is `true`. If you
/// try to use `Assert::<{ false }>::OK`, the compiler rejects the code.
pub struct Assert<const COND: bool>;

// TODO(module-099): implement `Assert<true>` with a `pub const OK: () = ()`.
// Leave `Assert<false>` with no implementation — that's the assertion mechanism.

/// Demonstrates a static assertion that passes.
///
/// Uses `Assert` to verify a compile-time condition (e.g. `true`, or
/// `std::mem::size_of::<usize>() >= 4`). The function compiles only if the
/// condition holds.
pub fn demonstrate_static_assertion() -> &'static str {
    todo!("module-099: use Assert::<true>::OK to verify a compile-time condition, then return \"static assertion passed\"")
}
