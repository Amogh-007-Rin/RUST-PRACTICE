//! Module 079: Message Queues & Event-Driven Systems — reference solution.

#![allow(clippy::new_without_default)]
#![allow(clippy::type_complexity)]

use std::collections::HashMap;

pub struct PubSub<T> {
    subscribers: HashMap<String, Vec<Box<dyn Fn(&T)>>>,
}

impl<T> PubSub<T> {
    pub fn new() -> Self {
        Self {
            subscribers: HashMap::new(),
        }
    }
}

impl<T: 'static> PubSub<T> {
    pub fn subscribe(&mut self, topic: &str, handler: impl Fn(&T) + 'static) {
        self.subscribers
            .entry(topic.to_string())
            .or_default()
            .push(Box::new(handler));
    }
}

pub fn publish<T>(ps: &PubSub<T>, topic: &str, event: &T) {
    if let Some(handlers) = ps.subscribers.get(topic) {
        for handler in handlers {
            handler(event);
        }
    }
}
