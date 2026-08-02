use module_085_solutions::*;

#[derive(Debug, Clone, PartialEq)]
struct Position {
    x: f32,
    y: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct Velocity {
    dx: f32,
    dy: f32,
}

#[derive(Debug, Clone, PartialEq)]
struct Health(i32);

#[test]
fn spawn_and_is_alive() {
    let mut world = World::new();
    let e = world.spawn();
    assert!(world.is_alive(e));
    assert_eq!(e.version(), 0);
}

#[test]
fn despawn_removes_entity() {
    let mut world = World::new();
    let e = world.spawn();
    assert!(world.is_alive(e));
    assert!(world.despawn(e));
    assert!(!world.is_alive(e));
    assert!(!world.despawn(e));
}

#[test]
fn insert_and_get_component() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position { x: 1.0, y: 2.0 });
    let pos = world.get::<Position>(e).unwrap();
    assert_eq!(pos.x, 1.0);
    assert_eq!(pos.y, 2.0);
}

#[test]
fn get_returns_none_for_missing_component() {
    let mut world = World::new();
    let e = world.spawn();
    assert!(world.get::<Position>(e).is_none());
}

#[test]
fn get_mut_modifies_component() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Health(100));
    world.get_mut::<Health>(e).unwrap().0 -= 30;
    assert_eq!(world.get::<Health>(e).unwrap().0, 70);
}

#[test]
fn remove_component() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position { x: 0.0, y: 0.0 });
    let removed = world.remove::<Position>(e);
    assert!(removed.is_some());
    assert!(world.get::<Position>(e).is_none());
}

#[test]
fn query_single_component() {
    let mut world = World::new();
    let e1 = world.spawn();
    let e2 = world.spawn();
    let e3 = world.spawn();
    world.insert(e1, Position { x: 1.0, y: 0.0 });
    world.insert(e2, Position { x: 2.0, y: 0.0 });
    world.insert(e3, Velocity { dx: 0.0, dy: 1.0 });

    let positions = world.query::<Position>();
    assert_eq!(positions.len(), 2);
}

#[test]
fn query2_two_components() {
    let mut world = World::new();
    let e1 = world.spawn();
    let e2 = world.spawn();
    let e3 = world.spawn();
    world.insert(e1, Position { x: 0.0, y: 0.0 });
    world.insert(e1, Velocity { dx: 1.0, dy: 0.0 });
    world.insert(e2, Position { x: 5.0, y: 5.0 });
    world.insert(e3, Velocity { dx: -1.0, dy: 0.0 });

    let both = world.query2::<Position, Velocity>();
    assert_eq!(both.len(), 1);
    assert_eq!(both[0].0, e1);
}

#[test]
fn system_executor_runs_systems_in_order() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position { x: 0.0, y: 0.0 });
    world.insert(e, Velocity { dx: 1.0, dy: 2.0 });

    let mut executor = SystemExecutor::new();
    executor.add_system(|world: &mut World| {
        let entities: Vec<_> = world
            .query2::<Position, Velocity>()
            .into_iter()
            .map(|(e, _, vel)| (e, vel.dx, vel.dy))
            .collect();
        for (entity, dx, dy) in entities {
            let pos = world.get_mut::<Position>(entity).unwrap();
            pos.x += dx;
            pos.y += dy;
        }
    });

    executor.run(&mut world);
    let pos = world.get::<Position>(e).unwrap();
    assert_eq!(pos.x, 1.0);
    assert_eq!(pos.y, 2.0);

    executor.run(&mut world);
    let pos = world.get::<Position>(e).unwrap();
    assert_eq!(pos.x, 2.0);
    assert_eq!(pos.y, 4.0);
}

#[test]
fn despawn_cleans_up_components() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Position { x: 0.0, y: 0.0 });
    world.insert(e, Health(100));
    world.despawn(e);
    assert!(world.query::<Position>().is_empty());
    assert!(world.query::<Health>().is_empty());
}

#[test]
fn replace_component() {
    let mut world = World::new();
    let e = world.spawn();
    world.insert(e, Health(100));
    world.insert(e, Health(50));
    assert_eq!(world.get::<Health>(e).unwrap().0, 50);
}
