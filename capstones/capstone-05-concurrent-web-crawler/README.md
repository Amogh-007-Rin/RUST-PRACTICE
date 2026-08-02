# Capstone 05: Concurrent Rate-Limited Web Crawler

**Covers modules:** 041–050 (Async Rust)
**Estimated time:** 4-8 hours

## Project Brief

You're building an **async web crawler** that fetches a set of URLs concurrently, respecting a configurable concurrency limit and per-domain rate limiting. The crawler orchestrates requests via Tokio tasks, channels, and `select!`-based cancellation, aggregating results into a summary vector. This is the shape of every production-grade scraping and monitoring tool: limited parallelism, domain-aware throttling, graceful shutdown.

The design separates HTTP concerns from orchestration logic through a `Fetcher` trait: a real `HttpFetcher` backed by `reqwest`, and a `MockFetcher` that returns pre-configured responses for deterministic testing. This separation is the Module 041–050 payoff — async traits, `Semaphore`, `CancellationToken`, and `select!` come together in a testable, real-world pattern.

## Requirements

1. **Domain extraction.** `extract_domain` parses a URL string and returns the hostname, stripping the scheme (`http://`/`https://`), port, and path.
2. **Per-domain rate limiter.** `DomainRateLimiter` tracks the last request time per domain and enforces a minimum interval (`1.0 / requests_per_second`) between requests to the same domain. Multiple domains are rate-limited independently.
3. **Fetcher abstraction.** A `Fetcher` trait with a single `fetch(&self, url: &str) -> CrawlResult` method. The `HttpFetcher` implementation uses `reqwest` with a configurable timeout; the `MockFetcher` returns pre-programmed responses for testing.
4. **Concurrency limit.** `Crawler::crawl` spawns up to `concurrency_limit` simultaneous fetches, using `tokio::sync::Semaphore` to bound parallelism.
5. **Cancellation.** `Crawler::crawl` accepts a `tokio_util::sync::CancellationToken`. When the token fires, in-flight fetches are abandoned via `tokio::select!`, and no new fetches start. Already-completed results are returned.
6. **Result aggregation.** Every fetch produces a `CrawlResult` (URL, status code, body size, wall-clock duration, optional error). Results are collected into a shared `Vec<CrawlResult>` behind a `tokio::sync::Mutex`.
7. **Timing.** Each result's `duration_ms` field records the actual wall-clock time spent inside `fetch` (not including rate-limit wait time).

## Stretch Goals

- **Weighted / token-bucket rate limiter.** Replace the simple fixed-interval rate limiter with a token-bucket algorithm that allows short bursts.
- **Per-domain concurrency sub-limits.** Cap simultaneous requests to the same domain, in addition to the global concurrency limit.
- **Redirect following.** Automatically follow HTTP redirects and record the final URL in the result.
- **Retry with backoff.** Retry failed requests with exponential backoff (up to a configurable max retry count).
- **Structured output.** Serialize results as JSON via `serde_json` and support a `--output` flag in a CLI wrapper.

## Acceptance Criteria

The provided tests in `starter/tests/capstone_05.rs` define "done":

- [ ] `extract_domain` correctly handles full URLs (scheme, host, port, path).
- [ ] `DomainRateLimiter` enforces the configured rate: two requests to the same domain within the interval are separated.
- [ ] `DomainRateLimiter` does not throttle requests to different domains.
- [ ] `Crawler::crawl` with a `MockFetcher` fetches every URL and returns one result per URL.
- [ ] `Crawler::crawl` respects the concurrency limit (at most N fetches in-flight simultaneously).
- [ ] `Crawler::crawl` stops early when the `CancellationToken` is fired and returns partial results.
- [ ] Failed requests (e.g. network error or non-2xx status) are recorded with `error: Some(...)`.
- [ ] Starter compiles but `todo!()` implementations cause test panics. Solution passes all tests.

```bash
cargo test -p capstone-05-starter          # compiles, tests panic on todo!()
cargo test -p capstone-05-solution         # all pass
```

## Design Notes / Hints

Which Block E modules apply where:

- **Module 041 (async/await basics):** every fetch and rate-limit wait is `async`. The `Fetcher` trait uses `#[async_trait]` (or native async traits if using nightly).
- **Module 042–044 (Tokio):** `tokio::spawn` for tasks, `tokio::sync::Semaphore` for concurrency control, `tokio::sync::Mutex` for shared results.
- **Module 045 (select!):** `tokio::select!` races the fetch against `cancelled()` — this is the cleanest pattern for cooperative cancellation.
- **Module 046–047 (streams / channels):** not required here; the semaphore + task-spawn pattern is a simpler alternative for bounded concurrency.
- **Module 048 (timeouts / intervals):** `tokio::time::sleep` inside the rate limiter, `reqwest::Client::timeout` for HTTP timeouts.
- **Module 049 (cancellation):** `tokio_util::sync::CancellationToken` — clone it into every spawned task and select on `cancelled()`.
- **Module 050 (error handling):** the `Fetcher::fetch` trait method returns `CrawlResult` directly (with an `error` field) rather than `Result`, keeping the orchestration path infallible.

Start with `extract_domain` (pure, no async), then `DomainRateLimiter` (async, makes `wait` work), then `MockFetcher` / `HttpFetcher` (the trait boundary), and finally `Crawler::crawl` (the orchestration). The tests are ordered roughly the same way.
