//! Module 037: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//! All macros are `#[macro_export]`-ed so the integration tests can use them.

/// Builds a `Vec` from any number of expressions, allowing a trailing comma.
#[macro_export]
macro_rules! my_vec {
    ($($elem:expr),* $(,)?) => {{
        // TODO(module-037): push each `$elem` into the vector so the result
        // contains every captured element, e.g. `my_vec![1, 2, 3]` yields
        // `vec![1, 2, 3]`. Hint: `$(vec.push($elem);)*`.
        ::std::vec::Vec::new()
    }};
}

/// Returns the smallest of one or more expressions.
#[macro_export]
macro_rules! min {
    ($a:expr $(, $rest:expr)* $(,)?) => {
        // TODO(module-037): one argument should expand to itself; for
        // multiple arguments, recurse with `$crate::min!($($rest),*)` and
        // combine via `::std::cmp::min`.
        $a
    };
}

/// Sums zero or more expressions as `u64`.
#[macro_export]
macro_rules! sum {
    ($($x:expr),* $(,)?) => {{
        // TODO(module-037): expand `$x as u64 + sum!($($tail),*)` style
        // recursion — one expression recurses on the tail, and the empty
        // input must expand to `0u64`. Add the missing `()` base-case arm.
        0u64
    }};
}

/// Counts the token trees passed to it.
#[macro_export]
macro_rules! count_tt {
    ($($tt:tt)*) => {{
        // TODO(module-037): count every token tree — add a `()` base-case
        // arm expanding to `0usize`, and make this arm expand to
        // `1usize + $crate::count_tt!($($tt)*)` after splitting off the
        // head `tt` from the tail.
        0usize
    }};
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
            // TODO(module-037): make `new` store its arguments — the
            // constructor should be `pub fn new($($field: $ty),*) -> Self`
            // with body `Self { $($field),* }` instead of the defaults
            // below.
            pub fn new($(_: $ty),*) -> Self {
                Self {
                    $($field: Default::default()),*
                }
            }
        }
    };
}
