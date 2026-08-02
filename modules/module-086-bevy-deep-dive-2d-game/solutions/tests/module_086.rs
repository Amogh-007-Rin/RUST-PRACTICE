use module_086_solutions::{movement_system, Health, Name, Position, Velocity, World};

// --- World creation and entities -------------------------------------------

#[test]
fn world_starts_empty() {
    let world = World::new();
    assert_eq!(world.entity_count(), 0);
}

#[test]
fn create_entity_returns_unique_ids() {
    let mut world = World::new();
    let id1 = world.create_entity();
    let id2 = world.create_entity();
    let id3 = world.create_entity();
    assert_ne!(id1, id2);
    assert_ne!(id2, id3);
    assert_eq!(world.entity_count(), 3);
}

#[test]
fn entity_exists_after_creation() {
    let mut world = World::new();
    let id = world.create_entity();
    assert!(world.entity_exists(id));
    assert!(!world.entity_exists(999));
}

// --- Adding and retrieving components --------------------------------------

#[test]
fn add_and_get_component() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Position { x: 10.0, y: 20.0 });

    let pos = world.get_component::<Position>(id).unwrap();
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 20.0);
}

#[test]
fn get_component_returns_none_when_missing() {
    let mut world = World::new();
    let id = world.create_entity();
    assert!(world.get_component::<Position>(id).is_none());
}

#[test]
fn multiple_component_types_on_same_entity() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Position { x: 1.0, y: 2.0 });
    world.add_component(id, Velocity { dx: 3.0, dy: 4.0 });
    world.add_component(id, Name("player".to_string()));

    let pos = world.get_component::<Position>(id).unwrap();
    let vel = world.get_component::<Velocity>(id).unwrap();
    let name = world.get_component::<Name>(id).unwrap();

    assert_eq!(pos.x, 1.0);
    assert_eq!(vel.dy, 4.0);
    assert_eq!(name.0, "player");
}

#[test]
fn mutable_component_access() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Health(100));

    if let Some(health) = world.get_component_mut::<Health>(id) {
        health.0 -= 10;
    }
    assert_eq!(world.get_component::<Health>(id).unwrap().0, 90);
}

// --- Queries ----------------------------------------------------------------

#[test]
fn query_returns_entities_with_component() {
    let mut world = World::new();
    let id1 = world.create_entity();
    let id2 = world.create_entity();
    world.add_component(id1, Position { x: 0.0, y: 0.0 });
    world.add_component(id2, Position { x: 5.0, y: 5.0 });

    let results = world.query::<Position>();
    assert_eq!(results.len(), 2);
    let ids: Vec<u64> = results.iter().map(|(id, _)| *id).collect();
    assert!(ids.contains(&id1));
    assert!(ids.contains(&id2));
}

#[test]
fn query_skips_entities_without_component() {
    let mut world = World::new();
    let id1 = world.create_entity();
    let id2 = world.create_entity();
    world.add_component(id1, Position { x: 1.0, y: 1.0 });
    // id2 has no Position

    let results = world.query::<Position>();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].0, id1);
}

#[test]
fn query_mut_allows_modification() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Position { x: 1.0, y: 2.0 });

    for (_id, pos) in world.query_mut::<Position>() {
        pos.x += 10.0;
        pos.y += 10.0;
    }
    let pos = world.get_component::<Position>(id).unwrap();
    assert_eq!(pos.x, 11.0);
    assert_eq!(pos.y, 12.0);
}

#[test]
fn query_both_returns_entities_with_pair_of_components() {
    let mut world = World::new();
    let id1 = world.create_entity();
    world.add_component(id1, Position { x: 0.0, y: 0.0 });
    world.add_component(id1, Velocity { dx: 1.0, dy: 0.0 });

    let id2 = world.create_entity();
    world.add_component(id2, Position { x: 5.0, y: 5.0 });
    // id2 has no Velocity

    let id3 = world.create_entity();
    world.add_component(id3, Velocity { dx: 2.0, dy: 2.0 });
    // id3 has no Position

    let id4 = world.create_entity();
    world.add_component(id4, Position { x: 10.0, y: 10.0 });
    world.add_component(id4, Velocity { dx: 3.0, dy: 3.0 });

    let results = world.query_both::<Position, Velocity>();
    assert_eq!(results.len(), 2);
    let ids: Vec<u64> = results.iter().map(|(id, _, _)| *id).collect();
    assert!(ids.contains(&id1));
    assert!(ids.contains(&id4));
}

// --- Removing components ----------------------------------------------------

#[test]
fn remove_component_deletes_it() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Position { x: 1.0, y: 2.0 });
    assert!(world.get_component::<Position>(id).is_some());

    world.remove_component::<Position>(id);
    assert!(world.get_component::<Position>(id).is_none());
}

// --- Running systems --------------------------------------------------------

#[test]
fn run_system_executes_closure() {
    let mut world = World::new();
    let id = world.create_entity();
    world.add_component(id, Position { x: 0.0, y: 0.0 });

    world.run_system(|w| {
        let id = w.create_entity();
        w.add_component(id, Position { x: 99.0, y: 99.0 });
    });

    assert_eq!(world.entity_count(), 2);
    let positions = world.query::<Position>();
    assert_eq!(positions.len(), 2);
}

// --- Movement system --------------------------------------------------------

#[test]
fn movement_system_updates_all_positions() {
    let mut world = World::new();

    let e1 = world.create_entity();
    world.add_component(e1, Position { x: 0.0, y: 0.0 });
    world.add_component(e1, Velocity { dx: 1.0, dy: 2.0 });

    let e2 = world.create_entity();
    world.add_component(e2, Position { x: 10.0, y: 20.0 });
    world.add_component(e2, Velocity { dx: 3.0, dy: -1.0 });

    let e3 = world.create_entity();
    world.add_component(e3, Position { x: 100.0, y: 100.0 });
    // e3 has no velocity — should be unaffected

    movement_system(&mut world, 0.5);

    let pos1 = world.get_component::<Position>(e1).unwrap();
    assert_eq!(pos1.x, 0.5);
    assert_eq!(pos1.y, 1.0);

    let pos2 = world.get_component::<Position>(e2).unwrap();
    assert_eq!(pos2.x, 11.5);
    assert_eq!(pos2.y, 19.5);

    let pos3 = world.get_component::<Position>(e3).unwrap();
    assert_eq!(pos3.x, 100.0);
    assert_eq!(pos3.y, 100.0);
}

#[test]
fn movement_system_multiple_ticks() {
    let mut world = World::new();
    let e1 = world.create_entity();
    world.add_component(e1, Position { x: 0.0, y: 0.0 });
    world.add_component(e1, Velocity { dx: 1.0, dy: 0.0 });

    for _ in 0..10 {
        movement_system(&mut world, 1.0);
    }

    let pos = world.get_component::<Position>(e1).unwrap();
    assert_eq!(pos.x, 10.0);
    assert_eq!(pos.y, 0.0);
}

// --- Default implementation -------------------------------------------------

#[test]
fn world_default_works() {
    let mut world = World::default();
    let id = world.create_entity();
    world.add_component(id, Health(100));
    assert_eq!(world.get_component::<Health>(id).unwrap().0, 100);
}
