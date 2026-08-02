//! Module 027: Design Patterns in Rust — exercise scaffold.
//!
//! Fill in every `TODO(module-027)` below so the integration tests in
//! `tests/module_027.rs` pass. The tests define "done".

/// A fully configured pizza.
#[derive(Debug, PartialEq)]
pub struct Pizza {
    pub size: u32,
    pub toppings: Vec<String>,
    pub cheese: bool,
}

/// The **builder** pattern: accumulate configuration through chainable
/// methods, then produce a `Pizza` with `build`.
///
/// Fields are only read by the methods you are about to implement.
#[allow(dead_code)]
pub struct PizzaBuilder {
    size: u32,
    toppings: Vec<String>,
    cheese: bool,
}

impl PizzaBuilder {
    /// Starts a build for a pizza of the given `size`.
    pub fn new(size: u32) -> Self {
        Self {
            size,
            toppings: Vec::new(),
            cheese: false,
        }
    }

    /// Adds a topping.
    #[must_use]
    pub fn add_topping(self, _topping: &str) -> Self {
        // TODO(module-027): push the topping and return `self`. The
        // parameter needs `mut` once you actually push.
        panic!("not implemented")
    }

    /// Requests extra cheese.
    #[must_use]
    pub fn extra_cheese(self) -> Self {
        // TODO(module-027): set `cheese = true` and return `self`. Needs
        // `mut self` as well.
        panic!("not implemented")
    }

    /// Produces the finished `Pizza`.
    #[must_use]
    pub fn build(self) -> Pizza {
        // TODO(module-027): move the fields into a `Pizza`.
        panic!("not implemented")
    }
}

/// Marker state: the connection is not yet established.
pub struct Disconnected;

/// Marker state: the connection is established and can send data.
pub struct Connected;

/// The **typestate** pattern: the connection's *type* encodes what you may
/// do with it. Only `TcpConnection<Connected>` has `send`.
pub struct TcpConnection<S> {
    address: String,
    sent: Vec<String>,
    _state: std::marker::PhantomData<S>,
}

impl TcpConnection<Disconnected> {
    /// Creates a disconnected connection to `address`.
    pub fn new(address: &str) -> Self {
        Self {
            address: address.to_string(),
            sent: Vec::new(),
            _state: std::marker::PhantomData,
        }
    }

    /// Establishes the connection, moving into the `Connected` state.
    #[must_use]
    pub fn connect(self) -> TcpConnection<Connected> {
        // TODO(module-027): move `address` and `sent` into a
        // `TcpConnection<Connected>`.
        panic!("not implemented")
    }
}

impl TcpConnection<Connected> {
    /// Sends a message; the message is recorded in the connection log.
    #[must_use]
    pub fn send(self, _data: &str) -> TcpConnection<Connected> {
        // TODO(module-027): push the message and return `self`. Needs
        // `mut self`.
        panic!("not implemented")
    }

    /// Closes the connection, moving back to the `Disconnected` state.
    #[must_use]
    pub fn disconnect(self) -> TcpConnection<Disconnected> {
        // TODO(module-027): move the fields into a
        // `TcpConnection<Disconnected>`.
        panic!("not implemented")
    }
}

impl<S> TcpConnection<S> {
    /// The address this connection targets.
    pub fn address(&self) -> &str {
        &self.address
    }

    /// The messages sent so far.
    pub fn sent_messages(&self) -> &[String] {
        &self.sent
    }
}

/// The **RAII** pattern: `LogTimer` reports its own lifetime. Construction
/// starts a stopwatch; `Drop` prints the elapsed time when the timer leaves
/// scope — no explicit cleanup call required.
///
/// Fields are only read by the methods you are about to implement.
#[allow(dead_code)]
pub struct LogTimer {
    name: String,
    start: std::time::Instant,
}

impl LogTimer {
    /// Starts a timer with the given `name`.
    pub fn start(name: &str) -> Self {
        Self {
            name: name.to_string(),
            start: std::time::Instant::now(),
        }
    }

    /// The time elapsed since the timer started.
    pub fn elapsed(&self) -> std::time::Duration {
        // TODO(module-027): `self.start.elapsed()`.
        panic!("not implemented")
    }

    /// Stops the timer and returns the elapsed duration.
    pub fn stop(self) -> std::time::Duration {
        // TODO(module-027): capture the elapsed time, print
        // `"{} finished after {:?}"`, and return it.
        panic!("not implemented")
    }
}

impl Drop for LogTimer {
    fn drop(&mut self) {
        // TODO(module-027): print the elapsed time, e.g.
        // `println!("{} dropped after {:?}", self.name, self.elapsed());`
    }
}

/// A node in a tree of values.
#[derive(Debug, PartialEq)]
pub enum Node {
    Number(i32),
    Text(String),
    List(Vec<Node>),
}

/// The **visitor** pattern: an interface for *processing* tree nodes
/// without coupling the tree to any particular operation.
pub trait Visitor {
    /// Called for every `Node::Number`.
    fn visit_number(&mut self, n: i32);
    /// Called for every `Node::Text`.
    fn visit_text(&mut self, s: &str);
}

/// Walks `node` depth-first, invoking the matching `Visitor` methods.
pub fn walk(_node: &Node, _visitor: &mut dyn Visitor) {
    // TODO(module-027): match on `node`:
    //   Number(n)   => visitor.visit_number(*n),
    //   Text(s)     => visitor.visit_text(s),
    //   List(items) => walk each item recursively.
    panic!("not implemented")
}
