//! Module 088: Blockchain & Smart Contracts in Rust — exercise scaffold.
//!
//! Rust is the dominant language for blockchain tooling (Solana programs,
//! Substrate pallets, and smart-contract runtimes). This module implements
//! a simplified blockchain data structure — blocks, SHA-256 hashing,
//! proof-of-work mining, and chain validation — to teach the patterns
//! that power real on-chain code.
//!
//! Fill in every `// TODO(module-088)` below.

#[allow(unused_imports)]
use sha2::{Digest, Sha256};

/// A single block in the chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    /// Position in the chain (0 = genesis).
    pub index: u64,
    /// Unix timestamp when the block was created.
    pub timestamp: u64,
    /// Arbitrary payload (transaction data, in a real chain).
    pub data: Vec<u8>,
    /// Hash of the previous block ("0" for genesis).
    pub previous_hash: String,
    /// SHA-256 hash of this block's contents (set after mining).
    pub hash: String,
    /// Proof-of-work nonce (set during mining).
    pub nonce: u64,
}

/// Computes a SHA-256 hex string for the block's header fields.
///
/// Hashed fields (fed into `Sha256` in this order):
///   - `index` (8 bytes, little-endian)
///   - `timestamp` (8 bytes, little-endian)
///   - `data` (raw bytes)
///   - `previous_hash` (as UTF-8 bytes)
///   - `nonce` (8 bytes, little-endian)
///
/// Returns a lowercase hex string (e.g. `"00a4b2..."`).
pub fn calculate_hash(block: &Block) -> String {
    // TODO(module-088): create a `Sha256` hasher, feed each field in the
    // order above, finalize, and format as lowercase hex using `format!("{:x}", ...)`.
    let _ = block;
    panic!("TODO(module-088): implement calculate_hash");
}

/// Mines a block by finding a nonce that produces a hash with at least
/// `difficulty` leading zero hex characters.
///
/// Starts at `nonce = 0` and increments until the condition is met.
/// Stores the found nonce and hash on the block.
///
/// The "prefix" to match is `"0".repeat(difficulty)`. For example,
/// `difficulty = 2` means the hash must start with `"00"`.
pub fn mine_block(block: &mut Block, difficulty: usize) {
    // TODO(module-088): set `block.nonce = 0`, then loop:
    //   compute `block.hash = calculate_hash(block)`,
    //   check if `block.hash.starts_with("0".repeat(difficulty))`,
    //   if so break, otherwise increment `block.nonce`.
    // Be careful: update `block.hash` inside the loop so each iteration
    // hashes a different nonce.
    let _ = (block, difficulty);
    panic!("TODO(module-088): implement mine_block");
}

/// Validates an entire chain of blocks.
///
/// Returns `true` only if ALL of these hold:
///   1. Every block's hash matches `calculate_hash(&block)`.
///   2. Every block's `previous_hash` matches the *actual* hash of the
///      preceding block (block[i-1].hash == block[i].previous_hash),
///      for all i > 0.
///   3. An empty chain is considered valid (returns `true`).
pub fn validate_chain(chain: &[Block]) -> bool {
    // TODO(module-088): for each block, verify its hash computes correctly.
    // For every block after the first, verify `previous_hash` links to the
    // previous block's hash.
    let _ = chain;
    panic!("TODO(module-088): implement validate_chain");
}

/// Creates the genesis block (index 0, timestamp 0, data "genesis",
/// previous_hash "0"). The hash is computed (no mining — the genesis
/// block is accepted as-is).
pub fn create_genesis_block() -> Block {
    // TODO(module-088): build the genesis block, compute its hash with
    // nonce = 0, and return it.
    panic!("TODO(module-088): implement create_genesis_block");
}
