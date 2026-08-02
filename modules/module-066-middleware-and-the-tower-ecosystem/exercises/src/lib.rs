//! Module 066: Middleware & the Tower Ecosystem — exercise scaffold.
//!
//! Implement the custom Tower layers and wire them into the router.
//! Find the `// TODO(module-066)` comments below and fill them in until
//! `cargo test -p module-066-exercises` passes.

use axum::{
    body::Body,
    http::{Request, Response},
    response::IntoResponse,
    Router,
};
use std::{
    collections::HashMap,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Instant,
};
use tower::{Layer, Service};

/// A token bucket for rate limiting.
#[derive(Debug, Clone)]
pub struct TokenBucket {
    pub tokens: f64,
    pub max_tokens: f64,
    pub refill_rate: f64,
    pub last_refill: Instant,
}

impl TokenBucket {
    pub fn new(max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            tokens: max_tokens,
            max_tokens,
            refill_rate,
            last_refill: Instant::now(),
        }
    }

    /// Refill tokens based on elapsed time and try to consume one.
    /// Returns `true` if a token was consumed, `false` if the bucket is empty.
    pub fn try_consume(&mut self) -> bool {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.tokens = (self.tokens + elapsed * self.refill_rate).min(self.max_tokens);
        self.last_refill = now;

        if self.tokens >= 1.0 {
            self.tokens -= 1.0;
            true
        } else {
            false
        }
    }
}

/// Shared state for the rate limiter: a map from client ID to token bucket.
pub type RateLimitStore = Arc<Mutex<HashMap<String, TokenBucket>>>;

/// Creates a new empty rate limit store.
pub fn new_rate_limit_store() -> RateLimitStore {
    Arc::new(Mutex::new(HashMap::new()))
}

/// A layer that adds an `x-request-id` header to every response.
#[derive(Clone, Debug)]
pub struct RequestIdLayer {
    pub request_id: String,
}

impl RequestIdLayer {
    pub fn new(request_id: impl Into<String>) -> Self {
        Self {
            request_id: request_id.into(),
        }
    }
}

impl<S> Layer<S> for RequestIdLayer {
    type Service = RequestIdService<S>;

    fn layer(&self, _inner: S) -> Self::Service {
        // TODO(module-066): Return a `RequestIdService` that wraps `inner`
        // and holds the configured request ID value.
        panic!("not implemented: RequestIdLayer::layer")
    }
}

/// The service produced by `RequestIdLayer`. It wraps an inner service and
/// adds an `x-request-id` header to the response.
#[derive(Clone, Debug)]
pub struct RequestIdService<S> {
    pub inner: S,
    pub request_id: String,
}

impl<S> Service<Request<Body>> for RequestIdService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        // TODO(module-066): Call the inner service, then add an
        // `x-request-id` header with the configured value to the response.
        // You'll need to box a future that awaits the inner future and
        // modifies the response.
        panic!("not implemented: RequestIdService::call")
    }
}

/// A layer that rate-limits requests using a token bucket per client.
#[derive(Clone, Debug)]
pub struct RateLimitLayer {
    pub store: RateLimitStore,
    pub max_tokens: f64,
    pub refill_rate: f64,
}

impl RateLimitLayer {
    pub fn new(store: RateLimitStore, max_tokens: f64, refill_rate: f64) -> Self {
        Self {
            store,
            max_tokens,
            refill_rate,
        }
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, _inner: S) -> Self::Service {
        // TODO(module-066): Return a `RateLimitService` that wraps `inner`
        // and holds the shared store + bucket configuration.
        panic!("not implemented: RateLimitLayer::layer")
    }
}

/// The service produced by `RateLimitLayer`. It checks the token bucket
/// for the requesting client and either forwards the request or returns 429.
#[derive(Clone, Debug)]
pub struct RateLimitService<S> {
    pub inner: S,
    pub store: RateLimitStore,
    pub max_tokens: f64,
    pub refill_rate: f64,
}

impl<S> Service<Request<Body>> for RateLimitService<S>
where
    S: Service<Request<Body>, Response = Response<Body>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + 'static,
{
    type Response = Response<Body>;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, _req: Request<Body>) -> Self::Future {
        // TODO(module-066): Extract the client ID from the `x-client-id`
        // header (default to "default" if absent). Look up or create a
        // `TokenBucket` for that client in the shared store. Call
        // `try_consume()` on it. If it returns `false`, return a 429
        // response immediately. Otherwise, forward the request to the
        // inner service.
        panic!("not implemented: RateLimitService::call")
    }
}

/// A simple handler that returns a greeting.
#[allow(dead_code)]
async fn hello() -> &'static str {
    "Hello from the middleware stack!"
}

/// A handler that returns the current rate limit store state as text.
#[allow(dead_code)]
async fn rate_limit_status(
    axum::extract::State(store): axum::extract::State<RateLimitStore>,
) -> impl IntoResponse {
    let store = store.lock().unwrap();
    let mut entries = Vec::new();
    for (client_id, bucket) in store.iter() {
        entries.push(format!(
            "{client_id}: {}/{}",
            bucket.tokens, bucket.max_tokens
        ));
    }
    entries.sort();
    entries.join("; ")
}

/// Builds the router with all middleware layers applied.
pub fn build_router(_store: RateLimitStore) -> Router {
    // TODO(module-066): Build a router with:
    // - GET /hello -> hello handler
    // - GET /status -> rate_limit_status handler (with shared state)
    // - CorsLayer::permissive() from tower-http
    // - RequestIdLayer with request ID "req-001"
    // - RateLimitLayer with the shared store, max_tokens=2.0, refill_rate=1.0
    //
    // Remember: layers apply in reverse order of .layer() calls. The last
    // .layer() is the outermost (runs first on request, last on response).
    panic!("not implemented: build_router")
}
