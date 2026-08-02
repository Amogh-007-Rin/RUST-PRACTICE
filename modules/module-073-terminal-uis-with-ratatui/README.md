# Module 073: Terminal UIs — `ratatui` Basics

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 75–100 min
**Prerequisites:** Module 071 (Building CLI Tools I), Module 072 (Building CLI Tools II)

## Learning Objectives
- Understand the `ratatui` architecture: immediate-mode rendering, state separation
- Build a TUI app state machine with modes (normal, input, selection)
- Implement pure state logic (cursor movement, selection, filtering) that's fully testable
- Render widgets (list, tabs, input) using `ratatui`'s API
- Separate the rendering loop from the state logic for testability

## Why This Matters
Terminal UIs are making a comeback: `btm`, `gitui`, `lazygit`, and `k9s` all use `ratatui`. The key insight is that `ratatui` is **immediate-mode**: you don't build a widget tree and mutate it; you describe the entire UI on every frame from your app state. This makes the state logic pure and testable, while the rendering is a thin layer on top.

## Concept

Graphical user interfaces in the terminal? Yes, and they're surprisingly powerful. Tools like `htop`, `lazygit`, and `btop` prove that you don't need a GUI framework to build interactive, visually rich applications. `ratatui` (a fork of the older `tui-rs`) is the modern Rust crate for building terminal UIs, and it's built on a principle that makes it uniquely testable: **immediate-mode rendering with separated state**.

### Immediate-mode rendering

In a traditional GUI framework (like GTK or React), you build a widget tree, mutate it as the user interacts, and the framework figures out what changed and redraws it. `ratatui` is different: on every frame, you describe the *entire* UI from scratch based on your app state. There's no persistent widget tree to manage.

```rust
fn render(frame: &mut Frame, state: &AppState) {
    let list = List::new(state.items.iter().map(|i| Line::from(i.as_str())))
        .highlight_style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(list, frame.area());
}
```

This function doesn't mutate anything; it just reads the state and draws. The state lives elsewhere, and your event handlers mutate it. This separation is what makes `ratatui` apps testable.

### The architecture

A `ratatui` app has three parts:

1. **State**: a struct that holds all the data your UI needs (selected item, input buffer, mode, etc.)
2. **Event loop**: reads user input (keyboard, mouse) and updates the state
3. **Render function**: takes the state and draws the UI on every frame

```rust
struct AppState {
    mode: Mode,
    items: Vec<String>,
    selected: usize,
    input: String,
}

enum Mode {
    Normal,
    Input,
}

fn handle_key(state: &mut AppState, key: KeyEvent) {
    match state.mode {
        Mode::Normal => match key.code {
            KeyCode::Char('j') => state.selected = (state.selected + 1).min(state.items.len() - 1),
            KeyCode::Char('k') => state.selected = state.selected.saturating_sub(1),
            KeyCode::Char('i') => state.mode = Mode::Input,
            _ => {}
        },
        Mode::Input => match key.code {
            KeyCode::Esc => state.mode = Mode::Normal,
            KeyCode::Char(c) => state.input.push(c),
            KeyCode::Backspace => { state.input.pop(); }
            _ => {}
        },
    }
}
```

The `handle_key` function is pure state logic: it takes a mutable state and a key event, and updates the state. You can test this without a terminal.

### Testing the state logic

The key insight: **test the state logic, not the rendering**. Your `handle_key` function doesn't know about terminals, frames, or widgets. It just mutates a struct. So you can write unit tests:

```rust
#[test]
fn j_moves_selection_down() {
    let mut state = AppState {
        mode: Mode::Normal,
        items: vec!["a".into(), "b".into(), "c".into()],
        selected: 0,
        input: String::new(),
    };
    handle_key(&mut state, KeyEvent::new(KeyCode::Char('j'), KeyModifiers::NONE));
    assert_eq!(state.selected, 1);
}

#[test]
fn i_switches_to_input_mode() {
    let mut state = AppState {
        mode: Mode::Normal,
        items: vec![],
        selected: 0,
        input: String::new(),
    };
    handle_key(&mut state, KeyEvent::new(KeyCode::Char('i'), KeyModifiers::NONE));
    assert!(matches!(state.mode, Mode::Input));
}
```

These tests run in milliseconds, no terminal required.

### Widgets

`ratatui` provides a variety of widgets:

- **List**: a scrollable list of items with optional selection highlighting
- **Tabs**: a tab bar for switching between views
- **Paragraph**: a text block with wrapping and styling
- **Block**: a border around other widgets with an optional title
- **Input** (custom): a text input field (you build this with a `Paragraph`)

```rust
fn render(frame: &mut Frame, state: &AppState) {
    let area = frame.area();
    
    // A list with selection
    let items: Vec<Line> = state.items.iter().enumerate().map(|(i, item)| {
        let style = if i == state.selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default()
        };
        Line::from(Span::styled(item, style))
    }).collect();
    
    let list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Items"));
    frame.render_widget(list, area);
}
```

### The render loop

The actual terminal interaction happens in `main.rs`:

```rust
fn main() -> Result<()> {
    let mut terminal = init_terminal()?;
    let mut state = AppState::default();
    
    loop {
        terminal.draw(|frame| render(frame, &state))?;
        
        if let Event::Key(key) = event::read()? {
            if key.code == KeyCode::Char('q') {
                break;
            }
            handle_key(&mut state, key);
        }
    }
    
    restore_terminal()?;
    Ok(())
}
```

This loop: draw the UI, read an event, update the state, repeat. The `init_terminal` and `restore_terminal` functions set up and tear down the terminal (raw mode, alternate screen).

### Why the state/render separation matters

If you put all your logic in the render function, you can't test it without a terminal. By separating state from rendering:

- **State logic** (in `lib.rs`): pure functions that mutate a struct. Fully testable.
- **Rendering** (in `main.rs`): a thin layer that reads the state and draws widgets. Hard to test, but it's just a few lines.
- **Event handling** (in `lib.rs`): pure functions that update the state based on input. Fully testable.

This is the same pattern as MVC or Elm: the model (state) is separate from the view (rendering).

### Filtering and search

A common TUI pattern: the user presses `/` to enter search mode, types a query, and the list filters in real-time. This is just more state logic:

```rust
struct AppState {
    items: Vec<String>,
    selected: usize,
    filter: String,
    mode: Mode,
}

impl AppState {
    fn filtered_items(&self) -> Vec<&String> {
        if self.filter.is_empty() {
            self.items.iter().collect()
        } else {
            self.items.iter().filter(|i| i.contains(&self.filter)).collect()
        }
    }
}
```

When the user types in filter mode, you update `state.filter`. The `filtered_items` method computes the filtered list on the fly. The render function calls `filtered_items` and draws the result.

### Mode transitions

TUI apps often have modes (like Vim): normal mode for navigation, insert mode for input, visual mode for selection. Model this as an enum:

```rust
enum Mode {
    Normal,
    Input,
    Visual,
}
```

Your `handle_key` function matches on the mode and dispatches accordingly. Transitions happen when the user presses specific keys (e.g., `i` to enter input mode, `Esc` to exit).

### Common pitfalls

- **Putting logic in the render function**: makes it untestable. Keep render pure.
- **Not handling terminal restoration**: if your app panics, the terminal is left in a broken state. Use a cleanup function or a `Drop` guard.
- **Blocking the event loop**: if you do heavy computation in the event loop, the UI freezes. Move it to a background thread or use async.
- **Hardcoding terminal size**: use `frame.area()` to get the current size, don't assume 80x24.

## Common Pitfalls
- **Mixing state and rendering**: if your render function has `if` statements that mutate state, you've coupled them. Keep render pure.
- **Not testing state transitions**: the state logic is the heart of your TUI. Test every key binding.
- **Forgetting to restore the terminal**: if your app crashes, the user's terminal is broken. Always restore it, even on panic.
- **Using `println!` in a TUI app**: it breaks the rendering. Use `ratatui`'s widgets for all output.
- **Blocking the event loop**: long-running operations freeze the UI. Use async or background threads.

## Key Terms
- **Immediate-mode rendering**: describe the entire UI on every frame from the current state
- **State machine**: a struct that holds all UI data and transitions between modes
- **Widget**: a UI element (list, tabs, paragraph) that `ratatui` renders
- **Raw mode**: terminal mode where keys are sent immediately without waiting for Enter
- **Alternate screen**: a separate terminal buffer that TUI apps use (so your shell history isn't overwritten)

## Exercise

In `exercises/`, you'll build the state logic for a TUI task manager. The `AppState` struct and `handle_key` function are partially defined — fill in the `TODO(module-073)` markers to:

1. Implement mode transitions (Normal ↔ Input)
2. Handle cursor movement (j/k or arrow keys)
3. Handle input mode (typing, backspace, escape)
4. Implement filtering (when in Input mode, filter the list)

The integration tests verify the state logic. A `main.rs` stub is provided but not tested (interactive rendering needs a terminal).

## Further Reading
- [ratatui documentation](https://docs.rs/ratatui/latest/ratatui/)
- [ratatui examples](https://github.com/ratatui/ratatui/tree/main/examples)
- [Immediate mode GUI](https://caseymuratori.com/blog_0001.html)

## Running This Module's Tests

All tests run with `cargo test -p module-073-exercises` and `cargo test -p module-073-solutions`. The tests cover the state logic only; the interactive rendering in `main.rs` is not exercised by tests (it needs a real terminal).
