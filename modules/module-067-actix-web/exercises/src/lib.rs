//! Module 067: Actix-web — exercise scaffold.
//!
//! Implement the handlers and wire the app.
//! Find the `// TODO(module-067)` comments below and fill them in until
//! `cargo test -p module-067-exercises` passes.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::atomic::AtomicUsize;

/// Shared application state. `web::Data` wraps this in an `Arc`, so it
/// doesn't need to be `Clone`.
pub struct AppState {
    pub counter: AtomicUsize,
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

/// A simple handler that returns a greeting.
pub async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix-web!")
}

/// A handler that greets by name. `web::Path<String>` extracts the
/// `{name}` capture from the URL.
pub async fn hello_name(_path: web::Path<String>) -> HttpResponse {
    // TODO(module-067): Extract the name from the path and return
    // "Hello, {name}!" as the response body.
    panic!("not implemented: hello_name")
}

/// A handler that creates an item. `web::Data<AppState>` extracts the
/// shared state; `web::Json<NewItem>` extracts the JSON body.
pub async fn create_item(_data: web::Data<AppState>, _item: web::Json<NewItem>) -> HttpResponse {
    // TODO(module-067): Assign the next id from the shared counter
    // (`counter.fetch_add(1, Ordering::SeqCst) + 1`) and respond with
    // `HttpResponse::Created().json(CreatedItem { id, name })`.
    panic!("not implemented: create_item")
}

/// A handler that returns the current counter value.
pub async fn item_count(_data: web::Data<AppState>) -> HttpResponse {
    // TODO(module-067): Return the current counter value as the body.
    panic!("not implemented: item_count")
}

/// Configure the actix-web app with routes.
pub fn configure_app(_cfg: &mut web::ServiceConfig) {
    // TODO(module-067): Configure the app with:
    // - GET / -> hello
    // - GET /hello/{name} -> hello_name
    // - POST /items -> create_item
    // - GET /items/count -> item_count
    panic!("not implemented: configure_app")
}
