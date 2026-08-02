use module_095_solutions::rate_limiter::{SlidingWindow, TokenBucket};
use module_095_solutions::url_shortener::{
    decode_id, encode_id, ShortenError, Shortener, Storage, StorageError, DEFAULT_ALPHABET,
};

// ---------------------------------------------------------------------------
// Token bucket
// ---------------------------------------------------------------------------

#[test]
fn token_bucket_allows_burst_then_blocks() {
    let mut bucket = TokenBucket::new(2, 1000);
    assert!(bucket.try_consume(0));
    assert!(bucket.try_consume(0));
    assert!(!bucket.try_consume(0), "burst of 2 exhausted");
    assert!(!bucket.try_consume(500), "no refill yet");
}

#[test]
fn token_bucket_refills_over_time() {
    let mut bucket = TokenBucket::new(2, 1000);
    assert!(bucket.try_consume(0));
    assert!(bucket.try_consume(0));
    assert!(!bucket.try_consume(0));
    assert!(bucket.try_consume(1000), "one token refilled after 1000ms");
    assert!(!bucket.try_consume(1000), "only one token refilled");
    assert!(bucket.try_consume(2000));
}

#[test]
fn token_bucket_never_exceeds_capacity() {
    let mut bucket = TokenBucket::new(3, 100);
    assert_eq!(bucket.available(0), 3.0);
    assert_eq!(bucket.available(10_000), 3.0);
    for _ in 0..3 {
        assert!(bucket.try_consume(10_000));
    }
    assert_eq!(bucket.available(10_000), 0.0);
    assert_eq!(bucket.available(10_250), 2.0);
    assert_eq!(bucket.available(10_250), 2.0, "read-only, no mutation");
    assert_eq!(bucket.available(20_000), 3.0);
}

#[test]
fn token_bucket_zero_interval_means_always_full() {
    let mut bucket = TokenBucket::new(1, 0);
    assert!(bucket.try_consume(0));
    assert!(bucket.try_consume(1), "interval 0 refills instantly");
    assert!(bucket.try_consume(2));
}

#[test]
fn token_bucket_capacity_one_is_a_lock() {
    let mut bucket = TokenBucket::new(1, 500);
    assert!(bucket.try_consume(0));
    assert!(!bucket.try_consume(100));
    assert!(!bucket.try_consume(499));
    assert!(bucket.try_consume(500));
}

// ---------------------------------------------------------------------------
// Sliding window
// ---------------------------------------------------------------------------

#[test]
fn sliding_window_allows_up_to_max() {
    let mut window = SlidingWindow::new(3, 1000);
    assert!(window.allow(0));
    assert!(window.allow(10));
    assert!(window.allow(20));
    assert!(!window.allow(30), "3 requests per 1000ms exceeded");
}

#[test]
fn sliding_window_expires_old_requests() {
    let mut window = SlidingWindow::new(3, 1000);
    window.allow(0);
    window.allow(10);
    window.allow(20);
    assert!(!window.allow(30));
    assert!(window.allow(1000), "t=0 request expired");
    assert!(!window.allow(1001), "t=10 request still inside");
    assert!(window.allow(1010));
}

#[test]
fn sliding_window_active_requests_counts_only_inside() {
    let mut window = SlidingWindow::new(5, 1000);
    assert_eq!(window.active_requests(0), 0);
    window.allow(0);
    window.allow(100);
    assert_eq!(window.active_requests(500), 2);
    assert_eq!(
        window.active_requests(1000),
        1,
        "t=0 expired at exactly 1000"
    );
    assert_eq!(window.active_requests(1100), 0);
}

#[test]
fn sliding_window_long_gap_resets() {
    let mut window = SlidingWindow::new(2, 1000);
    window.allow(0);
    window.allow(0);
    assert!(!window.allow(0));
    assert!(window.allow(5000), "whole window long gone");
    assert!(window.allow(5000));
    assert!(!window.allow(5000));
}

// ---------------------------------------------------------------------------
// encode / decode
// ---------------------------------------------------------------------------

#[test]
fn encode_decode_round_trips() {
    for id in 0..1000 {
        let code = encode_id(id, DEFAULT_ALPHABET);
        assert_eq!(decode_id(&code, DEFAULT_ALPHABET), Some(id), "id {id}");
    }
}

#[test]
fn encode_zero_is_first_character() {
    assert_eq!(encode_id(0, DEFAULT_ALPHABET), "0");
    assert_eq!(encode_id(0, "ab"), "a");
}

#[test]
fn encode_is_bijective_for_small_alphabet() {
    let codes: Vec<String> = (0..8).map(|id| encode_id(id, "ab")).collect();
    let mut unique: Vec<&str> = codes.iter().map(String::as_str).collect();
    unique.sort();
    unique.dedup();
    assert_eq!(unique.len(), 8, "every id must get a unique code");
    for (id, code) in codes.iter().enumerate() {
        assert_eq!(decode_id(code, "ab"), Some(id as u64));
    }
}

#[test]
fn decode_rejects_foreign_characters() {
    assert_eq!(decode_id("a-b", DEFAULT_ALPHABET), None);
    assert_eq!(decode_id("!!", DEFAULT_ALPHABET), None);
    assert_eq!(decode_id("", DEFAULT_ALPHABET), None);
    assert_eq!(decode_id("0", ""), None, "empty alphabet decodes nothing");
}

#[test]
fn encode_id_with_empty_alphabet_is_empty() {
    assert_eq!(encode_id(7, ""), "");
}

// ---------------------------------------------------------------------------
// Storage trait
// ---------------------------------------------------------------------------

#[test]
fn hash_map_storage_round_trip() {
    let mut storage = module_095_solutions::url_shortener::HashMapStorage::new();
    assert_eq!(storage.get("abc"), None);
    assert!(storage
        .insert("abc".into(), "https://example.com".into())
        .is_ok());
    assert_eq!(storage.get("abc"), Some("https://example.com"));
    assert!(storage.contains("abc"));
    assert_eq!(
        storage.insert("abc".into(), "https://other.com".into()),
        Err(StorageError::CodeTaken),
        "must never silently overwrite"
    );
    assert_eq!(storage.get("abc"), Some("https://example.com"));
}

#[test]
fn storage_trait_works_with_custom_backend() {
    struct VecStorage {
        links: Vec<(String, String)>,
    }

    impl Storage for VecStorage {
        fn get(&self, code: &str) -> Option<&str> {
            self.links
                .iter()
                .find(|(c, _)| c == code)
                .map(|(_, url)| url.as_str())
        }

        fn insert(&mut self, code: String, url: String) -> Result<(), StorageError> {
            if self.links.iter().any(|(c, _)| *c == code) {
                return Err(StorageError::CodeTaken);
            }
            self.links.push((code, url));
            Ok(())
        }
    }

    let storage = VecStorage { links: Vec::new() };
    let mut shortener = Shortener::new(storage);
    let code = shortener.create("https://vec-storage.dev", None).unwrap();
    assert_eq!(shortener.resolve(&code), Some("https://vec-storage.dev"));
}

// ---------------------------------------------------------------------------
// Shortener
// ---------------------------------------------------------------------------

#[test]
fn shortener_generates_working_codes() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    let code = shortener.create("https://example.com/page", None).unwrap();
    assert_eq!(shortener.resolve(&code), Some("https://example.com/page"));
    assert_eq!(code.len(), 4, "default min code length is 4");
}

#[test]
fn shortener_codes_are_unique() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    let mut codes = Vec::new();
    for i in 0..50 {
        codes.push(
            shortener
                .create(&format!("https://example.com/{i}"), None)
                .unwrap(),
        );
    }
    let mut sorted = codes.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), 50, "every code must be distinct");
}

#[test]
fn shortener_rejects_bad_urls() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    for bad in [
        "example.com",
        "ftp://example.com",
        "",
        "https://",
        " javascript:alert(1)",
    ] {
        assert_eq!(
            shortener.create(bad, None),
            Err(ShortenError::InvalidUrl),
            "url {bad:?} must be rejected"
        );
    }
}

#[test]
fn shortener_custom_codes() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    let code = shortener
        .create("https://example.com", Some("rust"))
        .unwrap();
    assert_eq!(code, "rust");
    assert_eq!(shortener.resolve("rust"), Some("https://example.com"));
}

#[test]
fn shortener_rejects_invalid_custom_codes() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    for bad in [
        "",
        "ab",
        "with space",
        "toolong12345678901234567890123",
        "has_underscore!",
    ] {
        assert_eq!(
            shortener.create("https://example.com", Some(bad)),
            Err(ShortenError::InvalidCode),
            "code {bad:?} must be rejected"
        );
    }
}

#[test]
fn shortener_custom_code_collision_is_an_error() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    shortener
        .create("https://first.example", Some("taken"))
        .unwrap();
    assert_eq!(
        shortener.create("https://second.example", Some("taken")),
        Err(ShortenError::CodeTaken)
    );
    assert_eq!(
        shortener.resolve("taken"),
        Some("https://first.example"),
        "original link must be untouched"
    );
}

#[test]
fn shortener_handles_collisions_with_retry() {
    let mut shortener = Shortener::with_alphabet_and_min_len(
        module_095_solutions::url_shortener::HashMapStorage::new(),
        "ab",
        2,
    );
    let mut codes = Vec::new();
    for i in 0..5 {
        let code = shortener
            .create(&format!("https://example.com/{i}"), None)
            .expect("retry loop must eventually find a free code");
        codes.push(code);
    }
    let mut sorted = codes.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), 5, "all five codes must be distinct");
    for code in &codes {
        assert!(shortener.resolve(code).is_some());
    }
}

#[test]
fn shortener_resolve_unknown_is_none() {
    let mut shortener = Shortener::new(module_095_solutions::url_shortener::HashMapStorage::new());
    assert_eq!(shortener.resolve("nope"), None);
    shortener.create("https://example.com", None).unwrap();
    assert_eq!(shortener.resolve("nope"), None);
}
