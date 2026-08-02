//! Module 037: reference solution.
//!
//! Five idiomatic `macro_rules!` macros: a mini-`vec!`, a recursive
//! variadic minimum, a recursive variadic sum, a token-tree counter, and a
//! struct-defining factory.

/// Builds a `Vec` from any number of expressions, allowing a trailing comma.
#[macro_export]
macro_rules! my_vec {
    () => {
        ::std::vec::Vec::new()
    };
    ($($elem:expr),* $(,)?) => {{
        let mut vec = ::std::vec::Vec::new();
        $(vec.push($elem);)*
        vec
    }};
}

/// Returns the smallest of one or more expressions.
#[macro_export]
macro_rules! min {
    ($a:expr $(,)?) => {
        $a
    };
    ($a:expr, $($rest:expr),+ $(,)?) => {{
        let first = $a;
        ::std::cmp::min(first, $crate::min!($($rest),+))
    }};
}

/// Sums zero or more expressions as `u64`.
#[macro_export]
macro_rules! sum {
    () => {
        0u64
    };
    ($head:expr $(, $tail:expr)* $(,)?) => {{
        $head as u64 + $crate::sum!($($tail),*)
    }};
}

/// Counts the token trees passed to it.
#[macro_export]
macro_rules! count_tt {
    () => {
        0usize
    };
    ($_head:tt $($tail:tt)*) => {
        1usize + $crate::count_tt!($($tail)*)
    };
}

/// Defines a `pub struct` with `pub` fields and a `new` constructor.
#[macro_export]
macro_rules! def_struct {
    ($name:ident { $($field:ident: $ty:ty),* $(,)? }) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            $(pub $field: $ty),*
        }
        impl $name {
            pub fn new($($field: $ty),*) -> Self {
                Self { $($field),* }
            }
        }
    };
}
