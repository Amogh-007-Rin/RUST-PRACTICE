use axum_test::http::StatusCode;
use axum_test::TestServer;
use capstone_07_task_management_api_solution::build_app;
use serde_json::{json, Value};
use sqlx::SqlitePool;

async fn setup() -> (TestServer, SqlitePool) {
    let pool = SqlitePool::connect("sqlite::memory:?cache=shared")
        .await
        .expect("connect");
    let app = build_app(pool.clone(), "test-secret".into()).await;
    let server = TestServer::new(app).unwrap();
    (server, pool)
}

async fn register_and_login(server: &TestServer) -> String {
    let resp = server
        .post("/api/auth/register")
        .json(&json!({"username": "alice", "password": "secret123"}))
        .await;
    resp.assert_status_ok();
    resp.json::<Value>()["token"].as_str().unwrap().to_string()
}

#[tokio::test]
async fn health_check_works() {
    let (server, _pool) = setup().await;
    let resp = server.get("/api/health").await;
    resp.assert_status_ok();
    let body: Value = resp.json();
    assert_eq!(body["status"], "ok");
}

#[tokio::test]
async fn register_and_login_flow() {
    let (server, _pool) = setup().await;

    let resp = server
        .post("/api/auth/register")
        .json(&json!({"username": "bob", "password": "password123"}))
        .await;
    resp.assert_status_ok();
    let token1 = resp.json::<Value>()["token"].as_str().unwrap().to_string();
    assert!(!token1.is_empty());

    let resp = server
        .post("/api/auth/login")
        .json(&json!({"username": "bob", "password": "password123"}))
        .await;
    resp.assert_status_ok();
    let token2 = resp.json::<Value>()["token"].as_str().unwrap().to_string();
    assert!(!token2.is_empty());
}

#[tokio::test]
async fn register_duplicate_username_fails() {
    let (server, _pool) = setup().await;

    server
        .post("/api/auth/register")
        .json(&json!({"username": "eve", "password": "password123"}))
        .await;

    let resp = server
        .post("/api/auth/register")
        .json(&json!({"username": "eve", "password": "different456"}))
        .await;
    assert_eq!(resp.status_code(), 409);
}

#[tokio::test]
async fn login_bad_password_returns_401() {
    let (server, _pool) = setup().await;

    server
        .post("/api/auth/register")
        .json(&json!({"username": "carl", "password": "secret123"}))
        .await;

    let resp = server
        .post("/api/auth/login")
        .json(&json!({"username": "carl", "password": "wrongpass"}))
        .await;
    assert_eq!(resp.status_code(), 401);
}

#[tokio::test]
async fn crud_task_operations() {
    let (server, _pool) = setup().await;
    let token = register_and_login(&server).await;

    let resp = server
        .post("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&json!({"title": "Learn Rust", "description": "Read the book"}))
        .await;
    resp.assert_status(StatusCode::CREATED);
    let created: Value = resp.json();
    let task_id = created["id"].as_str().unwrap().to_string();
    assert_eq!(created["title"], "Learn Rust");

    let resp = server
        .get(&format!("/api/tasks/{}", task_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    resp.assert_status_ok();
    let fetched: Value = resp.json();
    assert_eq!(fetched["id"], task_id);
    assert_eq!(fetched["status"], "todo");

    let resp = server
        .put(&format!("/api/tasks/{}", task_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&json!({"status": "in_progress", "priority": "high"}))
        .await;
    resp.assert_status_ok();
    let updated: Value = resp.json();
    assert_eq!(updated["status"], "in_progress");
    assert_eq!(updated["priority"], "high");

    let resp = server
        .delete(&format!("/api/tasks/{}", task_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    assert_eq!(resp.status_code(), 204);

    let resp = server
        .get(&format!("/api/tasks/{}", task_id))
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    assert_eq!(resp.status_code(), 404);
}

#[tokio::test]
async fn unauthorized_access_returns_401() {
    let (server, _pool) = setup().await;

    let resp = server.get("/api/tasks").await;
    assert_eq!(resp.status_code(), 401);

    let resp = server
        .post("/api/tasks")
        .json(&json!({"title": "test"}))
        .await;
    assert_eq!(resp.status_code(), 401);

    let resp = server.get("/api/tasks/some-id").await;
    assert_eq!(resp.status_code(), 401);
}

#[tokio::test]
async fn invalid_token_returns_401() {
    let (server, _pool) = setup().await;

    let resp = server
        .get("/api/tasks")
        .add_header("Authorization", "Bearer bad-token-here")
        .await;
    assert_eq!(resp.status_code(), 401);
}

#[tokio::test]
async fn task_filtering_by_status_and_priority() {
    let (server, _pool) = setup().await;
    let token = register_and_login(&server).await;

    let tasks = vec![
        json!({"title": "Todo low", "status": "todo", "priority": "low"}),
        json!({"title": "Todo high", "status": "todo", "priority": "high"}),
        json!({"title": "Done med", "status": "done", "priority": "medium"}),
        json!({"title": "In progress high", "status": "in_progress", "priority": "high"}),
    ];

    for task in &tasks {
        server
            .post("/api/tasks")
            .add_header("Authorization", format!("Bearer {}", token))
            .json(task)
            .await;
    }

    let resp = server
        .get("/api/tasks?status=todo")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    let filtered: Value = resp.json();
    let arr = filtered.as_array().unwrap();
    assert_eq!(arr.len(), 2, "expected 2 todo tasks");

    let resp = server
        .get("/api/tasks?priority=high")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    let filtered: Value = resp.json();
    let arr = filtered.as_array().unwrap();
    assert_eq!(arr.len(), 2, "expected 2 high priority tasks");

    let resp = server
        .get("/api/tasks?status=todo&priority=high")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    let filtered: Value = resp.json();
    let arr = filtered.as_array().unwrap();
    assert_eq!(arr.len(), 1);
}

#[tokio::test]
async fn task_ownership_isolation() {
    let (server, _pool) = setup().await;

    let token_alice = {
        let resp = server
            .post("/api/auth/register")
            .json(&json!({"username": "alice", "password": "secret123"}))
            .await;
        resp.json::<Value>()["token"].as_str().unwrap().to_string()
    };

    let token_bob = {
        let resp = server
            .post("/api/auth/register")
            .json(&json!({"username": "bob", "password": "secret123"}))
            .await;
        resp.json::<Value>()["token"].as_str().unwrap().to_string()
    };

    let resp = server
        .post("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token_alice))
        .json(&json!({"title": "Alice's task"}))
        .await;
    let alice_task: Value = resp.json();
    let alice_task_id = alice_task["id"].as_str().unwrap().to_string();

    let resp = server
        .post("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token_bob))
        .json(&json!({"title": "Bob's task"}))
        .await;
    let bob_task: Value = resp.json();
    let bob_task_id = bob_task["id"].as_str().unwrap().to_string();

    let resp = server
        .get("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token_alice))
        .await;
    let alice_tasks: Value = resp.json();
    let alice_arr = alice_tasks.as_array().unwrap();
    assert_eq!(alice_arr.len(), 1);
    assert_eq!(alice_arr[0]["id"], alice_task_id);

    let resp = server
        .get("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token_bob))
        .await;
    let bob_tasks: Value = resp.json();
    let bob_arr = bob_tasks.as_array().unwrap();
    assert_eq!(bob_arr.len(), 1);
    assert_eq!(bob_arr[0]["id"], bob_task_id);

    let resp = server
        .get(&format!("/api/tasks/{}", alice_task_id))
        .add_header("Authorization", format!("Bearer {}", token_bob))
        .await;
    assert_eq!(resp.status_code(), 404);

    let resp = server
        .get(&format!("/api/tasks/{}", bob_task_id))
        .add_header("Authorization", format!("Bearer {}", token_alice))
        .await;
    assert_eq!(resp.status_code(), 404);
}

#[tokio::test]
async fn register_validation_rejects_empty_username() {
    let (server, _pool) = setup().await;

    let resp = server
        .post("/api/auth/register")
        .json(&json!({"username": "  ", "password": "secret123"}))
        .await;
    assert_eq!(resp.status_code(), 400);
}

#[tokio::test]
async fn register_validation_rejects_short_password() {
    let (server, _pool) = setup().await;

    let resp = server
        .post("/api/auth/register")
        .json(&json!({"username": "user", "password": "12345"}))
        .await;
    assert_eq!(resp.status_code(), 400);
}

#[tokio::test]
async fn create_task_rejects_empty_title() {
    let (server, _pool) = setup().await;
    let token = register_and_login(&server).await;

    let resp = server
        .post("/api/tasks")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&json!({"title": ""}))
        .await;
    assert_eq!(resp.status_code(), 400);
}

#[tokio::test]
async fn update_nonexistent_task_returns_404() {
    let (server, _pool) = setup().await;
    let token = register_and_login(&server).await;

    let resp = server
        .put("/api/tasks/nonexistent-id")
        .add_header("Authorization", format!("Bearer {}", token))
        .json(&json!({"title": "updated"}))
        .await;
    assert_eq!(resp.status_code(), 404);
}

#[tokio::test]
async fn delete_nonexistent_task_returns_404() {
    let (server, _pool) = setup().await;
    let token = register_and_login(&server).await;

    let resp = server
        .delete("/api/tasks/nonexistent-id")
        .add_header("Authorization", format!("Bearer {}", token))
        .await;
    assert_eq!(resp.status_code(), 404);
}
