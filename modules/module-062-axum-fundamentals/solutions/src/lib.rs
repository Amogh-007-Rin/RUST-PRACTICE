//! Module 062: Axum fundamentals — routing, handlers, extractors, shared state.
//!
//! Everything here compiles down to the raw HTTP dance from Module 061;
//! axum just removes the tedium. Handlers are plain async functions, and
//! their arguments (the *extractors*) pull structured data out of the raw
//! request.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

/// Shared application state. `Router` state must be `Clone`, so cheaply
/// clonable shared handles (here: an `Arc` around an atomic counter) are
/// the idiomatic choice.
#[derive(Clone, Debug, Default)]
pub struct AppState {
    pub counter: Arc<AtomicUsize>,
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }
}

/// A freshly created item, echoed back with the server-assigned id.
#[derive(Serialize)]
pub struct CreatedItem {
    pub id: usize,
    pub name: String,
}

/// The JSON body a client sends to create an item.
#[derive(Deserialize, Debug)]
pub struct NewItem {
    pub name: String,
}

/// Query-string parameters for `/search`. Fields with `#[serde(default)]`
/// are optional; missing keys become their default value instead of a
/// deserialization error.
#[derive(Deserialize, Debug)]
pub struct SearchParams {
    #[serde(default)]
    pub q: String,
    #[serde(default = "default_limit")]
    pub limit: usize,
}

fn default_limit() -> usize {
    10
}

/// Assembles the application: routes, handlers, and shared state.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(root))
        .route("/hello/{name}", get(hello))
        .route("/search", get(search))
        .route("/items", post(create_item))
        .route("/items/count", get(item_count))
        .with_state(state)
}

async fn root() -> &'static str {
    "Hello from Axum!"
}

/// `Path<String>` captures the `{name}` segment from the URL.
async fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {name}!")
}

/// `Query<T>` deserializes the query string (`?q=rust&limit=5`) into `T`.
async fn search(Query(params): Query<SearchParams>) -> String {
    format!("{}/{}", params.q, params.limit)
}

/// `State<AppState>` gives access to shared state; `Json<T>` gives the
/// deserialized JSON body.
async fn create_item(
    State(state): State<AppState>,
    Json(item): Json<NewItem>,
) -> (StatusCode, Json<CreatedItem>) {
    let id = state.counter.fetch_add(1, Ordering::SeqCst) + 1;
    (
        StatusCode::CREATED,
        Json(CreatedItem {
            id,
            name: item.name,
        }),
    )
}

async fn item_count(State(state): State<AppState>) -> String {
    state.counter.load(Ordering::SeqCst).to_string()
}
