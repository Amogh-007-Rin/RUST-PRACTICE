use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Wall,
    Player,
    Enemy,
    Collectible,
    Exit,
}

#[derive(Debug, Clone)]
pub struct Entity {
    pub id: u64,
    pub pos: Position,
    pub tile_type: TileType,
    pub alive: bool,
}

pub struct GameState {
    pub entities: HashMap<u64, Entity>,
    pub next_id: u64,
    pub player_id: Option<u64>,
    pub map_width: i32,
    pub map_height: i32,
    pub score: u32,
    pub tick: u64,
    pub game_over: bool,
    pub won: bool,
    pub collectibles_total: u32,
}

impl GameState {
    pub fn new(width: i32, height: i32) -> Self {
        let mut state = Self {
            entities: HashMap::new(),
            next_id: 0,
            player_id: None,
            map_width: width,
            map_height: height,
            score: 0,
            tick: 0,
            game_over: false,
            won: false,
            collectibles_total: 0,
        };

        for x in 0..width {
            state.spawn_entity(Position { x, y: 0 }, TileType::Wall);
            state.spawn_entity(Position { x, y: height - 1 }, TileType::Wall);
        }
        for y in 1..height - 1 {
            state.spawn_entity(Position { x: 0, y }, TileType::Wall);
            state.spawn_entity(Position { x: width - 1, y }, TileType::Wall);
        }

        let player_id = state.spawn_entity(Position { x: 1, y: 1 }, TileType::Player);
        state.player_id = Some(player_id);

        let exit_pos = Position {
            x: width - 2,
            y: height - 2,
        };

        let interior_collectible_positions = [
            Position { x: 2, y: 2 },
            Position { x: 3, y: 3 },
            Position { x: 4, y: 2 },
            Position { x: 2, y: 4 },
        ];

        for pos in &interior_collectible_positions {
            if pos.x > 0 && pos.x < width - 1 && pos.y > 0 && pos.y < height - 1 && *pos != exit_pos
            {
                state.spawn_entity(*pos, TileType::Collectible);
                state.collectibles_total += 1;
            }
        }

        if exit_pos.x > 1 && exit_pos.y > 1 {
            state.spawn_entity(exit_pos, TileType::Exit);
        }

        state
    }

    pub fn spawn_entity(&mut self, pos: Position, tile_type: TileType) -> u64 {
        let id = self.next_id;
        self.next_id += 1;

        self.entities.insert(
            id,
            Entity {
                id,
                pos,
                tile_type,
                alive: true,
            },
        );

        id
    }

    pub fn move_entity(&mut self, entity_id: u64, dir: Direction) -> bool {
        if self.game_over || self.won {
            return false;
        }

        let current_pos = match self.entities.get(&entity_id) {
            Some(e) if e.alive => e.pos,
            _ => return false,
        };

        let new_pos = match dir {
            Direction::Up => Position {
                x: current_pos.x,
                y: current_pos.y - 1,
            },
            Direction::Down => Position {
                x: current_pos.x,
                y: current_pos.y + 1,
            },
            Direction::Left => Position {
                x: current_pos.x - 1,
                y: current_pos.y,
            },
            Direction::Right => Position {
                x: current_pos.x + 1,
                y: current_pos.y,
            },
        };

        if new_pos.x < 0
            || new_pos.x >= self.map_width
            || new_pos.y < 0
            || new_pos.y >= self.map_height
        {
            return false;
        }

        let entities_at_new = self.entities_at(new_pos);

        for other_id in &entities_at_new {
            if *other_id == entity_id {
                continue;
            }

            if let Some(other) = self.entities.get(other_id) {
                if !other.alive {
                    continue;
                }

                match other.tile_type {
                    TileType::Wall => return false,
                    TileType::Exit => {
                        if self.score >= self.collectibles_total {
                            self.won = true;
                            if let Some(entity) = self.entities.get_mut(&entity_id) {
                                entity.pos = new_pos;
                            }
                            return true;
                        } else {
                            return false;
                        }
                    }
                    TileType::Enemy => {
                        self.game_over = true;
                        if let Some(entity) = self.entities.get_mut(&entity_id) {
                            entity.pos = new_pos;
                        }
                        return true;
                    }
                    TileType::Collectible => {
                        self.score += 1;
                        if let Some(collectible) = self.entities.get_mut(other_id) {
                            collectible.alive = false;
                        }
                        if let Some(entity) = self.entities.get_mut(&entity_id) {
                            entity.pos = new_pos;
                        }
                        return true;
                    }
                    _ => {}
                }
            }
        }

        if let Some(entity) = self.entities.get_mut(&entity_id) {
            entity.pos = new_pos;
        }
        true
    }

    pub fn entities_at(&self, pos: Position) -> Vec<u64> {
        self.entities
            .iter()
            .filter(|(_, e)| e.alive && e.pos == pos)
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn render(&self) -> String {
        let mut grid = vec![vec!['.'; self.map_width as usize]; self.map_height as usize];

        for entity in self.entities.values() {
            if !entity.alive {
                continue;
            }
            let ch = match entity.tile_type {
                TileType::Wall => '#',
                TileType::Player => '@',
                TileType::Enemy => 'E',
                TileType::Collectible => '*',
                TileType::Exit => 'X',
                TileType::Empty => '.',
            };
            let x = entity.pos.x as usize;
            let y = entity.pos.y as usize;
            if x < self.map_width as usize && y < self.map_height as usize {
                grid[y][x] = ch;
            }
        }

        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn update_enemies(&mut self) {
        if self.game_over || self.won {
            return;
        }

        let player_pos = match self.player_id.and_then(|pid| self.entities.get(&pid)) {
            Some(p) if p.alive => p.pos,
            _ => return,
        };

        let enemy_ids: Vec<u64> = self
            .entities
            .iter()
            .filter(|(_, e)| e.alive && e.tile_type == TileType::Enemy)
            .map(|(id, _)| *id)
            .collect();

        for enemy_id in enemy_ids {
            let enemy_pos = match self.entities.get(&enemy_id) {
                Some(e) if e.alive => e.pos,
                _ => continue,
            };

            let dx = player_pos.x - enemy_pos.x;
            let dy = player_pos.y - enemy_pos.y;

            let preferred_dirs = if dx.abs() >= dy.abs() {
                if dx > 0 {
                    vec![
                        Direction::Right,
                        Direction::Down,
                        Direction::Up,
                        Direction::Left,
                    ]
                } else {
                    vec![
                        Direction::Left,
                        Direction::Down,
                        Direction::Up,
                        Direction::Right,
                    ]
                }
            } else if dy > 0 {
                vec![
                    Direction::Down,
                    Direction::Right,
                    Direction::Left,
                    Direction::Up,
                ]
            } else {
                vec![
                    Direction::Up,
                    Direction::Right,
                    Direction::Left,
                    Direction::Down,
                ]
            };

            for dir in preferred_dirs {
                let new_pos = match dir {
                    Direction::Up => Position {
                        x: enemy_pos.x,
                        y: enemy_pos.y - 1,
                    },
                    Direction::Down => Position {
                        x: enemy_pos.x,
                        y: enemy_pos.y + 1,
                    },
                    Direction::Left => Position {
                        x: enemy_pos.x - 1,
                        y: enemy_pos.y,
                    },
                    Direction::Right => Position {
                        x: enemy_pos.x + 1,
                        y: enemy_pos.y,
                    },
                };

                if new_pos.x < 0
                    || new_pos.x >= self.map_width
                    || new_pos.y < 0
                    || new_pos.y >= self.map_height
                {
                    continue;
                }

                let entities_at_new = self.entities_at(new_pos);
                let blocked = entities_at_new.iter().any(|id| {
                    self.entities
                        .get(id)
                        .map(|e| e.tile_type == TileType::Wall || e.tile_type == TileType::Enemy)
                        .unwrap_or(false)
                });

                if blocked {
                    continue;
                }

                if new_pos == player_pos {
                    self.game_over = true;
                    if let Some(enemy) = self.entities.get_mut(&enemy_id) {
                        enemy.pos = new_pos;
                    }
                    return;
                }

                if let Some(enemy) = self.entities.get_mut(&enemy_id) {
                    enemy.pos = new_pos;
                }
                break;
            }
        }
    }

    pub fn check_win_condition(&self, collectibles_needed: u32) -> bool {
        self.score >= collectibles_needed
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
