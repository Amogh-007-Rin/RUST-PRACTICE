# Module 062: Axum Fundamentals

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 061 (HTTP fundamentals — know what a request line and a status line are), Module 041–042 (async + tokio runtime).

## Learning Objectives

- Build an `axum::Router` by registering handlers on routes.
- Write handlers as plain async functions returning anything that implements `IntoResponse`.
- Use the `Path`, `Query`, `Json`, and `State` extractors to pull structured data out of a raw request.
- Share application-wide state with `Router::with_state`.
- Test a router end-to-end without sockets, via `tower::ServiceExt::oneshot`.

## Why This Matters

Axum is one of the most popular production Rust web frameworks, and it's the backbone of the rest of Block G — every later module (REST APIs, auth, middleware, the capstone) builds on the exact pieces in this module: router, handler, extractor, state. More importantly, the extractor pattern you learn here is the axum-specific flavor of a general idea: *declaratively describing what your handler needs, and letting the framework fetch it*. That pattern shows up in actix-web, tower, and most Rust async frameworks you'll meet.

## Concept

### Routing: the table of the web

In Module 061 you hand-wrote the `route` function — a match on `(method, target)`. Axum's `Router` is that same match, structured:

```
                         Router
              ┌────────────┼────────────┐
              │            │            │
        GET /          GET /hello/{name}   POST /items
              │            │            │
           root()       hello()       create_item()
                              │            │
                          Path<String>   Json<NewItem>
                                          State<AppState>
```

Each `.route("/path", method(handler))` line registers one row in this table. The `{name}` in `/hello/{name}` is a *capture*: one URL segment that can be anything, handed to the handler. When a request arrives, axum walks this table: method must match (`GET` vs `POST`), and the path pattern must match. If nothing matches, you get a `404` — same as your hand-written match in Module 061, but with pattern matching, not string equality.

`Router::new()` returns a router with no rows; `.route(...)` consumes the router and returns a new one with a row added (a builder pattern — each line returns the next stage). `Router` is `Clone`; cloning is cheap (it's an `Arc` inside), which is why tests clone it for every request.

### Handlers: just async functions

A handler is a plain `async fn`. Its return type must implement `IntoResponse`, which axum implements for a long list of types — `&'static str`, `String`, `StatusCode`, tuples like `(StatusCode, Json<T>)`, and more. The tuple form is the workhorse: the first element is the status, the second is the body.

You can return `impl IntoResponse` directly from a handler — no ceremony. Inside the body you can use the full language: `format!`, match, iterators, and `?` (as long as the error type also implements `IntoResponse` — more on that in Module 063).

### Extractors: declare what you need

Every argument of a handler (beyond the first, which has no special meaning) is an *extractor*: something that implements `FromRequestParts` (for the request head: path, query, headers) or `FromRequest` (for the whole request, including the body). When axum calls your handler, it runs each extractor first; if one fails, your handler never runs — axum turns the failure into a rejection (usually a `400` or `422`).

| Extractor | Extracts | Failure mode |
|---|---|---|
| `Path<T>` | URL path segments matched by `{...}` captures | `400` if it can't parse into `T` |
| `Query<T>` | The `?key=value` query string, deserialized into `T` | `400` on malformed query |
| `Json<T>` | The request body, deserialized into `T` | `400` on bad JSON / wrong shape |
| `State<T>` | The shared state you passed to `with_state` | compile error if the types don't line up |

The key idea: *the signature is the contract*. `Query<SearchParams>` says "I want the query string as a `SearchParams`" — you never touch the raw string. And because `SearchParams` is `Deserialize`, you get serde's machinery for free: `#[serde(default)]` makes a field optional (missing key → default value instead of an error).

Two arguments in the same handler run in order. `(State<AppState>, Json<NewItem>)` first extracts state, then the body. Note that extractors which consume the body (`Json`) must come after ones that only read the head (`State`, `Path`, `Query`) — you can't read the body twice.

### Shared state: state beyond a single request

Stateless servers are nice but boring. Real services need a counter, a store, a config — something shared across requests. Axum's answer is `Router::with_state(state)`, which bakes a value of type `T` into the router. Any handler can then take `State<T>` and get a reference to it.

The constraint that shapes everything: **state must be `Clone`** (the router needs to hand it out on every request, and the router itself must stay `Clone`). The idiomatic fix: put cheaply-clonable handles inside — `Arc<Mutex<...>>`, `Arc<AtomicUsize>`, or a `SqlitePool` (Module 064). In this module's exercise, `AppState { counter: Arc<AtomicUsize> }` is shared across all handlers; `create_item` bumps it to mint ids, `item_count` reads it back. That's a two-handler proof that state truly is shared.

### `oneshot`: testing without a socket

`Router` itself implements tower's `Service` trait (you'll meet `tower` properly in Module 066). That means you can call it like a function — no port, no `TcpListener`, no network:

```rust
use axum::body::Body;
use axum::http::Request;
use tower::ServiceExt;

let response = router
    .oneshot(Request::builder().uri("/hello/world").body(Body::empty()).unwrap())
    .await
    .unwrap();
```

`oneshot` runs the request through the same routing + extraction + handler pipeline a real HTTP request would take, minus the bytes on the wire. It's fast, deterministic, and needs no ports — which is why every module from here to the capstone uses it. (The `unwrap`s are fine in tests; this is `tests/` code.)

### Putting it together

```
Client request ──► Router (tower Service)
                     │  route match: GET /hello/{name} ✔
                     │  extractors: Path<String> → "world"
                     ▼
                  hello("world")  →  "Hello, world!"  (IntoResponse)
                     │
                     ▼
                  HTTP 200, body "Hello, world!"  ──► Client
```

## Common Pitfalls

- **Old `:param` syntax.** Axum 0.8 uses `{name}` for path captures — `:name` panics at startup. If you're following a pre-2025 tutorial, translate.
- **Non-`Clone` state.** `with_state` requires `Clone`. Forgetting that `Mutex`/`SqlitePool` aren't `Clone` is the classic error; wrap them in `Arc` first.
- **Body-consuming extractors in the wrong position.** `Json<T>` consumes the request body, so it must be the last extractor in a handler signature. `(Json<T>, State<S>)` compiles but panics at runtime with "body already extracted".
- **Missing `#[serde(default)]`.** A `Query<T>` with a field the client didn't send fails the whole extraction. Optional query params need `#[serde(default)]` on the field (or `Option<T>`).
- **Treating `Router::new()` as mutable.** It's a builder — every `.route()` returns a *new* router. `let app = Router::new().route(...)...;` in one expression, don't try to mutate.

## Key Terms

- **Router:** the type that maps (method, path pattern) to handlers; also a `tower::Service`.
- **Handler:** an `async fn` taking extractors and returning something `IntoResponse`.
- **Extractor:** a handler argument that pulls data (path, query, body, state, headers) out of the request; implementors of `FromRequest`/`FromRequestParts`.
- **Capture:** a `{name}` segment in a route path that matches any single URL segment.
- **`IntoResponse`:** the trait for "can become an HTTP response" — implemented for `String`, `StatusCode`, tuples, JSON wrappers, and more.
- **Shared state:** a value baked into the router via `with_state`, retrievable in handlers via `State<T>`.
- **`oneshot`:** calling a `Service` once with a request and awaiting a response — the standard way to test axum routers.

## Exercise

In `exercises/`, the router is fully wired and `root`/`hello` work. Three handlers are stubbed with `panic!` and marked `// TODO(module-062)`:

1. `search` — use the `Query<SearchParams>` extractor (already in the signature) and report the parameters as `"{q}/{limit}"`.
2. `create_item` — assign the next id from `state.counter`, respond with `StatusCode::CREATED` and the created item as JSON.
3. `item_count` — return the current counter value as a string.

Run the tests:

```bash
cargo test -p module-062-exercises
```

The tests use `tower::ServiceExt::oneshot` — no sockets involved. When all nine pass, compare with `solutions/`.

## Further Reading

- [Axum documentation](https://docs.rs/axum) — the `Router`, extractor, and `IntoResponse` pages
- [Axum extractors chapter](https://docs.rs/axum/latest/axum/extract/index.html)
- [The Tower `Service` trait (what `oneshot` calls)](https://docs.rs/tower/latest/tower/trait.Service.html)
- [Module 061: HTTP & Web Fundamentals — what axum compiles down to](modules/module-061-http-and-web-fundamentals/README.md)
