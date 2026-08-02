//! Module 073: integration tests.

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use module_073_exercises::{handle_key, AppState, Mode};

fn key(code: KeyCode) -> KeyEvent {
    KeyEvent::new(code, KeyModifiers::NONE)
}

#[test]
fn j_moves_selection_down() {
    let mut state = AppState::default();
    state.selected = 0;
    handle_key(&mut state, key(KeyCode::Char('j')));
    assert_eq!(state.selected, 1);
}

#[test]
fn k_moves_selection_up() {
    let mut state = AppState::default();
    state.selected = 2;
    handle_key(&mut state, key(KeyCode::Char('k')));
    assert_eq!(state.selected, 1);
}

#[test]
fn down_arrow_moves_selection_down() {
    let mut state = AppState::default();
    state.selected = 0;
    handle_key(&mut state, key(KeyCode::Down));
    assert_eq!(state.selected, 1);
}

#[test]
fn up_arrow_moves_selection_up() {
    let mut state = AppState::default();
    state.selected = 2;
    handle_key(&mut state, key(KeyCode::Up));
    assert_eq!(state.selected, 1);
}

#[test]
fn selection_clamps_at_bottom() {
    let mut state = AppState::default();
    state.selected = state.items.len() - 1;
    handle_key(&mut state, key(KeyCode::Char('j')));
    assert_eq!(state.selected, state.items.len() - 1);
}

#[test]
fn selection_clamps_at_top() {
    let mut state = AppState::default();
    state.selected = 0;
    handle_key(&mut state, key(KeyCode::Char('k')));
    assert_eq!(state.selected, 0);
}

#[test]
fn i_switches_to_input_mode() {
    let mut state = AppState::default();
    assert_eq!(state.mode, Mode::Normal);
    handle_key(&mut state, key(KeyCode::Char('i')));
    assert_eq!(state.mode, Mode::Input);
}

#[test]
fn esc_switches_to_normal_mode_and_clears_input() {
    let mut state = AppState::default();
    state.mode = Mode::Input;
    state.input = "test".to_string();
    handle_key(&mut state, key(KeyCode::Esc));
    assert_eq!(state.mode, Mode::Normal);
    assert_eq!(state.input, "");
}

#[test]
fn typing_in_input_mode_appends_to_input() {
    let mut state = AppState::default();
    state.mode = Mode::Input;
    handle_key(&mut state, key(KeyCode::Char('h')));
    handle_key(&mut state, key(KeyCode::Char('i')));
    assert_eq!(state.input, "hi");
}

#[test]
fn backspace_removes_from_input() {
    let mut state = AppState::default();
    state.mode = Mode::Input;
    state.input = "hello".to_string();
    handle_key(&mut state, key(KeyCode::Backspace));
    assert_eq!(state.input, "hell");
}

#[test]
fn filtered_items_returns_all_when_input_empty() {
    let state = AppState::default();
    let filtered = state.filtered_items();
    assert_eq!(filtered.len(), 3);
}

#[test]
fn filtered_items_filters_by_input() {
    let mut state = AppState::default();
    state.input = "milk".to_string();
    let filtered = state.filtered_items();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], "Buy milk");
}

#[test]
fn filtered_items_case_insensitive() {
    let mut state = AppState::default();
    state.input = "CODE".to_string();
    let filtered = state.filtered_items();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0], "Write code");
}
