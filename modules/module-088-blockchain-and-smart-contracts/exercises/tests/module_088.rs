use module_088_exercises::{
    calculate_hash, create_genesis_block, mine_block, validate_chain, Block,
};

// --- Genesis block ---------------------------------------------------------

#[test]
fn genesis_block_has_expected_fields() {
    let genesis = create_genesis_block();
    assert_eq!(genesis.index, 0);
    assert_eq!(genesis.timestamp, 0);
    assert_eq!(genesis.data, b"genesis");
    assert_eq!(genesis.previous_hash, "0");
    assert_eq!(genesis.nonce, 0);
    assert!(!genesis.hash.is_empty(), "genesis hash must be computed");
}

#[test]
fn genesis_hash_is_deterministic() {
    let g1 = create_genesis_block();
    let g2 = create_genesis_block();
    assert_eq!(g1.hash, g2.hash, "same data -> same hash");
}

// --- Hash calculation ------------------------------------------------------

#[test]
fn same_data_produces_same_hash() {
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx".to_vec(),
        previous_hash: "abc".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    b1.hash = calculate_hash(&b1);

    let b2 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx".to_vec(),
        previous_hash: "abc".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    assert_eq!(calculate_hash(&b2), b1.hash);
}

#[test]
fn different_data_different_hash() {
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"foo".to_vec(),
        previous_hash: "abc".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    b1.hash = calculate_hash(&b1);

    let b2 = Block {
        index: 1,
        timestamp: 100,
        data: b"bar".to_vec(),
        previous_hash: "abc".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    assert_ne!(calculate_hash(&b2), b1.hash);
}

#[test]
fn nonce_changes_hash() {
    let block = Block {
        index: 1,
        timestamp: 100,
        data: b"data".to_vec(),
        previous_hash: "abc".to_string(),
        hash: String::new(),
        nonce: 0,
    };
    let h0 = calculate_hash(&block);
    let block_n1 = Block {
        nonce: 1,
        ..block.clone()
    };
    assert_ne!(calculate_hash(&block_n1), h0);
}

// --- Mining ----------------------------------------------------------------

#[test]
fn mine_block_finds_nonce_for_difficulty_1() {
    let genesis = create_genesis_block();
    let mut block = Block {
        index: 1,
        timestamp: 200,
        data: b"hello".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut block, 1);
    assert!(block.hash.starts_with("0"));
    assert_eq!(block.hash, calculate_hash(&block));
}

#[test]
fn mine_block_finds_nonce_for_difficulty_2() {
    let genesis = create_genesis_block();
    let mut block = Block {
        index: 1,
        timestamp: 200,
        data: b"hello".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut block, 2);
    assert!(block.hash.starts_with("00"));
    assert_eq!(block.hash, calculate_hash(&block));
}

#[test]
fn mine_block_sets_nonce() {
    let genesis = create_genesis_block();
    let mut block = Block {
        index: 1,
        timestamp: 200,
        data: b"hello".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut block, 1);
    assert!(block.nonce > 0);
}

// --- Chain validation ------------------------------------------------------

#[test]
fn valid_chain_passes_validation() {
    let genesis = create_genesis_block();
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx1".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b1, 1);

    let mut b2 = Block {
        index: 2,
        timestamp: 200,
        data: b"tx2".to_vec(),
        previous_hash: b1.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b2, 1);

    assert!(validate_chain(&[genesis, b1, b2]));
}

#[test]
fn empty_chain_is_valid() {
    assert!(validate_chain(&[]));
}

#[test]
fn single_block_chain_is_valid() {
    let genesis = create_genesis_block();
    assert!(validate_chain(&[genesis]));
}

#[test]
fn tampered_hash_invalidates_chain() {
    let genesis = create_genesis_block();
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx1".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b1, 1);

    // Tamper: change data but keep the old hash
    let mut b1_tampered = b1.clone();
    b1_tampered.data = b"stolen".to_vec();
    // hash is now wrong

    assert!(!validate_chain(&[genesis.clone(), b1_tampered]));
}

#[test]
fn broken_link_invalidates_chain() {
    let genesis = create_genesis_block();
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx1".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b1, 1);

    let mut b2 = Block {
        index: 2,
        timestamp: 200,
        data: b"tx2".to_vec(),
        previous_hash: b1.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b2, 1);

    // Break the link: point b2 to a wrong previous_hash
    let mut b2_broken = b2.clone();
    b2_broken.previous_hash = "deadbeef".to_string();
    b2_broken.hash = String::new();
    // We didn't remine, so hash doesn't match either way — validate_chain
    // checks both hash correctness and linking. Either check catches it.

    assert!(!validate_chain(&[genesis, b1, b2_broken]));
}

#[test]
fn tampered_previous_hash_invalidates_chain() {
    let genesis = create_genesis_block();
    let mut b1 = Block {
        index: 1,
        timestamp: 100,
        data: b"tx".to_vec(),
        previous_hash: genesis.hash.clone(),
        hash: String::new(),
        nonce: 0,
    };
    mine_block(&mut b1, 1);

    let mut b1_bad = b1.clone();
    b1_bad.previous_hash = "wrong".to_string();
    // previous_hash changed but hash was not recomputed — the hash check
    // catches it since hash was computed with the old previous_hash.
    assert!(!validate_chain(&[genesis, b1_bad]));
}
