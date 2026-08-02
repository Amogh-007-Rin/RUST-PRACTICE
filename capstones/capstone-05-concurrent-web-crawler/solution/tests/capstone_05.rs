use capstone_05_solution::{extract_domain, CrawlConfig, CrawlResult, DomainRateLimiter};
use std::sync::Arc;
use tokio_util::sync::CancellationToken;

// ---------------------------------------------------------------------------
// extract_domain tests
// ---------------------------------------------------------------------------

#[test]
fn test_extract_domain_basic() {
    assert_eq!(
        extract_domain("https://example.com"),
        Some("example.com".to_string())
    );
}

#[test]
fn test_extract_domain_with_path() {
    assert_eq!(
        extract_domain("https://example.com/some/path?q=1"),
        Some("example.com".to_string())
    );
}

#[test]
fn test_extract_domain_with_port() {
    assert_eq!(
        extract_domain("http://example.com:8080/path"),
        Some("example.com".to_string())
    );
}

#[test]
fn test_extract_domain_http() {
    assert_eq!(
        extract_domain("http://www.example.org"),
        Some("www.example.org".to_string())
    );
}

#[test]
fn test_extract_domain_no_scheme() {
    assert_eq!(extract_domain("example.com"), None);
}

// ---------------------------------------------------------------------------
// DomainRateLimiter tests
// ---------------------------------------------------------------------------

#[tokio::test]
async fn test_rate_limiter_does_not_block_first_request() {
    let limiter = DomainRateLimiter::new(10.0);
    let start = tokio::time::Instant::now();
    limiter.wait("example.com").await;
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 50);
}

#[tokio::test]
async fn test_rate_limiter_enforces_rate() {
    let limiter = DomainRateLimiter::new(10.0); // 100ms between requests
    let start = tokio::time::Instant::now();
    limiter.wait("example.com").await;
    limiter.wait("example.com").await;
    let elapsed = start.elapsed();
    // Second request should have waited at least ~100ms.
    assert!(elapsed.as_millis() >= 90);
}

#[tokio::test]
async fn test_rate_limiter_different_domains_no_blocking() {
    let limiter = DomainRateLimiter::new(1.0); // 1 req/s = 1000ms interval
    let start = tokio::time::Instant::now();
    limiter.wait("a.com").await;
    limiter.wait("b.com").await;
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 100);
}

#[tokio::test]
async fn test_rate_limiter_zero_rate_no_blocking() {
    let limiter = DomainRateLimiter::new(0.0);
    let start = tokio::time::Instant::now();
    limiter.wait("example.com").await;
    limiter.wait("example.com").await;
    limiter.wait("example.com").await;
    let elapsed = start.elapsed();
    assert!(elapsed.as_millis() < 50);
}

// ---------------------------------------------------------------------------
// Crawler tests
// ---------------------------------------------------------------------------

use capstone_05_solution::{Crawler, MockFetcher};

fn make_config(concurrency: usize) -> CrawlConfig {
    CrawlConfig {
        concurrency_limit: concurrency,
        requests_per_second: 1000.0, // effectively no rate limit
        timeout_secs: 5,
    }
}

fn ok_result(url: &str, body_size: usize) -> CrawlResult {
    CrawlResult {
        url: url.to_string(),
        status: 200,
        body_size,
        duration_ms: 0,
        error: None,
    }
}

#[tokio::test]
async fn test_crawl_all_urls_fetched() {
    let mock = Arc::new(MockFetcher::new());
    mock.add_response("http://a.com".into(), ok_result("http://a.com", 100))
        .await;
    mock.add_response("http://b.com".into(), ok_result("http://b.com", 200))
        .await;

    let config = make_config(2);
    let crawler = Crawler::new(config, mock.clone());
    let urls: Vec<String> = vec!["http://a.com".into(), "http://b.com".into()];

    let results = crawler.crawl(&urls, CancellationToken::new()).await;
    assert_eq!(results.len(), 2);

    let mut urls_found: Vec<&str> = results.iter().map(|r| r.url.as_str()).collect();
    urls_found.sort();
    assert_eq!(urls_found, vec!["http://a.com", "http://b.com"]);
    assert_eq!(mock.get_call_count("http://a.com").await, 1);
    assert_eq!(mock.get_call_count("http://b.com").await, 1);
}

#[tokio::test]
async fn test_crawl_records_duration() {
    let mock = Arc::new(MockFetcher::new().with_delay(std::time::Duration::from_millis(10)));
    mock.add_response("http://t.com".into(), ok_result("http://t.com", 42))
        .await;

    let config = make_config(1);
    let crawler = Crawler::new(config, mock);
    let urls: Vec<String> = vec!["http://t.com".into()];

    let results = crawler.crawl(&urls, CancellationToken::new()).await;
    assert_eq!(results.len(), 1);
    assert!(results[0].duration_ms > 0);
}

#[tokio::test]
async fn test_crawl_failed_request_error_field() {
    let mock = Arc::new(MockFetcher::new());
    mock.add_response(
        "http://fail.com".into(),
        CrawlResult {
            url: "http://fail.com".into(),
            status: 500,
            body_size: 0,
            duration_ms: 0,
            error: Some("internal server error".into()),
        },
    )
    .await;

    let config = make_config(1);
    let crawler = Crawler::new(config, mock);
    let urls: Vec<String> = vec!["http://fail.com".into()];

    let results = crawler.crawl(&urls, CancellationToken::new()).await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].status, 500);
    assert!(results[0].error.is_some());
}

#[tokio::test]
async fn test_crawl_concurrency_limit_respected() {
    let mock = Arc::new(MockFetcher::new().with_delay(std::time::Duration::from_millis(50)));
    for i in 0..5 {
        mock.add_response(
            format!("http://site{i}.com"),
            ok_result(&format!("http://site{i}.com"), i * 10),
        )
        .await;
    }

    let config = CrawlConfig {
        concurrency_limit: 2,
        requests_per_second: 1000.0,
        timeout_secs: 5,
    };
    let crawler = Crawler::new(config, mock.clone());
    let urls: Vec<String> = (0..5).map(|i| format!("http://site{i}.com")).collect();

    crawler.crawl(&urls, CancellationToken::new()).await;
    assert_eq!(mock.max_concurrent(), 2);
}

#[tokio::test]
async fn test_crawl_cancellation_stops_early() {
    let delay = std::time::Duration::from_millis(100);
    let mock = Arc::new(MockFetcher::new().with_delay(delay));
    for i in 0..10 {
        mock.add_response(
            format!("http://site{i}.com"),
            ok_result(&format!("http://site{i}.com"), i * 10),
        )
        .await;
    }

    let config = make_config(2);
    let crawler = Crawler::new(config, mock.clone());
    let urls: Vec<String> = (0..10).map(|i| format!("http://site{i}.com")).collect();
    let cancel = CancellationToken::new();

    let cancel_clone = cancel.clone();
    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        cancel_clone.cancel();
    });

    let results = crawler.crawl(&urls, cancel).await;
    assert!(results.len() < urls.len(), "should have partial results");
}

#[tokio::test]
async fn test_crawl_no_urls() {
    let mock = Arc::new(MockFetcher::new());
    let config = make_config(1);
    let crawler = Crawler::new(config, mock);
    let urls: Vec<String> = vec![];

    let results = crawler.crawl(&urls, CancellationToken::new()).await;
    assert!(results.is_empty());
}
