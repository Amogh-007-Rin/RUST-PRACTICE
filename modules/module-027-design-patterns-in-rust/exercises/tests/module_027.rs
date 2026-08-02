use module_027_exercises::{walk, LogTimer, Node, Pizza, PizzaBuilder, TcpConnection, Visitor};

#[test]
fn pizza_builder_builds_the_configured_pizza() {
    let pizza = PizzaBuilder::new(12)
        .add_topping("pepperoni")
        .add_topping("olives")
        .extra_cheese()
        .build();
    assert_eq!(
        pizza,
        Pizza {
            size: 12,
            toppings: vec!["pepperoni".to_string(), "olives".to_string()],
            cheese: true,
        }
    );
}

#[test]
fn pizza_builder_applies_defaults() {
    let plain = PizzaBuilder::new(10).build();
    assert_eq!(
        plain,
        Pizza {
            size: 10,
            toppings: Vec::new(),
            cheese: false,
        }
    );
}

#[test]
fn typestate_connection_chain() {
    let connection = TcpConnection::new("127.0.0.1:8080")
        .connect()
        .send("hello")
        .send("world");
    assert_eq!(connection.address(), "127.0.0.1:8080");
    assert_eq!(connection.sent_messages(), &["hello", "world"]);

    let disconnected = connection.disconnect();
    assert_eq!(disconnected.address(), "127.0.0.1:8080");
    assert_eq!(disconnected.sent_messages().len(), 2);
}

#[test]
fn raii_timer_measures_elapsed() {
    let timer = LogTimer::start("work");
    assert!(timer.elapsed() >= std::time::Duration::ZERO);

    let elapsed = timer.stop();
    assert!(elapsed >= std::time::Duration::ZERO);
}

#[test]
fn raii_timer_reports_when_dropped() {
    let timer = LogTimer::start("dropped");
    drop(timer);
}

struct Collector {
    numbers: Vec<i32>,
    texts: Vec<String>,
}

impl Visitor for Collector {
    fn visit_number(&mut self, n: i32) {
        self.numbers.push(n);
    }

    fn visit_text(&mut self, s: &str) {
        self.texts.push(s.to_string());
    }
}

#[test]
fn visitor_walks_the_tree_depth_first() {
    let root = Node::List(vec![
        Node::Number(7),
        Node::Text("hello".to_string()),
        Node::List(vec![Node::Number(3), Node::Text("nested".to_string())]),
        Node::Number(-1),
    ]);

    let mut collector = Collector {
        numbers: Vec::new(),
        texts: Vec::new(),
    };
    walk(&root, &mut collector);

    assert_eq!(collector.numbers, vec![7, 3, -1]);
    assert_eq!(collector.texts, vec!["hello", "nested"]);
}
