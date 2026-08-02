//! Capstone 05 solution.
//!
//! A concurrent rate-limited web crawler using Tokio: bounded concurrency
//! via `Semaphore`, per-domain throttling via `DomainRateLimiter`, and
//! graceful cancellation via `CancellationToken` + `select!`.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{Mutex, Semaphore};
use tokio_util::sync::CancellationToken;

/// Configuration for the crawler.
#[derive(Debug, Clone)]
pub struct CrawlConfig {
    pub concurrency_limit: usize,
    pub requests_per_second: f64,
    pub timeout_secs: u64,
}

/// Result of a single URL crawl.
#[derive(Debug, Clone, PartialEq)]
pub struct CrawlResult {
    pub url: String,
    pub status: u16,
    pub body_size: usize,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Abstract HTTP fetcher, enabling mocked testing.
#[async_trait]
pub trait Fetcher: Send + Sync {
    async fn fetch(&self, url: &str) -> CrawlResult;
}

/// Real HTTP fetcher backed by `reqwest`.
pub struct HttpFetcher {
    client: reqwest::Client,
}

impl HttpFetcher {
    pub fn new(timeout_secs: u64) -> Self {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(timeout_secs))
            .build()
            .unwrap();
        Self { client }
    }
}

#[async_trait]
impl Fetcher for HttpFetcher {
    async fn fetch(&self, url: &str) -> CrawlResult {
        match self.client.get(url).send().await {
            Ok(resp) => {
                let status = resp.status().as_u16();
                match resp.text().await {
                    Ok(body) => CrawlResult {
                        url: url.to_string(),
                        status,
                        body_size: body.len(),
                        duration_ms: 0,
                        error: None,
                    },
                    Err(e) => CrawlResult {
                        url: url.to_string(),
                        status,
                        body_size: 0,
                        duration_ms: 0,
                        error: Some(e.to_string()),
                    },
                }
            }
            Err(e) => CrawlResult {
                url: url.to_string(),
                status: 0,
                body_size: 0,
                duration_ms: 0,
                error: Some(e.to_string()),
            },
        }
    }
}

/// Mock fetcher that returns pre-programmed responses for testing.
pub struct MockFetcher {
    responses: Mutex<HashMap<String, CrawlResult>>,
    delay: Duration,
    call_count: Mutex<HashMap<String, usize>>,
    concurrent: std::sync::atomic::AtomicUsize,
    max_concurrent: std::sync::atomic::AtomicUsize,
}

impl Default for MockFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl MockFetcher {
    pub fn new() -> Self {
        Self {
            responses: Mutex::new(HashMap::new()),
            delay: Duration::ZERO,
            call_count: Mutex::new(HashMap::new()),
            concurrent: std::sync::atomic::AtomicUsize::new(0),
            max_concurrent: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    pub fn with_delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub async fn add_response(&self, url: String, result: CrawlResult) {
        self.responses.lock().await.insert(url, result);
    }

    pub async fn get_call_count(&self, url: &str) -> usize {
        self.call_count.lock().await.get(url).copied().unwrap_or(0)
    }

    pub fn max_concurrent(&self) -> usize {
        self.max_concurrent
            .load(std::sync::atomic::Ordering::SeqCst)
    }
}

#[async_trait]
impl Fetcher for MockFetcher {
    async fn fetch(&self, url: &str) -> CrawlResult {
        let current = self
            .concurrent
            .fetch_add(1, std::sync::atomic::Ordering::SeqCst)
            + 1;
        let prev = self
            .max_concurrent
            .load(std::sync::atomic::Ordering::SeqCst);
        if current > prev {
            self.max_concurrent
                .store(current, std::sync::atomic::Ordering::SeqCst);
        }

        if !self.delay.is_zero() {
            tokio::time::sleep(self.delay).await;
        }

        {
            let mut count = self.call_count.lock().await;
            *count.entry(url.to_string()).or_insert(0) += 1;
        }

        self.concurrent
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);

        let responses = self.responses.lock().await;
        match responses.get(url) {
            Some(result) => result.clone(),
            None => CrawlResult {
                url: url.to_string(),
                status: 404,
                body_size: 0,
                duration_ms: 0,
                error: Some("no mock response configured".to_string()),
            },
        }
    }
}

/// Extract the hostname portion of a URL.
///
/// Strips the scheme (http:// or https://), port, and path; returns `None`
/// when the URL has no recognisable scheme.
///
/// # Examples
///
/// ```
/// # use capstone_05_solution::extract_domain;
/// assert_eq!(extract_domain("https://example.com/path"), Some("example.com".into()));
/// ```
pub fn extract_domain(url: &str) -> Option<String> {
    let without_scheme = url
        .strip_prefix("http://")
        .or_else(|| url.strip_prefix("https://"))?;
    let host = without_scheme.split('/').next()?;
    let host = host.split(':').next()?;
    if host.is_empty() {
        None
    } else {
        Some(host.to_string())
    }
}

/// Per-domain rate limiter.
///
/// Tracks the last request time for each domain and delays subsequent
/// requests to the same domain to maintain a maximum `requests_per_second`
/// rate.
pub struct DomainRateLimiter {
    min_interval: Duration,
    last: Mutex<HashMap<String, Instant>>,
}

impl DomainRateLimiter {
    pub fn new(requests_per_second: f64) -> Self {
        let min_interval = if requests_per_second > 0.0 {
            Duration::from_secs_f64(1.0 / requests_per_second)
        } else {
            Duration::ZERO
        };
        Self {
            min_interval,
            last: Mutex::new(HashMap::new()),
        }
    }

    /// Wait until a request to `domain` can be issued without exceeding the
    /// configured rate limit.
    pub async fn wait(&self, domain: &str) {
        if self.min_interval.is_zero() {
            return;
        }
        loop {
            let mut last = self.last.lock().await;
            let now = Instant::now();
            match last.get(domain) {
                Some(&prev) if now - prev < self.min_interval => {
                    let sleep_time = self.min_interval - (now - prev);
                    drop(last);
                    tokio::time::sleep(sleep_time).await;
                    continue;
                }
                _ => {
                    last.insert(domain.to_string(), now);
                    break;
                }
            }
        }
    }
}

/// The concurrent rate-limited web crawler.
pub struct Crawler {
    config: CrawlConfig,
    fetcher: Arc<dyn Fetcher>,
    rate_limiter: Arc<DomainRateLimiter>,
}

impl Crawler {
    pub fn new(config: CrawlConfig, fetcher: Arc<dyn Fetcher>) -> Self {
        let rate_limiter = Arc::new(DomainRateLimiter::new(config.requests_per_second));
        Self {
            config,
            fetcher,
            rate_limiter,
        }
    }

    /// Crawl a list of URLs concurrently, respecting `concurrency_limit`
    /// and per-domain rate limiting. When `cancel` is triggered, in-flight
    /// fetches are abandoned and partial results are returned.
    pub async fn crawl(&self, urls: &[String], cancel: CancellationToken) -> Vec<CrawlResult> {
        let semaphore = Arc::new(Semaphore::new(self.config.concurrency_limit));
        let results: Arc<Mutex<Vec<CrawlResult>>> = Arc::new(Mutex::new(Vec::new()));
        let mut handles = Vec::with_capacity(urls.len());

        for url in urls.iter() {
            let _permit = Arc::clone(&semaphore).acquire_owned().await.unwrap();
            let fetcher = Arc::clone(&self.fetcher);
            let rate_limiter = Arc::clone(&self.rate_limiter);
            let results = Arc::clone(&results);
            let cancel = cancel.clone();
            let url = url.clone();

            let handle = tokio::spawn(async move {
                tokio::select! {
                    _ = cancel.cancelled() => {}
                    _ = async {
                        if let Some(domain) = extract_domain(&url) {
                            rate_limiter.wait(&domain).await;
                        }
                        let start = Instant::now();
                        let result = fetcher.fetch(&url).await;
                        let duration_ms = start.elapsed().as_millis() as u64;
                        let mut r = result;
                        r.duration_ms = duration_ms;
                        results.lock().await.push(r);
                    } => {}
                }
                drop(_permit);
            });
            handles.push(handle);
        }

        for handle in handles {
            let _ = handle.await;
        }

        Arc::try_unwrap(results).unwrap().into_inner()
    }
}
