//! Module 069: Deployment & Observability — reference solution.

use axum::{http::StatusCode, routing::get, Json, Router};
use serde::Serialize;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tracing_subscriber::EnvFilter;

#[derive(Clone, Serialize)]
pub struct HealthStatus {
    pub status: String,
    pub uptime_seconds: u64,
}

#[derive(Clone)]
pub struct AppState {
    pub start_time: std::time::Instant,
    pub degraded: Arc<AtomicBool>,
}

pub fn build_app_with_observability(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health_check))
        .with_state(state)
}

pub fn init_tracing() -> impl Drop {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let subscriber = tracing_subscriber::fmt().with_env_filter(filter).finish();
    tracing::subscriber::set_default(subscriber)
}

pub async fn health_check(
    state: axum::extract::State<AppState>,
) -> (StatusCode, Json<HealthStatus>) {
    let uptime_seconds = state.start_time.elapsed().as_secs();
    let degraded = state.degraded.load(Ordering::Relaxed);
    let (status, code) = if degraded {
        ("degraded", StatusCode::SERVICE_UNAVAILABLE)
    } else {
        ("healthy", StatusCode::OK)
    };
    tracing::info!(
        status = status,
        uptime_seconds = uptime_seconds,
        "health check"
    );
    (
        code,
        Json(HealthStatus {
            status: status.to_string(),
            uptime_seconds,
        }),
    )
}
