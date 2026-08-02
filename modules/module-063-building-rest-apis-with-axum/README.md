# Module 063: Building REST APIs with Axum

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 062 (Axum fundamentals — routing, handlers, extractors, state).

## Learning Objectives

- Implement a full set of CRUD endpoints for a REST resource (create, read, update, delete).
- Use `serde` and the `Json<T>` extractor for typed request/response bodies.
- Validate request inputs and return appropriate HTTP status codes (201, 204, 404, 422).
- Share mutable state safely across handlers via `Arc<Mutex<...>>` and `State<T>`.
- Map domain errors to HTTP responses with a custom `IntoResponse` implementation.

## Why This Matters

Every backend developer building APIs in Rust will write exactly this pattern: a router with CRUD routes, serde-powered JSON bodies, shared state, and error mapping. It's the table stakes of web development, and in Rust it's the foundation for the rest of Block G — once you can build a REST API, you add a database (Module 064), then auth (Module 065), then middleware (Module 066), each layer slotting into the same pattern.

## Concept

### REST resources as state machines

A REST API models a *resource* — here, a `Todo` with an `id`, `title`, and `completed` flag. The resource lives in some store (an in-memory `HashMap` for this module, a database in Module 064), and the five CRUD endpoints are the legal transitions on that store:

```
         POST /todos          ──► insert entry ──► 201 Created
         GET  /todos          ──► read all      ──► 200 Ok
         GET  /todos/{id}     ──► read one      ──► 200 Ok | 404 Not Found
         PUT  /todos/{id}     ──► replace one   ──► 200 Ok | 404 Not Found
         DELETE /todos/{id}   ──► remove one    ──► 204 No Content | 404 Not Found
```

Each endpoint maps to one HTTP verb and one path pattern. Axum expresses this directly in the router builder:

```rust
Router::new()
    .route("/todos", get(list_todos).post(create_todo))
    .route("/todos/{id}", get(get_todo).put(update_todo).delete(delete_todo))
    .with_state(state)
```

The chaining reads naturally: `get(list_todos).post(create_todo)` is a *method router* — when the path matches, axum dispatches to the handler whose HTTP verb matches the request. If no verb matches, you get a `405 Method Not Allowed` automatically.

### JSON bodies with serde

Request and response bodies are JSON. Two types of serde structs appear:

**Request bodies** implement only `Deserialize` — incoming data the client sends:

```rust
#[derive(Debug, Deserialize)]
pub struct NewTodo { pub title: String }
```

**Response bodies** implement `Serialize` — outgoing data the server sends back:

```rust
#[derive(Debug, Serialize)]
pub struct Todo { pub id: u64, pub title: String, pub completed: bool }
```

The `Json<T>` extractor handles both directions: `Json<NewTodo>` in a parameter deserializes the request body; `Json(todo)` in a return value serializes the response body and sets `Content-Type: application/json`.

The `Json` extractor is *consuming* — it reads the request body, so it must be the last extractor in the signature. Put `State<AppState>` before `Json<NewTodo>`, never after.

### Status codes tell the story

A good REST API communicates outcome through HTTP status codes, not just through body content. Every CRUD result maps to one code:

| Outcome | Status | Meaning |
|---|---|---|
| Created | `201 CREATED` | A new resource exists; Location header is implied |
| Fetched | `200 OK` | Here is the data |
| Updated | `200 OK` | Here is the updated representation |
| Deleted | `204 NO CONTENT` | The resource was removed; nothing to return |
| Missing | `404 NOT FOUND` | That id does not exist |
| Invalid | `422 UNPROCESSABLE ENTITY` | The input looks like JSON but fails business rules |

When a handler returns a tuple like `(StatusCode::CREATED, Json(todo))`, axum composes the two into a full response — the `StatusCode` becomes the status line and `Json<T>` becomes the body with the right content type.

### Path parameters

`Path<u64>` extracts the `{id}` segment from the URL. If the client hits `/todos/abc`, the `Path<u64>` parser fails and axum returns a `400 Bad Request` automatically — the handler never runs. This is the same `Path` extractor from Module 062, now used in a REST context for resource identification.

### Custom error types

Returning `Result<T, E>` from a handler only works if `E` implements `IntoResponse`. The pattern:

```rust
pub enum AppError {
    NotFound,
    Invalid(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::NotFound => (StatusCode::NOT_FOUND, "todo not found".to_string()),
            AppError::Invalid(msg) => (StatusCode::UNPROCESSABLE_ENTITY, msg),
        };
        (status, Json(json!({"error": message}))).into_response()
    }
}
```

Now every handler can use `?` with `AppError` and errors automatically become proper JSON error responses. The `json!` macro from `serde_json` builds an ad-hoc JSON object for the error body — no separate struct needed.

The full flow:

```
Request comes in
    │
    ▼
Path<u64> extractor ── fails ──► 400 Bad Request (axum's default)
    │ passes
    ▼
Json<NewTodo> extractor ── fails ──► 400 Bad Request (axum's default)
    │ passes
    ▼
Handler logic runs
    │
    ├── business rule fails ──► AppError::Invalid → 422 with JSON error body
    ├── resource not found  ──► AppError::NotFound  → 404 with JSON error body
    └── success             ──► 200/201/204 with JSON/message body
```

### Validation

Validation happens in the handler, after extraction succeeds but before mutation. The pattern:

1. Extract the input.
2. Check business rules (non-empty title, valid range, etc.).
3. If invalid, return `AppError::Invalid("why")`.
4. If valid, mutate state and return success.

This keeps validation logic in one place — in the handler body, immediately after extraction — rather than scattered across serde attributes or custom extractors. For a title field, the check is `input.title.trim().is_empty()`; if true, return `422 UNPROCESSABLE ENTITY` with a descriptive message.

### Putting it all together

A complete REST API request flows through:

```
Client: POST /todos {"title": "buy milk"}
    │
    ▼
Router matches (POST, /todos) → create_todo handler
    │
    ▼
State<AppState> extracts shared map + id counter
    │
    ▼
Json<NewTodo> extracts {"title": "buy milk"} → NewTodo { title: "buy milk" }
    │
    ▼
Handler validates: title not empty ✓
    │
    ▼
Handler mutates: insert Todo { id: 1, title: "buy milk", completed: false }
    │
    ▼
Handler returns: (StatusCode::CREATED, Json(todo))
    │
    ▼
Client receives: 201 Created, body: {"id":1,"title":"buy milk","completed":false}
```

## Common Pitfalls

- **Forgetting validation.** A handler that inserts a blank title "as-is" is a bug — every resource mutation endpoint must validate inputs before mutating state. The `trim()` call is important; `"   "` is not meaningfully non-empty.
- **Returning `200` for everything.** Use `201 CREATED` for creates and `204 NO CONTENT` for deletes. Status codes carry meaning that middleware and clients rely on.
- **Not checking existence before delete/update.** `PUT /todos/999` on a nonexistent resource must return `404`, not silently succeed or panic. Check before mutating.
- **Using the wrong `Ordering` for atomic counters.** `fetch_add` needs at least `Ordering::SeqCst` in this context to guarantee sequential ids across concurrent requests — `Ordering::Relaxed` can produce duplicate ids under contention.
- **Returning the raw error type.** A handler returning `Result<T, sqlx::Error>` leaks implementation details to the client. Map errors to an `AppError` variant with a user-facing message.

## Key Terms

- **CRUD:** Create, Read, Update, Delete — the five standard actions on a REST resource.
- **Path parameter:** A `{name}` segment in a route path extracted as `Path<T>` in the handler.
- **Status code:** The three-digit HTTP response code indicating outcome (2xx = success, 4xx = client error).
- **`IntoResponse`:** The trait that converts a type into an HTTP response; the key to custom error types.
- **Validation:** Checking business rules (non-empty, in range, valid format) on inputs before mutating state.
- **`422 UNPROCESSABLE ENTITY`:** The standard HTTP status for "this JSON parses but is semantically wrong."

## Exercise

Open `exercises/src/lib.rs`. The router, state types, error types, and handler signatures are all written. Three handlers contain `// TODO(module-063)` with `panic!()` stubs:

1. **`create_todo`** — Validate the title is non-empty after trimming. Assign the next id from `state.next_id` (using `fetch_add` with `Ordering::SeqCst` and adding 1). Store the new `Todo` (with `completed: false`). Return `(StatusCode::CREATED, Json(todo))`.

2. **`update_todo`** — Validate the title (non-empty after trimming). Look up the todo by id in the shared `HashMap`; return `AppError::NotFound` if absent. Replace the title and completed fields (keeping the same id). Return `Json(todo)` with status `200 OK`.

3. **`delete_todo`** — Remove the todo by id from the `HashMap`. Return `StatusCode::NO_CONTENT` on success or `AppError::NotFound` if the id doesn't exist.

The tests in `tests/module_063.rs` cover the full CRUD lifecycle, blank-title rejection, and missing-resource 404s — all through `tower::ServiceExt::oneshot`. Run:

```bash
cargo test -p module-063-exercises
```

Compare with `solutions/` when all tests pass.

## Further Reading

- [Axum documentation — routing and handlers](https://docs.rs/axum/latest/axum/routing/index.html)
- [REST API best practices](https://stackoverflow.blog/2020/03/02/best-practices-for-rest-api-design/)
- [serde documentation](https://serde.rs/)
- [Module 062: Axum Fundamentals](modules/module-062-axum-fundamentals/README.md)
