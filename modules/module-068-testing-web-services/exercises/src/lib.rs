//! Module 068: Testing Web Services — exercise scaffold.
//!
//! The concept: when your handler calls a database, you can't test it against
//! a real prod database without side-effects. Instead, you define the
//! database dependency as a *trait*, provide a real implementation for
//! production and an in-memory mock for tests, then use test helper
//! functions to send requests against your router.
//!
//! Find the `// TODO(module-068)` comments below and fill them in until
//! `cargo test -p module-068-exercises` passes.

use async_trait::async_trait;
#[allow(unused_imports)]
use axum::{
    body::{to_bytes, Body},
    extract::{Path, State},
    http::{Request, StatusCode},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
#[allow(unused_imports)]
use tower::ServiceExt;

/// A todo item.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// The database interface: a trait so tests can swap the real implementation
/// for an in-memory one.
#[async_trait]
pub trait TodoStore: Clone + Send + Sync + 'static {
    async fn create_todo(&self, title: String) -> Todo;
    async fn list_todos(&self) -> Vec<Todo>;
    async fn get_todo(&self, id: u64) -> Option<Todo>;
    async fn delete_todo(&self, id: u64) -> bool;
}

/// Production store (not used in tests — just a type placeholder).
#[derive(Clone, Default)]
pub struct SqlTodoStore;

#[async_trait]
impl TodoStore for SqlTodoStore {
    async fn create_todo(&self, _title: String) -> Todo {
        unimplemented!("SqlTodoStore is a placeholder for a real sqlx-backed store")
    }
    async fn list_todos(&self) -> Vec<Todo> {
        unimplemented!()
    }
    async fn get_todo(&self, _id: u64) -> Option<Todo> {
        unimplemented!()
    }
    async fn delete_todo(&self, _id: u64) -> bool {
        unimplemented!()
    }
}

/// In-memory store for tests: backed by a `HashMap` protected by a tokio
/// `Mutex`. Fully thread-safe and clonable.
#[derive(Clone, Default)]
pub struct InMemoryStore {
    pub todos: Arc<Mutex<HashMap<u64, Todo>>>,
    pub next_id: Arc<Mutex<u64>>,
}

impl InMemoryStore {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(Mutex::new(1)),
        }
    }
}

#[async_trait]
impl TodoStore for InMemoryStore {
    async fn create_todo(&self, title: String) -> Todo {
        // TODO(module-068): Lock `next_id`, take the current value and
        // increment, then lock `todos` and insert a new `Todo { id, title,
        // completed: false }`. Return the new `Todo`.
        panic!("not implemented: InMemoryStore::create_todo({title:?})")
    }

    async fn list_todos(&self) -> Vec<Todo> {
        // TODO(module-068): Lock `todos`, collect the values, sort by id,
        // and return them as a `Vec<Todo>`.
        panic!("not implemented: InMemoryStore::list_todos()")
    }

    async fn get_todo(&self, id: u64) -> Option<Todo> {
        // TODO(module-068): Lock `todos`, look up the id, and return a
        // clone of the value (or `None` if absent).
        panic!("not implemented: InMemoryStore::get_todo({id})")
    }

    async fn delete_todo(&self, id: u64) -> bool {
        // TODO(module-068): Lock `todos`, remove the entry, and return
        // whether a value was removed.
        panic!("not implemented: InMemoryStore::delete_todo({id})")
    }
}

/// Application state shared across handlers. Uses a generic `TodoStore`
/// so tests can inject `InMemoryStore` and prod can use `SqlTodoStore`.
#[derive(Clone)]
pub struct AppState<S: TodoStore> {
    pub store: S,
}

/// Builds a test app with the given store. The returned `Router` exposes
/// the same endpoints as a real app but uses an in-memory store.
pub fn test_app<S: TodoStore>(store: S) -> Router {
    Router::new()
        .route("/todos", get(list_todos::<S>).post(create_todo::<S>))
        .route("/todos/{id}", get(get_todo::<S>).delete(delete_todo::<S>))
        .with_state(AppState { store })
}

/// Sends an HTTP request to the router and returns the (status, json body).
/// Uses `tower::ServiceExt::oneshot` — no sockets, no ports.
pub async fn send_request(
    app: &Router,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    // TODO(module-068): Build a `Request<Body>` with the given method, URI
    // (path), and optional JSON body. For POST requests, set the
    // `content-type: application/json` header. Call `app.clone().oneshot(...)`.
    // Read the response status and body bytes, parse the body as `Value`
    // (or `Value::Null` if empty), and return `(status, json_value)`.
    panic!("not implemented: send_request(app={app:?}, method={method:?}, path={path:?}, body={body:?})")
}

// --- handlers ---

async fn list_todos<S: TodoStore>(State(state): State<AppState<S>>) -> Json<Vec<Todo>> {
    Json(state.store.list_todos().await)
}

async fn create_todo<S: TodoStore>(
    State(state): State<AppState<S>>,
    Json(input): Json<NewTodo>,
) -> (StatusCode, Json<Todo>) {
    let todo = state.store.create_todo(input.title).await;
    (StatusCode::CREATED, Json(todo))
}

async fn get_todo<S: TodoStore>(
    State(state): State<AppState<S>>,
    Path(id): Path<u64>,
) -> Result<Json<Todo>, StatusCode> {
    state
        .store
        .get_todo(id)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_todo<S: TodoStore>(
    State(state): State<AppState<S>>,
    Path(id): Path<u64>,
) -> impl IntoResponse {
    if state.store.delete_todo(id).await {
        StatusCode::NO_CONTENT
    } else {
        StatusCode::NOT_FOUND
    }
}

#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
}
