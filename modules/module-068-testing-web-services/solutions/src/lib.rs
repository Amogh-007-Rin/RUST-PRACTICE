//! Module 068: Testing Web Services — reference solution.

use async_trait::async_trait;
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
use tower::ServiceExt;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

#[async_trait]
pub trait TodoStore: Clone + Send + Sync + 'static {
    async fn create_todo(&self, title: String) -> Todo;
    async fn list_todos(&self) -> Vec<Todo>;
    async fn get_todo(&self, id: u64) -> Option<Todo>;
    async fn delete_todo(&self, id: u64) -> bool;
}

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
        let mut next_id = self.next_id.lock().await;
        let id = *next_id;
        *next_id += 1;
        drop(next_id);

        let todo = Todo {
            id,
            title,
            completed: false,
        };
        self.todos.lock().await.insert(id, todo.clone());
        todo
    }

    async fn list_todos(&self) -> Vec<Todo> {
        let todos = self.todos.lock().await;
        let mut list: Vec<Todo> = todos.values().cloned().collect();
        list.sort_by_key(|t| t.id);
        list
    }

    async fn get_todo(&self, id: u64) -> Option<Todo> {
        self.todos.lock().await.get(&id).cloned()
    }

    async fn delete_todo(&self, id: u64) -> bool {
        self.todos.lock().await.remove(&id).is_some()
    }
}

#[derive(Clone)]
pub struct AppState<S: TodoStore> {
    pub store: S,
}

pub fn test_app<S: TodoStore>(store: S) -> Router {
    Router::new()
        .route("/todos", get(list_todos::<S>).post(create_todo::<S>))
        .route("/todos/{id}", get(get_todo::<S>).delete(delete_todo::<S>))
        .with_state(AppState { store })
}

pub async fn send_request(
    app: &Router,
    method: &str,
    path: &str,
    body: Option<Value>,
) -> (StatusCode, Value) {
    let mut builder = Request::builder().method(method).uri(path);
    if body.is_some() {
        builder = builder.header("content-type", "application/json");
    }
    let request = builder
        .body(Body::from(body.map(|v| v.to_string()).unwrap_or_default()))
        .unwrap();

    let response = app.clone().oneshot(request).await.unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let value = if bytes.is_empty() {
        Value::Null
    } else {
        serde_json::from_slice(&bytes).unwrap()
    };
    (status, value)
}

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
