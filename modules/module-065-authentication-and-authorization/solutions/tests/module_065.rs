//! Integration tests for module 065: the register/login flow, the
//! protected `/me` route, and the JWT edge cases.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use module_065_solutions::{build_router, hash_password, issue_token, verify_password, AppState};

async fn send(app: &Router, req: Request<Body>) -> (StatusCode, Value) {
    let response = app.clone().oneshot(req).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes)
            .unwrap_or(Value::String(String::from_utf8_lossy(&bytes).into_owned()))
    };
    (status, value)
}

fn post_json(uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method("POST")
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn get_with_token(uri: &str, token: &str) -> Request<Body> {
    Request::builder()
        .uri(uri)
        .header("authorization", format!("Bearer {token}"))
        .body(Body::empty())
        .unwrap()
}

fn app() -> Router {
    build_router(AppState::new("test-secret"))
}

async fn register(app: &Router, username: &str, password: &str) -> Value {
    let (status, body) = send(
        app,
        post_json(
            "/register",
            json!({"username": username, "password": password}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED, "register should succeed");
    body
}

#[tokio::test]
async fn password_hashes_are_not_plaintext() {
    let hash = hash_password("hunter2").unwrap();
    assert_ne!(hash, "hunter2");
    assert!(
        hash.starts_with("$argon2id$"),
        "expected an argon2id PHC string, got {hash}"
    );
}

#[tokio::test]
async fn verify_password_accepts_the_right_password_only() {
    let hash = hash_password("correct horse").unwrap();
    assert!(verify_password("correct horse", &hash));
    assert!(!verify_password("wrong", &hash));
    assert!(!verify_password("correct horse", "not-a-real-hash"));
}

#[tokio::test]
async fn register_returns_201_with_token_and_username() {
    let app = app();
    let (status, body) = send(
        &app,
        post_json(
            "/register",
            json!({"username": "alice", "password": "s3cret"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert!(body["token"].as_str().unwrap().split('.').count() == 3);
    assert_eq!(body["username"], "alice");
}

#[tokio::test]
async fn duplicate_register_returns_409() {
    let app = app();
    register(&app, "alice", "s3cret").await;
    let (status, _) = send(
        &app,
        post_json(
            "/register",
            json!({"username": "alice", "password": "other"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::CONFLICT);
}

#[tokio::test]
async fn register_rejects_empty_credentials() {
    let app = app();
    for body in [
        json!({"username": "", "password": "x"}),
        json!({"username": "bob", "password": ""}),
        json!({"username": "  ", "password": "x"}),
    ] {
        let (status, _) = send(&app, post_json("/register", body)).await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
}

#[tokio::test]
async fn login_returns_a_token_for_registered_users() {
    let app = app();
    register(&app, "alice", "s3cret").await;
    let (status, body) = send(
        &app,
        post_json("/login", json!({"username": "alice", "password": "s3cret"})),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(body["token"].is_string());
    assert_eq!(body["username"], "alice");
}

#[tokio::test]
async fn login_rejects_wrong_password_and_unknown_user() {
    let app = app();
    register(&app, "alice", "s3cret").await;
    let (status, _) = send(
        &app,
        post_json("/login", json!({"username": "alice", "password": "nope"})),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
    let (status, _) = send(
        &app,
        post_json(
            "/login",
            json!({"username": "mallory", "password": "s3cret"}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_returns_the_username_for_a_valid_token() {
    let app = app();
    let registration = register(&app, "alice", "s3cret").await;
    let token = registration["token"].as_str().unwrap();
    let (status, body) = send(&app, get_with_token("/me", token)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!("alice"));
}

#[tokio::test]
async fn me_rejects_missing_or_garbage_tokens() {
    let app = app();
    let no_header = Request::builder().uri("/me").body(Body::empty()).unwrap();
    let (status, _) = send(&app, no_header).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = send(&app, get_with_token("/me", "garbage.token.value")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);

    let (status, _) = send(&app, get_with_token("/me", "Basic dXNlcjpwYXNz")).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn me_rejects_tokens_signed_with_a_different_secret() {
    let app = build_router(AppState::new("test-secret"));
    let forged = issue_token("attacker-secret", "mallory").unwrap();
    let (status, _) = send(&app, get_with_token("/me", &forged)).await;
    assert_eq!(status, StatusCode::UNAUTHORIZED);
}
