//! Integration tests for module 068: testing web services. Verifies that
//! the test helpers (`test_app`, `send_request`) and the mock store work
//! correctly together, so that a real test suite could be built on them.

use axum::http::StatusCode;
use serde_json::json;

use module_068_exercises::{send_request, test_app, InMemoryStore};

fn make_store() -> InMemoryStore {
    InMemoryStore::new()
}

#[tokio::test]
async fn send_request_get_empty_store_returns_empty_list() {
    let app = test_app(make_store());
    let (status, body) = send_request(&app, "GET", "/todos", None).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn send_request_post_creates_a_todo() {
    let app = test_app(make_store());
    let (status, body) =
        send_request(&app, "POST", "/todos", Some(json!({"title": "learn rust"}))).await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["id"], 1);
    assert_eq!(body["title"], "learn rust");
    assert_eq!(body["completed"], false);
}

#[tokio::test]
async fn full_crud_lifecycle() {
    let app = test_app(make_store());

    // Create two todos.
    let (s, _) = send_request(&app, "POST", "/todos", Some(json!({"title": "A"}))).await;
    assert_eq!(s, StatusCode::CREATED);
    send_request(&app, "POST", "/todos", Some(json!({"title": "B"}))).await;

    // List returns both.
    let (s, body) = send_request(&app, "GET", "/todos", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);

    // Get one by id.
    let (s, body) = send_request(&app, "GET", "/todos/1", None).await;
    assert_eq!(s, StatusCode::OK);
    assert_eq!(body["title"], "A");

    // Get missing.
    let (s, _) = send_request(&app, "GET", "/todos/999", None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    // Delete one.
    let (s, _) = send_request(&app, "DELETE", "/todos/1", None).await;
    assert_eq!(s, StatusCode::NO_CONTENT);

    // Gone now.
    let (s, _) = send_request(&app, "GET", "/todos/1", None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);

    // Delete missing.
    let (s, _) = send_request(&app, "DELETE", "/todos/999", None).await;
    assert_eq!(s, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn trait_is_dyn_safe_and_clonable() {
    // Compile-time checks: the trait bound Send + Sync + Clone + 'static
    // means we can pass an Arc-wrapped store into a Router.
    let store = make_store();
    let _app = test_app(store);
}

#[tokio::test]
async fn in_memory_store_is_isolated_between_tests() {
    // Create two separate apps with separate stores.
    let app1 = test_app(make_store());
    let app2 = test_app(make_store());

    send_request(
        &app1,
        "POST",
        "/todos",
        Some(json!({"title": "only in app1"})),
    )
    .await;
    let (_, body1) = send_request(&app1, "GET", "/todos", None).await;
    let (_, body2) = send_request(&app2, "GET", "/todos", None).await;

    assert_eq!(body1.as_array().unwrap().len(), 1);
    assert_eq!(body2.as_array().unwrap().len(), 0);
}
