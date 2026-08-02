# Module 068: Testing Web Services

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 063 (CRUD with axum), Module 064 (database operations with a trait/abstraction). This module generalizes the test helpers you've been using.

## Learning Objectives

- Write reusable test helper functions for HTTP services using `tower::ServiceExt::oneshot`.
- Define a database access trait and swap the production implementation for an in-memory mock in tests.
- Build an `InMemoryStore` behind an `async_trait`-powered interface using `tokio::sync::Mutex`.
- Structure tests so each test gets a clean, isolated state with no need for a running database.

## Why This Matters

In Modules 062–065, you tested routers by manually constructing `Request<Body>` and calling `oneshot` inside every test. That works, but it doesn't scale — as your API grows, every test file repeats the same 15 lines of boilerplate. This module teaches you to extract those helpers into reusable functions (`send_request`) and to decouple your handlers from your database implementation with a trait, so you can run hundreds of tests in milliseconds against an in-memory store instead of seconds against a real database. This is the pattern production Rust backends use for their test suites.

## Concept

### The test pyramid

Good test suites are shaped like a pyramid: many fast unit/integration tests at the bottom, fewer slower end-to-end tests at the top. In Rust web services:

```
                    ┌─────────┐
                    │   E2E   │  Real DB, real HTTP (few)
                    ├─────────┤
                    │   API   │  In-memory store, oneshot (many)
                    ├─────────┤
                    │  Unit   │  Pure functions, no I/O (most)
                    └─────────┘
```

For the "API" layer — the majority of your tests — you want the full request → router → handler → response pipeline, but with no database server and no socket. The two tools for this are `oneshot` (you've used it) and a mock store behind a trait (you're building it here).

### Test helper functions

Every test in earlier modules did this dance:

```rust
let response = app.clone()
    .oneshot(Request::builder().uri("/todos").body(Body::empty()).unwrap())
    .await.unwrap();
let status = response.status();
let bytes = to_bytes(response.into_body(), usize::MAX).await.unwrap();
let body: Value = serde_json::from_slice(&bytes).unwrap();
```

That's six lines for every request. With a `send_request` helper, you collapse it to one:

```rust
let (status, body) = send_request(&app, "GET", "/todos", None).await;
```

The helper encapsulates the entire pipeline:
1. Build a `Request<Body>` — with optional JSON body and `content-type` header.
2. Call `oneshot` to get a `Response`.
3. Extract status code and parse body as `serde_json::Value`.

The key insight: `Router` implements `tower::Service`, so `app.clone().oneshot(req)` runs the full routing + handler pipeline without a socket. The helper is just a convenience wrapper — the pipeline underneath is exactly the same code that runs in production.

### Dependency injection via traits

The production database is a real `SqlitePool` or `PgPool`. Tests can't use it — they'd need a running server, they'd interfere with each other, and they'd be slow. The solution: define the database dependency as a trait, implement it differently for prod vs. test:

```
┌──────────────────────────────────────────┐
│  Handler: create_todo(state.store, ...)  │
│                                          │
│  state.store: impl TodoStore             │
│                                          │
│  ┌─────────────┐    ┌───────────────┐   │
│  │SqlTodoStore  │    │InMemoryStore   │   │
│  │(real sqlx)   │    │(HashMap+tokio) │   │
│  │  Prod only   │    │  Tests only    │   │
│  └─────────────┘    └───────────────┘   │
└──────────────────────────────────────────┘
```

The trait defines the contract:

```rust
#[async_trait]
pub trait TodoStore: Clone + Send + Sync + 'static {
    async fn create_todo(&self, title: String) -> Todo;
    async fn list_todos(&self) -> Vec<Todo>;
    async fn get_todo(&self, id: u64) -> Option<Todo>;
    async fn delete_todo(&self, id: u64) -> bool;
}
```

The bounds `Clone + Send + Sync + 'static` are deliberate: axum's `State` requires `Clone`, and tokio tasks require `Send + Sync`. The `#[async_trait]` macro from the `async-trait` crate is necessary because Rust's native async fn in traits (from edition 2021) still has restrictions — `async-trait` desugars each method into a `Pin<Box<dyn Future>>` return type, which works everywhere.

The `AppState` becomes generic over the store type:

```rust
pub struct AppState<S: TodoStore> {
    pub store: S,
}
```

A test injects `InMemoryStore`; production injects `SqlTodoStore`. The handler code is identical — it only ever calls methods on the trait.

### Building the in-memory store

`InMemoryStore` holds a `HashMap<u64, Todo>` behind a `tokio::sync::Mutex`:

```rust
pub struct InMemoryStore {
    todos: Arc<Mutex<HashMap<u64, Todo>>>,
    next_id: Arc<Mutex<u64>>,
}
```

Why `tokio::sync::Mutex` instead of `std::sync::Mutex`? Because the trait methods are `async`, and holding a `std::sync::Mutex` guard across an `.await` point is a recipe for deadlock under contention. Tokio's `Mutex` is designed for async — you can await while holding the guard safely.

Why `Arc<Mutex<...>>`? Because `InMemoryStore` must implement `Clone` (for `with_state`), and `Mutex` is not `Clone` on its own. Wrapping in `Arc` gives cheap, shared-ownership clones.

The `create_todo` implementation shows the pattern:

```rust
async fn create_todo(&self, title: String) -> Todo {
    let mut next_id = self.next_id.lock().await;
    let id = *next_id;
    *next_id += 1;
    drop(next_id);  // release the lock before inserting
    let todo = Todo { id, title, completed: false };
    self.todos.lock().await.insert(id, todo.clone());
    todo
}
```

Notice `drop(next_id)` — we lock the id counter only long enough to get and increment the next id, then release before acquiring the `todos` lock. This avoids holding two locks simultaneously and keeps the critical section short.

### What you're testing

The tests verify that the mock store and the test helpers work together correctly. This is meta-testing — you're testing the test infrastructure itself. Once it works, you'd use it to test real handlers:

```rust
let app = test_app(InMemoryStore::new());
// seed data
send_request(&app, "POST", "/todos", some_data).await;
// now test handler behavior
let (status, body) = send_request(&app, "GET", "/todos", None).await;
assert_eq!(status, StatusCode::OK);
```

Each test creates its own `InMemoryStore::new()`, so tests are isolated — no shared state between tests, no cleanup needed. This is a huge advantage over testing against a real database, where you'd need `DELETE FROM todos` between tests and worry about transaction rollbacks.

### The full pattern, end to end

```
test_function()
    │
    ▼
store = InMemoryStore::new()
    │
    ▼
app = test_app(store)           // Router with trait-bound handlers
    │
    ▼
(status, json) = send_request(&app, method, path, body).await
    │                     │
    │                     ▼
    │              app.oneshot(Request { method, uri, body })
    │                     │
    │                     ▼
    │              Router matches route → handler runs
    │                     │
    │                     ▼
    │              handler calls state.store.create_todo(...)
    │                     │          (InMemoryStore impl)
    │                     ▼
    │              HashMap insert → Todo returned
    │                     │
    │                     ▼
    │              Response { status: 201, body: Json(todo) }
    │                     │
    ▼                     ▼
assert status == 201, body.id == 1
```

## Common Pitfalls

- **Using `std::sync::Mutex` across `.await` points.** If you hold a `std::sync::MutexGuard` and then `.await`, the task might be moved to a different thread while holding the lock, causing deadlock or a panic (tokio detects this). Use `tokio::sync::Mutex` for async code.
- **Forgetting `Clone + Send + Sync + 'static` on the trait.** Without these bounds, `with_state` won't accept your store type. Axum requires state to be cloneable and sendable across threads.
- **Not isolating test state.** Reusing the same `InMemoryStore` across tests causes ordering-dependent failures. Always call `new()` at the start of each test.
- **Building `Request<Body>` incorrectly.** Forgetting the `content-type: application/json` header on POST requests causes axum's `Json` extractor to reject the request as not having a supported content type — the handler never runs.
- **Using `serde_json::from_slice` on an empty body.** A `204 No Content` response has no body. Check for empty bytes and return `Value::Null`.

## Key Terms

- **Test helper:** A reusable function that sends a request and unwraps the response, reducing boilerplate in test files.
- **Mock / fake:** A test-only implementation of a trait that returns canned data or uses simple in-memory storage instead of real I/O.
- **Dependency injection:** Passing dependencies (the store) into a component (the router) rather than hardcoding them, enabling test substitution.
- **`async-trait`:** A proc-macro crate that enables async methods in trait definitions, desugaring them to `Pin<Box<dyn Future>>`.
- **Test isolation:** Each test creates its own state so tests don't depend on execution order or shared mutable data.

## Exercise

Open `exercises/src/lib.rs`. The trait, structs, handlers, and `test_app()` are all written. One function contains a `// TODO(module-068)` stub:

**`send_request`** — Build a `Request<Body>` from the given method, path, and optional JSON body. For requests with a body, set the `content-type: application/json` header. Call `app.clone().oneshot(request).await.unwrap()`, extract the status code, read the response body bytes with `to_bytes`, parse as `serde_json::Value` (or `Value::Null` if empty), and return `(status, value)`.

The tests in `tests/module_068.rs` verify the helper works with GET, POST, and DELETE requests, and that stores are isolated between test apps. Run:

```bash
cargo test -p module-068-exercises
```

Compare with `solutions/` when all tests pass.

## Further Reading

- [async-trait crate documentation](https://docs.rs/async-trait)
- [tower::ServiceExt::oneshot](https://docs.rs/tower/latest/tower/trait.ServiceExt.html#method.oneshot)
- [The Test Pyramid (Martin Fowler)](https://martinfowler.com/articles/practical-test-pyramid.html)
- [Module 063: Building REST APIs with Axum](modules/module-063-building-rest-apis-with-axum/README.md)
