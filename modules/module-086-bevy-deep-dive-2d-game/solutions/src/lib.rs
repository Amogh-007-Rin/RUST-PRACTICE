//! Module 086: Bevy ECS deep dive — reference solution.
//!
//! Implements a simplified Entity-Component-System pattern using only `std`,
//! mirroring the core architecture of Bevy's ECS.

use std::any::{Any, TypeId};
use std::collections::HashMap;

/// An entity in the world: just an id.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Entity {
    pub id: u64,
}

/// The ECS world. Owns all entities and components.
pub struct World {
    pub entities: Vec<Entity>,
    pub components: HashMap<TypeId, Vec<Option<Box<dyn Any>>>>,
    next_id: u64,
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            entities: Vec::new(),
            components: HashMap::new(),
            next_id: 0,
        }
    }

    /// Creates a new entity and returns its id.
    pub fn create_entity(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.push(Entity { id });
        for slots in self.components.values_mut() {
            slots.push(None);
        }
        id
    }

    /// Attaches a component to an existing entity.
    pub fn add_component<T: 'static>(&mut self, entity_id: u64, component: T) {
        let idx = self.find_entity_index(entity_id).expect("entity not found");
        let type_id = TypeId::of::<T>();
        let slots = self
            .components
            .entry(type_id)
            .or_insert_with(|| (0..self.entities.len()).map(|_| None).collect());
        while slots.len() <= idx {
            slots.push(None);
        }
        slots[idx] = Some(Box::new(component));
    }

    /// Returns a shared reference to entity's component `T`, or `None`.
    pub fn get_component<T: 'static>(&self, entity_id: u64) -> Option<&T> {
        let idx = self.find_entity_index(entity_id)?;
        let type_id = TypeId::of::<T>();
        let slots = self.components.get(&type_id)?;
        slots
            .get(idx)?
            .as_ref()
            .map(|b| b.downcast_ref::<T>().unwrap())
    }

    /// Returns a mutable reference to entity's component `T`, or `None`.
    pub fn get_component_mut<T: 'static>(&mut self, entity_id: u64) -> Option<&mut T> {
        let idx = self.find_entity_index(entity_id)?;
        let type_id = TypeId::of::<T>();
        let slots = self.components.get_mut(&type_id)?;
        slots
            .get_mut(idx)?
            .as_mut()
            .map(|b| b.downcast_mut::<T>().unwrap())
    }

    /// Returns all `(entity_id, &T)` pairs for entities that have component `T`.
    pub fn query<T: 'static>(&self) -> Vec<(u64, &T)> {
        let type_id = TypeId::of::<T>();
        let Some(slots) = self.components.get(&type_id) else {
            return Vec::new();
        };
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                slots
                    .get(i)
                    .and_then(Option::as_ref)
                    .map(|b| (e.id, b.downcast_ref::<T>().unwrap()))
            })
            .collect()
    }

    /// Returns all `(entity_id, &mut T)` pairs for entities that have
    /// component `T`.
    pub fn query_mut<T: 'static>(&mut self) -> Vec<(u64, &mut T)> {
        let type_id = TypeId::of::<T>();
        let Some(slots) = self.components.get_mut(&type_id) else {
            return Vec::new();
        };
        let mut results = Vec::new();
        for (i, e) in self.entities.iter().enumerate() {
            if let Some(Some(b)) = slots.get_mut(i) {
                let ptr: *mut T = b.downcast_mut::<T>().unwrap();
                // SAFETY: Each entity has at most one component of type T,
                // so we produce at most one &mut T per entity — no aliasing.
                unsafe {
                    results.push((e.id, &mut *ptr));
                }
            }
        }
        results
    }

    /// Returns all `(entity_id, &A, &B)` pairs for entities that have
    /// *both* components `A` and `B`.
    pub fn query_both<A: 'static, B: 'static>(&self) -> Vec<(u64, &A, &B)> {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let slots_a = match self.components.get(&type_a) {
            Some(s) => s,
            None => return Vec::new(),
        };
        let slots_b = match self.components.get(&type_b) {
            Some(s) => s,
            None => return Vec::new(),
        };
        self.entities
            .iter()
            .enumerate()
            .filter_map(|(i, e)| {
                let a = slots_a.get(i).and_then(Option::as_ref)?;
                let b = slots_b.get(i).and_then(Option::as_ref)?;
                Some((
                    e.id,
                    a.downcast_ref::<A>().unwrap(),
                    b.downcast_ref::<B>().unwrap(),
                ))
            })
            .collect()
    }

    /// Removes component `T` from the given entity.
    pub fn remove_component<T: 'static>(&mut self, entity_id: u64) {
        if let Some(idx) = self.find_entity_index(entity_id) {
            let type_id = TypeId::of::<T>();
            if let Some(slots) = self.components.get_mut(&type_id) {
                if idx < slots.len() {
                    slots[idx] = None;
                }
            }
        }
    }

    /// Runs a system function against the world.
    pub fn run_system(&mut self, f: impl FnOnce(&mut World)) {
        f(self);
    }

    /// Returns the number of entities in the world.
    pub fn entity_count(&self) -> usize {
        self.entities.len()
    }

    /// Checks whether an entity with the given id exists.
    pub fn entity_exists(&self, entity_id: u64) -> bool {
        self.find_entity_index(entity_id).is_some()
    }

    /// Returns the entity's index in the internal `entities` Vec.
    fn find_entity_index(&self, entity_id: u64) -> Option<usize> {
        self.entities.iter().position(|e| e.id == entity_id)
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

// ---------------------------------------------------------------------------
// Example components for a 2D game
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

/// A named tag for an entity.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Name(pub String);

/// A health value.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Health(pub i32);

/// Moves every entity that has both Position and Velocity by
/// `dt * velocity` per tick.
pub fn movement_system(world: &mut World, dt: f64) {
    let updates: Vec<(u64, f64, f64)> = world
        .query_both::<Position, Velocity>()
        .into_iter()
        .map(|(id, pos, vel)| (id, pos.x + vel.dx * dt, pos.y + vel.dy * dt))
        .collect();
    for (id, new_x, new_y) in updates {
        if let Some(pos) = world.get_component_mut::<Position>(id) {
            pos.x = new_x;
            pos.y = new_y;
        }
    }
}
