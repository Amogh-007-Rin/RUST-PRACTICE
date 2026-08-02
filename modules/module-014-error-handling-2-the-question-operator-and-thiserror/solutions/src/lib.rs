//! Module 014: Error Handling II — the `?` operator, custom error types, and
//! `thiserror` (reference solution).

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
    let length = name.chars().count();
    if !(3..=20).contains(&length) {
        return Err(AppError::InvalidInput(
            "username must be between 3 and 20 characters".to_string(),
        ));
    }
    if !name.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err(AppError::InvalidInput(
            "username may only contain letters, digits, and underscores".to_string(),
        ));
    }
    Ok(name.to_string())
}

/// Reads the file at `path`.
///
/// Use `?` so the `std::io::Error` is converted into `AppError::Io` for free.
pub fn read_config(path: &Path) -> Result<String, AppError> {
    let contents = std::fs::read_to_string(path)?;
    Ok(contents)
}

/// Returns the first entry equal to `needle`.
pub fn find_entry<'a>(entries: &'a [String], needle: &str) -> Result<&'a String, AppError> {
    entries
        .iter()
        .find(|entry| entry.as_str() == needle)
        .ok_or_else(|| AppError::NotFound(needle.to_string()))
}

/// Parses `s` as a TCP port number (a `u16` other than zero).
pub fn parse_port(s: &str) -> Result<u16, AppError> {
    let port: u16 = s.parse()?;
    if port == 0 {
        return Err(AppError::InvalidInput("port must be non-zero".to_string()));
    }
    Ok(port)
}

/// Reads a port number from a config file: read the file, then parse it.
pub fn load_port_config(path: &Path) -> Result<u16, AppError> {
    let contents = read_config(path)?;
    parse_port(contents.trim())
}
