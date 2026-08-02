//! Case study B: the core of a URL shortener.
//!
//! The pieces that make a URL shortener an interview-worthy case study:
//!
//! 1. **`encode_id` / `decode_id`** — a bijective base-N encoding, so every
//!    id has exactly one code and every code decodes to exactly one id.
//! 2. **`Storage` trait** — the system is defined against an interface,
//!    not against a concrete database; in-memory now, Postgres in
//!    production, and the trait lets tests swap in a fake.
//! 3. **Collision handling** — custom slugs can collide with existing
//!    codes, and the generator must say so instead of silently overwriting.
//!
//! (Analytics — click counts, referrers — is where this case study goes in
//! Capstone 10. Here we build the core.)

use std::collections::HashMap;

/// The classic 62-character alphabet: digits, lowercase, uppercase.
pub const DEFAULT_ALPHABET: &str = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

/// Encodes `id` as a base-N string using `alphabet` (N = alphabet length).
///
/// The encoding is bijective: `decode_id(encode_id(id, a), a) == Some(id)`
/// for every `id`. `id 0` encodes to the first alphabet character.
pub fn encode_id(id: u64, alphabet: &str) -> String {
    // TODO(module-095): the same algorithm as decimal-to-binary: repeatedly
    // divide by the base, pushing the remainder as a character, then
    // reverse. Special case `id == 0` → the first alphabet character.
    // Empty alphabets return an empty string.
    let _ = (id, alphabet);
    panic!("stub: encode_id is not implemented yet");
}

/// Decodes a base-N string back into an id, or `None` when the string
/// contains a character outside `alphabet` (or the alphabet is empty).
pub fn decode_id(code: &str, alphabet: &str) -> Option<u64> {
    // TODO(module-095): fold over the characters, `value = value * base +
    // digit`, where `digit` is the character's position in `alphabet`.
    // Characters not in the alphabet → `None`.
    let _ = (code, alphabet);
    panic!("stub: decode_id is not implemented yet");
}

/// Why a storage insert failed. Right now there is exactly one way to fail,
/// which is the point: the error enum is *complete*.
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
/// knows nothing about *how* this is stored — that's what makes it
/// testable and replaceable.
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

/// A `Storage` backed by an in-memory `HashMap`. This is what the tests
/// use; production would implement `Storage` for a database pool instead.
pub struct HashMapStorage {
    #[allow(dead_code)]
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
        // TODO(module-095): look the code up and return the URL as a `&str`
        // (hint: `self.links.get(code).map(String::as_str)`).
        let _ = code;
        panic!("stub: HashMapStorage::get is not implemented yet");
    }

    fn insert(&mut self, code: String, url: String) -> Result<(), StorageError> {
        // TODO(module-095): `HashMap::insert` returns the *previous* value
        // — a `Some(_)` here means the code was taken.
        let _ = (code, url);
        panic!("stub: HashMapStorage::insert is not implemented yet");
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
    #[allow(dead_code)]
    storage: S,
    #[allow(dead_code)]
    alphabet: String,
    #[allow(dead_code)]
    min_code_len: usize,
    #[allow(dead_code)]
    next_id: u64,
}

impl<S: Storage> Shortener<S> {
    /// Creates a shortener over `storage` using `DEFAULT_ALPHABET` and
    /// 4-character minimum codes.
    pub fn new(storage: S) -> Self {
        Self::with_alphabet_and_min_len(storage, DEFAULT_ALPHABET, 4)
    }

    /// Creates a shortener with a custom alphabet and minimum code length.
    /// A tiny alphabet (like `"ab"`) is a great way to force collisions and
    /// test the retry logic.
    pub fn with_alphabet_and_min_len(storage: S, alphabet: &str, min_code_len: usize) -> Self {
        Self {
            storage,
            alphabet: alphabet.to_string(),
            min_code_len,
            next_id: 0,
        }
    }

    /// Shortens `url`, returning the generated code. `custom_code` requests
    /// a specific slug when `Some`, otherwise the next counter id is
    /// encoded.
    pub fn create(&mut self, url: &str, custom_code: Option<&str>) -> Result<String, ShortenError> {
        // TODO(module-095): three steps:
        // 1. Validate `url` (must start with "http://" or "https://").
        // 2. If `custom_code` is `Some`, validate it (3..=24 chars, all in
        //    the alphabet) and `insert` it — mapping `CodeTaken` to
        //    `ShortenError::CodeTaken`.
        // 3. Otherwise loop: encode `next_id` (padded with the alphabet's
        //    first character up to `min_code_len`), try to insert; on
        //    `CodeTaken` (theoretical for counter ids) bump the id and
        //    retry; on success, advance `next_id` and return the code.
        let _ = (url, custom_code);
        panic!("stub: Shortener::create is not implemented yet");
    }

    /// Resolves a code back to its URL, or `None` when unknown.
    pub fn resolve(&self, code: &str) -> Option<&str> {
        // TODO(module-095): delegate to the storage.
        let _ = code;
        panic!("stub: Shortener::resolve is not implemented yet");
    }
}
