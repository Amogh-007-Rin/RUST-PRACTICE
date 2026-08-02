//! Module 076: gRPC with Tonic — exercise scaffold.
//!
//! Implement a service registry pattern that mimics gRPC's request/response model.

#![allow(clippy::new_without_default)]

use std::marker::PhantomData;

/// A generic service trait — the core abstraction behind gRPC's generated code.
///
/// Each service defines a Request and Response type, and a `call` method
/// that processes a request and returns a response.
pub trait Service {
    type Request;
    type Response;

    fn call(&self, req: Self::Request) -> Self::Response;
}

/// A registry that maps service names to service instances.
///
/// In a real gRPC server, the generated code registers services with a
/// dispatcher. This registry simulates that pattern.
pub struct ServiceRegistry<Req, Res> {
    // TODO(module-076): replace the PhantomData with a real field:
    // `handlers: HashMap<String, Box<dyn Fn(Req) -> Res>>`
    _phantom: PhantomData<(Req, Res)>,
}

impl<Req, Res> ServiceRegistry<Req, Res> {
    /// Create a new, empty registry.
    pub fn new() -> Self {
        // TODO(module-076): implement
        panic!("TODO(module-076): implement ServiceRegistry::new")
    }

    /// Register a service handler under a name.
    pub fn register(&mut self, _name: &str, _handler: impl Fn(Req) -> Res + 'static) {
        // TODO(module-076): implement
        panic!("TODO(module-076): implement ServiceRegistry::register")
    }

    /// Dispatch a request to the named service and return the response.
    /// Returns None if no service is registered under that name.
    pub fn dispatch(&self, _name: &str, _req: Req) -> Option<Res> {
        // TODO(module-076): implement
        panic!("TODO(module-076): implement ServiceRegistry::dispatch")
    }
}

// ---------- Key-Value Service demo ----------

/// A key-value operation: Get, Set, or Delete.
#[derive(Debug, Clone, PartialEq)]
pub enum KvOperation {
    Get(String),
    Set(String, String),
    Delete(String),
}

/// Response from a key-value operation.
#[derive(Debug, Clone, PartialEq)]
pub enum KvResponse {
    Found(String),
    NotFound,
    Stored,
    Deleted,
}

/// A mock key-value store that implements the Service trait.
pub struct KvStore {
    // TODO(module-076): add a `data: RefCell<HashMap<String, String>>` field
}

impl KvStore {
    /// Create an empty KV store.
    pub fn new() -> Self {
        // TODO(module-076): implement
        panic!("TODO(module-076): implement KvStore::new")
    }
}

impl Service for KvStore {
    type Request = KvOperation;
    type Response = KvResponse;

    fn call(&self, _req: KvOperation) -> KvResponse {
        // TODO(module-076): match on the operation and return appropriate response:
        // KvOperation::Get(key) -> KvResponse::Found(value) or NotFound
        // KvOperation::Set(key, value) -> KvResponse::Stored
        // KvOperation::Delete(key) -> KvResponse::Deleted
        //
        // Note: call() takes &self, so you'll need interior mutability
        // (RefCell or Mutex) on the underlying HashMap.
        panic!("TODO(module-076): implement KvStore::call")
    }
}
