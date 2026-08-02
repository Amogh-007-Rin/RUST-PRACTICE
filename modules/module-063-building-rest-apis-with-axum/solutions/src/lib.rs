//! Module 063: Building REST APIs with axum — CRUD endpoints, JSON via
//! serde, request validation, and an in-memory store shared through state.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicU64, Ordering},
    Arc, Mutex,
};

/// The resource being managed. Stored by id in the shared store.
#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct Todo {
    pub id: u64,
    pub title: String,
    pub completed: bool,
}

/// Payload for creating a todo.
#[derive(Debug, Deserialize)]
pub struct NewTodo {
    pub title: String,
}

/// Payload for replacing a todo.
#[derive(Debug, Deserialize)]
pub struct TodoUpdate {
    pub title: String,
    #[serde(default)]
    pub completed: bool,
}

/// Shared application state: the todo store plus an id generator.
#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub todos: Arc<Mutex<HashMap<u64, Todo>>>,
    pub next_id: Arc<AtomicU64>,
}

/// Every error the API can produce, mapped to an HTTP response.
#[derive(Debug)]
pub enum AppError {
    NotFound,
    Invalid(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "todo not found".to_string()),
            AppError::Invalid(message) => (StatusCode::UNPROCESSABLE_ENTITY, message),
        };
        (status, Json(json!({ "error": message }))).into_response()
    }
}

type ApiResult<T> = Result<T, AppError>;

/// Assembles the application: one resource, five REST endpoints.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route(
            "/todos/{id}",
            get(get_todo).put(update_todo).delete(delete_todo),
        )
        .with_state(state)
}

pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap();
    let mut list: Vec<Todo> = todos.values().cloned().collect();
    list.sort_by_key(|todo| todo.id);
    Json(list)
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(input): Json<NewTodo>,
) -> ApiResult<(StatusCode, Json<Todo>)> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(AppError::Invalid("title must not be empty".to_string()));
    }
    let id = state.next_id.fetch_add(1, Ordering::SeqCst) + 1;
    let todo = Todo {
        id,
        title: title.to_string(),
        completed: false,
    };
    state.todos.lock().unwrap().insert(id, todo.clone());
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn get_todo(State(state): State<AppState>, Path(id): Path<u64>) -> ApiResult<Json<Todo>> {
    let todos = state.todos.lock().unwrap();
    todos.get(&id).cloned().map(Json).ok_or(AppError::NotFound)
}

pub async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
    Json(input): Json<TodoUpdate>,
) -> ApiResult<Json<Todo>> {
    let title = input.title.trim();
    if title.is_empty() {
        return Err(AppError::Invalid("title must not be empty".to_string()));
    }
    let mut todos = state.todos.lock().unwrap();
    let todo = todos.get_mut(&id).ok_or(AppError::NotFound)?;
    todo.title = title.to_string();
    todo.completed = input.completed;
    Ok(Json(todo.clone()))
}

pub async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<u64>,
) -> ApiResult<StatusCode> {
    let mut todos = state.todos.lock().unwrap();
    if todos.remove(&id).is_some() {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err(AppError::NotFound)
    }
}
