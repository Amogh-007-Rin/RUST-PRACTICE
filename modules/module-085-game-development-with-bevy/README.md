# Module 085: Introduction to Game Development in Rust

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 60–90 min
**Prerequisites:** Module 081 (Introduction to WebAssembly)

## Learning Objectives
- You will be able to explain the Entity-Component-System (ECS) pattern and why it's the dominant architecture for modern game engines
- You will be able to build a minimal ECS from scratch in pure Rust: entities as versioned IDs, components stored in type-erased maps, systems as ordered closures
- You will understand how Bevy (and other ECS engines) realize these concepts with type-safe APIs and parallel system execution
- You will be able to reason about the tradeoffs between ECS and traditional OOP game architectures

## Why This Matters
Every major modern game engine — Bevy, Unity DOTS, Unreal's Mass Entity, Fleet — uses some variant of ECS. The pattern solves the "game object" problem: in OOP, you'd model a player as `class Player extends Entity` with `Health`, `Position`, `Sprite` fields. But what about an NPC with health and position but no sprite? What about a pickup with position and sprite but no health? Inheritance explodes into a combinatorial nightmare. ECS sidesteps this by separating identity (entity), data (components), and behavior (systems). This module builds the core ECS machinery from scratch so you understand what Bevy does under the hood.

## Concept

### The ECS Pattern

**Entities** are just unique IDs. They have no data, no behavior — they're glorified indices. In our implementation, an `Entity` is a `(id, version)` pair. The version lets us detect stale handles: if you despawn entity 5 and spawn a new one, the old handle `(id=5, version=0)` won't match the new `(id=5, version=1)`.

**Components** are plain data structs. `Position { x: f32, y: f32 }`, `Velocity { dx: f32, dy: 1.0 }`, `Health(i32)`. They have no methods — just data. Components are stored in the `World`, keyed by entity.

**Systems** are functions that operate on the `World`. A movement system might query all entities with `Position` and `Velocity`, then update each position by its velocity. Systems are run in order each "tick" (frame).

```
+------------------+     +------------------+     +------------------+
| Entity 1         |     | Entity 2         |     | Entity 3         |
| Position(0, 0)   |     | Position(5, 5)   |     | Position(10, 0)  |
| Velocity(1, 0)   |     | Velocity(-1, 0)  |     | Health(100)      |
+------------------+     +------------------+     +------------------+

Movement System: for each (Entity, Position, Velocity), pos += vel
  -> Entity 1: Position(1, 0)
  -> Entity 2: Position(4, 5)
  -> Entity 3: (no Velocity, skipped)
```

### The World: Type-Erased Component Storage

The `World` is the central data structure. It holds:
- A map of entity IDs to versions (for liveness checks)
- A map of `TypeId -> Box<dyn Any>`, where each value is a `HashMap<Entity, C>` for some component type `C`

When you call `world.insert(entity, Position { x: 0.0, y: 0.0 })`, the `World`:
1. Computes `TypeId::of::<Position>()`
2. Looks up (or creates) the store for that type
3. Downcasts the `Box<dyn Any>` to `HashMap<Entity, Position>`
4. Inserts `(entity, component)` into the map

This is type erasure: the `World` doesn't know about `Position` at compile time. It only knows "there's some type with this TypeId, and I can downcast to it." Bevy uses a similar approach internally, though with more sophisticated storage (archetypes, sparse sets) for better cache performance.

### Queries: Finding Entities with Specific Components

A query asks: "give me all entities that have component `C`" (or `A` AND `B`, etc.). Our `query::<C>()` method:
1. Looks up the store for `TypeId::of::<C>()`
2. Downcasts to `HashMap<Entity, C>`
3. Returns all `(Entity, &C)` pairs

`query2::<A, B>()` intersects two stores: it iterates the smaller one, checks if each entity exists in the other, and yields `(Entity, &A, &B)` triples.

This is the simplest possible query implementation. Real ECS engines use archetypes (groups of entities with identical component sets) or sparse sets (dense arrays indexed by entity) for better iteration performance. Our `HashMap`-based approach is O(n) per query, which is fine for learning but would be too slow for a game with thousands of entities.

### Systems: Behavior as Data

A system is just `Fn(&mut World)`. You register systems with a `SystemExecutor`, which runs them in order each tick. This is the simplest possible system scheduler — no parallelism, no dependency analysis, no resource access tracking.

Bevy's system scheduler is far more sophisticated: it analyzes which components each system reads/writes, builds a dependency graph, and runs independent systems in parallel across multiple threads. Our sequential executor is the conceptual foundation; Bevy adds the parallelism and safety checks on top.

### Why ECS Over OOP for Games?

Consider a traditional OOP approach:
```rust,ignore
struct Entity {
    position: Position,
    // ... every possible field, most unused by most entities
}

impl Entity {
    fn update(&mut self) {
        // Giant if/else or match on "what kind of entity am I?"
    }
}
```

Problems:
- **Wasted memory**: every entity carries fields it doesn't use
- **Cache misses**: entities are scattered in memory; iterating all positions means jumping around the heap
- **Rigid hierarchy**: adding a new "kind" of entity means modifying the base class or creating a new subclass

ECS solves all three:
- **Memory efficiency**: entities only store the components they have
- **Cache-friendly**: all `Position` components are contiguous in memory (in a `HashMap`, they're at least in the same allocation; in Bevy's archetype storage, they're in a dense `Vec`)
- **Flexible composition**: want a new entity type? Just combine existing components. No inheritance, no class hierarchy.

### ASCII: ECS World Layout

```
World
+------------------------------------------+
| entities: HashMap<u32, u32>              |
|   0 -> 0  (entity 0, version 0)          |
|   1 -> 0  (entity 1, version 0)          |
|   2 -> 0  (entity 2, version 0)          |
+------------------------------------------+
| component_stores: HashMap<TypeId, ...>   |
|                                          |
| TypeId(Position) -> HashMap<Entity, Pos> |
|   Entity(0,0) -> Position(0, 0)          |
|   Entity(1,0) -> Position(5, 5)          |
|   Entity(2,0) -> Position(10, 0)         |
|                                          |
| TypeId(Velocity) -> HashMap<Entity, Vel> |
|   Entity(0,0) -> Velocity(1, 0)          |
|   Entity(1,0) -> Velocity(-1, 0)         |
|                                          |
| TypeId(Health) -> HashMap<Entity, Health>|
|   Entity(2,0) -> Health(100)             |
+------------------------------------------+
```

## Common Pitfalls
- **Forgetting to clean up components on despawn.** If you remove an entity from `entities` but not from the component stores, you leak memory and queries return stale data. Always iterate all stores and remove the entity.
- **Type confusion in `downcast_mut`.** If you store a `HashMap<Entity, Position>` but downcast to `HashMap<Entity, Velocity>`, you get `None`. Always use `TypeId::of::<C>()` consistently.
- **Stale entity handles.** If you despawn entity 5 and spawn a new one, the old `Entity { id: 5, version: 0 }` is stale. The new entity is `Entity { id: 5, version: 1 }`. Always check `is_alive` before accessing components.
- **Borrow checker fights in systems.** If a system needs to read `Position` and write `Velocity` for the same entity, you can't hold `&Position` and `&mut Velocity` simultaneously. In our mini ECS this isn't a problem (we use `HashMap`), but in Bevy's archetype storage, you'd use `QuerySet` or split queries.
- **Not handling missing components gracefully.** `world.get::<C>(entity)` returns `Option<&C>`. Don't `.unwrap()` unless you're certain the entity has that component.

## Key Terms
- **Entity:** A unique ID (with version for staleness detection). No data, no behavior.
- **Component:** A plain data struct attached to an entity. No methods.
- **System:** A function that operates on the `World`, typically via queries.
- **World:** The central data structure holding all entities and components.
- **TypeId:** A Rust standard library type that uniquely identifies a type at runtime. Used for type-erased storage.
- **Query:** A request for all entities with specific component types.
- **Archetype:** (Bevy-specific) A group of entities with identical component sets, stored contiguously for cache-friendly iteration.

## Exercise

In `exercises/src/lib.rs`, implement the `World` and `SystemExecutor`:

1. `World::new` — initialize empty maps
2. `World::spawn` — allocate a new entity ID, store version 0
3. `World::despawn` — remove entity from `entities` and all component stores
4. `World::insert` — get or create the component store for type `C`, insert the component
5. `World::remove` — remove a component from an entity
6. `World::get` / `World::get_mut` — look up a component on an entity
7. `World::is_alive` — check if an entity exists with matching version
8. `World::query` — iterate all entities with component `C`
9. `World::query2` — iterate all entities with both `A` and `B`
10. `SystemExecutor::new` — initialize empty system list
11. `SystemExecutor::add_system` — push a system onto the list
12. `SystemExecutor::run` — call each system in order

The tests verify:
- Spawning and despawning entities
- Inserting, getting, and removing components
- Querying single and dual components
- Running systems that modify components
- Cleaning up components on despawn

## Running This Module's Tests

```bash
cargo test -p module-085-exercises    # must fail (TODOs not filled)
cargo test -p module-085-solutions    # must pass
```

## Further Reading
- [Bevy ECS documentation](https://docs.rs/bevy/latest/bevy/ecs/index.html) — the real thing
- [The Rust Book §17 — Object-Oriented Programming](https://doc.rust-lang.org/book/ch17-00-oop.html) — contrasts OOP with data-oriented design
- [ECS back and forth](https://github.com/SanderMertens/ecs-faq) — deep dive into ECS tradeoffs
- [Bevy's archetype storage](https://github.com/bevyengine/bevy/blob/main/crates/bevy_ecs/src/storage/mod.rs) — how Bevy actually stores components
