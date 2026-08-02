//! Integration tests for module 066. These exercise the middleware stack
//! via `tower::ServiceExt::oneshot` — no sockets, no running server.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use tower::ServiceExt;

use module_066_exercises::{build_router, new_rate_limit_store};

async fn get(app: &Router, uri: &str) -> (StatusCode, String, axum::http::HeaderMap) {
    let response = app
        .clone()
        .oneshot(Request::builder().uri(uri).body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (status, String::from_utf8(bytes.to_vec()).unwrap(), headers)
}

async fn get_with_headers(
    app: &Router,
    uri: &str,
    headers: Vec<(&str, &str)>,
) -> (StatusCode, String, axum::http::HeaderMap) {
    let mut builder = Request::builder().uri(uri);
    for (key, value) in headers {
        builder = builder.header(key, value);
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let resp_headers = response.headers().clone();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    (
        status,
        String::from_utf8(bytes.to_vec()).unwrap(),
        resp_headers,
    )
}

#[tokio::test]
async fn hello_returns_200() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    let (status, body, _) = get(&app, "/hello").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "Hello from the middleware stack!");
}

#[tokio::test]
async fn request_id_header_is_added() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    let (status, _, headers) = get(&app, "/hello").await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        headers.get("x-request-id").map(|v| v.to_str().unwrap()),
        Some("req-001")
    );
}

#[tokio::test]
async fn cors_headers_are_present() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("OPTIONS")
                .uri("/hello")
                .header("origin", "http://example.com")
                .header("access-control-request-method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    assert!(response
        .headers()
        .contains_key("access-control-allow-origin"));
}

#[tokio::test]
async fn rate_limit_allows_within_budget() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    let (status, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-a")]).await;
    assert_eq!(status, StatusCode::OK);
}

#[tokio::test]
async fn rate_limit_rejects_when_exhausted() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    // max_tokens=2.0, so first two requests succeed, third is rejected.
    let (s1, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-b")]).await;
    let (s2, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-b")]).await;
    let (s3, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-b")]).await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(s3, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn rate_limit_is_per_client() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    // client-c uses 2 tokens, client-d is independent.
    let (s1, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-c")]).await;
    let (s2, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-c")]).await;
    let (s3, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-c")]).await;
    let (s4, _, _) = get_with_headers(&app, "/hello", vec![("x-client-id", "client-d")]).await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(s3, StatusCode::TOO_MANY_REQUESTS);
    assert_eq!(s4, StatusCode::OK);
}

#[tokio::test]
async fn rate_limit_defaults_client_id() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    // No x-client-id header — should default to "default".
    let (s1, _, _) = get(&app, "/hello").await;
    let (s2, _, _) = get(&app, "/hello").await;
    let (s3, _, _) = get(&app, "/hello").await;
    assert_eq!(s1, StatusCode::OK);
    assert_eq!(s2, StatusCode::OK);
    assert_eq!(s3, StatusCode::TOO_MANY_REQUESTS);
}

#[tokio::test]
async fn status_endpoint_shows_rate_limit_state() {
    let store = new_rate_limit_store();
    let app = build_router(store);
    let _ = get_with_headers(&app, "/hello", vec![("x-client-id", "tracked")]).await;
    let (status, body, _) = get(&app, "/status").await;
    assert_eq!(status, StatusCode::OK);
    assert!(body.contains("tracked"));
}
