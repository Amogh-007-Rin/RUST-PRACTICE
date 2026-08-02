# Module 095: System Design with Rust

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 90–120 min
**Prerequisites:** Module 094 (Rust-Specific Interview Questions)

## Learning Objectives
- You will be able to structure a system design answer: functional requirements → API → data model → scale considerations.
- You will be able to implement a token bucket rate limiter and a sliding window rate limiter as testable libraries.
- You will be able to implement a URL shortener core with bijective encoding, a storage trait abstraction, and collision handling.
- You will be able to explain why injecting time as a parameter (rather than calling `SystemTime::now()` inside methods) makes a library testable without fake clocks.
- You will be able to discuss Rust-specific design tradeoffs in an interview: `trait` objects vs generics for storage, `Result`-based error enums, and zero-dependency core libraries.

## Why This Matters
System design interviews test your ability to make architectural decisions under uncertainty. Most candidates practice with whiteboard diagrams in Java or Python — but the *thinking* transfers. When you can also *implement* the core of a rate limiter or URL shortener in Rust, you demonstrate something rarer: the ability to go from architecture to working, testable code with explicit error handling, trait-based abstractions, and no hidden dependencies. This module gives you two interview-ready case studies with Rust-flavored implementations you can discuss concretely.

## Concept

System design interviews follow a pattern. You get an open-ended prompt ("Design a rate limiter" or "Design a URL shortener"), and you have 30–45 minutes to show you can think at scale. The structure that works:

1. **Functional requirements** — what does the system do? List the API endpoints, the data it stores, the behaviors it guarantees.
2. **Non-functional requirements** — how fast? how available? how consistent? Pick two; you can't have all three.
3. **API design** — sketch the HTTP endpoints or function signatures.
4. **Data model** — what gets stored where? SQL? NoSQL? In-memory?
5. **Scale considerations** — what breaks at 10x? 100x? What's the bottleneck?
6. **Deep dive** — pick one component and go deep: the algorithm, the data structure, the failure mode.

Most candidates stop at step 5. The ones who go to step 6 — who can *implement* the core algorithm — are the ones who get strong hires. This module gives you two step-6 implementations.

### Case Study A: Rate Limiter

A rate limiter answers one question: "Should this request be allowed through?" The two classic algorithms are:

**Token bucket** — a bucket holds up to `capacity` tokens. Tokens are added one at a time every `refill_interval_ms`. Each request consumes one token. If the bucket is empty, the request is rejected. This allows bursts (drain the whole bucket instantly) but enforces a steady rate over time.

**Sliding window** — a queue of timestamps, at most `max_requests` per `window_ms`. Each accepted request pushes `now` onto the queue. Before checking, evict timestamps older than the window. This is stricter: no bursts at all, because the window never refills early.

The critical design decision — and the one that makes these implementations interview-worthy — is **injecting time as a parameter**. Both algorithms take `now_ms: u64` explicitly. This means:

- Tests can advance the clock by hand instead of sleeping.
- Production code passes `SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis() as u64`.
- No `tokio::time::sleep`, no fake-clock crates, no `Instant` lifetimes.

```rust
pub struct TokenBucket {
    capacity: u32,
    tokens: f64,
    refill_interval_ms: u64,
    last_refill_ms: u64,
}

impl TokenBucket {
    pub fn try_consume(&mut self, now_ms: u64) -> bool {
        self.refill(now_ms);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}
```

The refill logic is the subtle part. When `now_ms` advances, compute how many complete intervals have elapsed, add that many tokens (capped at capacity), and advance `last_refill_ms` *only by the time that contributed tokens*. This preserves partial intervals:

```text
State machine for TokenBucket:

  [full bucket]
      |
      | try_consume (tokens >= 1)
      v
  [tokens -= 1]  -----> [refill: add elapsed/interval tokens, cap at capacity]
      |                          |
      | try_consume (tokens < 1) | try_consume (refill first)
      v                          v
  [reject]               [tokens updated, last_refill_ms advanced]
```

If you naively set `last_refill_ms = now_ms` after every refill, you lose partial elapsed time. Example: capacity=1, interval=500ms. Consume at t=0 (bucket empty). Check at t=100, t=499 — both reject (0 complete intervals). Check at t=500 — should accept (one full interval). But if you set `last_refill_ms = 499` at t=499, then at t=500 the elapsed is only 1ms → 0 tokens → reject. The fix: advance `last_refill_ms` by `added * interval`, not by `now_ms`.

**When to use which:** token bucket for APIs that allow bursts (a user can make 100 requests instantly, then must wait); sliding window for strict rate limits (exactly N per minute, no exceptions).

### Case Study B: URL Shortener

A URL shortener maps short codes to long URLs. The core pieces:

1. **Encoding** — convert a numeric id to a short string. The classic approach: base-62 encoding (digits + lowercase + uppercase = 62 characters). `encode_id(12345, "0123...XYZ")` → `"3d7"`. The encoding must be bijective: every id maps to exactly one code, and every code decodes to exactly one id.

2. **Storage abstraction** — define a `Storage` trait: `get(code) -> Option<url>`, `insert(code, url) -> Result<(), Error>`. The shortener is generic over `S: Storage`. Tests use `HashMapStorage`; production uses a database pool that implements the same trait. This is the dependency inversion principle: the core logic doesn't know or care how codes are stored.

3. **Collision handling** — when a user requests a custom slug (`/rust`), check if it's taken. If so, return an error — never silently overwrite. For auto-generated codes, use a counter (`next_id`) and encode it. Collisions are theoretically impossible (each id is unique), but the retry loop is belt-and-branes: if `insert` returns `CodeTaken`, bump the id and try again.

```rust
pub trait Storage {
    fn get(&self, code: &str) -> Option<&str>;
    fn insert(&mut self, code: String, url: String) -> Result<(), StorageError>;
}

pub struct Shortener<S: Storage> {
    storage: S,
    alphabet: String,
    min_code_len: usize,
    next_id: u64,
}
```

The `create` method validates the URL (must start with `http://` or `https://`), validates custom codes (length 3–24, all characters in the alphabet), and either inserts the custom code or loops encoding counter ids until one succeeds.

**Scale considerations** (for the interview): at 1M URLs/day, the counter reaches 365M in a year. Base-62 with 6-character codes gives 62^6 ≈ 56 billion codes — plenty of headroom. The storage trait lets you swap `HashMap` for `sqlx::PgPool` without changing the shortener logic. Analytics (click counts, referrers) are a separate concern — that's where Capstone 10 picks up.

### Rust-Flavored Design Choices

- **`Result`-based errors** — `ShortenError` and `StorageError` are enums with explicit variants. No `unwrap()`, no panics in the library. The caller decides how to handle errors.
- **Trait objects vs generics** — `Shortener<S: Storage>` uses generics (static dispatch). For an interview, you can discuss when you'd switch to `dyn Storage` (dynamic dispatch): when the storage backend is determined at runtime, or when you need to store heterogeneous shorteners in a collection.
- **Zero dependencies** — the core library uses only `std`. This is a feature: it's easy to audit, easy to embed, easy to test. Production would add `serde` for serialization, `tracing` for logging, but the core stays dependency-free.

## Common Pitfalls
- **Calling `SystemTime::now()` inside the algorithm** — this makes the library untestable without sleeping. Inject time as a parameter.
- **Advancing `last_refill_ms` to `now_ms` unconditionally** — this loses partial intervals. Advance by `added * interval` instead.
- **Using `HashMap::insert` and checking the return value** — `insert` overwrites before you can check. Use `contains_key` first, or the `entry` API.
- **Silently overwriting on collision** — a URL shortener that overwrites existing codes is a security vulnerability. Always return an error.
- **Forgetting to validate URLs** — `javascript:alert(1)` is a valid URL syntactically but a security nightmare. Require `http://` or `https://`.

## Key Terms
- **Token bucket:** a rate limiting algorithm that allows bursts up to a capacity, then enforces a steady refill rate.
- **Sliding window:** a rate limiting algorithm that enforces a strict limit per time window, with no bursts.
- **Bijective encoding:** a mapping where every input has exactly one output, and every output decodes to exactly one input.
- **Dependency inversion:** defining an interface (trait) that the core logic depends on, with concrete implementations (HashMap, database) plugged in later.
- **Time injection:** passing the current time as a parameter rather than reading it from the system clock, making the code testable without fake clocks.

## Exercise

Open `exercises/` and find the `// TODO(module-095)` markers. You'll implement:

1. **`rate_limiter.rs`** — `TokenBucket::try_consume`, `TokenBucket::available`, `SlidingWindow::allow`, `SlidingWindow::active_requests`.
2. **`url_shortener.rs`** — `encode_id`, `decode_id`, `HashMapStorage::get`, `HashMapStorage::insert`, `Shortener::create`, `Shortener::resolve`.

The integration tests in `tests/module_095.rs` define "done." They never sleep — time is injected as `now_ms`, so the clock jumps forward arbitrarily. That's the testability payoff of the design.

When you're done:

```bash
cargo test -p module-095-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading
- [The Rust Book: Traits](https://doc.rust-lang.org/book/ch10-02-traits.html) — the `Storage` trait is a real-world example of trait-based abstraction.
- [std::collections::HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) — the `entry` API for collision-safe inserts.
- [Rate Limiting, an approach (blog)](https://blog.bearer.sh/rate-limiting-api-design/) — a practical overview of token bucket vs sliding window.
- [System Design Primer: Rate Limiting](https://github.com/donnemartin/system-design-primer#rate-limiter) — the canonical interview-prep resource, with diagrams.
