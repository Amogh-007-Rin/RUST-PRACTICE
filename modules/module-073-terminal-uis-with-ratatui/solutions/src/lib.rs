//! Module 073: Terminal UIs with ratatui — reference solution.

use crossterm::event::{KeyCode, KeyEvent};

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
    /// Return items filtered by the current input (case-insensitive).
    pub fn filtered_items(&self) -> Vec<&String> {
        if self.input.is_empty() {
            self.items.iter().collect()
        } else {
            let query = self.input.to_lowercase();
            self.items
                .iter()
                .filter(|item| item.to_lowercase().contains(&query))
                .collect()
        }
    }
}

/// Handle a key event and update the state.
pub fn handle_key(state: &mut AppState, key: KeyEvent) {
    match state.mode {
        Mode::Normal => match key.code {
            KeyCode::Char('j') | KeyCode::Down => {
                if state.selected + 1 < state.items.len() {
                    state.selected += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                state.selected = state.selected.saturating_sub(1);
            }
            KeyCode::Char('i') => {
                state.mode = Mode::Input;
            }
            _ => {}
        },
        Mode::Input => match key.code {
            KeyCode::Esc => {
                state.mode = Mode::Normal;
                state.input.clear();
            }
            KeyCode::Char(c) => {
                state.input.push(c);
            }
            KeyCode::Backspace => {
                state.input.pop();
            }
            _ => {}
        },
    }
}
