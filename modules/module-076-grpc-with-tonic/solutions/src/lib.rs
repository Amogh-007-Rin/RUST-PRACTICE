//! Module 076: gRPC with Tonic — reference solution.

#![allow(clippy::new_without_default)]

use std::cell::RefCell;
use std::collections::HashMap;

/// A generic service trait — the core abstraction behind gRPC's generated code.
pub trait Service {
    type Request;
    type Response;

    fn call(&self, req: Self::Request) -> Self::Response;
}

/// A registry that maps service names to handler functions.
///
/// In a real gRPC server, the generated code registers services with a
/// dispatcher. This registry stores closures keyed by name.
pub struct ServiceRegistry<Req, Res> {
    handlers: HashMap<String, Box<dyn Fn(Req) -> Res>>,
}

impl<Req, Res> ServiceRegistry<Req, Res> {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str, handler: impl Fn(Req) -> Res + 'static) {
        self.handlers.insert(name.to_string(), Box::new(handler));
    }

    pub fn dispatch(&self, name: &str, req: Req) -> Option<Res> {
        self.handlers.get(name).map(|h| h(req))
    }
}

// ---------- Key-Value Service demo ----------

#[derive(Debug, Clone, PartialEq)]
pub enum KvOperation {
    Get(String),
    Set(String, String),
    Delete(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KvResponse {
    Found(String),
    NotFound,
    Stored,
    Deleted,
}

/// A mock key-value store that implements the Service trait.
///
/// Uses RefCell for interior mutability — `call()` takes `&self` but the
/// underlying storage needs to mutate. In a real gRPC service this would
/// be an async database connection behind an Arc.
pub struct KvStore {
    data: RefCell<HashMap<String, String>>,
}

impl KvStore {
    pub fn new() -> Self {
        Self {
            data: RefCell::new(HashMap::new()),
        }
    }
}

impl Service for KvStore {
    type Request = KvOperation;
    type Response = KvResponse;

    fn call(&self, req: KvOperation) -> KvResponse {
        match req {
            KvOperation::Get(key) => self
                .data
                .borrow()
                .get(&key)
                .cloned()
                .map(KvResponse::Found)
                .unwrap_or(KvResponse::NotFound),
            KvOperation::Set(key, value) => {
                self.data.borrow_mut().insert(key, value);
                KvResponse::Stored
            }
            KvOperation::Delete(key) => {
                self.data.borrow_mut().remove(&key);
                KvResponse::Deleted
            }
        }
    }
}
