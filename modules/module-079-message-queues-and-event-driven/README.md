# Module 079: Message Queues & Event-Driven Systems

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 45–75 min
**Prerequisites:** Module 021 (Closures), Module 033 (Channels)

## Learning Objectives
- Implement a generic in-memory publish/subscribe (pub/sub) event bus
- Understand the pub/sub pattern: topics, subscribers, and events
- Use closures and trait objects to store subscriber callbacks
- Recognise when pub/sub is the right architectural choice vs. direct RPC

## Why This Matters
Event-driven architecture is everywhere in production Rust: Kafka consumers/writers, Redis pub/sub, `tokio::sync::broadcast`, and in-process event buses in Axum/Actix-web apps. The pub/sub pattern decouples producers from consumers — services can emit events without knowing who's listening. This is the architectural pattern behind microservice communication, audit logging, real-time notifications, and CQRS/event sourcing.

## Concept

In a traditional request-response system, a caller knows exactly who it's talking to:

```
OrderService → PaymentService.process(order)  // tightly coupled
```

In an event-driven system, the caller emits an event and doesn't care who handles it:

```
OrderService → publish("order.created", event)  // loosely coupled
                ↓           ↓           ↓
          PaymentSvc   AuditLog   NotificationSvc
```

### The pub/sub pattern

Pub/sub (publish/subscribe) has three components:

1. **Topics**: named channels (e.g., `"order.created"`, `"user.deleted"`)
2. **Publishers**: code that emits events to a topic
3. **Subscribers**: code that registers interest in a topic and receives events

### Building an in-memory event bus

The simplest implementation is a HashMap keyed by topic name, storing a list of handler closures:

```rust
use std::collections::HashMap;

struct PubSub<T> {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&T)>>>,
}
```

Each subscriber is a `Box<dyn Fn(&T)>` — a heap-allocated closure that takes an event reference and does something with it. The generic `T` makes our bus work with any event type.

```rust
impl<T: 'static> PubSub<T> {
    fn subscribe(&mut self, topic: &str, handler: impl Fn(&T) + 'static) {
        self.subscribers
            .entry(topic.to_string())
            .or_default()
            .push(Box::new(handler));
    }
}
```

The `+ 'static` bound is needed because we're storing the closure in a `Box` with no lifetime — it must own all its captured data or hold only static references.

### Publishing events

Publishing iterates over all handlers for a topic and calls each one:

```rust
fn publish<T>(ps: &PubSub<T>, topic: &str, event: &T) {
    if let Some(handlers) = ps.subscribers.get(topic) {
        for handler in handlers {
            handler(event);
        }
    }
}
```

This calls every subscriber synchronously. In a real async system, you'd spawn tasks or send messages over channels instead.

### Using the event bus

```rust
let mut bus: PubSub<String> = PubSub::new();

// Subscribe with a closure that captures external state
let received: Rc<RefCell<Vec<String>>> = Rc::new(RefCell::new(vec![]));
let r = received.clone();
bus.subscribe("orders", move |event| {
    r.borrow_mut().push(event.clone());
});

// Publish an event
publish(&bus, "orders", &"order-42".to_string());

// The subscriber recorded it
assert_eq!(&*received.borrow(), &vec!["order-42".to_string()]);
```

Notice the pattern for testing: `Rc<RefCell<Vec<T>>>` lets the closure capture a reference-counted, mutably-borrowed vector. After publishing, the test reads from the vector to verify the subscriber was called.

### Design trade-offs

| Aspect | In-memory pub/sub | Redis pub/sub | Kafka |
|--------|-------------------|---------------|-------|
| Persistence | ❌ (lost on crash) | ❌ (ephemeral) | ✅ (disk-backed) |
| Delivery | Synchronous | Network (async) | Network (async) |
| Ordering | Call order | Best-effort | Partition-ordered |
| Scale | Single process | Multi-process | Multi-machine |

Our implementation is in-memory and single-process — perfect for decoupling components within one application (e.g., a GUI app or a CLI tool). For multi-service architectures, you'd swap in Redis or Kafka.

### When to use pub/sub

- **Decoupling**: the payment service shouldn't know about the audit logger. Publish an event and let both subscribe.
- **One-to-many**: a single event triggers multiple independent actions.
- **Extensibility**: add new subscribers without modifying the publisher.
- **Audit trails**: every state change emits an event consumed by an audit logger.
- **CQRS**: commands update state, queries read state — events bridge the two.

### When NOT to use pub/sub

- **Request-response**: if the caller needs a response, use direct RPC, not pub/sub.
- **Guaranteed delivery**: in-memory pub/sub loses events on crash. For reliability, use a persistent message queue.
- **Ordering-dependent workflows**: if handler A must run before handler B, pub/sub's unordered dispatch won't work.

### From in-memory to production

Real production event buses extend this pattern with:
- **Async dispatch**: `tokio::sync::broadcast` for async subscribers
- **Persistence**: write events to a WAL before publishing
- **Back-pressure**: bounded channels prevent OOM when publishers outpace consumers
- **Dead-letter queues**: events that can't be processed go to a retry queue
- **Schema evolution**: events use protobuf/serde with versioned schemas

But the core pattern — a topic → subscriber map with callback dispatch — remains the same.

## Common Pitfalls
- **Closure lifetime issues**: `Fn(&T)` captures `&T` — don't try to store `T` by value unless you clone it. Use `move` closures when capturing external data.
- **Deadlocks with RefCell**: if a subscriber tries to modify the bus while it's being iterated (e.g., subscribing during a publish), you'll get a runtime panic. In our design, `publish` takes `&PubSub` (immutable), preventing this.
- **Unbounded growth**: subscribers that aren't removed can accumulate, causing memory leaks. Production systems need unsubscribe mechanisms.
- **Assuming ordered delivery**: multiple subscribers on the same topic are called sequentially, but you shouldn't rely on their relative order.

## Key Terms
- **Pub/sub**: a messaging pattern where publishers send events to topics without knowing the subscribers
- **Topic**: a named channel that events are published to and subscribed from
- **Event**: a piece of data representing something that happened (order created, user deleted)
- **Event bus**: the infrastructure that routes events from publishers to subscribers
- **Observer pattern**: the OOP ancestor of pub/sub (subjects notify observers directly)
- **Decoupling**: separating components so changes in one don't require changes in another

## Exercise

In `exercises/`, fill in the `TODO(module-079)` markers to:

1. **`PubSub::new`** — initialise the subscriber map
2. **`PubSub::subscribe`** — register a handler closure for a topic
3. **`publish`** — call all subscribers for a topic with the event

Run `cargo test -p module-079-exercises` to verify.

## Further Reading
- [Observer pattern (Rust Design Patterns)](https://rust-unofficial.github.io/patterns/patterns/behavioural/observer.html)
- [tokio::sync::broadcast docs](https://docs.rs/tokio/latest/tokio/sync/broadcast/index.html)
- [Apache Kafka (event streaming platform)](https://kafka.apache.org/)
- [Redis Pub/Sub](https://redis.io/docs/latest/develop/interact/pubsub/)
