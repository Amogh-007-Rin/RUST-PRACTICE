//! Module 039: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.
//!
//! This crate demonstrates Cargo features and `cfg` gating. The `demo`
//! feature is declared in *this crate's* `Cargo.toml`. Run the tests with
//! `cargo test -p module-039-exercises --features demo`.

/// The package name, baked in at compile time by Cargo.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// The package version from `Cargo.toml`.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Reports whether the `demo` feature is enabled.
#[cfg(feature = "demo")]
pub fn build_tag() -> &'static str {
    // TODO(module-039): return "demo-feature-enabled".
    "placeholder"
}

/// The `not(feature = "demo")` counterpart: this one wins without the flag.
#[cfg(not(feature = "demo"))]
pub fn build_tag() -> &'static str {
    "no-demo-feature"
}

/// Returns "unix" on unix-like targets, "non-unix" elsewhere.
pub fn platform_tag() -> &'static str {
    #[cfg(unix)]
    {
        "unix"
    }
    #[cfg(not(unix))]
    {
        "non-unix"
    }
}

/// Adds two numbers. Only compiled when the `demo` feature is on.
#[cfg(feature = "demo")]
pub fn demo_sum(a: u32, _b: u32) -> u32 {
    // TODO(module-039): rename `_b` to `b` and return `a + b`.
    a
}

/// Squares a number. Only compiled when the `demo` feature is on.
#[cfg(feature = "demo")]
pub fn demo_square(x: u32) -> u32 {
    // TODO(module-039): return `x * x`.
    x
}
