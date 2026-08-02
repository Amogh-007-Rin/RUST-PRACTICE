//! Module 086: Bevy ECS deep dive — exercise scaffold.
//!
//! Bevy's Entity-Component-System (ECS) architecture needs special
//! dependencies, so this crate implements a simplified ECS from scratch
//! using only `std`. You'll build `World`, `Entity`, component storage
//! with `HashMap<TypeId, Vec<Option<Box<dyn Any>>>>`, querying, and
//! system execution — the same patterns Bevy uses under the hood.
//!
//! Fill in every `// TODO(module-086)` below.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An entity in the world: just an id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    pub id: u64,
}

/// The ECS world. Owns all entities and components.
///
/// Component storage: a mapping from `TypeId` to a `Vec` of optional
/// component boxes, indexed by entity position in `entities`.
pub struct World {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    #[allow(dead_code)]
    next_id: u64,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        // TODO(module-086): initialize all fields.
        panic!("TODO(module-086): implement World::new");
    }

    /// Creates a new entity and returns its id.
    pub fn create_entity(&mut self) -> u64 {
        // TODO(module-086): allocate an entity with a unique id, push it
        // onto `self.entities`, and return the id. For each component type
        // already tracked in `self.components`, push a `None` so the Vecs
        // stay aligned.
        panic!("TODO(module-086): implement World::create_entity");
    }

    /// Attaches a component to an existing entity.
    pub fn add_component<T: 'static>(&mut self, entity_id: u64, component: T) {
        // TODO(module-086): find the entity's index, ensure the component
        // Vec for `TypeId::of::<T>()` is padded to match, then store
        // `Some(Box::new(component))` at the entity's index.
        let _ = (entity_id, component);
        panic!("TODO(module-086): implement World::add_component");
    }

    /// Returns a shared reference to entity's component `T`, or `None` if
    /// the entity doesn't have it.
    pub fn get_component<T: 'static>(&self, entity_id: u64) -> Option<&T> {
        // TODO(module-086): find the entity's index, look up the component
        // storage, downcast the Any box.
        let _ = entity_id;
        panic!("TODO(module-086): implement World::get_component");
    }

    /// Returns a mutable reference to entity's component `T`, or `None`.
    pub fn get_component_mut<T: 'static>(&mut self, entity_id: u64) -> Option<&mut T> {
        // TODO(module-086): same as `get_component` but downcast to `&mut T`.
        let _ = entity_id;
        panic!("TODO(module-086): implement World::get_component_mut");
    }

    /// Returns all `(entity_id, &T)` pairs for entities that have component `T`.
    pub fn query<T: 'static>(&self) -> Vec<(u64, &T)> {
        // TODO(module-086): iterate over entities, check if the component
        // slot for this type is `Some`, collect matching pairs.
        panic!("TODO(module-086): implement World::query");
    }

    /// Returns all `(entity_id, &mut T)` pairs for entities that have
    /// component `T`.
    pub fn query_mut<T: 'static>(&mut self) -> Vec<(u64, &mut T)> {
        // TODO(module-086): same as `query` but with mutable references.
        panic!("TODO(module-086): implement World::query_mut");
    }

    /// Returns all `(entity_id, &A, &B)` pairs for entities that have
    /// *both* components `A` and `B`.
    pub fn query_both<A: 'static, B: 'static>(&self) -> Vec<(u64, &A, &B)> {
        // TODO(module-086): find entities that have both A and B, collect
        // (id, &A, &B) tuples. Borrow-checker hint: get the component
        // Vecs once, then index into them.
        panic!("TODO(module-086): implement World::query_both");
    }

    /// Removes component `T` from the given entity.
    pub fn remove_component<T: 'static>(&mut self, entity_id: u64) {
        // TODO(module-086): replace the entity's slot for type T with None.
        let _ = entity_id;
        panic!("TODO(module-086): implement World::remove_component");
    }

    /// Runs a system function against the world. A system is any closure
    /// that takes `&mut World`.
    pub fn run_system(&mut self, f: impl FnOnce(&mut World)) {
        // TODO(module-086): call `f(self)`.
        let _ = f;
        panic!("TODO(module-086): implement World::run_system");
    }

    /// Returns the number of entities in the world.
    pub fn entity_count(&self) -> usize {
        // TODO(module-086): return `self.entities.len()`.
        panic!("TODO(module-086): implement World::entity_count");
    }

    /// Checks whether an entity with the given id exists.
    pub fn entity_exists(&self, entity_id: u64) -> bool {
        // TODO(module-086): search `self.entities` for a matching id.
        let _ = entity_id;
        panic!("TODO(module-086): implement World::entity_exists");
    }

    /// Returns the entity's index in the internal `entities` Vec.
    #[allow(dead_code)]
    fn find_entity_index(&self, entity_id: u64) -> Option<usize> {
        // TODO(module-086): linear scan for `entities.iter().position(...)`.
        let _ = entity_id;
        panic!("TODO(module-086): implement World::find_entity_index");
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Example components for a 2D game (used by the tests)
// ---------------------------------------------------------------------------

/// Position in 2D space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

/// Velocity for 2D movement.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Velocity {
    pub dx: f64,
    pub dy: f64,
}

/// A named tag for an entity (e.g. player, enemy).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

/// A health value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health(pub i32);

/// Moves every entity that has both Position and Velocity by
/// `dt * velocity` per tick.
///
/// ```text
///   new_x = x + dx * dt
///   new_y = y + dy * dt
/// ```
pub fn movement_system(world: &mut World, dt: f64) {
    // TODO(module-086): query all entities with Position + Velocity
    // (use `query_both`), then update each position.
    let _ = (world, dt);
    panic!("TODO(module-086): implement movement_system");
}
