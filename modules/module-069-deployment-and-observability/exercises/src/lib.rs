//! Module 069: Deployment & Observability — exercise scaffold.
//!
//! Production services need to report their health and emit structured logs.
//! `tracing` is Rust's structured logging ecosystem; `tracing-subscriber`
//! is the sink that controls where logs go.
//!
//! Find the `// TODO(module-069)` comments below and fill them in until
//! `cargo test -p module-069-exercises` passes.

#[allow(unused_imports)]
use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

/// Application health metadata returned by the `/health` endpoint.
#[derive(Clone, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
}

/// Shared state: includes a flag for simulating a "degraded" service.
#[derive(Clone)]
pub struct AppState {
    pub start_time: std::time::Instant,
    pub degraded: Arc<AtomicBool>,
}

/// Builds a router with:
/// - A `/health` endpoint that reports uptime and a status string.
/// - A tracing span on every request (via the tower tracing layer pattern,
///   though here we keep it simple: the handler itself instruments).
pub fn build_app_with_observability(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}

/// Initialises a `tracing-subscriber` that:
/// - Outputs to stderr.
/// - Filters to `INFO` level by default, respecting the `RUST_LOG`
///   environment variable (via `EnvFilter`).
/// - Returns a guard that must be kept alive for the duration of the
///   program — when it drops, buffered logs are flushed.
///
/// The return type `impl Drop` hides the specific guard type.
pub fn init_tracing() -> impl Drop {
    // TODO(module-069): Create a `tracing_subscriber::fmt()` subscriber with
    // an `EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))`
    // filter. Call `.finish()` to get the subscriber, then use
    // `tracing::subscriber::set_default(subscriber)` to install it
    // (returning a guard). Replace this `NoopGuard` placeholder.
    //
    // Step-by-step:
    // 1. `let filter = tracing_subscriber::EnvFilter::try_from_default_env()...`
    // 2. `let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();`
    // 3. `tracing::subscriber::set_default(subscriber).unwrap()`
    struct NoopGuard;
    impl Drop for NoopGuard {
        fn drop(&mut self) {}
    }
    NoopGuard
}

/// The health check handler: reports uptime and whether the service is
/// healthy or degraded.
pub async fn health_check(
    _state: axum::extract::State<AppState>,
) -> (StatusCode, Json<HealthStatus>) {
    // TODO(module-069): Compute uptime in seconds from `state.start_time`,
    // check `state.degraded`, and return:
    // - `StatusCode::OK` with `status: "healthy"` if not degraded.
    // - `StatusCode::SERVICE_UNAVAILABLE` with `status: "degraded"` if degraded.
    // Use `tracing::info!` to log the health status.
    // Wrap the result in `Json(...)`.
    panic!("not implemented: health_check()")
}
