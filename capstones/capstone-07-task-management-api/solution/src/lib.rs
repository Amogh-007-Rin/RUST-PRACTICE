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
struct User {
    id: String,
    username: String,
    password_hash: String,
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
struct ErrorBody {
    error: String,
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
        state: &Arc<AppState>,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized)?;

        let token_data = decode::<Claims>(
            token,
            &DecodingKey::from_secret(state.jwt_secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|_| AppError::Unauthorized)?;

        Ok(AuthUser {
            user_id: token_data.claims.sub,
        })
    }
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn now_iso() -> String {
    humantime::format_rfc3339_seconds(SystemTime::now()).to_string()
}

fn make_token(user_id: &str, secret: &str) -> Result<String, AppError> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        exp: now + 86_400,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(e.into()))
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
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    if payload.username.trim().is_empty() {
        return Err(AppError::BadRequest("username required".into()));
    }
    if payload.password.len() < 6 {
        return Err(AppError::BadRequest("password too short".into()));
    }

    let existing = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users WHERE username = ?")
        .bind(&payload.username)
        .fetch_one(&state.db)
        .await?;

    if existing > 0 {
        return Err(AppError::Conflict("username taken".into()));
    }

    let salt = SaltString::generate(&mut rand_core::OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(payload.password.as_bytes(), &salt)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("hash error: {e}")))?
        .to_string();

    let user_id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();

    sqlx::query("INSERT INTO users (id, username, password_hash, created_at) VALUES (?, ?, ?, ?)")
        .bind(&user_id)
        .bind(&payload.username)
        .bind(&password_hash)
        .bind(&now)
        .execute(&state.db)
        .await?;

    let token = make_token(&user_id, &state.jwt_secret)?;
    info!(username = %payload.username, "user registered");
    Ok(Json(AuthResponse { token }))
}

async fn login(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query_as::<_, User>(
        "SELECT id, username, password_hash FROM users WHERE username = ?",
    )
    .bind(&payload.username)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::Unauthorized)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)
        .map_err(|e| AppError::Internal(anyhow::anyhow!("parse hash error: {e}")))?;
    Argon2::default()
        .verify_password(payload.password.as_bytes(), &parsed_hash)
        .map_err(|_| AppError::Unauthorized)?;

    let token = make_token(&user.id, &state.jwt_secret)?;
    info!(username = %payload.username, "user logged in");
    Ok(Json(AuthResponse { token }))
}

async fn list_tasks(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Query(filter): Query<TaskFilter>,
) -> Result<Json<Vec<Task>>, AppError> {
    let mut query = String::from("SELECT id, user_id, title, description, status, priority, created_at, updated_at FROM tasks WHERE user_id = ?");
    let mut params: Vec<String> = vec![auth.user_id.clone()];

    if let Some(ref status) = filter.status {
        let status_str = task_status_to_str(status);
        params.push(status_str.to_string());
        query.push_str(" AND status = ?");
    }
    if let Some(ref priority) = filter.priority {
        let priority_str = priority_to_str(priority);
        params.push(priority_str.to_string());
        query.push_str(" AND priority = ?");
    }
    query.push_str(" ORDER BY created_at DESC");

    let mut q = sqlx::query_as::<_, Task>(&query);
    for param in &params {
        q = q.bind(param);
    }
    let tasks = q.fetch_all(&state.db).await.map_err(AppError::from)?;

    Ok(Json(tasks))
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
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Json(payload): Json<CreateTaskRequest>,
) -> Result<(StatusCode, Json<Task>), AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("title required".into()));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let now = now_iso();
    let status_str = task_status_to_str(&payload.status);
    let priority_str = priority_to_str(&payload.priority);

    sqlx::query(
        "INSERT INTO tasks (id, user_id, title, description, status, priority, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&id)
    .bind(&auth.user_id)
    .bind(&payload.title)
    .bind(&payload.description)
    .bind(status_str)
    .bind(priority_str)
    .bind(&now)
    .bind(&now)
    .execute(&state.db)
    .await?;

    let task = Task {
        id,
        user_id: auth.user_id,
        title: payload.title,
        description: payload.description,
        status: payload.status,
        priority: payload.priority,
        created_at: now.clone(),
        updated_at: now,
    };

    info!(task_id = %task.id, "task created");
    Ok((StatusCode::CREATED, Json(task)))
}

async fn get_task(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
) -> Result<Json<Task>, AppError> {
    let task = sqlx::query_as::<_, Task>(
        "SELECT id, user_id, title, description, status, priority, created_at, updated_at FROM tasks WHERE id = ? AND user_id = ?",
    )
    .bind(&id)
    .bind(&auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    Ok(Json(task))
}

async fn update_task(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
    Json(payload): Json<UpdateTaskRequest>,
) -> Result<Json<Task>, AppError> {
    let existing = sqlx::query_as::<_, Task>(
        "SELECT id, user_id, title, description, status, priority, created_at, updated_at FROM tasks WHERE id = ? AND user_id = ?",
    )
    .bind(&id)
    .bind(&auth.user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or(AppError::NotFound)?;

    let title = payload.title.unwrap_or(existing.title);
    let description = payload.description.unwrap_or(existing.description);
    let status = payload.status.unwrap_or(existing.status);
    let priority = payload.priority.unwrap_or(existing.priority);
    let now = now_iso();

    let status_str = task_status_to_str(&status);
    let priority_str = priority_to_str(&priority);

    sqlx::query(
        "UPDATE tasks SET title = ?, description = ?, status = ?, priority = ?, updated_at = ? WHERE id = ? AND user_id = ?",
    )
    .bind(&title)
    .bind(&description)
    .bind(status_str)
    .bind(priority_str)
    .bind(&now)
    .bind(&id)
    .bind(&auth.user_id)
    .execute(&state.db)
    .await?;

    let task = Task {
        id,
        user_id: auth.user_id,
        title,
        description,
        status,
        priority,
        created_at: existing.created_at,
        updated_at: now,
    };

    info!(task_id = %task.id, "task updated");
    Ok(Json(task))
}

async fn delete_task(
    State(state): State<Arc<AppState>>,
    auth: AuthUser,
    Path(id): Path<String>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM tasks WHERE id = ? AND user_id = ?")
        .bind(&id)
        .bind(&auth.user_id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    info!(task_id = %id, "task deleted");
    Ok(StatusCode::NO_CONTENT)
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
