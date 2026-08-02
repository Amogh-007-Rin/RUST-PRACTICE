//! Module 081: Introduction to WebAssembly — exercise scaffold.
//!
//! This crate simulates the core parts of a WebAssembly runtime *on the host*:
//! linear memory, the import/export table, and the module structure. Real WASM
//! runs in a browser (or any host that can instantiate the bytecode); here you
//! build the same mental model in pure Rust so everything is testable with
//! plain `cargo test`.
//!
//! Fill in every `// TODO(module-081)` below so the integration tests in
//! `tests/` pass.

use std::collections::HashMap;

/// Errors produced by operations on linear memory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryError {
    /// A load or store reached outside the allocated bytes.
    OutOfBounds {
        /// Byte offset where the access started.
        offset: usize,
        /// Number of bytes the access wanted to read or write.
        length: usize,
        /// Current size of the memory in bytes.
        size: usize,
    },
    /// `grow` would push the memory past its configured maximum.
    GrowOverflow {
        /// Current size in bytes.
        current: usize,
        /// The configured maximum in bytes.
        max: usize,
    },
}

/// A simulated WebAssembly "linear memory": a flat, byte-addressable array.
///
/// In real WASM every instance gets one (or more) of these; the JS host and
/// the wasm module share the same buffer, which is why "passing" data between
/// them means writing bytes here and reading them back.
#[derive(Debug, Clone)]
pub struct LinearMemory {
    bytes: Vec<u8>,
    max_size: Option<usize>,
}

impl LinearMemory {
    /// Creates a memory with `size` bytes and no maximum.
    pub fn new(size: usize) -> Self {
        Self {
            bytes: vec![0u8; size],
            max_size: None,
        }
    }

    /// Creates a memory with `size` bytes and an optional maximum size.
    pub fn with_max(size: usize, max: Option<usize>) -> Self {
        Self {
            bytes: vec![0u8; size],
            max_size: max,
        }
    }

    /// Current size of the memory in bytes.
    pub fn size(&self) -> usize {
        self.bytes.len()
    }

    /// The configured maximum size, if any.
    pub fn max_size(&self) -> Option<usize> {
        self.max_size
    }

    /// Grows the memory by `additional` bytes.
    ///
    /// Returns the previous size (this matches WASM's `memory.grow`).
    pub fn grow(&mut self, additional: usize) -> Result<usize, MemoryError> {
        // TODO(module-081): append `additional` zero bytes and return the old
        // size. If `max_size` is set and the result would exceed it, return
        // `MemoryError::GrowOverflow` instead and leave the memory untouched.
        panic!("TODO(module-081): implement LinearMemory::grow (additional = {additional})");
    }

    /// Loads one byte at `offset`.
    pub fn load_u8(&self, offset: usize) -> Result<u8, MemoryError> {
        // TODO(module-081): return the byte at `offset`, or an
        // `OutOfBounds` error when `offset` is past the end.
        panic!("TODO(module-081): implement LinearMemory::load_u8 (offset = {offset})");
    }

    /// Stores one byte at `offset`.
    pub fn store_u8(&mut self, offset: usize, value: u8) -> Result<(), MemoryError> {
        // TODO(module-081): write `value` at `offset`, or return an
        // `OutOfBounds` error when `offset` is past the end.
        panic!("TODO(module-081): implement LinearMemory::store_u8 (offset = {offset}, value = {value})");
    }

    /// Loads a little-endian `u32` starting at `offset`.
    ///
    /// `0xDEADBEEF` at offset 0 must read back as bytes
    /// `[0xEF, 0xBE, 0xAD, 0xDE]`.
    pub fn load_u32(&self, offset: usize) -> Result<u32, MemoryError> {
        // TODO(module-081): read 4 bytes at `offset` (checking bounds!) and
        // assemble them as a little-endian u32. Hint: combine with shifts.
        panic!("TODO(module-081): implement LinearMemory::load_u32 (offset = {offset})");
    }

    /// Stores a little-endian `u32` starting at `offset`.
    pub fn store_u32(&mut self, offset: usize, value: u32) -> Result<(), MemoryError> {
        // TODO(module-081): write `value` as 4 little-endian bytes at
        // `offset`. Fail with `OutOfBounds` if fewer than 4 bytes remain.
        panic!("TODO(module-081): implement LinearMemory::store_u32 (offset = {offset}, value = {value})");
    }

    /// Borrows the whole memory as a byte slice (like WASM's exported memory
    /// being accessed from JS via a typed array).
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Mutable borrow of the whole memory.
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        &mut self.bytes
    }
}

/// Errors produced by looking up exports or imports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportError {
    /// No export with this name exists.
    NotFound(String),
    /// An export with this name was already registered.
    AlreadyExported(String),
}

/// A simulated wasm module: a name, an import list, an export table, and a
/// linear memory.
///
/// Real `.wasm` files carry the same information in binary *sections*
/// (type, import, function, memory, export, code, ...) — the structure here
/// is the decoded model of those sections.
#[derive(Debug, Clone)]
pub struct WasmModule {
    name: String,
    exports: HashMap<String, String>,
    imports: Vec<(String, String)>,
    memory: LinearMemory,
}

impl WasmModule {
    /// Creates a module with the given name and memory size.
    pub fn new(name: &str, memory_size: usize) -> Self {
        Self {
            name: name.to_string(),
            exports: HashMap::new(),
            imports: Vec::new(),
            memory: LinearMemory::new(memory_size),
        }
    }

    /// The module's name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Number of imports declared by the module.
    pub fn import_count(&self) -> usize {
        self.imports.len()
    }

    /// Number of exports registered.
    pub fn export_count(&self) -> usize {
        self.exports.len()
    }

    /// Names of all registered exports, sorted alphabetically.
    pub fn export_names(&self) -> Vec<String> {
        let mut names: Vec<String> = self.exports.keys().cloned().collect();
        names.sort();
        names
    }

    /// The module's linear memory (shared with the host).
    pub fn memory(&self) -> &LinearMemory {
        &self.memory
    }

    /// Mutable access to the module's linear memory.
    pub fn memory_mut(&mut self) -> &mut LinearMemory {
        &mut self.memory
    }

    /// Declares an import `(module, name)`.
    ///
    /// Returns `false` if the exact pair is already imported.
    pub fn add_import(&mut self, module: &str, name: &str) -> bool {
        // TODO(module-081): push `(module, name)` onto the import list and
        // return `true`, unless the pair is already present (then `false`).
        panic!(
            "TODO(module-081): implement WasmModule::add_import (module = {module}, name = {name})"
        );
    }

    /// Registers an export: an external `export_name` pointing at an internal
    /// function name. Fails if `export_name` is already taken.
    pub fn add_export(&mut self, export_name: &str, function: &str) -> Result<(), ExportError> {
        // TODO(module-081): insert the mapping. If `export_name` already
        // exists, return `ExportError::AlreadyExported` instead.
        panic!("TODO(module-081): implement WasmModule::add_export (export_name = {export_name}, function = {function})");
    }

    /// Resolves an export name to the internal function it points at.
    pub fn export(&self, export_name: &str) -> Result<&str, ExportError> {
        // TODO(module-081): look up `export_name` in the export table and
        // return the function name it maps to. Missing names produce
        // `ExportError::NotFound`.
        panic!("TODO(module-081): implement WasmModule::export (export_name = {export_name})");
    }
}

#[cfg(test)]
mod tests {
    // Unit tests live in `tests/` for this module; nothing needed here.
}
