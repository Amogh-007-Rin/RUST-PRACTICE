# Module 086: Bevy Deep Dive — ECS Patterns

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 90–120 min
**Prerequisites:** Module 085 (intro to Bevy & ECS). Modules 001–080.

## Learning Objectives

- You will be able to explain the Entity-Component-System pattern and why game engines prefer it to inheritance-based object models.
- You will be able to implement a simplified ECS from scratch — entities, components stored by type, and queries that find entities with specific component combinations.
- You will be able to write a system that operates on all entities matching a component query, and iterate it across multiple ticks.
- You will be able to explain how Bevy's real ECS differs from the simplified model (archetypes, sparse sets, `Commands`, `Query<T>` with `&mut` splitting).

## Why This Matters

Bevy is the dominant Rust game engine, and its entire architecture is built on ECS. Every frame, Bevy dispatches dozens of systems — movement, rendering, collision detection, AI — each of which queries for entities with specific component combinations and mutates them. The pattern is so powerful that it has spread beyond game dev: data-oriented design libraries in Rust (like `hecs` and `legion`) and even some embedded databases use ECS-style component storage. Understanding ECS from the inside — building `World`, component storage, and queries by hand — means you will never be confused by Bevy's `Query<&Transform, &mut Velocity>` parameters or wonder why a system isn't running on the entities you expect.

## Concept

### Why ECS defeats inheritance

In a traditional OOP game engine, you might model objects like this:

```
GameObject
  ├── Character
  │     ├── Player
  │     └── Enemy
  └── Prop
        ├── BreakableProp
        └── StaticProp
```

The problem: what happens when you want a prop that *moves*? Or an enemy that's *breakable*? You either duplicate code, mix in functionality through deep inheritance chains, or end up with monstrous base classes. ECS solves this by separating identity (Entity), data (Components), and behavior (Systems):

- **Entity** — just an ID. No data, no behavior. A `u64`.
- **Component** — a plain data struct attached to an entity. `Position { x, y }`, `Velocity { dx, dy }`, `Health(100)`.
- **System** — a function that queries entities matching a component signature and mutates their components.

An entity has whatever components you attach to it. A "player" is just an entity with `Position`, `Velocity`, `Sprite`, and `PlayerControlled`. An "enemy" is an entity with `Position`, `Velocity`, `Sprite`, and `AI`. A "breakable wall" is an entity with `Position`, `Sprite`, and `Breakable`. No inheritance — just composition through components.

### The data model this module builds

The core data structure is a `World` that stores:

```
World {
    entities: [Entity { id: 0 }, Entity { id: 1 }, Entity { id: 2 }, ...],
    components: {
        TypeId::of::<Position>()  → [Some(Pos{x:0,y:0}),  None,              Some(Pos{x:5,y:5})],
        TypeId::of::<Velocity>()  → [Some(Vel{dx:1,dy:2}), None,              None                  ],
        TypeId::of::<Health>()    → [None,                 Some(Health(100)), None                  ],
    }
}
```

Each component type gets its own `Vec<Option<Box<dyn Any>>>` that is always the same length as the entities Vec. Index 0 maps to the first entity, index 1 to the second, and so on. A `None` means "this entity doesn't have this component." When you create an entity, every component Vec gets a `None` appended so they stay aligned.

Under the hood, `TypeId` is a unique identifier the compiler assigns to every type, `Box<dyn Any>` provides type erasure, and `downcast_ref::<T>()` recovers the concrete type when you query. This is exactly the pattern `std::any` was designed for — heterogeneous collections with runtime type recovery.

The key operations are:

```rust
// Querying: find all entities that have a Position
let results: Vec<(u64, &Position)> = world.query::<Position>();

// Multi-component query: entities with both Position AND Velocity
let results: Vec<(u64, &Position, &Velocity)> = world.query_both::<Position, Velocity>();

// Running a system
world.run_system(|world| {
    for (id, pos, vel) in world.query_both::<Position, Velocity>() {
        // modify world through a mutable reference
    }
});
```

### The movement system

A classic ECS system is the movement update — for every entity with both `Position` and `Velocity`, advance the position:

```
position.x += velocity.dx * delta_time
position.y += velocity.dy * delta_time
```

In Bevy, this looks like:

```rust,ignore
fn movement_system(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity)>) {
    for (mut transform, velocity) in query.iter_mut() {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
```

In our simplified ECS, the same logic runs against `query_both::<Position, Velocity>()` with manual iteration and delta-time multiplication. The pattern is identical — Bevy just wraps it in a framework that handles scheduling, parallelism, and change-tracking.

### How Bevy's real ECS differs

The simplified model in this exercise stores components in parallel `Vec<Option<Box<dyn Any>>>` per type. This works but has overhead — every query iterates *all* entities, checking `is_some()` for every slot. Bevy optimizes this with **archetypes**: entities with the same set of components are grouped into the same contiguous storage, so a `Query<(&Transform, &Velocity)>` iterates only over entities that *definitely* have both, with no `is_some()` checks and great cache locality.

Bevy also adds:

- **`Commands`** — deferred world mutations (spawn entities, add/remove components) that queue up during a system and apply at the end, avoiding borrow conflicts with the `Query` iterator.
- **Automatic `&mut` splitting** — Bevy's scheduler analyzes system queries at startup and runs non-conflicting systems in parallel (two systems reading `Position` can run simultaneously; a system writing `Position` cannot).
- **Change detection** — Bevy tracks whether a component was modified since the last frame, so systems can conditionally update only changed entities.
- **Resources** — singleton values not attached to any entity (e.g., `Time`, `AssetServer`, custom global state), stored separately from component storage.

The exercise intentionally uses the simplest correct model. Once you understand how the parallel Vecs, `TypeId`, and downcasting work, Bevy's archetypes become a performance optimization rather than mysterious magic.

### The ECS loop

Every frame of a Bevy game follows this cycle:

```
┌─────────────────────────────────────────────────┐
│  1. Event dispatch (input, window, custom)       │
│  2. System execution (movement, physics, AI, UI) │
│  3. Command application (spawn/despawn entities) │
│  4. Render (draw all entities with meshes)       │
└─────────────────────────────────────────────────┘
```

The exercise captures the system execution step — step 2 above — where your `movement_system` advances positions and any other system reads or mutates components based on queries.

## Common Pitfalls

- **Storing components as `Box<dyn Any>` then forgetting to downcast.** When you store `Box::new(Position { x: 0.0, y: 0.0 })` and later try to read it back, you must call `downcast_ref::<Position>()` — type erasure is not automatic recovery.
- **Mismatched Vec lengths.** When adding an entity, every component type's Vec must grow by one `None`. When adding a component to an existing entity, the Vec for that type must be long enough (pad with `None` if needed).
- **Method 2 in `get_component_mut` exclusive borrow issues.** In the simplified model, calling `query_mut::<T>()` locks the entire `Component` Vec for `T` mutably — you cannot call another method that borrows `World` simultaneously. Bevy's real ECS handles this through archetypes and careful borrowing, but in the simplified version you must collect query results into a scratch Vec before mutating.
- **Querying for a component type that has never been added anywhere.** The `components` HashMap won't have an entry for that `TypeId` — handle missing keys gracefully rather than panicking.

## Key Terms

- **Entity:** a unique identifier (u64) with no data or behavior. Think of it as the "row key" in component storage.
- **Component:** a plain data struct attached to an entity. Stored in a per-type vector indexed by entity position.
- **System:** a function that queries entities with specific component combinations and mutates them. The only place behavior lives.
- **World:** the container holding all entities and components. The central data structure everything operates on.
- **TypeId:** a compile-time unique identifier for every type, used as the key in `components: HashMap<TypeId, Vec<...>>`.
- **Archetype (conceptual):** a group of entities sharing the same set of component types — Bevy's optimization over the parallel-Vec model.
- **Query:** the filter expression (`Query<&Position, &mut Velocity>`) that determines which entities a system operates on.
- **Commands:** deferred mutations (spawning, despawning, adding components) that apply after all systems in a stage complete.

## Exercise

In `exercises/src/lib.rs` you implement a simplified ECS. The scaffold provides `World`, `Entity`, and example components (`Position`, `Velocity`, `Name`, `Health`). Fill in the `// TODO(module-086)` stubs:

1. **`World::new`** — initialize empty Vecs and HashMaps.
2. **`World::create_entity`** — assign a unique id, push the entity, pad all component Vecs.
3. **`World::add_component`** — find the entity's index, ensure the component Vec is long enough, store the boxed component.
4. **`World::get_component`** and **`get_component_mut`** — look up and downcast the value.
5. **`World::query`** and **`query_mut`** — iterate all entities, collect those with matching components.
6. **`World::query_both`** — find entities that have *both* of two component types.
7. **`World::remove_component`** — set the slot to `None`.
8. **`World::run_system`** — invoke the closure with `&mut World`.
9. **`World::entity_count`**, **`entity_exists`**, **`find_entity_index`** — basic entity bookkeeping.
10. **`movement_system`** — query all `(Position, Velocity)` pairs and advance positions by `dt * velocity`.

The integration tests in `tests/module_086.rs` cover entity creation, component add/get, querying for single and paired components, component removal, system execution, and multi-tick movement simulation.

## Further Reading

- [Bevy Book — "ECS" chapter](https://bevyengine.org/learn/book/getting-started/ecs/) — the official Bevy introduction to entities, components, and systems.
- [Bevy's `Query` documentation](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html) — the real query API with combinators, change detection, and optional/without filters.
- [hecs crate](https://docs.rs/hecs/latest/hecs/) — an archetypal ECS library for non-Bevy Rust projects; a good bridge between the simplified model and Bevy's full engine.
- [Module 085 — Introduction to Game Development in Rust](../module-085-introduction-to-game-development-in-rust/README.md) — the Bevy setup and ECS overview that precedes this deep dive.
