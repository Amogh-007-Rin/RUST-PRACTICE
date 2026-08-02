//! Integration tests for module 062. These exercise the router directly
//! with `tower::ServiceExt::oneshot` — no sockets, no running server:
//! the request travels through the same handler pipeline a real HTTP
//! request would, minus the wire.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use module_062_solutions::{build_router, AppState};

/// Sends a `GET` request and returns (status, body-as-string).
async fn get(app: &Router, uri: &str) -> (StatusCode, String) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8(bytes.to_vec()).unwrap())
}

/// Sends a `POST` request with a JSON body and returns (status, body-as-json).
async fn post_json(app: &Router, uri: &str, body: Value) -> (StatusCode, Value) {
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(uri)
                .header("content-type", "application/json")
                .body(Body::from(body.to_string()))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

fn app() -> Router {
    build_router(AppState::new())
}

#[tokio::test]
async fn root_returns_200_with_greeting() {
    let (status, body) = get(&app(), "/").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello from Axum!");
}

#[tokio::test]
async fn hello_captures_path_parameter() {
    let (status, body) = get(&app(), "/hello/world").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello, world!");
}

#[tokio::test]
async fn search_reads_query_parameters() {
    let (status, body) = get(&app(), "/search?q=rust&limit=5").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "rust/5");
}

#[tokio::test]
async fn search_applies_default_limit() {
    let (status, body) = get(&app(), "/search?q=rust").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "rust/10");
}

#[tokio::test]
async fn search_defaults_all_parameters() {
    let (status, body) = get(&app(), "/search").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "/10");
}

#[tokio::test]
async fn create_item_returns_201_with_assigned_id() {
    let app = app();
    let (status, body) = post_json(&app, "/items", json!({"name": "buy milk"})).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["id"], 1);
    assert_eq!(body["name"], "buy milk");
}

#[tokio::test]
async fn ids_increment_across_creates() {
    let app = app();
    let (_, first) = post_json(&app, "/items", json!({"name": "one"})).await;
    let (_, second) = post_json(&app, "/items", json!({"name": "two"})).await;
    assert_eq!(first["id"], 1);
    assert_eq!(second["id"], 2);
}

#[tokio::test]
async fn item_count_tracks_creates() {
    let app = app();
    post_json(&app, "/items", json!({"name": "one"})).await;
    post_json(&app, "/items", json!({"name": "two"})).await;
    let (status, body) = get(&app, "/items/count").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "2");
}

#[tokio::test]
async fn unknown_route_returns_404() {
    let (status, _) = get(&app(), "/nope").await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
