# Module 067: Actix-web

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 062 (Axum fundamentals — you need to understand routing, handlers, and extractors), Module 066 (middleware — actix-web has its own middleware model).

## Learning Objectives

- Build an actix-web service with routing, handlers, and extractors.
- Understand how actix-web's actor-model heritage shapes its API.
- Compare actix-web and axum honestly: extraction, middleware, state management, ecosystem.
- Write integration tests for an actix-web service using `actix_web::test`.
- Decide when to reach for actix-web vs. axum in a real project.

## Why This Matters

Actix-web is one of Rust's oldest and most battle-tested web frameworks. It powers high-traffic production services and has a mature ecosystem. Understanding actix-web gives you a second perspective on Rust web development — and the differences between it and axum illuminate design tradeoffs you'll encounter in any framework choice. Many Rust job postings mention actix-web; knowing both frameworks makes you more versatile.

## Concept

### Actix-web's heritage: the actor model

Actix-web is built on top of `actix`, an actor framework. An *actor* is a computational unit that runs in its own task, communicates via message passing, and maintains its own state. You don't interact with actors directly in typical actix-web code — the framework abstracts them away — but the heritage shows up in:

- **`HttpServer::new(factory)`**: the factory is a closure that returns a fresh `App` for each worker thread. Each worker is an actor.
- **State is per-worker by default**: `App::app_data()` injects data that's cloned per worker, not shared globally. For shared state, you wrap it in `Arc` or use `actix_web::web::Data` (which is an `Arc` under the hood).
- **Middleware is actor-based**: actix-web middleware wraps the actor message flow, not a simple request-response function.

This is different from axum, which is built on Tower (a service-composition library). Axum's model is "a handler is a function"; actix-web's model is "a handler runs inside an actor".

### Routing and handlers

Actix-web's routing looks similar to axum's, but the details differ:

```rust,ignore
use actix_web::{web, App, HttpServer, HttpResponse};

async fn hello() -> HttpResponse {
    HttpResponse::Ok().body("Hello from actix-web!")
}

async fn hello_name(path: web::Path<String>) -> HttpResponse {
    let name = path.into_inner();
    HttpResponse::Ok().body(format!("Hello, {name}!"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/hello/{name}", web::get().to(hello_name))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Key differences from axum:
- Handlers return `HttpResponse` (or `impl Responder`), not `impl IntoResponse`.
- Path captures are extracted via `web::Path<T>`, similar to axum's `Path<T>`.
- Routes are registered with `.route(path, method().to(handler))`, not `.route(path, method(handler))`.

### Extractors

Actix-web extractors implement the `FromRequest` trait. They're declared as handler arguments, just like axum:

| Extractor | Extracts |
|---|---|
| `web::Path<T>` | URL path segments matched by `{...}` captures |
| `web::Query<T>` | The `?key=value` query string, deserialized into `T` |
| `web::Json<T>` | The request body, deserialized into `T` |
| `web::Data<T>` | Shared application state (wraps `Arc<T>`) |
| `HttpRequest` | The raw request (for headers, etc.) |

The key difference: actix-web extractors are *async* by default. `FromRequest::from_request` returns a `Future`. This means extraction can do async work (e.g., reading the body, checking a database). Axum's extractors are split into `FromRequestParts` (sync, for the request head) and `FromRequest` (async, for the body).

### Shared state

Actix-web's `web::Data<T>` is the idiomatic way to share state. It's an `Arc<T>` under the hood, so `T` doesn't need to be `Clone`:

```rust,ignore
use actix_web::{web, App, HttpServer};
use std::sync::atomic::{AtomicUsize, Ordering};

struct AppState {
    counter: AtomicUsize,
}

async fn increment(data: web::Data<AppState>) -> String {
    let id = data.counter.fetch_add(1, Ordering::SeqCst) + 1;
    format!("ID: {id}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let data = web::Data::new(AppState {
        counter: AtomicUsize::new(0),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .route("/increment", web::get().to(increment))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

Note the `move` closure in `HttpServer::new(move || ...)`. The `data.clone()` clones the `Arc`, not the inner `AppState`. Each worker gets a handle to the same shared state.

### Middleware

Actix-web middleware implements the `Middleware` trait (or `Transform` for more control). The framework provides built-in middleware for logging, CORS, compression, and sessions. Using them looks like:

```rust,ignore
use actix_web::middleware::{Logger, DefaultHeaders};
use actix_cors::Cors;

App::new()
    .wrap(Logger::default())
    .wrap(DefaultHeaders::new().add(("x-request-id", "req-001")))
    .wrap(Cors::permissive())
```

`.wrap()` adds middleware. The order matters: the last `.wrap()` is the outermost (runs first on request, last on response), same as axum's `.layer()`.

### Testing with `actix_web::test`

Actix-web provides a test utilities module for integration testing without sockets:

```rust,ignore
use actix_web::{test, web, App};

#[actix_web::test]
async fn test_hello() {
    let app = test::init_service(
        App::new().route("/", web::get().to(|| async { "Hello!" }))
    ).await;

    let req = test::TestRequest::get().uri("/").to_request();
    let resp = test::call_service(&app, req).await;
    assert!(resp.status().is_success());
}
```

`test::init_service` builds the app; `test::call_service` sends a request and returns a response. No port binding, no network. This is actix-web's equivalent of axum's `oneshot`.

### Actix-web vs. Axum: an honest comparison

| Aspect | Axum | Actix-web |
|---|---|---|
| **Foundation** | Tower (service composition) | Actix (actor model) |
| **Handler signature** | `async fn` returning `impl IntoResponse` | `async fn` returning `HttpResponse` or `impl Responder` |
| **State sharing** | `Router::with_state(T)` requires `T: Clone` | `web::Data<T>` wraps `Arc<T>`, no `Clone` needed |
| **Middleware** | Tower layers (uniform, composable) | Actix middleware (actor-based, more complex) |
| **Ecosystem** | Tower ecosystem (shared with hyper, tonic) | Actix ecosystem (actix-rt, actix-files, etc.) |
| **Async runtime** | Tokio (required) | Tokio (default) or actix-rt (actix's own runtime) |
| **Maturity** | Newer (2021), rapidly evolving | Older (2017), stable API |
| **Performance** | Excellent (Tower overhead is minimal) | Excellent (actor model is efficient) |
| **Learning curve** | Lower (simpler model) | Higher (actor concepts, more abstractions) |

**When to choose axum:**
- You're already using Tower-based crates (hyper, tonic)
- You want a simpler mental model
- You prefer the Tower ecosystem's composability

**When to choose actix-web:**
- You need a battle-tested, stable API
- You're working with existing actix-based code
- You want per-worker state isolation (actor model benefits)
- You need actix-specific features (e.g., actix-files for static file serving)

Both frameworks are excellent. The Rust web ecosystem is small enough that you'll likely encounter both in production.

### The request lifecycle in actix-web

```
TCP connection ──► actix-web HttpServer
                     │
                     ▼
                  Worker actor (one per CPU core)
                     │
                     ▼
              ┌──── Middleware stack ────┐
              │  Logger                 │  ← logs request
              │  Cors                   │  ← handles preflight
              │  DefaultHeaders         │  ← adds response headers
              └─────────────────────────┘
                     │
                     ▼
                  Router (path matching)
                     │
                     ▼
                  Extractors (FromRequest)
                     │
                     ▼
                  Handler (your async fn)
                     │
                     ▼
              ┌──── Middleware stack ────┐  (reverse order)
              │  DefaultHeaders         │  ← (pass-through)
              │  Cors                   │  ← adds CORS headers
              │  Logger                 │  ← logs response
              └─────────────────────────┘
                     │
                     ▼
                  HttpResponse ──► TCP response
```

The key difference from axum: each worker is an actor with its own state. Requests are messages sent to the actor. Middleware wraps the message-handling flow, not just the request-response function.

## Common Pitfalls

- **Forgetting `move` in `HttpServer::new`.** If you capture `web::Data` in the factory closure, you need `move ||` to move the `Arc` into each worker. Without `move`, you get a compile error about borrowed values.
- **Non-`Send` state in handlers.** Actix-web handlers run on a single-threaded runtime by default (actix-rt). If you use `#[actix_web::main]`, you're on actix-rt. If you use `#[tokio::main]`, you're on Tokio. Mixing them causes issues. Stick to one runtime.
- **`web::Data` vs. `app_data`.** `web::Data::new(T)` creates an `Arc<T>`. `.app_data(data.clone())` registers it with the app. Forgetting `.app_data()` means handlers can't extract it.
- **Middleware order.** `.wrap(A).wrap(B)` means B is outermost. If your CORS middleware is inside your auth middleware, preflight `OPTIONS` requests hit the auth check and fail.
- **Blocking the actor.** Actix-web workers are actors running on a single-threaded runtime. Blocking I/O (e.g., `std::fs::read`) blocks the whole worker. Use `web::block(|| ...)` for blocking operations.

## Key Terms

- **Actor:** a computational unit that runs in its own task, communicates via message passing, and maintains its own state. Actix-web's heritage.
- **`HttpServer`:** the top-level type that binds to a port and spawns worker actors.
- **`App`:** the application builder — registers routes, middleware, and state.
- **`web::Data<T>`:** shared application state, wrapped in an `Arc<T>`.
- **`FromRequest`:** the trait for extractors — types that can be constructed from a request.
- **`HttpResponse`:** the response type returned by handlers.
- **`test::init_service`:** builds an actix-web app for testing without sockets.
- **`test::call_service`:** sends a request to a test app and returns the response.

## Exercise

In `exercises/`, the actix-web service is scaffolded with routes registered but handlers stubbed with `panic!` and marked `// TODO(module-067)`:

1. **`hello`** — return a greeting with the name from the path capture.
2. **`create_item`** — extract the JSON body, assign an id from the shared counter, return `201 Created` with the item.
3. **`item_count`** — return the current counter value.
4. **Wire the app** with `Logger` middleware and a custom header middleware.

The tests use `actix_web::test` — no sockets. Run:

```bash
cargo test -p module-067-exercises
```

When all tests pass, compare with `solutions/`.

## Further Reading

- [Actix-web documentation](https://docs.rs/actix-web)
- [Actix-web book](https://actix.rs/docs/)
- [Actix vs. Axum comparison (blog)](https://www.lpalmieri.com/posts/actix-web-vs-axum/)
- [Module 062: Axum Fundamentals — the other side of the comparison](modules/module-062-axum-fundamentals/README.md)
