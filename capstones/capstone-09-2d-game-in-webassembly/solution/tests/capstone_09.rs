use capstone_09_solution::*;

fn make_game() -> GameState {
    GameState::new(8, 8)
}

#[test]
fn game_initialization_creates_walls_around_border() {
    let game = make_game();
    for x in 0..game.map_width {
        assert!(
            !game.entities_at(Position { x, y: 0 }).is_empty(),
            "missing wall at ({x}, 0)"
        );
        assert!(
            !game
                .entities_at(Position {
                    x,
                    y: game.map_height - 1
                })
                .is_empty(),
            "missing wall at ({x}, {})",
            game.map_height - 1
        );
    }
    for y in 1..game.map_height - 1 {
        assert!(
            !game.entities_at(Position { x: 0, y }).is_empty(),
            "missing wall at (0, {y})"
        );
        assert!(
            !game
                .entities_at(Position {
                    x: game.map_width - 1,
                    y
                })
                .is_empty(),
            "missing wall at ({}, {y})",
            game.map_width - 1
        );
    }
}

#[test]
fn player_starts_at_one_one() {
    let game = make_game();
    let pid = game.player_id.unwrap();
    let player = game.entities.get(&pid).unwrap();
    assert_eq!(player.pos, Position { x: 1, y: 1 });
    assert_eq!(player.tile_type, TileType::Player);
}

#[test]
fn moving_into_wall_fails() {
    let mut game = make_game();
    let pid = game.player_id.unwrap();

    let moved = game.move_entity(pid, Direction::Up);
    assert!(!moved);

    let player = game.entities.get(&pid).unwrap();
    assert_eq!(player.pos, Position { x: 1, y: 1 });
}

#[test]
fn moving_into_empty_space_succeeds() {
    let mut game = make_game();
    let pid = game.player_id.unwrap();

    let moved = game.move_entity(pid, Direction::Down);
    assert!(moved);

    let player = game.entities.get(&pid).unwrap();
    assert_eq!(player.pos, Position { x: 1, y: 2 });
}

#[test]
fn collecting_collectible_increases_score() {
    let mut game = make_game();
    let pid = game.player_id.unwrap();

    for _ in 0..3 {
        game.move_entity(pid, Direction::Down);
    }
    game.move_entity(pid, Direction::Right);

    let player = game.entities.get(&pid).unwrap();
    assert_eq!(player.pos, Position { x: 2, y: 4 });
    assert!(game.score >= 1, "expected score >= 1, got {}", game.score);
    assert!(!game.game_over);
}

#[test]
fn moving_into_enemy_triggers_game_over() {
    let mut game = make_game();
    let pid = game.player_id.unwrap();

    let enemy_id = game.spawn_entity(Position { x: 5, y: 5 }, TileType::Enemy);

    assert!(!game.game_over);
    let enemy = game.entities.get(&enemy_id).unwrap();
    assert_eq!(enemy.pos, Position { x: 5, y: 5 });

    for _ in 0..4 {
        game.move_entity(pid, Direction::Right);
    }
    for _ in 0..4 {
        game.move_entity(pid, Direction::Down);
    }

    assert!(
        game.game_over,
        "game should be over after moving into enemy"
    );

    let player = game.entities.get(&pid).unwrap();
    assert_eq!(player.pos, Position { x: 5, y: 5 });
}

#[test]
fn exit_requires_minimum_score() {
    let mut game = make_game();
    let pid = game.player_id.unwrap();

    game.move_entity(pid, Direction::Right);
    game.move_entity(pid, Direction::Right);

    let _exit_id = game.spawn_entity(Position { x: 3, y: 2 }, TileType::Exit);

    game.collectibles_total = 3;
    assert!(!game.check_win_condition(3));

    let moved = game.move_entity(pid, Direction::Down);
    assert!(
        !moved,
        "should not be able to enter exit without enough score"
    );

    assert!(!game.won);
    game.score = 3;
    assert!(game.check_win_condition(3));

    let moved = game.move_entity(pid, Direction::Down);
    assert!(moved, "should be able to enter exit with enough score");
    assert!(game.won);
}

#[test]
fn render_produces_correct_ascii_map() {
    let game = GameState::new(5, 5);
    let output = game.render();
    let lines: Vec<&str> = output.lines().collect();

    assert_eq!(lines.len(), 5, "map should have 5 rows");
    for line in &lines {
        assert_eq!(line.len(), 5, "each row should have 5 columns");
    }

    assert_eq!(lines[0], "#####", "top wall should be all #");
    assert_eq!(lines[4], "#####", "bottom wall should be all #");

    assert!(lines[1].contains('@'), "player @ should appear on row 1");
    assert!(lines[3].contains('X'), "exit X should appear");

    let non_empty = game
        .entities
        .values()
        .filter(|e| e.alive && e.tile_type == TileType::Collectible)
        .count();
    let collectible_chars = output.chars().filter(|&c| c == '*').count();
    assert_eq!(collectible_chars, non_empty);
}

#[test]
fn entity_spawning_and_querying() {
    let mut game = GameState::new(10, 10);
    let id = game.spawn_entity(Position { x: 5, y: 7 }, TileType::Collectible);

    let found = game.entities_at(Position { x: 5, y: 7 });
    assert!(found.contains(&id));
    assert_eq!(found.len(), 1);

    let empty = game.entities_at(Position { x: 6, y: 6 });
    assert!(empty.is_empty());
}

#[test]
fn enemy_ai_moves_toward_player() {
    let mut game = GameState::new(8, 8);
    let pid = game.player_id.unwrap();

    let enemy_id = game.spawn_entity(Position { x: 5, y: 1 }, TileType::Enemy);

    let enemy = game.entities.get(&enemy_id).unwrap();
    assert_eq!(enemy.pos, Position { x: 5, y: 1 });

    game.move_entity(pid, Direction::Down);
    game.move_entity(pid, Direction::Down);

    game.update_enemies();

    let enemy = game.entities.get(&enemy_id).unwrap();
    assert!(
        enemy.pos.y > 1 || enemy.pos.x != 5,
        "enemy should have moved toward player at (1, 3)"
    );
}

#[test]
fn multiple_enemies_do_not_panic() {
    let mut game = GameState::new(8, 8);

    game.spawn_entity(Position { x: 5, y: 1 }, TileType::Enemy);
    game.spawn_entity(Position { x: 5, y: 2 }, TileType::Enemy);
    game.spawn_entity(Position { x: 5, y: 3 }, TileType::Enemy);

    for _ in 0..10 {
        game.update_enemies();
        if game.game_over {
            break;
        }
    }

    assert!(true);
}
