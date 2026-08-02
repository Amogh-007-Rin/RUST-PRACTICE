//! Capstone 07 starter scaffold.
//!
//! Many items are marked `#[allow(unused)]` because they exist as scaffolding
//! for the student to fill in. Remove these attributes as you implement each section.

#![allow(dead_code, unused_imports, unused_variables)]

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::{FromRequestParts, Path, Query, State},
    http::{request::Parts, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use tower_http::trace::TraceLayer;
use tracing::info;

// ---------------------------------------------------------------------------
// Models
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Done,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(rename_all = "snake_case")]
pub enum Priority {
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Task {
    pub id: String,
    pub user_id: String,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub priority: Priority,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password_hash: String,
}

// ---------------------------------------------------------------------------
// Request / Response types
// ---------------------------------------------------------------------------

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: Option<String>,
    #[serde(default = "default_task_status")]
    pub status: TaskStatus,
    #[serde(default = "default_priority")]
    pub priority: Priority,
}

fn default_task_status() -> TaskStatus {
    TaskStatus::Todo
}
fn default_priority() -> Priority {
    Priority::Medium
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<Option<String>>,
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Deserialize)]
pub struct TaskFilter {
    pub status: Option<TaskStatus>,
    pub priority: Option<Priority>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub error: String,
}

// ---------------------------------------------------------------------------
// App state
// ---------------------------------------------------------------------------

pub struct AppState {
    pub db: SqlitePool,
    pub jwt_secret: String,
}

// ---------------------------------------------------------------------------
// Errors
// ---------------------------------------------------------------------------

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Not found")]
    NotFound,
    #[error("Bad request: {0}")]
    BadRequest(String),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match &self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),
            AppError::NotFound => (StatusCode::NOT_FOUND, "Not found".to_string()),
            AppError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg.clone()),
            AppError::Conflict(msg) => (StatusCode::CONFLICT, msg.clone()),
            AppError::Internal(e) => {
                tracing::error!(?e, "Internal error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Internal server error".to_string(),
                )
            }
        };
        (status, Json(ErrorBody { error: message })).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        AppError::Internal(e.into())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(e: serde_json::Error) -> Self {
        AppError::Internal(e.into())
    }
}

// ---------------------------------------------------------------------------
// Auth extractor
// ---------------------------------------------------------------------------

pub struct AuthUser {
    pub user_id: String,
}

#[async_trait]
impl FromRequestParts<Arc<AppState>> for AuthUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        // TODO: Implement JWT extraction and verification
        // 1. Read the "Authorization" header from parts.headers
        // 2. Strip the "Bearer " prefix to get the token string
        // 3. Use _state.jwt_secret for the signing key
        // 4. Decode and verify the JWT using jsonwebtoken
        // 5. Return AuthUser { user_id: claims.sub }

        // Stub: always reject until implemented
        Err(AppError::Unauthorized)
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn now_iso() -> String {
    humantime::format_rfc3339_seconds(SystemTime::now()).to_string()
}

fn make_token(_user_id: &str, _secret: &str) -> Result<String, AppError> {
    // TODO: Generate a JWT token
    // 1. Create Claims { sub: user_id, exp: now + 86400 }
    // 2. Encode with jsonwebtoken::encode using EncodingKey::from_secret
    todo!("implement make_token")
}

async fn run_migrations(pool: &SqlitePool) {
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS users (
            id TEXT PRIMARY KEY,
            username TEXT UNIQUE NOT NULL,
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .expect("create users table");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS tasks (
            id TEXT PRIMARY KEY,
            user_id TEXT NOT NULL REFERENCES users(id),
            title TEXT NOT NULL,
            description TEXT,
            status TEXT NOT NULL DEFAULT 'todo',
            priority TEXT NOT NULL DEFAULT 'medium',
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )",
    )
    .execute(pool)
    .await
    .expect("create tasks table");
}

// ---------------------------------------------------------------------------
// Handlers
// ---------------------------------------------------------------------------

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({"status": "ok"}))
}

async fn register(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // TODO: Implement user registration
    // 1. Validate username (non-empty) and password (min 6 chars)
    // 2. Check if username already exists in the database
    // 3. Hash the password with argon2
    // 4. Generate a UUID for the user
    // 5. Insert the user into the users table
    // 6. Call make_token to generate a JWT
    // 7. Return AuthResponse { token }
    todo!("implement register handler")
}

async fn login(
    State(_state): State<Arc<AppState>>,
    Json(_payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    // TODO: Implement login
    // 1. Fetch the user by username from the database
    // 2. Verify the password hash using argon2
    // 3. Call make_token to generate a JWT
    // 4. Return AuthResponse { token }
    todo!("implement login handler")
}

async fn list_tasks(
    State(_state): State<Arc<AppState>>,
    _auth: AuthUser,
    Query(_filter): Query<TaskFilter>,
) -> Result<Json<Vec<Task>>, AppError> {
    // TODO: List tasks for the authenticated user
    // 1. Build a SQL query selecting all task columns WHERE user_id = auth.user_id
    // 2. If filter.status is Some, add AND status = ?
    // 3. If filter.priority is Some, add AND priority = ?
    // 4. Order by created_at DESC
    // 5. Execute with sqlx::query_as::<_, Task>
    // 6. Return Json(tasks)
    todo!("implement list_tasks handler")
}

fn task_status_to_str(s: &TaskStatus) -> &str {
    match s {
        TaskStatus::Todo => "todo",
        TaskStatus::InProgress => "in_progress",
        TaskStatus::Done => "done",
    }
}

fn priority_to_str(p: &Priority) -> &str {
    match p {
        Priority::Low => "low",
        Priority::Medium => "medium",
        Priority::High => "high",
    }
}

async fn create_task(
    State(_state): State<Arc<AppState>>,
    _auth: AuthUser,
    Json(_payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    // TODO: Create a new task
    // 1. Validate the title is non-empty
    // 2. Generate a task UUID
    // 3. Get the current timestamp (now_iso)
    // 4. Convert status and priority to their string representations
    // 5. INSERT into tasks table
    // 6. Construct and return the Task with StatusCode::CREATED
    todo!("implement create_task handler")
}

async fn get_task(
    State(_state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(_id): Path<String>,
) -> Result<Json<Task>, AppError> {
    // TODO: Get a single task by ID
    // 1. Query SELECT ... FROM tasks WHERE id = ? AND user_id = ?
    // 2. Use fetch_optional; return Ok(Json(task)) if found, Err(AppError::NotFound) otherwise
    todo!("implement get_task handler")
}

async fn update_task(
    State(_state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(_id): Path<String>,
    Json(_payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    // TODO: Update an existing task
    // 1. Fetch the existing task (check ownership via user_id)
    // 2. Merge the update fields (title, description, status, priority)
    // 3. Get the current timestamp for updated_at
    // 4. UPDATE the task in the database
    // 5. Return the updated Task
    todo!("implement update_task handler")
}

async fn delete_task(
    State(_state): State<Arc<AppState>>,
    _auth: AuthUser,
    Path(_id): Path<String>,
) -> Result<StatusCode, AppError> {
    // TODO: Delete a task
    // 1. DELETE FROM tasks WHERE id = ? AND user_id = ?
    // 2. Check rows_affected(); return NO_CONTENT if deleted, NOT_FOUND otherwise
    todo!("implement delete_task handler")
}

// ---------------------------------------------------------------------------
// Router builder
// ---------------------------------------------------------------------------

pub async fn build_app(pool: SqlitePool, jwt_secret: String) -> Router {
    run_migrations(&pool).await;

    let state = Arc::new(AppState {
        db: pool,
        jwt_secret,
    });

    Router::new()
        .route("/api/health", get(health_check))
        .route("/api/auth/register", post(register))
        .route("/api/auth/login", post(login))
        .route("/api/tasks", get(list_tasks).post(create_task))
        .route(
            "/api/tasks/:id",
            get(get_task).put(update_task).delete(delete_task),
        )
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
