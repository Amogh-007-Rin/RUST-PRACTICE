//! Module 065: Authentication & authorization — exercise scaffold.
//!
//! The HTTP layer (routes, handlers, state) is complete. Your job is the
//! security core: argon2 password hashing, JWT signing/verification, and
//! the `AuthUser` extractor that guards protected routes.
//!
//! Find the `// TODO(module-065)` comments below and fill them in until
//! `cargo test -p module-065-exercises` passes.

#[allow(unused_imports)]
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
#[allow(unused_imports)]
use axum::http::header::AUTHORIZATION;
use axum::{
    extract::{FromRequestParts, State},
    http::{request::Parts, StatusCode},
    routing::{get, post},
    Json, Router,
};
#[allow(unused_imports)]
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[allow(unused_imports)]
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
        _parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        // TODO(module-065): Read the `Authorization` header, strip the
        // `Bearer ` prefix, verify the token with `verify_token` against
        // `state.jwt_secret`, and return `Ok(AuthUser(claims))`. Reject
        // with `StatusCode::UNAUTHORIZED` and a short message if the header
        // is missing, malformed, or the token is invalid.
        panic!("not implemented: AuthUser extractor")
    }
}

/// Hashes a password with argon2id and a fresh random salt. The returned
/// string contains everything needed to verify later: algorithm, salt,
/// and hash — store it as-is.
pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // TODO(module-065): Generate a salt with
    // `SaltString::generate(&mut OsRng)`, hash with
    // `Argon2::default().hash_password(password.as_bytes(), &salt)?`
    // and return the `.to_string()` of the result.
    panic!("not implemented: hash_password({password:?})")
}

/// Verifies a password against a stored hash. Never fails loudly: a
/// malformed hash simply verifies as false.
pub fn verify_password(password: &str, stored_hash: &str) -> bool {
    // TODO(module-065): Parse the stored hash with `PasswordHash::new`,
    // then `Argon2::default().verify_password(password.as_bytes(), &parsed)`
    // and report whether it succeeded.
    panic!("not implemented: verify_password({password:?}, {stored_hash:?})")
}

/// Signs a token for `username` valid for `TOKEN_TTL_SECONDS`.
pub fn issue_token(_secret: &str, username: &str) -> Result<String, jsonwebtoken::errors::Error> {
    // TODO(module-065): Build `Claims { sub: username, exp: now + TOKEN_TTL_SECONDS }`
    // (unix seconds via `SystemTime::now().duration_since(UNIX_EPOCH)`),
    // then `encode(&Header::default(), &claims,
    // &EncodingKey::from_secret(secret.as_bytes()))`.
    panic!("not implemented: issue_token(secret, username={username:?})")
}

/// Verifies a token's signature and expiry, returning its claims.
pub fn verify_token(_secret: &str, token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    // TODO(module-065): `decode::<Claims>(token,
    // &DecodingKey::from_secret(secret.as_bytes()),
    // &Validation::new(Algorithm::HS256))` and return `.claims`.
    panic!("not implemented: verify_token(secret, token={token:?})")
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
