//! Case study A: a rate limiter, as a testable library.
//!
//! The design idea you should take to an interview: **time is a parameter,
//! not a hidden dependency.** Both algorithms take `now_ms` explicitly, so
//! tests can advance the clock by hand instead of sleeping — and production
//! code passes `SystemTime::now()` timestamps in.

use std::collections::VecDeque;

/// Token bucket: a bucket that fills with tokens at a fixed rate and drains
/// one token per request.
pub struct TokenBucket {
    capacity: u32,
    tokens: f64,
    refill_interval_ms: u64,
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
    pub fn try_consume(&mut self, now_ms: u64) -> bool {
        self.refill(now_ms);
        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }

    /// How many tokens would be available at `now_ms` after a refill, never
    /// exceeding `capacity`. Read-only.
    pub fn available(&self, now_ms: u64) -> f64 {
        if now_ms <= self.last_refill_ms {
            return self.tokens;
        }
        if self.refill_interval_ms == 0 {
            return self.capacity as f64;
        }
        let elapsed = now_ms - self.last_refill_ms;
        let added = elapsed / self.refill_interval_ms;
        (self.tokens + added as f64).min(self.capacity as f64)
    }
    /// Refills the bucket to `now_ms`. A bucket never overflows, and a
    /// zero refill interval means "always full".
    fn refill(&mut self, now_ms: u64) {
        if now_ms <= self.last_refill_ms {
            return;
        }
        if self.refill_interval_ms == 0 {
            self.tokens = self.capacity as f64;
            self.last_refill_ms = now_ms;
            return;
        }
        let elapsed = now_ms - self.last_refill_ms;
        let added = elapsed / self.refill_interval_ms;
        if added > 0 {
            self.tokens = (self.tokens + added as f64).min(self.capacity as f64);
            self.last_refill_ms += added * self.refill_interval_ms;
        }
    }
}

/// Sliding window: at most `max_requests` requests per `window_ms` of wall
/// clock. Each accepted request is a timestamp in a queue; timestamps older
/// than the window are evicted before counting.
pub struct SlidingWindow {
    max_requests: u32,
    window_ms: u64,
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
        // Evict everything that has left the (half-open) window.
        while let Some(&front) = self.requests.front() {
            if front + self.window_ms <= now_ms {
                self.requests.pop_front();
            } else {
                break;
            }
        }
        if (self.requests.len() as u32) < self.max_requests {
            self.requests.push_back(now_ms);
            true
        } else {
            false
        }
    }

    /// Number of requests still inside the window at `now_ms`.
    pub fn active_requests(&self, now_ms: u64) -> usize {
        self.requests
            .iter()
            .filter(|&&t| t + self.window_ms > now_ms)
            .count()
    }
}
