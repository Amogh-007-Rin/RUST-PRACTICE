//! Module 085: Introduction to Game Development in Rust — Mini ECS (solution)

use std::any::{Any, TypeId};
use std::collections::HashMap;

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

trait ComponentStore: Any {
    #[allow(dead_code)]
    fn remove_entity(&mut self, entity: Entity);
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

impl<T: 'static> ComponentStore for HashMap<Entity, T> {
    fn remove_entity(&mut self, entity: Entity) {
        self.remove(&entity);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

#[allow(dead_code)]
pub struct World {
    entities: HashMap<u32, u32>,
    next_id: u32,
    component_stores: HashMap<TypeId, Box<dyn ComponentStore>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities: HashMap::new(),
            next_id: 0,
            component_stores: HashMap::new(),
        }
    }

    pub fn spawn(&mut self) -> Entity {
        let id = self.next_id;
        self.next_id += 1;
        self.entities.insert(id, 0);
        Entity { id, version: 0 }
    }

    pub fn despawn(&mut self, entity: Entity) -> bool {
        if self.entities.remove(&entity.id).is_some() {
            for store in self.component_stores.values_mut() {
                if let Some(map) = store
                    .as_any_mut()
                    .downcast_mut::<HashMap<Entity, Box<dyn Any>>>()
                {
                    map.remove(&entity);
                }
            }
            true
        } else {
            false
        }
    }

    pub fn insert<C: 'static>(&mut self, entity: Entity, component: C) {
        let type_id = TypeId::of::<C>();
        let store = self
            .component_stores
            .entry(type_id)
            .or_insert_with(|| Box::new(HashMap::<Entity, C>::new()));
        let map = store
            .as_any_mut()
            .downcast_mut::<HashMap<Entity, C>>()
            .expect("type mismatch");
        map.insert(entity, component);
    }

    pub fn remove<C: 'static>(&mut self, entity: Entity) -> Option<C> {
        let type_id = TypeId::of::<C>();
        let store = self.component_stores.get_mut(&type_id)?;
        let map = store
            .as_any_mut()
            .downcast_mut::<HashMap<Entity, C>>()
            .expect("type mismatch");
        map.remove(&entity)
    }

    pub fn get<C: 'static>(&self, entity: Entity) -> Option<&C> {
        let type_id = TypeId::of::<C>();
        let store = self.component_stores.get(&type_id)?;
        let map = store.as_any().downcast_ref::<HashMap<Entity, C>>()?;
        map.get(&entity)
    }

    pub fn get_mut<C: 'static>(&mut self, entity: Entity) -> Option<&mut C> {
        let type_id = TypeId::of::<C>();
        let store = self.component_stores.get_mut(&type_id)?;
        let map = store.as_any_mut().downcast_mut::<HashMap<Entity, C>>()?;
        map.get_mut(&entity)
    }

    pub fn is_alive(&self, entity: Entity) -> bool {
        self.entities
            .get(&entity.id)
            .is_some_and(|&v| v == entity.version)
    }

    pub fn query<C: 'static>(&self) -> Vec<(Entity, &C)> {
        let type_id = TypeId::of::<C>();
        let Some(store) = self.component_stores.get(&type_id) else {
            return Vec::new();
        };
        let Some(map) = store.as_any().downcast_ref::<HashMap<Entity, C>>() else {
            return Vec::new();
        };
        map.iter().map(|(&e, c)| (e, c)).collect()
    }

    pub fn query2<A: 'static, B: 'static>(&self) -> Vec<(Entity, &A, &B)> {
        let type_a = TypeId::of::<A>();
        let type_b = TypeId::of::<B>();
        let Some(store_a) = self.component_stores.get(&type_a) else {
            return Vec::new();
        };
        let Some(map_a) = store_a.as_any().downcast_ref::<HashMap<Entity, A>>() else {
            return Vec::new();
        };
        let Some(store_b) = self.component_stores.get(&type_b) else {
            return Vec::new();
        };
        let Some(map_b) = store_b.as_any().downcast_ref::<HashMap<Entity, B>>() else {
            return Vec::new();
        };
        map_a
            .keys()
            .filter(|e| map_b.contains_key(e))
            .filter_map(|&e| {
                let a = map_a.get(&e)?;
                let b = map_b.get(&e)?;
                Some((e, a, b))
            })
            .collect()
    }
}

impl Default for World {
    fn default() -> Self {
        Self::new()
    }
}

pub type SystemFn = Box<dyn Fn(&mut World)>;

pub struct SystemExecutor {
    systems: Vec<SystemFn>,
}

impl SystemExecutor {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system<F: Fn(&mut World) + 'static>(&mut self, system: F) {
        self.systems.push(Box::new(system));
    }

    pub fn run(&self, world: &mut World) {
        for system in &self.systems {
            system(world);
        }
    }
}

impl Default for SystemExecutor {
    fn default() -> Self {
        Self::new()
    }
}
