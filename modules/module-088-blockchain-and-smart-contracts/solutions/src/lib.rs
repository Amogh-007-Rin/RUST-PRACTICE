//! Module 088: Blockchain & Smart Contracts in Rust — reference solution.

use sha2::{Digest, Sha256};

/// A single block in the chain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block {
    pub index: u64,
    pub timestamp: u64,
    pub data: Vec<u8>,
    pub previous_hash: String,
    pub hash: String,
    pub nonce: u64,
}

/// Computes a SHA-256 hex string for the block's header fields.
pub fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.index.to_le_bytes());
    hasher.update(block.timestamp.to_le_bytes());
    hasher.update(&block.data);
    hasher.update(block.previous_hash.as_bytes());
    hasher.update(block.nonce.to_le_bytes());
    format!("{:x}", hasher.finalize())
}

/// Mines a block by finding a nonce that produces a hash with at least
/// `difficulty` leading zero hex characters.
pub fn mine_block(block: &mut Block, difficulty: usize) {
    let prefix = "0".repeat(difficulty);
    block.nonce = 0;
    loop {
        block.hash = calculate_hash(block);
        if block.hash.starts_with(&prefix) {
            break;
        }
        block.nonce += 1;
    }
}

/// Validates an entire chain of blocks.
pub fn validate_chain(chain: &[Block]) -> bool {
    for i in 0..chain.len() {
        let block = &chain[i];
        if block.hash != calculate_hash(block) {
            return false;
        }
        if i > 0 {
            let prev = &chain[i - 1];
            if block.previous_hash != prev.hash {
                return false;
            }
        }
    }
    true
}

/// Creates the genesis block.
pub fn create_genesis_block() -> Block {
    let mut genesis = Block {
        index: 0,
        timestamp: 0,
        data: b"genesis".to_vec(),
        previous_hash: "0".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    genesis.hash = calculate_hash(&genesis);
    genesis
}
