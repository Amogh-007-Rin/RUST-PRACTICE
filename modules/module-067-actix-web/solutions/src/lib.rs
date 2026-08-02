//! Module 067: Actix-web — reference solution.
//!
//! A small actix-web REST service with handlers, extractors, and shared state.

use actix_web::{web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicUsize, Ordering};

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
pub async fn hello_name(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {name}!"))
}

/// A handler that creates an item. `web::Data<AppState>` extracts the
/// shared state; `web::Json<NewItem>` extracts the JSON body.
pub async fn create_item(data: web::Data<AppState>, item: web::Json<NewItem>) -> HttpResponse {
    let id = data.counter.fetch_add(1, Ordering::SeqCst) + 1;
    HttpResponse::Created().json(CreatedItem {
        id,
        name: item.into_inner().name,
    })
}

/// A handler that returns the current counter value.
pub async fn item_count(data: web::Data<AppState>) -> HttpResponse {
    let count = data.counter.load(Ordering::SeqCst);
    HttpResponse::Ok().body(count.to_string())
}

/// Configure the actix-web app with routes and shared state.
pub fn configure_app(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(hello))
        .route("/hello/{name}", web::get().to(hello_name))
        .route("/items", web::post().to(create_item))
        .route("/items/count", web::get().to(item_count));
}
