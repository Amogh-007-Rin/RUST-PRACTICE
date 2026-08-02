//! Module 062: Axum fundamentals — exercise scaffold.
//!
//! The routes are wired up and the simplest handlers work. Your job is to
//! implement the handlers that use extractors: `search` (Query),
//! `create_item` (Json + State) and `item_count` (State).
//!
//! Find the `// TODO(module-062)` comments below and fill them in until
//! `cargo test -p module-062-exercises` passes.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::{atomic::AtomicUsize, Arc};

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
    // TODO(module-062): Return a response that reports the query parameters
    // as "q/limit". The extractor is already declared in the signature; you
    // just need to use `params.q` and `params.limit`.
    panic!("not implemented: search({params:?})")
}

/// `State<AppState>` gives access to shared state; `Json<T>` gives the
/// deserialized JSON body.
async fn create_item(
    State(state): State<AppState>,
    Json(item): Json<NewItem>,
) -> (StatusCode, Json<CreatedItem>) {
    // TODO(module-062): Assign the next id from the shared counter
    // (`counter.fetch_add(1, Ordering::SeqCst) + 1`) and respond with
    // `StatusCode::CREATED` and the created item in JSON.
    panic!("not implemented: create_item(state={state:?}, item={item:?})")
}

async fn item_count(State(state): State<AppState>) -> String {
    // TODO(module-062): Return the current value of the shared counter as a
    // string (see `create_item` — it increments this same counter).
    panic!("not implemented: item_count(state={state:?})")
}
