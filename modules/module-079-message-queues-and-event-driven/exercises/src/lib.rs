//! Module 079: Message Queues & Event-Driven Systems — exercise scaffold.
//!
//! Implement an in-memory pub/sub event bus.

#![allow(clippy::new_without_default)]

use std::marker::PhantomData;

/// An in-memory publish/subscribe event bus.
///
/// Subscribers are closures stored by topic name.
/// All handlers for a topic are called when that topic is published to.
pub struct PubSub<T> {
    // TODO(module-079): replace PhantomData with the real field:
    // `subscribers: HashMap<String, Vec<Box<dyn Fn(&T)>>>`
    _phantom: PhantomData<T>,
}

impl<T> PubSub<T> {
    /// Create an empty PubSub bus.
    pub fn new() -> Self {
        // TODO(module-079): implement
        panic!("TODO(module-079): implement PubSub::new")
    }
}

impl<T: 'static> PubSub<T> {
    /// Subscribe a handler to a topic. The handler will be called each time
    /// an event is published to that topic.
    pub fn subscribe(&mut self, _topic: &str, _handler: impl Fn(&T) + 'static) {
        // TODO(module-079): insert the handler into the subscribers map
        // for the given topic. Create the entry list if it doesn't exist.
        panic!("TODO(module-079): implement subscribe")
    }
}

/// Publish an event to all subscribers of the given topic.
///
/// If there are no subscribers for the topic, this is a no-op.
pub fn publish<T>(_ps: &PubSub<T>, _topic: &str, _event: &T) {
    // TODO(module-079): look up the topic in ps.subscribers.
    // Call each handler closure with the event reference.
    panic!("TODO(module-079): implement publish")
}
