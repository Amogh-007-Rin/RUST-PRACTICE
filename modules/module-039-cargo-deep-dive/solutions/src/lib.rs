//! Module 039: reference solution.
//!
//! Cargo features and `cfg` gating: the `demo` feature (declared in this
//! crate's own `Cargo.toml`) unlocks extra items; `cfg(unix)` selects
//! platform-specific behavior; `env!` embeds Cargo metadata.

/// The package name, baked in at compile time by Cargo.
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");

/// The package version from `Cargo.toml`.
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Reports whether the `demo` feature is enabled.
#[cfg(feature = "demo")]
pub fn build_tag() -> &'static str {
    "demo-feature-enabled"
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
pub fn demo_sum(a: u32, b: u32) -> u32 {
    a + b
}

/// Squares a number. Only compiled when the `demo` feature is on.
#[cfg(feature = "demo")]
pub fn demo_square(x: u32) -> u32 {
    x * x
}
