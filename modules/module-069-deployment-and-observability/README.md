# Module 069: Deployment & Observability

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 063 (Axum router, handlers, state), Module 042 (Tokio runtime). This module adds observability on top of any existing axum service.

## Learning Objectives

- Set up structured logging with `tracing` and `tracing-subscriber`, including runtime filter configuration via the `RUST_LOG` environment variable.
- Implement a `/health` endpoint that reports service status (healthy/degraded) and uptime.
- Understand how to instrument handlers with tracing spans for request-level observability.
- Return a subscriber guard that ensures buffered logs are flushed on shutdown.

## Why This Matters

A service that "works when you `cargo run` it" is not a production service. Production services need health checks (so orchestrators like Kubernetes know when to restart them) and structured logging (so you can find the request that caused a 500 error among millions of others). `tracing` is Rust's standard structured logging framework — every major async library emits tracing spans, and `tracing-subscriber` is how you capture them. The pattern of `init_tracing()` returning a guard and a `/health` endpoint returning structured JSON is what every production Rust service does on startup.

## Concept

### Structured logging vs. println

`println!("User {} logged in", username)` is a dead end — you can't search it, filter it, or aggregate it. Structured logging attaches key-value *fields* to each log event:

```rust
tracing::info!(username = "alice", action = "login", "user authenticated");
```

The output might look like:

```
2026-08-02T10:15:30.123Z  INFO module_069: user authenticated username=alice action=login
```

The difference: `username=alice` is a structured field, not embedded in the message string. Log aggregation systems (ElasticSearch, Loki, Datadog) index these fields, letting you query `username:alice` across all services. The message string is for human readability; the fields are for machines.

`tracing` goes further than `log`: it supports *spans* — named regions of execution with a start and end:

```rust
#[tracing::instrument]
async fn handle_request(Path(id): Path<u64>) {
    tracing::info!("processing request");
    // ... this entire function execution is a span ...
}
```

The `#[instrument]` attribute creates a span named `handle_request`. When you look at the logs, you can see which events happened inside which span — crucial for tracing a single request through a service that handles many concurrently.

### Setting up a subscriber

`tracing` emits events and spans, but they go nowhere until you install a *subscriber*. `tracing-subscriber` provides a `fmt` subscriber that writes to stderr with configurable formatting:

```rust
use tracing_subscriber::EnvFilter;

let filter = EnvFilter::try_from_default_env()
    .unwrap_or_else(|_| EnvFilter::new("info"));

tracing_subscriber::fmt()
    .with_env_filter(filter)
    .init();
```

The `EnvFilter` reads the `RUST_LOG` environment variable. A developer can run:

```bash
RUST_LOG=debug cargo run          # verbose
RUST_LOG=my_crate=debug cargo run # only my crate
RUST_LOG=warn cargo run           # only warnings and errors
```

`try_from_default_env()` tries to parse `RUST_LOG`. If it's not set or is invalid, the `unwrap_or_else` falls back to `info` level — so production logs at `info` by default, and developers can bump to `debug` or `trace` without recompiling.

### The subscriber guard

`init()` sets the subscriber as the *global default* — the subscriber that all tracing macros write to. But it doesn't return anything, so you can't control the subscriber's lifetime. The alternative is `set_default`:

```rust
let subscriber = tracing_subscriber::fmt()
    .with_env_filter(filter)
    .finish();
let guard = tracing::subscriber::set_default(subscriber);
// guard implements Drop — when dropped, buffered logs flush
```

`set_default` returns a guard. When the guard drops, the subscriber is unset and any buffered log lines are flushed. This is important for short-lived processes (CLI tools, test binaries) and for clean shutdown. The return type `impl Drop` hides the concrete guard type from the caller — they just need to know it's a guard they must keep alive.

In a `main.rs`, you'd typically write:

```rust
fn main() {
    let _guard = init_tracing();  // lives for the whole program
    // ... start service ...
}
// guard drops here, flushing remaining logs
```

### Health check endpoints

Orchestrators (Kubernetes, Docker Swarm, Nomad) probe services at a `/health` (or `/healthz`) endpoint to decide whether the service is alive. If the endpoint returns a non-2xx status, the orchestrator restarts the service. If it returns 5xx, it marks the pod as unhealthy and stops routing traffic.

A health check endpoint typically returns:

```json
{
  "status": "healthy",
  "uptime_seconds": 12345
}
```

- `status`: `"healthy"` (200) means everything is fine. `"degraded"` (503) means the service is alive but not fully functional — e.g., the database connection pool is exhausted, but the process hasn't crashed.
- `uptime_seconds`: how long the process has been running. Helps detect restart storms.

The handler records the start time at application initialization (`std::time::Instant::now()`), then computes `start.elapsed().as_secs()` on every request. `Instant` is monotonic — it never goes backward — so uptime is always accurate even across NTP adjustments.

A "degraded" signal might come from an atomic boolean flag:

```rust
use std::sync::atomic::{AtomicBool, Ordering};

let degraded = Arc::new(AtomicBool::new(false));
// ... somewhere else, when the DB pool is exhausted:
degraded.store(true, Ordering::Relaxed);
```

`AtomicBool` gives lock-free reads and writes across threads. `Ordering::Relaxed` is sufficient here — the value doesn't synchronize other data, it's just a flag.

The handler checks the flag and chooses the appropriate status:

```
Request: GET /health
    │
    ▼
State<AppState>  →  start_time, degraded flag
    │
    ▼
Compute uptime_seconds = start_time.elapsed().as_secs()
    │
    ▼
degraded.load(Relaxed)?
    ├── false → StatusCode::OK,      status = "healthy"
    └── true  → StatusCode::SERVICE_UNAVAILABLE, status = "degraded"
    │
    ▼
tracing::info!(status, uptime_seconds, "health check")
    │
    ▼
Response: 200/503, JSON body
```

### What `StatusCode::SERVICE_UNAVAILABLE` means

HTTP 503 is the standard code for "the server is not ready to handle the request." It's distinct from 500 (internal error) — 500 means "something went wrong, and it might be a bug," while 503 means "the service is deliberately not serving traffic." Kubernetes reads 503 from a readiness probe and stops sending traffic, while still allowing liveness probes to check if the process is running.

### Putting it all together

```
Application startup
    │
    ├── State { start_time: Instant::now(), degraded: false }
    │
    ├── init_tracing()  →  subscriber with EnvFilter from RUST_LOG
    │                       _guard kept alive for the program's lifetime
    │
    └── build_app_with_observability(state)
           │
           └── Router with GET /health → health_check handler
                   │
                   ▼
         Request: GET /health
                   │
         Handler reads state, computes status
                   │
         Logs with tracing::info!
                   │
         Returns JSON response
```

## Common Pitfalls

- **Calling `init()` twice.** `tracing_subscriber::fmt().init()` panics if a global default is already set. In tests, either use `set_default` (which allows nesting) or only call `init_tracing` once in a `#[ctor]` or `once_cell`. The tests in this module account for this — `init_tracing()` uses `set_default`.
- **Using `SystemTime` for uptime.** `SystemTime::now()` can jump backward (NTP correction, daylight saving). Use `std::time::Instant` for monotonic timing — it's guaranteed to never go backward.
- **Forgetting to keep the guard alive.** If you write `init_tracing();` (without `let _guard =`), the guard drops immediately and the subscriber is unset before any request runs. Always bind the guard.
- **Health check doing too much work.** A health check should be cheap — query a boolean flag, compute elapsed time. Don't query a database or call an external service from the health endpoint; if you need a "depends on database" status, cache it in an `AtomicBool` updated by a background task.
- **Hardcoding log levels.** `RUST_LOG` is the standard mechanism. Hardcoding `tracing_subscriber::fmt().with_max_level(Level::INFO)` prevents runtime changes — in production, you'll want to bump to `debug` without rebuilding.

## Key Terms

- **Structured logging:** Emitting logs with key-value fields instead of plain strings, enabling machine-readable querying and aggregation.
- **Span:** A named, timed region of execution that groups related log events — the core concept of `tracing`.
- **Subscriber:** A sink that receives and formats tracing events/spans; `tracing-subscriber` is the standard implementation.
- **`RUST_LOG`:** The environment variable that controls log filtering (`error`, `warn`, `info`, `debug`, `trace`) per module.
- **Health check:** A lightweight endpoint (`/health`) that reports whether a service is running and ready to serve traffic.
- **Liveness probe:** A check that asks "is the process alive?" — failing causes a restart.
- **Readiness probe:** A check that asks "is the service ready for traffic?" — failing causes the orchestrator to stop routing requests.
- **Monotonic clock:** A clock that only moves forward (`Instant`), suitable for measuring elapsed time.

## Exercise

Open `exercises/src/lib.rs`. Two functions contain `// TODO(module-069)` stubs:

1. **`init_tracing()`** — Create a `tracing_subscriber::fmt()` subscriber with an `EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"))`. Call `.finish()` on the builder to get a subscriber, then `tracing::subscriber::set_default(subscriber)` to install it. Unwrap the result and return the guard. The return type is `impl Drop`.

2. **`health_check()`** — Compute `uptime_seconds` from `state.start_time.elapsed().as_secs()`. Check `state.degraded.load(Ordering::Relaxed)`. If degraded, return `(StatusCode::SERVICE_UNAVAILABLE, Json(HealthStatus { status: "degraded", uptime_seconds }))`. Otherwise, return `(StatusCode::OK, Json(HealthStatus { status: "healthy", uptime_seconds }))`. Log the health status with `tracing::info!`.

The tests in `tests/module_069.rs` verify healthy/degraded responses, uptime counting, and that `init_tracing` returns a working guard. Run:

```bash
cargo test -p module-069-exercises
```

Compare with `solutions/` when all tests pass.

## Further Reading

- [tracing documentation](https://docs.rs/tracing) — the `#[instrument]` attribute, spans, and field recording
- [tracing-subscriber documentation](https://docs.rs/tracing-subscriber) — `EnvFilter` syntax, formatters, and layer composition
- [Kubernetes health checks](https://kubernetes.io/docs/tasks/configure-pod-container/configure-liveness-readiness-startup-probes/)
- [Module 063: Building REST APIs with Axum](modules/module-063-building-rest-apis-with-axum/README.md)
