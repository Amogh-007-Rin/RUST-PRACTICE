//! Module 085: Introduction to Game Development in Rust — Mini ECS
//!
//! You will build a minimal Entity-Component-System framework from scratch
//! in pure std Rust. This teaches the core concepts that Bevy (and every
//! other ECS game engine) builds upon.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// A unique identifier for an entity. Entities are just IDs — they have no
/// data of their own. Components are stored separately, keyed by entity.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Entity {
    id: u32,
    version: u32,
}

impl Entity {
    pub fn id(self) -> u32 {
        self.id
    }

    pub fn version(self) -> u32 {
        self.version
    }
}

/// The World holds all entities and their components. Components are stored
/// in a type-erased map: `HashMap<TypeId, Box<dyn Any>>` where each value is
/// a `HashMap<Entity, C>` for some component type `C`.
#[allow(dead_code)]
pub struct World {
    entities: HashMap<u32, u32>,
    next_id: u32,
    component_stores: HashMap<TypeId, Box<dyn Any>>,
}

impl World {
    pub fn new() -> Self {
        // TODO(module-085): Create a new empty World.
        panic!("not implemented")
    }

    /// Spawn a new entity and return its `Entity` handle.
    pub fn spawn(&mut self) -> Entity {
        // TODO(module-085): Allocate a new entity ID, store version 0.
        panic!("not implemented")
    }

    /// Despawn an entity, removing it and all its components.
    pub fn despawn(&mut self, entity: Entity) -> bool {
        // TODO(module-085): Remove the entity from `entities` and from every
        // component store. Return true if the entity existed.
        let _ = entity;
        panic!("not implemented")
    }

    /// Add a component to an entity. If the entity already has a component
    /// of this type, replace it.
    pub fn insert<C: 'static>(&mut self, entity: Entity, component: C) {
        // TODO(module-085): Get or create the store for type C, then insert.
        let _ = (entity, component);
        panic!("not implemented")
    }

    /// Remove a component from an entity. Returns the component if it existed.
    pub fn remove<C: 'static>(&mut self, entity: Entity) -> Option<C> {
        // TODO(module-085): Get the store for type C, remove the entry.
        let _ = entity;
        panic!("not implemented")
    }

    /// Get a reference to a component on an entity.
    pub fn get<C: 'static>(&self, entity: Entity) -> Option<&C> {
        // TODO(module-085): Look up the store for type C, then look up the entity.
        let _ = entity;
        panic!("not implemented")
    }

    /// Get a mutable reference to a component on an entity.
    pub fn get_mut<C: 'static>(&mut self, entity: Entity) -> Option<&mut C> {
        // TODO(module-085): Same as `get` but mutable.
        let _ = entity;
        panic!("not implemented")
    }

    /// Check if an entity is alive (has been spawned and not despawned).
    pub fn is_alive(&self, entity: Entity) -> bool {
        // TODO(module-085): Check if the entity's ID exists with matching version.
        let _ = entity;
        panic!("not implemented")
    }

    /// Iterate over all entities that have a component of type `C`, yielding
    /// `(Entity, &C)` pairs.
    pub fn query<C: 'static>(&self) -> Vec<(Entity, &C)> {
        // TODO(module-085): Get the store for C, iterate all entries.
        panic!("not implemented")
    }

    /// Iterate over all entities that have BOTH components of type `A` and `B`,
    /// yielding `(Entity, &A, &B)` triples.
    pub fn query2<A: 'static, B: 'static>(&self) -> Vec<(Entity, &A, &B)> {
        // TODO(module-085): Intersect the two component stores.
        panic!("not implemented")
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

/// A system is a function that takes a `&mut World` and does something.
/// Systems are run in order by the `SystemExecutor`.
pub type SystemFn = Box<dyn Fn(&mut World)>;

/// The SystemExecutor runs a list of systems in order each "tick".
#[allow(dead_code)]
pub struct SystemExecutor {
    systems: Vec<SystemFn>,
}

impl SystemExecutor {
    pub fn new() -> Self {
        // TODO(module-085): Create an empty executor.
        panic!("not implemented")
    }

    /// Add a system to the executor.
    pub fn add_system<F: Fn(&mut World) + 'static>(&mut self, system: F) {
        // TODO(module-085): Push the system onto the list.
        let _ = system;
        panic!("not implemented")
    }

    /// Run all systems once in order.
    pub fn run(&self, world: &mut World) {
        // TODO(module-085): Call each system with `world`.
        let _ = world;
        panic!("not implemented")
    }
}

impl Default for SystemExecutor {
    fn default() -> Self {
        Self::new()
    }
}
