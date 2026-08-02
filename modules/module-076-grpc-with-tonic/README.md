# Module 076: gRPC with Tonic

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 074 (Raw Networking), Module 075 (Serialization Deep Dive)

## Learning Objectives
- Understand the gRPC service model: request → response with a well-defined contract
- Implement a trait-based service abstraction that mimics gRPC's generated code
- Build a service registry for dispatching requests to named handlers
- Implement a mock key-value store using the service pattern with interior mutability

## Why This Matters
gRPC is the dominant service communication protocol in production Rust infrastructure. Tonic (the Rust gRPC framework) generates code from `.proto` service definitions — but beneath the codegen lies a simple pattern: a service trait with associated Request/Response types and a `call` method. Understanding this pattern before reaching for Tonic lets you reason about gRPC's architecture, implement custom interceptors (middleware), and debug serialization issues when they arise.

## Concept

gRPC (gRPC Remote Procedure Call) is Google's high-performance RPC framework. Unlike REST which exposes resources via HTTP verbs, gRPC defines **services** — a set of typed method signatures. A client calls a method on a stub (a local proxy), and the framework serializes the request, sends it over HTTP/2, deserializes the response, and returns it — all transparently.

### The service abstraction

At its core, every gRPC service follows a trait-based pattern:

```rust
pub trait Service {
    type Request;
    type Response;

    fn call(&self, req: Self::Request) -> Self::Response;
}
```

This is the fundamental contract. A service declares what kind of request it accepts and what kind of response it returns. Tonic's procedural macros generate exactly this — plus the serialization boilerplate — from `.proto` files.

### Request and response as enums

In gRPC, a service typically has multiple RPC methods. We can model this as a Rust enum:

```rust
#[derive(Debug, Clone)]
enum KvOperation {
    Get(String),
    Set(String, String),
    Delete(String),
}

#[derive(Debug, Clone, PartialEq)]
enum KvResponse {
    Found(String),
    NotFound,
    Stored,
    Deleted,
}
```

A `Get("user:1")` request maps to a `Found("Alice")` response, or `NotFound` if the key doesn't exist. This pattern mirrors how Tonic generates `Request` and `Response` structs per RPC method.

### Implementing the service

```rust
impl Service for KvStore {
    type Request = KvOperation;
    type Response = KvResponse;

    fn call(&self, req: KvOperation) -> KvResponse {
        match req {
            KvOperation::Get(key) => match self.data.get(&key) {
                Some(v) => KvResponse::Found(v.clone()),
                None => KvResponse::NotFound,
            },
            KvOperation::Set(key, value) => {
                self.data.insert(key, value);
                KvResponse::Stored
            }
            KvOperation::Delete(key) => {
                self.data.remove(&key);
                KvResponse::Deleted
            }
        }
    }
}
```

Notice a problem: `call` takes `&self`, but `Set` mutates the store. This is where **interior mutability** comes in. The real KvStore wraps its data in a `RefCell` (single-threaded) or `Mutex` (multi-threaded):

```rust
use std::cell::RefCell;

struct KvStore {
    data: RefCell<HashMap<String, String>>,
}

impl Service for KvStore {
    fn call(&self, req: KvOperation) -> KvResponse {
        match req {
            KvOperation::Get(key) => {
                self.data.borrow().get(&key).cloned()
                    .map(KvResponse::Found)
                    .unwrap_or(KvResponse::NotFound)
            }
            KvOperation::Set(key, value) => {
                self.data.borrow_mut().insert(key, value);
                KvResponse::Stored
            }
            // ...
        }
    }
}
```

In a real gRPC service, the framework provides shared state via `Arc<Mutex<...>>` or a database connection pool, so `call` can take `&self` while still mutating underlying resources.

### The service registry pattern

A gRPC server hosts multiple services. Each service is registered by name and dispatched by the framework. We can model this with a simple registry:

```rust
struct ServiceRegistry<Req, Res> {
    handlers: HashMap<String, Box<dyn Fn(Req) -> Res>>,
}

impl<Req, Res> ServiceRegistry<Req, Res> {
    fn register(&mut self, name: &str, handler: impl Fn(Req) -> Res + 'static) {
        self.handlers.insert(name.to_string(), Box::new(handler));
    }

    fn dispatch(&self, name: &str, req: Req) -> Option<Res> {
        self.handlers.get(name).map(|h| h(req))
    }
}
```

This is exactly what happens inside a Tonic server: a `Router` maps `/package.ServiceName/MethodName` paths to handler functions, and a Tower layer chain (middleware) wraps each call.

### The full gRPC lifecycle

```
Client                          Server
------                          ------

1. Client calls stub.get("key") →
2. Serialize request to protobuf →
3. Send over HTTP/2 ─────────────→
                                4. Deserialize protobuf
                                5. Route to service handler
                                6. handler.call(request)
                                7. response from handler
8. Deserialize protobuf ←──────── 8. Serialize response
9. Return response to caller
```

Steps 1, 2, 8, and 9 are generated by `tonic-build`. Step 5 is the service registry. Step 6 is the `Service` trait. Understanding this pipeline makes debugging gRPC failures straightforward — you can isolate whether the issue is at the transport layer, the serialization layer, or the business logic.

### Interior mutability in services

gRPC handlers are called concurrently. Each handler receives `&self` (shared reference), so mutation requires synchronized interior mutability. The standard pattern in Rust async services:

| Context | Pattern |
|---------|---------|
| Single-threaded mock | `RefCell<T>` |
| Multi-threaded | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |
| Production async | Database connection pool (`sqlx::PgPool`) |

The KvStore in our exercise uses `RefCell` because it's single-threaded and the call is synchronous — a deliberate simplification that mirrors the trait signature without introducing async complexity.

## Common Pitfalls
- **Forgetting interior mutability**: if your service trait has `&self` but you need to mutate, you'll get a compile error. Reach for `RefCell` or `Mutex`.
- **Not matching all enum variants**: if you add a new operation variant and forget to handle it in `call`, you'll get a warning. Use exhaustive match.
- **Confusing request/response types**: each gRPC method has its own request/response pair. An enum wrapping all operations is a simplification; real Tonic generates a struct per method.
- **Expecting `serde_json::Value` to work with bincode**: it doesn't (see Module 075). Use concrete types for binary RPC.

## Key Terms
- **RPC (Remote Procedure Call)**: calling a function on a different machine as if it were local
- **gRPC**: Google's open-source RPC framework using HTTP/2 and Protocol Buffers
- **Tonic**: the Rust gRPC framework (async, Tower-based, codegen from `.proto`)
- **Service registry**: a map from service name to handler function
- **Interior mutability**: mutating data through a shared (`&`) reference, via `RefCell`, `Mutex`, etc.

## Exercise

In `exercises/`, fill in the `TODO(module-076)` markers to:

1. **`ServiceRegistry`** — store named handlers and dispatch requests
2. **`KvStore`** — implement a `Service` that handles `Get`, `Set`, `Delete` operations with `RefCell<HashMap>` for storage

Run `cargo test -p module-076-exercises` to verify.

## Further Reading
- [gRPC official docs](https://grpc.io/docs/)
- [Tonic crate](https://docs.rs/tonic/latest/tonic/)
- [Protocol Buffers language guide](https://protobuf.dev/programming-guides/proto3/)
