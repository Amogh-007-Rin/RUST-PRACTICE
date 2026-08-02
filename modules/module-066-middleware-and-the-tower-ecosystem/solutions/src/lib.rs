//! Module 066: Middleware & the Tower Ecosystem — reference solution.
//!
//! Custom Tower layers (RequestIdLayer, RateLimitLayer) and tower-http
//! integration (CorsLayer) wired into an axum router.

use axum::{
    body::Body,
    http::{Request, Response, StatusCode},
    response::IntoResponse,
    routing::get,
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
use tower_http::cors::CorsLayer;

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

    fn layer(&self, inner: S) -> Self::Service {
        RequestIdService {
            inner,
            request_id: self.request_id.clone(),
        }
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

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let request_id = self.request_id.clone();
        let inner_future = self.inner.call(req);
        Box::pin(async move {
            let mut response = inner_future.await?;
            response.headers_mut().insert(
                axum::http::HeaderName::from_static("x-request-id"),
                axum::http::HeaderValue::from_str(&request_id).unwrap(),
            );
            Ok(response)
        })
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

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            store: self.store.clone(),
            max_tokens: self.max_tokens,
            refill_rate: self.refill_rate,
        }
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

    fn call(&mut self, req: Request<Body>) -> Self::Future {
        let client_id = req
            .headers()
            .get("x-client-id")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("default")
            .to_string();

        let allowed = {
            let mut store = self.store.lock().unwrap();
            let bucket = store
                .entry(client_id)
                .or_insert_with(|| TokenBucket::new(self.max_tokens, self.refill_rate));
            bucket.try_consume()
        };

        if !allowed {
            let response = StatusCode::TOO_MANY_REQUESTS.into_response();
            return Box::pin(async move { Ok(response) });
        }

        let inner_future = self.inner.call(req);
        Box::pin(inner_future)
    }
}

/// A simple handler that returns a greeting.
async fn hello() -> &'static str {
    "Hello from the middleware stack!"
}

/// A handler that returns the current rate limit store state as text.
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
pub fn build_router(store: RateLimitStore) -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/status", get(rate_limit_status))
        .with_state(store.clone())
        .layer(CorsLayer::permissive())
        .layer(RequestIdLayer::new("req-001"))
        .layer(RateLimitLayer::new(store, 2.0, 1.0))
}
