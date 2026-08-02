//! Integration tests for module 069: deployment & observability by
//! hitting the `/health` endpoint and checking the `init_tracing()` guard.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::Value;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use tower::ServiceExt;

use module_069_exercises::{build_app_with_observability, AppState};

async fn send(app: &Router, path: &str) -> (StatusCode, Value) {
    let req = Request::builder().uri(path).body(Body::empty()).unwrap();
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

fn make_state() -> AppState {
    AppState {
        start_time: std::time::Instant::now(),
        degraded: Arc::new(AtomicBool::new(false)),
    }
}

#[tokio::test]
async fn health_check_returns_200_and_healthy_when_not_degraded() {
    let app = build_app_with_observability(make_state());
    let (status, body) = send(&app, "/health").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["status"], "healthy");
    assert!(body["uptime_seconds"].as_u64().unwrap() < 5);
}

#[tokio::test]
async fn health_check_returns_503_when_degraded() {
    let state = AppState {
        start_time: std::time::Instant::now(),
        degraded: Arc::new(AtomicBool::new(true)),
    };
    let app = build_app_with_observability(state);
    let (status, body) = send(&app, "/health").await;
    assert_eq!(status, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(body["status"], "degraded");
}

#[tokio::test]
async fn init_tracing_returns_a_guard() {
    // init_tracing must not panic and must return a guard that implements
    // `Drop` — we can't call it twice in the same process (global
    // subscriber), so just call it once and drop.
    let _guard = module_069_exercises::init_tracing();
    drop(_guard);
}

#[tokio::test]
async fn health_check_uptime_increases() {
    let state = make_state();
    let app = build_app_with_observability(state);
    let (_, body1) = send(&app, "/health").await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    let (_, body2) = send(&app, "/health").await;
    assert!(
        body2["uptime_seconds"].as_u64().unwrap() > body1["uptime_seconds"].as_u64().unwrap(),
        "uptime should increase between calls"
    );
}
