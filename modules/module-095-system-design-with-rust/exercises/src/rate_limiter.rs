//! Case study A: a rate limiter, as a testable library.
//!
//! The design idea you should take to an interview: **time is a parameter,
//! not a hidden dependency.** Both algorithms take `now_ms` explicitly, so
//! tests can advance the clock by hand instead of sleeping — and production
//! code passes `SystemTime::now()` timestamps in. That one decision is what
//! makes the whole library testable without `tokio` or fake-clock crates.

use std::collections::VecDeque;

/// Token bucket: a bucket that fills with tokens at a fixed rate and drains
/// one token per request.
///
/// - `capacity`: how many tokens the bucket can hold (burst size).
/// - `refill_interval_ms`: time between one token and the next.
///
/// `try_consume` succeeds when at least one token is available, taking one.
/// This is the classic burst-tolerant limiter: allow N requests instantly,
/// then refill at a steady rate.
pub struct TokenBucket {
    #[allow(dead_code)]
    capacity: u32,
    #[allow(dead_code)]
    tokens: f64,
    #[allow(dead_code)]
    refill_interval_ms: u64,
    #[allow(dead_code)]
    last_refill_ms: u64,
}

impl TokenBucket {
    /// Creates a full bucket of `capacity` tokens that refills one token
    /// every `refill_interval_ms` milliseconds.
    pub fn new(capacity: u32, refill_interval_ms: u64) -> Self {
        Self {
            capacity,
            tokens: capacity as f64,
            refill_interval_ms,
            last_refill_ms: 0,
        }
    }

    /// Tries to take one token now. Returns `true` and consumes a token on
    /// success; returns `false` without consuming anything when the bucket
    /// is empty.
    ///
    /// `now_ms` must never go backwards (use a monotonic clock in
    /// production).
    pub fn try_consume(&mut self, now_ms: u64) -> bool {
        // TODO(module-095): first refill the bucket: add
        // `(now_ms - last_refill_ms) / refill_interval_ms` tokens, capped at
        // `capacity` (a bucket never overflows). Then, if `tokens >= 1`,
        // take one and return `true`; otherwise `false`.
        //
        // Edge case: `refill_interval_ms == 0` means "refills instantly" —
        // treat it as always full.
        let _ = now_ms;
        panic!("stub: TokenBucket::try_consume is not implemented yet");
    }

    /// How many tokens would be available at `now_ms` after a refill, never
    /// exceeding `capacity`. Read-only — useful for tests and for the
    /// `X-RateLimit-Remaining` response header.
    pub fn available(&self, now_ms: u64) -> f64 {
        // TODO(module-095): same refill arithmetic as `try_consume`, but
        // don't mutate anything.
        let _ = now_ms;
        panic!("stub: TokenBucket::available is not implemented yet");
    }
}

/// Sliding window: at most `max_requests` requests per `window_ms` of wall
/// clock. Each accepted request is a timestamp in a queue; timestamps older
/// than the window are evicted before counting.
///
/// This is the "exactly what it says on the tin" limiter: no bursts at all,
/// because the window never refills early.
pub struct SlidingWindow {
    #[allow(dead_code)]
    max_requests: u32,
    #[allow(dead_code)]
    window_ms: u64,
    #[allow(dead_code)]
    requests: VecDeque<u64>,
}

impl SlidingWindow {
    /// Creates an empty window allowing `max_requests` per `window_ms`.
    pub fn new(max_requests: u32, window_ms: u64) -> Self {
        Self {
            max_requests,
            window_ms,
            requests: VecDeque::new(),
        }
    }

    /// Returns `true` and records the request when the window is not full;
    /// returns `false` and records nothing when it is.
    pub fn allow(&mut self, now_ms: u64) -> bool {
        // TODO(module-095): evict expired timestamps from the front while
        // `*front + window_ms <= now_ms` (the window is half-open). Then
        // accept when the queue is shorter than `max_requests`, pushing
        // `now_ms` to the back — otherwise reject.
        let _ = now_ms;
        panic!("stub: SlidingWindow::allow is not implemented yet");
    }

    /// Number of requests still inside the window at `now_ms`.
    pub fn active_requests(&self, now_ms: u64) -> usize {
        // TODO(module-095): count the timestamps that are still within the
        // window. Don't mutate the queue — this is a read-only view.
        let _ = now_ms;
        panic!("stub: SlidingWindow::active_requests is not implemented yet");
    }
}
