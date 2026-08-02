//! Module 081: Introduction to WebAssembly — reference solution.
//!
//! This crate simulates the core parts of a WebAssembly runtime *on the host*:
//! linear memory, the import/export table, and the module structure. Real WASM
//! runs in a browser (or any host that can instantiate the bytecode); here you
//! build the same mental model in pure Rust so everything is testable with
//! plain `cargo test`.

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
        let new_size = self.bytes.len() + additional;
        if let Some(max) = self.max_size {
            if new_size > max {
                return Err(MemoryError::GrowOverflow {
                    current: self.bytes.len(),
                    max,
                });
            }
        }
        let old_size = self.bytes.len();
        self.bytes.resize(new_size, 0);
        Ok(old_size)
    }

    /// Loads one byte at `offset`.
    pub fn load_u8(&self, offset: usize) -> Result<u8, MemoryError> {
        self.bytes
            .get(offset)
            .copied()
            .ok_or(MemoryError::OutOfBounds {
                offset,
                length: 1,
                size: self.bytes.len(),
            })
    }

    /// Stores one byte at `offset`.
    pub fn store_u8(&mut self, offset: usize, value: u8) -> Result<(), MemoryError> {
        if offset >= self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                offset,
                length: 1,
                size: self.bytes.len(),
            });
        }
        self.bytes[offset] = value;
        Ok(())
    }

    /// Loads a little-endian `u32` starting at `offset`.
    ///
    /// `0xDEADBEEF` at offset 0 must read back as bytes
    /// `[0xEF, 0xBE, 0xAD, 0xDE]`.
    pub fn load_u32(&self, offset: usize) -> Result<u32, MemoryError> {
        let end = offset + 4;
        let bytes = self
            .bytes
            .get(offset..end)
            .ok_or(MemoryError::OutOfBounds {
                offset,
                length: 4,
                size: self.bytes.len(),
            })?;
        Ok(u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]))
    }

    /// Stores a little-endian `u32` starting at `offset`.
    pub fn store_u32(&mut self, offset: usize, value: u32) -> Result<(), MemoryError> {
        let end = offset + 4;
        if end > self.bytes.len() {
            return Err(MemoryError::OutOfBounds {
                offset,
                length: 4,
                size: self.bytes.len(),
            });
        }
        self.bytes[offset..end].copy_from_slice(&value.to_le_bytes());
        Ok(())
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
        let pair = (module.to_string(), name.to_string());
        if self.imports.contains(&pair) {
            false
        } else {
            self.imports.push(pair);
            true
        }
    }

    /// Registers an export: an external `export_name` pointing at an internal
    /// function name. Fails if `export_name` is already taken.
    pub fn add_export(&mut self, export_name: &str, function: &str) -> Result<(), ExportError> {
        if self.exports.contains_key(export_name) {
            return Err(ExportError::AlreadyExported(export_name.to_string()));
        }
        self.exports
            .insert(export_name.to_string(), function.to_string());
        Ok(())
    }

    /// Resolves an export name to the internal function it points at.
    pub fn export(&self, export_name: &str) -> Result<&str, ExportError> {
        self.exports
            .get(export_name)
            .map(String::as_str)
            .ok_or_else(|| ExportError::NotFound(export_name.to_string()))
    }
}
