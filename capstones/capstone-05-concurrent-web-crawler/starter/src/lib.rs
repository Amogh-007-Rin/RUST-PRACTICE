//! Capstone 05 starter scaffold.
//!
//! A concurrent rate-limited web crawler using Tokio.  Fill in the
//! `// TODO(capstone-05)` comments so the integration tests in `tests/`
//! pass.

use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
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
/// # use capstone_05_starter::extract_domain;
/// assert_eq!(extract_domain("https://example.com/path"), Some("example.com".into()));
/// ```
pub fn extract_domain(_url: &str) -> Option<String> {
    // TODO(capstone-05): implement extract_domain
    todo!("implement extract_domain")
}

/// Per-domain rate limiter.
///
/// Tracks the last request time for each domain and delays subsequent
/// requests to the same domain to maintain a maximum `requests_per_second`
/// rate.
pub struct DomainRateLimiter {
    // TODO(capstone-05): track the last request time per domain
}

impl DomainRateLimiter {
    pub fn new(_requests_per_second: f64) -> Self {
        // TODO(capstone-05): implement DomainRateLimiter::new
        todo!("implement DomainRateLimiter::new")
    }

    /// Wait until a request to `domain` can be issued without exceeding the
    /// configured rate limit.
    pub async fn wait(&self, _domain: &str) {
        // TODO(capstone-05): implement DomainRateLimiter::wait
        todo!("implement DomainRateLimiter::wait")
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
    pub async fn crawl(&self, _urls: &[String], _cancel: CancellationToken) -> Vec<CrawlResult> {
        // TODO(capstone-05): implement crawl
        let _ = (&self.config, &self.fetcher, &self.rate_limiter);
        todo!("implement crawl")
    }
}
