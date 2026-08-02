//! Module 014: Error Handling II — the `?` operator, custom error types, and
//! `thiserror`.
//!
//! Fill in the `TODO(module-014)` bodies below so the integration tests in
//! `tests/module_014.rs` pass.

use std::path::Path;

/// The single error type for this crate.
///
/// `#[from]` generates `From` impls so that `?` can convert the wrapped
/// errors automatically. The `#[error("...")]` attributes produce the
/// `Display` implementation.
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid input: {0}")]
    InvalidInput(String),
    #[error("entry not found: {0}")]
    NotFound(String),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid port number: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}

/// Returns `name` if it is a valid username (3–20 characters, only letters,
/// digits, and underscores).
pub fn validate_username(name: &str) -> Result<String, AppError> {
    // TODO(module-014): check the length with `name.chars().count()` and the
    // characters with `name.chars().all(...)`, returning
    // `Err(AppError::InvalidInput(...))` with a helpful message for each
    // failure. Return `Ok(name.to_string())` at the end.
    let _len = name.len();
    Err(AppError::InvalidInput("not implemented".to_string()))
}

/// Reads the file at `path`.
///
/// Use `?` so the `std::io::Error` is converted into `AppError::Io` for free.
pub fn read_config(path: &Path) -> Result<String, AppError> {
    // TODO(module-014): `let contents = std::fs::read_to_string(path)?;` then
    // `Ok(contents)`.
    let _parent = path.parent();
    Err(AppError::Io(std::io::Error::other("not implemented")))
}

/// Returns the first entry equal to `needle`.
pub fn find_entry<'a>(entries: &'a [String], needle: &str) -> Result<&'a String, AppError> {
    // TODO(module-014): `entries.iter().find(...)` gives `Option<&String>`.
    // Use `.ok_or_else(|| AppError::NotFound(needle.to_string()))` so a miss
    // becomes an error naming the missing entry.
    let _count = entries.len();
    let _needle = needle;
    Err(AppError::NotFound("not implemented".to_string()))
}

/// Parses `s` as a TCP port number (a `u16` other than zero).
pub fn parse_port(s: &str) -> Result<u16, AppError> {
    // TODO(module-014): `let port: u16 = s.parse()?;` converts a parse
    // failure into `AppError::ParseInt` automatically. Then reject the value
    // `0` with `AppError::InvalidInput`.
    let _len = s.len();
    Err(AppError::InvalidInput("not implemented".to_string()))
}

/// Reads a port number from a config file: read the file, then parse it.
pub fn load_port_config(path: &Path) -> Result<u16, AppError> {
    // TODO(module-014): chain the two helpers with `?` — read the contents
    // with `read_config(path)?`, trim it, then hand it to `parse_port(...)?`.
    // The `?` operator works on any error that converts into `AppError`.
    let _parent = path.parent();
    Err(AppError::InvalidInput("not implemented".to_string()))
}
