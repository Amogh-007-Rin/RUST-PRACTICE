//! Case study B: the core of a URL shortener.
//!
//! The pieces that make a URL shortener an interview-worthy case study:
//! bijective base-N encoding, a `Storage` abstraction, and explicit
//! collision handling. (Analytics moves to Capstone 10.)

use std::collections::HashMap;

/// The classic 62-character alphabet: digits, lowercase, uppercase.
pub const DEFAULT_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Encodes `id` as a base-N string using `alphabet` (N = alphabet length).
///
/// The encoding is bijective: `decode_id(encode_id(id, a), a) == Some(id)`
/// for every `id`. `id 0` encodes to the first alphabet character.
pub fn encode_id(mut id: u64, alphabet: &str) -> String {
    let alphabet: Vec<char> = alphabet.chars().collect();
    if alphabet.is_empty() {
        return String::new();
    }
    let base = alphabet.len() as u64;
    let mut chars = Vec::new();
    loop {
        chars.push(alphabet[(id % base) as usize]);
        id /= base;
        if id == 0 {
            break;
        }
    }
    chars.reverse();
    chars.into_iter().collect()
}

/// Decodes a base-N string back into an id, or `None` when the string
/// contains a character outside `alphabet` (or the alphabet is empty).
pub fn decode_id(code: &str, alphabet: &str) -> Option<u64> {
    let alphabet: Vec<char> = alphabet.chars().collect();
    if alphabet.is_empty() || code.is_empty() {
        return None;
    }
    let base = alphabet.len() as u64;
    let mut value = 0_u64;
    for c in code.chars() {
        let digit = alphabet.iter().position(|&a| a == c)? as u64;
        value = value * base + digit;
    }
    Some(value)
}

/// Why a storage insert failed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StorageError {
    /// A link with this code already exists.
    CodeTaken,
}

impl std::fmt::Display for StorageError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "code already taken")
    }
}

impl std::error::Error for StorageError {}

/// The storage abstraction: a code ↔ URL map. The rest of the shortener
/// knows nothing about *how* this is stored.
pub trait Storage {
    /// Returns the URL stored under `code`, if any.
    fn get(&self, code: &str) -> Option<&str>;

    /// Stores `url` under `code`. Fails with `CodeTaken` when the code
    /// already exists — callers must never silently overwrite.
    fn insert(&mut self, code: String, url: String) -> Result<(), StorageError>;

    /// Convenience: is `code` already in use?
    fn contains(&self, code: &str) -> bool {
        self.get(code).is_some()
    }
}

/// A `Storage` backed by an in-memory `HashMap`.
pub struct HashMapStorage {
    links: HashMap<String, String>,
}

impl HashMapStorage {
    /// Creates an empty in-memory store.
    pub fn new() -> Self {
        Self {
            links: HashMap::new(),
        }
    }
}

impl Default for HashMapStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage for HashMapStorage {
    fn get(&self, code: &str) -> Option<&str> {
        self.links.get(code).map(String::as_str)
    }

    fn insert(&mut self, code: String, url: String) -> Result<(), StorageError> {
        if self.links.contains_key(&code) {
            return Err(StorageError::CodeTaken);
        }
        self.links.insert(code, url);
        Ok(())
    }
}

/// Why `create` refused a request.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShortenError {
    /// The URL does not start with `http://` or `https://`.
    InvalidUrl,
    /// The custom code is empty, too long, or uses characters outside the
    /// alphabet.
    InvalidCode,
    /// The custom code is already in use.
    CodeTaken,
}

impl std::fmt::Display for ShortenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            ShortenError::InvalidUrl => "URL must start with http:// or https://",
            ShortenError::InvalidCode => "code must be 3-24 alphabet characters",
            ShortenError::CodeTaken => "code already in use",
        };
        write!(f, "{msg}")
    }
}

impl std::error::Error for ShortenError {}

/// The shortener itself: an id counter plus a code-generation policy, built
/// on top of any `Storage`.
pub struct Shortener<S: Storage> {
    storage: S,
    alphabet: String,
    min_code_len: usize,
    next_id: u64,
}

impl<S: Storage> Shortener<S> {
    /// Creates a shortener over `storage` using `DEFAULT_ALPHABET` and
    /// 4-character minimum codes.
    pub fn new(storage: S) -> Self {
        Self::with_alphabet_and_min_len(storage, DEFAULT_ALPHABET, 4)
    }

    /// Creates a shortener with a custom alphabet and minimum code length.
    pub fn with_alphabet_and_min_len(storage: S, alphabet: &str, min_code_len: usize) -> Self {
        Self {
            storage,
            alphabet: alphabet.to_string(),
            min_code_len,
            next_id: 0,
        }
    }

    /// Shortens `url`, returning the generated code.
    pub fn create(&mut self, url: &str, custom_code: Option<&str>) -> Result<String, ShortenError> {
        if !valid_url(url) {
            return Err(ShortenError::InvalidUrl);
        }
        if let Some(custom) = custom_code {
            if custom.len() < 3
                || custom.len() > 24
                || !custom.chars().all(|c| self.alphabet.contains(c))
            {
                return Err(ShortenError::InvalidCode);
            }
            self.storage
                .insert(custom.to_string(), url.to_string())
                .map_err(|_| ShortenError::CodeTaken)?;
            return Ok(custom.to_string());
        }
        // Counter-based codes are unique by construction; the retry loop is
        // belt-and-braces against a code that somehow collides anyway.
        loop {
            let mut code = encode_id(self.next_id, &self.alphabet);
            while code.len() < self.min_code_len {
                code.insert(0, self.alphabet.chars().next().unwrap());
            }
            self.next_id += 1;
            match self.storage.insert(code.clone(), url.to_string()) {
                Ok(()) => return Ok(code),
                Err(StorageError::CodeTaken) => continue,
            }
        }
    }

    /// Resolves a code back to its URL, or `None` when unknown.
    pub fn resolve(&self, code: &str) -> Option<&str> {
        self.storage.get(code)
    }
}

/// Minimal hand-rolled URL check: an http(s) scheme with a non-empty host.
/// Production would parse with the `url` crate; this keeps the case study
/// dependency-free.
fn valid_url(url: &str) -> bool {
    let rest = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"));
    matches!(rest, Some(r) if !r.is_empty() && !r.starts_with('/'))
}
