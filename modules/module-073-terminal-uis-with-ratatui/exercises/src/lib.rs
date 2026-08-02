//! Module 073: Terminal UIs with ratatui — exercise scaffold.
//!
//! Build the state logic for a TUI task manager.

use crossterm::event::KeyEvent;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Normal,
    Input,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AppState {
    pub items: Vec<String>,
    pub selected: usize,
    pub input: String,
    pub mode: Mode,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            items: vec![
                "Buy milk".to_string(),
                "Write code".to_string(),
                "Read book".to_string(),
            ],
            selected: 0,
            input: String::new(),
            mode: Mode::Normal,
        }
    }
}

impl AppState {
    /// Return items filtered by the current input.
    pub fn filtered_items(&self) -> Vec<&String> {
        // TODO(module-073): if input is empty, return all items.
        // Otherwise, return items that contain the input string (case-insensitive).
        panic!("TODO(module-073): implement filtered_items")
    }
}

/// Handle a key event and update the state.
pub fn handle_key(_state: &mut AppState, _key: KeyEvent) {
    // TODO(module-073): match on state.mode and handle keys:
    //
    // Normal mode:
    //   'j' or Down  -> move selection down (wrap or clamp)
    //   'k' or Up    -> move selection up
    //   'i'          -> switch to Input mode
    //   'q'          -> (no-op here; main.rs handles quit)
    //
    // Input mode:
    //   Esc          -> switch to Normal mode, clear input
    //   Char(c)      -> push c to input
    //   Backspace    -> pop from input
    //   Enter        -> (no-op for now)
    panic!("TODO(module-073): implement handle_key")
}
