# Module 088: Blockchain & Smart Contracts in Rust

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 90–120 min
**Prerequisites:** Modules 001–080. The `sha2` crate is a new dependency but requires no prior crypto knowledge.

## Learning Objectives

- You will be able to implement a simplified blockchain data structure: blocks with hashed headers, proof-of-work mining, and chain validation.
- You will be able to explain why Rust dominates blockchain tooling: deterministic execution, no GC pauses, WASM-compilable smart contracts, and memory safety for asset-critical code.
- You will be able to use the `sha2` crate for cryptographic hashing and understand how nonces produce proof-of-work.
- You will be able to describe how Solana programs and Substrate pallets differ from this simplified model and what they add.

## Why This Matters

Rust is the dominant blockchain language by design, not by chance. Solana's on-chain programs are compiled Rust. Polkadot's Substrate framework is Rust. NEAR Protocol's SDK is Rust. Cosmos SDK's CosmWasm compiles Rust to WASM for smart contracts. The common thread: blockchains are distributed systems where a bug is not an HTTP 500 but a loss of real assets. Rust's ownership model eliminates use-after-free, data races, and buffer overflows without a garbage collector — which is critical for deterministic on-chain execution. The data structures you implement in this module — blocks, hashing, proof-of-work, and chain validation — are the primitive building blocks of every cryptocurrency and smart-contract platform.

## Concept

### The block: a linked list with cryptographic integrity

A blockchain is a linked list where each node (block) contains the hash of the previous node. Because SHA-256 is collision-resistant, changing any bit in any block changes its hash, which breaks the link to the next block, which changes *its* hash, and so on — tampering with one block invalidates the entire suffix. This is the "chain" in blockchain:

```
Block 0 (genesis)            Block 1                  Block 2
┌──────────────────┐       ┌──────────────────┐      ┌──────────────────┐
│ index: 0          │       │ index: 1          │      │ index: 2          │
│ previous_hash: "0"│       │ previous_hash: H0 │      │ previous_hash: H1 │
│ data: "genesis"   │       │ data: "pay Alice" │      │ data: "pay Bob"   │
│ nonce: 0          │       │ nonce: 42         │      │ nonce: 7          │
│ hash: H0 = SHA(...)│──▶  │ hash: H1 = SHA(...)│──▶  │ hash: H2 = SHA(...)│
└──────────────────┘       └──────────────────┘      └──────────────────┘
```

A block's hash covers five fields in a fixed order:
1. `index` (8 bytes, little-endian)
2. `timestamp` (8 bytes, little-endian)
3. `data` (raw bytes — the transactions)
4. `previous_hash` (as UTF-8 bytes — linking to the parent)
5. `nonce` (8 bytes, little-endian — the proof-of-work variable)

```rust
fn calculate_hash(block: &Block) -> String {
    let mut hasher = Sha256::new();
    hasher.update(block.index.to_le_bytes());
    hasher.update(block.timestamp.to_le_bytes());
    hasher.update(&block.data);
    hasher.update(block.previous_hash.as_bytes());
    hasher.update(block.nonce.to_le_bytes());
    format!("{:x}", hasher.finalize())
}
```

The `to_le_bytes()` convention ensures the hash is deterministic across platforms (little-endian is the standard for most CPUs; real blockchains use more formal encodings like SCALE or Borsh, but little-endian is sufficient for this exercise).

### Proof-of-work: why nonces exist

If blocks could be added freely, anyone could rewrite history. Proof-of-work makes creating blocks *expensive* — you must find a nonce such that the block's hash starts with a certain number of zero hex characters. This is a brute-force search: try nonce = 0, hash it, check the prefix, try nonce = 1, repeat until found.

```rust
fn mine_block(block: &mut Block, difficulty: usize) {
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
```

Difficulty 1 means the hash must start with `0` — on average, 1 in 16 nonces will match (one hex character = 4 bits). Difficulty 2 means `00` — 1 in 256. Difficulty 4 means `0000` — 1 in 65,536. Real Bitcoin difficulty is orders of magnitude higher (around 20 leading zeros), requiring specialized hardware and enormous energy expenditure.

A useful mental model: "mining" is the act of finding a certificate that proves you did a specific amount of computational work. Anyone can verify the result instantly (one hash), but producing it requires the full search. This asymmetry — hard to produce, easy to verify — is the core of proof-of-work.

### Chain validation: three checks

To validate a chain, verify three conditions for every block:

1. **Hash correctness:** `block.hash == calculate_hash(block)`. If someone modified `data` without recomputing `hash`, this catches it.
2. **Link integrity (for i > 0):** `block[i-1].hash == block[i].previous_hash`. If someone swapped block 2 with a fake, its `previous_hash` won't match the real block 1's hash.
3. **Empty chain:** An empty chain is trivially valid. A single-block chain (genesis only) is valid as long as the genesis hash is correct.

```rust
fn validate_chain(chain: &[Block]) -> bool {
    for i in 0..chain.len() {
        if chain[i].hash != calculate_hash(&chain[i]) {
            return false;
        }
        if i > 0 && chain[i].previous_hash != chain[i - 1].hash {
            return false;
        }
    }
    true
}
```

These three checks together guarantee that every hash is authentic and every link is consistent. In a real blockchain, you'd also verify that timestamps increase monotonically, that transactions are properly signed, and that the block follows consensus rules — but for the data-structure level, hash + link is the foundation.

### The genesis block

The first block in any chain is the "genesis" block. It has no predecessor, so `previous_hash` is set to `"0"`. It is typically hardcoded or mined once and stored permanently in the node software. In this exercise, the genesis block has index 0, timestamp 0, data `"genesis"`, and its hash is computed directly (no mining required — the genesis block is trusted).

### What runs on-chain: Solana programs vs. this model

This exercise implements the *blockchain data structure* — the thing that stores transactions and validates the chain. In a real system, the blocks contain transactions, and some transactions invoke *smart contracts* (on-chain programs). In Solana:

```rust,ignore
use solana_program::{
    account_info::AccountInfo,
    entrypoint,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);

fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> entrypoint::ProgramResult {
    // Read instruction_data to decide what to do
    // Modify account data using the accounts slice
    Ok(())
}
```

This is pure Rust compiled to BPF bytecode and deployed to the Solana cluster. The program runs inside a deterministic VM — no `std::time`, no random number generator, no filesystem I/O — operating on serialized account data. The blockchain layer (which you build here) handles consensus, block production, and transaction ordering. The smart-contract layer handles the business logic (token transfers, DeFi calculations, NFT mints).

In Substrate (Polkadot's framework), the equivalent is a *pallet*:

```rust,ignore
#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;

    #[pallet::config]
    pub trait Config: frame_system::Config { }

    #[pallet::storage]
    pub type Something<T: Config> = StorageValue<_, u32>;

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        pub fn do_something(origin: OriginFor<T>, value: u32) -> DispatchResult {
            Something::<T>::put(value);
            Ok(())
        }
    }
}
```

Pallet storage is a typed key-value database built on top of the blockchain state, and pallets define the transactions that modify it. The Rust type system guarantees at compile time that storage keys are typed and that state transitions are correct — eliminating entire classes of smart-contract bugs that plague Solidity (reentrancy, integer overflow in storage, type confusion).

### The Rust-blockchain fit

Why does Rust win in this space? Four reasons:

1. **Deterministic execution:** All nodes must produce the same result for the same transactions. Rust has no garbage collector that could pause unpredictably; every allocation and deallocation is explicit.
2. **WASM target:** CosmWasm, Substrate's `contracts` pallet, and NEAR all compile Rust to WASM for on-chain execution. Rust's WASM support is first-class — the `wasm32-unknown-unknown` target has been stable for years.
3. **Memory safety:** A smart contract handling millions of dollars cannot segfault. Rust guarantees no use-after-free, no buffer overflows, and no data races — in safe code, at compile time.
4. **Ecosystem tooling:** The cryptographic libraries (`sha2`, `ed25519-dalek`, `curve25519-dalek`), serialization frameworks (SCALE via `parity-scale-codec`, Borsh, `serde`), and database backends are all mature Rust crates.

## Common Pitfalls

- **Forgetting to update `block.hash` inside the mining loop.** If you compute the hash in a separate variable and forget to write it back to `block.hash`, the next iteration hashes the same nonce again — infinite loop.
- **Using `block.nonce += 1` as a plain addition in a real embedded or no-std context without `wrapping_add`.** For this exercise, `u64` won't overflow at difficulty 1-4, but on real chains with high difficulty, nonces can wrap.
- **Hashing fields in the wrong order.** `calculate_hash` must always feed fields in the same order: index, timestamp, data, previous_hash, nonce. Swapping two fields produces a different hash even for the same values.
- **Validating the chain but forgetting to check `previous_hash` for every block after the first.** Checking only the hash of each block independently means someone could replace block 2 with a different block as long as they remine it — the link check is what prevents that.
- **Using `"0"` as the genesis `previous_hash` but then failing the link check.** The link check should skip index 0 (`if i > 0`) to avoid comparing against a meaningless genesis predecessor.

## Key Terms

- **Block:** a container holding an index, timestamp, data, previous block hash, own hash, and proof-of-work nonce.
- **Hash:** the output of a cryptographic hash function. SHA-256 in this module. Deterministic, collision-resistant, 256 bits.
- **Nonce:** a number used once — the variable component of a block that miners adjust to find a valid proof-of-work.
- **Proof-of-work:** the requirement that a block's hash starts with `difficulty` leading zeros, proving computational effort was expended.
- **Difficulty:** the number of leading zero hex characters required in the hash. Higher difficulty = more work.
- **Genesis block:** the first block in a chain (index 0), with no predecessor. Trusted by convention.
- **Chain validation:** the process of verifying that every block's hash is correct and every link to the predecessor is consistent.
- **Smart contract:** on-chain code (Solana program, Substrate pallet, CosmWasm contract) that executes when a transaction invokes it, modifying blockchain state.

## Exercise

In `exercises/src/lib.rs` you implement a simplified blockchain. The scaffold provides the `Block` struct. Fill in the `// TODO(module-088)` stubs:

1. **`calculate_hash`** — feed all five fields into a `Sha256` hasher, finalize, format as lowercase hex.
2. **`mine_block`** — loop nonce from 0 upward, recompute `block.hash` each iteration, break when the prefix matches.
3. **`validate_chain`** — for each block, verify hash correctness; for each non-first block, verify `previous_hash` links to the previous block's hash.
4. **`create_genesis_block`** — build block 0 with standard fields and compute its hash.

The integration tests in `tests/module_088.rs` cover deterministic hashing, mining at difficulty 1 and 2, valid chain validation, tampered hash detection, broken link detection, tampered previous_hash detection, and edge cases (empty chain, single-block chain).

## Further Reading

- [The `sha2` crate documentation](https://docs.rs/sha2/latest/sha2/) — the hashing library used in this module.
- [Bitcoin whitepaper (Section 4: Proof-of-Work)](https://bitcoin.org/bitcoin.pdf) — the original description of the proof-of-work mechanism.
- [Solana Documentation — "Developing with Rust"](https://solana.com/developers/guides/getstarted/rust-to-solana) — a walkthrough of writing and deploying an on-chain Solana program in Rust.
- [Substrate Developer Hub — "Build a Blockchain"](https://docs.substrate.io/tutorials/build-a-blockchain/) — the full Substrate tutorial for building a custom blockchain with Rust pallets.
