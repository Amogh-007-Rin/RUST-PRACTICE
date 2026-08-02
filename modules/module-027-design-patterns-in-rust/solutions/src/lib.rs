//! Module 027: Design Patterns in Rust — reference solution.

/// A fully configured pizza.
#[derive(Debug, PartialEq)]
pub struct Pizza {
    pub size: u32,
    pub toppings: Vec<String>,
    pub cheese: bool,
}

/// The **builder** pattern: accumulate configuration through chainable
/// methods, then produce a `Pizza` with `build`.
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
    pub fn add_topping(mut self, topping: &str) -> Self {
        self.toppings.push(topping.to_string());
        self
    }

    /// Requests extra cheese.
    #[must_use]
    pub fn extra_cheese(mut self) -> Self {
        self.cheese = true;
        self
    }

    /// Produces the finished `Pizza`.
    #[must_use]
    pub fn build(self) -> Pizza {
        Pizza {
            size: self.size,
            toppings: self.toppings,
            cheese: self.cheese,
        }
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
        TcpConnection {
            address: self.address,
            sent: self.sent,
            _state: std::marker::PhantomData,
        }
    }
}

impl TcpConnection<Connected> {
    /// Sends a message; the message is recorded in the connection log.
    #[must_use]
    pub fn send(mut self, data: &str) -> TcpConnection<Connected> {
        self.sent.push(data.to_string());
        self
    }

    /// Closes the connection, moving back to the `Disconnected` state.
    #[must_use]
    pub fn disconnect(self) -> TcpConnection<Disconnected> {
        TcpConnection {
            address: self.address,
            sent: self.sent,
            _state: std::marker::PhantomData,
        }
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
        self.start.elapsed()
    }

    /// Stops the timer and returns the elapsed duration.
    pub fn stop(self) -> std::time::Duration {
        let elapsed = self.elapsed();
        println!("{} finished after {:?}", self.name, elapsed);
        elapsed
    }
}

impl Drop for LogTimer {
    fn drop(&mut self) {
        println!("{} dropped after {:?}", self.name, self.elapsed());
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
pub fn walk(node: &Node, visitor: &mut dyn Visitor) {
    match node {
        Node::Number(n) => visitor.visit_number(*n),
        Node::Text(s) => visitor.visit_text(s),
        Node::List(items) => {
            for item in items {
                walk(item, visitor);
            }
        }
    }
}
