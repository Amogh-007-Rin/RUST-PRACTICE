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
    pub fn new(_width: i32, _height: i32) -> Self {
        todo!("initialize a new game with walls around the border, player at (1,1), collectibles scattered, one exit")
    }

    pub fn spawn_entity(&mut self, _pos: Position, _tile_type: TileType) -> u64 {
        todo!("spawn an entity and return its id")
    }

    pub fn move_entity(&mut self, _entity_id: u64, _dir: Direction) -> bool {
        todo!(
            "move an entity in a direction, handling collisions:\n\
               - Wall: don't move\n\
               - Collectible: collect it (remove, add score)\n\
               - Exit: if score >= required, win; otherwise can't exit\n\
               - Enemy: game over\n\
               Returns whether the move was successful"
        )
    }

    pub fn entities_at(&self, _pos: Position) -> Vec<u64> {
        todo!("get entities at a position")
    }

    pub fn render(&self) -> String {
        todo!(
            "render the game map as a string\n\
               Use characters: # wall, @ player, E enemy, * collectible, X exit, . empty"
        )
    }

    pub fn update_enemies(&mut self) {
        todo!("update enemy AI - move enemies toward player")
    }

    pub fn check_win_condition(&self, _collectibles_needed: u32) -> bool {
        todo!("check if player has enough score to win")
    }
}
