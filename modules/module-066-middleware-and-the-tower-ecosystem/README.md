# Module 066: Middleware & the Tower Ecosystem

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 062 (Axum fundamentals — you need to know what a `Router` is and how `oneshot` works), Module 065 (authentication — middleware is how auth gets wired in).

## Learning Objectives

- Explain what a Tower `Service` and `Layer` are, and how they compose.
- Build a custom `Layer` that wraps any `Service` to add cross-cutting behavior (a header, timing, rate limiting).
- Use `tower-http`'s `TraceLayer` and `CorsLayer` to add logging and CORS to an axum router.
- Implement a simple in-memory token-bucket rate limiter as a Tower layer.
- Test layer behavior through `oneshot` — no sockets, no running server.

## Why This Matters

Every production axum service you'll encounter uses Tower middleware. CORS, tracing, auth guards, rate limiting, request-id propagation — these are all layers. The Tower abstraction is what makes axum's middleware ecosystem so rich: a layer written for Tower works with axum, with hyper, with tonic (gRPC), and with any other Tower-based framework. Understanding `Service` and `Layer` is understanding the plumbing beneath every Rust web service.

## Concept

### The Tower abstraction: `Service` and `Layer`

Tower defines two traits that together describe a request-processing pipeline:

```
Request ──► [Service] ──► Response
                │
            Layer wraps a Service to produce a new Service
```

A `Service` is anything that takes a `Request` and returns a `Future<Response>`. That's it. An axum `Router` is a `Service`. A handler is a `Service`. A middleware wrapping a handler is a `Service`. The power is in the uniformity: anything that speaks `Service` can be composed with anything else.

A `Layer` is a factory for wrapping services. It takes a `Service` and returns a new `Service` with additional behavior:

```rust,ignore
trait Layer<S> {
    type Service;
    fn layer(&self, inner: S) -> Self::Service;
}
```

You don't usually implement `Layer` and `Service` by hand for production code — `tower-http` provides dozens of ready-made layers. But implementing one yourself is the fastest way to understand the model.

### What a layer actually does

Think of a layer as an onion ring around your handler. The request passes through each ring on the way in; the response passes back through on the way out:

```
Request ──► [CorsLayer] ──► [TraceLayer] ──► [RateLimitLayer] ──► Handler
Response ◄── [CorsLayer] ◄── [TraceLayer] ◄── [RateLimitLayer] ◄──┘
```

Each layer can:
- Inspect/modify the request before forwarding (add a header, reject early)
- Inspect/modify the response after the inner service returns (add timing headers)
- Decide not to call the inner service at all (rate limiting, auth rejection)

### Building a custom layer: adding a header

The simplest useful layer adds a response header. Here's the shape:

```rust,ignore
use tower::{Layer, Service};
use std::task::{Context, Poll};

struct AddHeaderLayer {
    header_value: String,
}

impl<S> Layer<S> for AddHeaderLayer {
    type Service = AddHeaderService<S>;
    fn layer(&self, inner: S) -> Self::Service {
        AddHeaderService {
            inner,
            header_value: self.header_value.clone(),
        }
    }
}

struct AddHeaderService<S> {
    inner: S,
    header_value: String,
}

impl<S, Req> Service<Req> for AddHeaderService<S>
where
    S: Service<Req, Response = axum::response::Response>,
    Req: Send + 'static,
{
    type Response = axum::response::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Req) -> Self::Future {
        // Note: for a real response-modifying layer you'd need to wrap the
        // future to modify the response after the inner service completes.
        // This simplified version shows the structure.
        self.inner.call(req)
    }
}
```

The key pattern: `Layer::layer()` wraps the inner service. `Service::call()` delegates to the inner service, optionally transforming the request or response. `poll_ready()` always delegates — it's the backpressure mechanism.

### `tower-http`: the middleware library you'll actually use

`tower-http` provides production-quality layers for the common cases:

- **`TraceLayer`** — logs every request/response with `tracing` spans (method, URI, status, duration). This is the single most useful layer in production: structured logs for every request, for free.
- **`CorsLayer`** — handles CORS preflight and response headers. For development, `CorsLayer::permissive()` allows everything; for production, you configure specific origins.
- **`CompressionLayer`** — gzip/deflate/brotli response bodies.
- **`TimeoutLayer`** — returns 408 if the handler takes too long.

Using them with axum is a one-liner each:

```rust,ignore
use axum::Router;
use tower_http::trace::TraceLayer;
use tower_http::cors::{CorsLayer, Any};

let app = Router::new()
    .route("/hello", get(|| async { "hi" }))
    .layer(TraceLayer::new_for_http())
    .layer(CorsLayer::new().allow_origin(Any));
```

`Router::layer()` applies the layer to every route registered *before* it. You can chain multiple `.layer()` calls — they stack in order.

### Rate limiting with a token bucket

Rate limiting is the exercise in this module. The idea: each client gets a "bucket" of tokens that refills over time. Each request costs one token. If the bucket is empty, reject with `429 Too Many Requests`.

For a simple in-memory implementation (single-process, no distributed state), a `HashMap<client_key, TokenBucket>` behind an `Arc<Mutex<...>>` works:

```
TokenBucket {
    tokens: f64,        // current tokens (fractional for smooth refill)
    max_tokens: f64,    // bucket capacity
    refill_rate: f64,   // tokens per second
    last_refill: Instant,
}
```

On each request:
1. Calculate elapsed time since last refill
2. Add `elapsed * refill_rate` tokens (capped at `max_tokens`)
3. If `tokens >= 1.0`, consume one and forward the request
4. Otherwise, return `429`

The layer extracts a client key from the request (e.g., a header, or the peer IP — in tests, a fixed key). This is a teaching implementation, not production-grade (no cleanup of stale entries, no distributed state), but it demonstrates the pattern.

### The request lifecycle with middleware

Here's the full picture for an axum request with layers:

```
TCP connection ──► hyper (HTTP parsing)
                     │
                     ▼
                  axum Router (tower Service)
                     │
                     ▼
              ┌──── Layer stack ────┐
              │  TraceLayer         │  ← logs request start
              │  CorsLayer          │  ← handles OPTIONS preflight
              │  RateLimitLayer     │  ← checks token bucket
              └─────────────────────┘
                     │
                     ▼
                  Handler (your async fn)
                     │
                     ▼
              ┌──── Layer stack ────┐  (reverse order)
              │  RateLimitLayer     │  ← (pass-through on response)
              │  CorsLayer          │  ← adds CORS headers
              │  TraceLayer         │  ← logs response + duration
              └─────────────────────┘
                     │
                     ▼
                  hyper (HTTP serialization) ──► TCP response
```

Each layer gets to act on both the request path (before the handler) and the response path (after). `TraceLayer` logs on both sides. `CorsLayer` adds headers on the response. `RateLimitLayer` may short-circuit on the request side (returning 429 without calling the handler).

## Common Pitfalls

- **Forgetting `poll_ready` delegation.** Your custom `Service::poll_ready()` must call `self.inner.poll_ready(cx)`. Forgetting this causes deadlocks — the service never reports readiness.
- **Layer ordering matters.** `.layer(A).layer(B)` means B wraps A: requests go through B first, then A. If your CORS layer is inside your auth layer, preflight `OPTIONS` requests hit the auth check and fail.
- **`.layer()` applies to routes registered *before* it.** `Router::new().route(...).layer(X).route(...)` — the second `.route()` is *not* wrapped by `X`. Add all routes first, then layers.
- **Blocking inside a layer.** Layers run on the async runtime. If your rate limiter does blocking I/O (e.g., reading from a file), it blocks the whole worker thread. Use `spawn_blocking` or keep it in-memory.
- **Token bucket without cleanup.** The exercise's `HashMap` grows forever. Production rate limiters need TTL eviction or a bounded data structure.

## Key Terms

- **`tower::Service`:** a trait for anything that processes a request and returns a future response — the fundamental abstraction.
- **`tower::Layer`:** a trait for wrapping a `Service` to produce a new `Service` with added behavior.
- **`tower-http`:** a crate of production-ready Tower layers (tracing, CORS, compression, timeouts, etc.).
- **`TraceLayer`:** a `tower-http` layer that logs every request/response with `tracing` spans.
- **`CorsLayer`:** a `tower-http` layer that handles CORS headers and preflight requests.
- **Token bucket:** a rate-limiting algorithm where tokens refill at a fixed rate and each request consumes one token.
- **Backpressure (`poll_ready`):** the mechanism by which a service signals it's ready to accept more work; layers must propagate this.

## Exercise

In `exercises/`, three layers are stubbed with `panic!` and marked `// TODO(module-066)`:

1. **`RequestIdLayer` / `RequestIdService`** — a custom layer that adds an `x-request-id` response header with a fixed value from the layer config.
2. **`RateLimitLayer` / `RateLimitService`** — a simple token-bucket rate limiter. The bucket is shared via `Arc<Mutex<HashMap<String, TokenBucket>>>`. Use the `x-client-id` request header as the client key (default to `"default"` if absent). Refill tokens based on elapsed time, consume one per request, return `429` if empty.
3. **Wire the layers** into `build_router` using `tower-http`'s `CorsLayer::permissive()` and your custom layers.

The tests use `oneshot` — no sockets. Run:

```bash
cargo test -p module-066-exercises
```

When all tests pass, compare with `solutions/`.

## Further Reading

- [Tower documentation](https://docs.rs/tower) — the `Service` and `Layer` traits
- [tower-http documentation](https://docs.rs/tower-http) — all available layers
- [Axum middleware guide](https://docs.rs/axum/latest/axum/middleware/index.html)
- [Token bucket algorithm (Wikipedia)](https://en.wikipedia.org/wiki/Token_bucket)
