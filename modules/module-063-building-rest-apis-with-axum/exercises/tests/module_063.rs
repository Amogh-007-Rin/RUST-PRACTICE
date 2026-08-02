//! Integration tests for module 063: full CRUD lifecycle, validation
//! failures, and 404s for missing resources — all through `oneshot`.

use axum::{
    body::{to_bytes, Body},
    http::{Request, StatusCode},
    Router,
};
use serde_json::{json, Value};
use tower::ServiceExt;

use module_063_exercises::{build_router, AppState};

async fn send(app: &Router, req: Request<Body>) -> (StatusCode, Value) {
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

fn get_request(uri: &str) -> Request<Body> {
    Request::builder().uri(uri).body(Body::empty()).unwrap()
}

fn json_request(method: &str, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header("content-type", "application/json")
        .body(Body::from(body.to_string()))
        .unwrap()
}

fn app() -> Router {
    build_router(AppState::default())
}

#[tokio::test]
async fn empty_store_lists_no_todos() {
    let (status, body) = send(&app(), get_request("/todos")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, json!([]));
}

#[tokio::test]
async fn create_todo_returns_201_with_stored_fields() {
    let (status, body) = send(
        &app(),
        json_request("POST", "/todos", json!({"title": "buy milk"})),
    )
    .await;
    assert_eq!(status, StatusCode::CREATED);
    assert_eq!(body["id"], 1);
    assert_eq!(body["title"], "buy milk");
    assert_eq!(body["completed"], false);
}

#[tokio::test]
async fn created_todos_appear_in_list() {
    let app = app();
    send(
        &app,
        json_request("POST", "/todos", json!({"title": "one"})),
    )
    .await;
    send(
        &app,
        json_request("POST", "/todos", json!({"title": "two"})),
    )
    .await;
    let (status, body) = send(&app, get_request("/todos")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body.as_array().unwrap().len(), 2);
    assert_eq!(body[0]["title"], "one");
    assert_eq!(body[1]["title"], "two");
    assert_eq!(body[0]["id"], 1);
    assert_eq!(body[1]["id"], 2);
}

#[tokio::test]
async fn create_todo_with_blank_title_returns_422() {
    for blank in ["", "   "] {
        let (status, body) = send(
            &app(),
            json_request("POST", "/todos", json!({"title": blank})),
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
        assert!(body["error"].is_string());
    }
}

#[tokio::test]
async fn get_todo_returns_200_for_existing() {
    let app = app();
    send(
        &app,
        json_request("POST", "/todos", json!({"title": "rust"})),
    )
    .await;
    let (status, body) = send(&app, get_request("/todos/1")).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "rust");
}

#[tokio::test]
async fn get_missing_todo_returns_404() {
    let (status, _) = send(&app(), get_request("/todos/999")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_todo_changes_fields() {
    let app = app();
    send(
        &app,
        json_request("POST", "/todos", json!({"title": "before"})),
    )
    .await;
    let (status, body) = send(
        &app,
        json_request(
            "PUT",
            "/todos/1",
            json!({"title": "after", "completed": true}),
        ),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body["title"], "after");
    assert_eq!(body["completed"], true);
    assert_eq!(body["id"], 1);
}

#[tokio::test]
async fn update_missing_todo_returns_404() {
    let (status, _) = send(
        &app(),
        json_request("PUT", "/todos/999", json!({"title": "nope"})),
    )
    .await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn update_todo_with_blank_title_returns_422() {
    let app = app();
    send(&app, json_request("POST", "/todos", json!({"title": "x"}))).await;
    let (status, _) = send(
        &app,
        json_request("PUT", "/todos/1", json!({"title": "  "})),
    )
    .await;
    assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn delete_todo_returns_204_and_removes_it() {
    let app = app();
    send(
        &app,
        json_request("POST", "/todos", json!({"title": "bye"})),
    )
    .await;
    let (status, _) = send(&app, json_request("DELETE", "/todos/1", json!({}))).await;
    assert_eq!(status, StatusCode::NO_CONTENT);
    let (status, _) = send(&app, get_request("/todos/1")).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    let (_, list) = send(&app, get_request("/todos")).await;
    assert_eq!(list, json!([]));
}

#[tokio::test]
async fn delete_missing_todo_returns_404() {
    let (status, _) = send(&app(), json_request("DELETE", "/todos/999", json!({}))).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}
