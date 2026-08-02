//! Module 065: Authentication & authorization — argon2 password hashing,
//! HS256 JWTs, and a protected route.

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::{FromRequestParts, State},
    http::{header::AUTHORIZATION, request::Parts, StatusCode},
    routing::{get, post},
    Json, Router,
};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// How long an issued token stays valid, in seconds.
pub const TOKEN_TTL_SECONDS: u64 = 3600;

/// A registered user: the username plus the argon2 password hash.
#[derive(Clone)]
pub struct StoredUser {
    pub username: String,
    pub password_hash: String,
}

/// Claims embedded in the JWT. `sub` is the username; `exp` is the unix
/// timestamp after which the token must be rejected.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: u64,
}

/// The JSON body for `/register` and `/login`.
#[derive(Debug, Deserialize)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

/// The response to a successful register/login.
#[derive(Serialize)]
pub struct AuthResponse {
    pub token: String,
    pub username: String,
}

/// Shared state: the (in-memory) user store and the JWT signing secret.
#[derive(Clone)]
pub struct AppState {
    pub users: Arc<Mutex<HashMap<String, StoredUser>>>,
    pub jwt_secret: String,
}

impl AppState {
    pub fn new(jwt_secret: impl Into<String>) -> Self {
        Self {
            users: Arc::new(Mutex::new(HashMap::new())),
            jwt_secret: jwt_secret.into(),
        }
    }
}

/// Extractor for protected routes: requires `Authorization: Bearer <token>`
/// and yields the token's claims.
pub struct AuthUser(pub Claims);

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = (StatusCode, String);

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or((
                StatusCode::UNAUTHORIZED,
                "missing Authorization header".to_string(),
            ))?;
        let token = header.strip_prefix("Bearer ").ok_or((
            StatusCode::UNAUTHORIZED,
            "expected `Bearer <token>`".to_string(),
        ))?;
        let claims = verify_token(&state.jwt_secret, token).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                "invalid or expired token".to_string(),
            )
        })?;
        Ok(AuthUser(claims))
    }
}

/// Hashes a password with argon2id and a fresh random salt. The returned
/// string contains everything needed to verify later: algorithm, salt,
/// and hash — store it as-is.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt = SaltString::generate(&mut OsRng);
    Ok(Argon2::default()
        .hash_password(password.as_bytes(), &salt)?
        .to_string())
}

/// Verifies a password against a stored hash. Never fails loudly: a
/// malformed hash simply verifies as false.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    let Ok(parsed) = PasswordHash::new(stored_hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

/// Signs a token for `username` valid for `TOKEN_TTL_SECONDS`.
pub fn issue_token(secret: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0);
    let claims = Claims {
        sub: username.to_string(),
        exp: now + TOKEN_TTL_SECONDS,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
}

/// Verifies a token's signature and expiry, returning its claims.
pub fn verify_token(secret: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )?;
    Ok(data.claims)
}

/// Assembles the application: public auth routes plus one protected route.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .route("/me", get(me))
        .with_state(state)
}

pub async fn register(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    if credentials.username.trim().is_empty() || credentials.password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "username and password are required".to_string(),
        ));
    }
    let mut users = state.users.lock().unwrap();
    if users.contains_key(&credentials.username) {
        return Err((StatusCode::CONFLICT, "username already taken".to_string()));
    }
    let password_hash = hash_password(&credentials.password).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "password hashing failed".to_string(),
        )
    })?;
    let token = issue_token(&state.jwt_secret, &credentials.username).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "token issuance failed".to_string(),
        )
    })?;
    users.insert(
        credentials.username.clone(),
        StoredUser {
            username: credentials.username.clone(),
            password_hash,
        },
    );
    Ok((
        StatusCode::CREATED,
        Json(AuthResponse {
            token,
            username: credentials.username,
        }),
    ))
}

pub async fn login(
    State(state): State<AppState>,
    Json(credentials): Json<Credentials>,
) -> Result<Json<AuthResponse>, (StatusCode, String)> {
    let users = state.users.lock().unwrap();
    let stored = users
        .get(&credentials.username)
        .ok_or((StatusCode::UNAUTHORIZED, "invalid credentials".to_string()))?;
    if !verify_password(&credentials.password, &stored.password_hash) {
        return Err((StatusCode::UNAUTHORIZED, "invalid credentials".to_string()));
    }
    let token = issue_token(&state.jwt_secret, &credentials.username).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "token issuance failed".to_string(),
        )
    })?;
    Ok(Json(AuthResponse {
        token,
        username: credentials.username,
    }))
}

/// Protected route: only reachable with a valid `Bearer` token.
pub async fn me(AuthUser(claims): AuthUser) -> String {
    claims.sub
}
