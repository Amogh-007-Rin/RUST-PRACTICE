//! Module 082: `wasm-bindgen` & JS interop — exercise scaffold.
//!
//! Everything here compiles and tests on a stock host machine. The real
//! `#[wasm_bindgen]` bindings are gated behind `cfg(target_arch = "wasm32")`
//! and only compiled when targeting wasm; on the host, a pure-Rust stub module
//! with the same API lets you exercise the exact interop surface with
//! `cargo test`.
//!
//! Fill in every `// TODO(module-082)` below.

/// An in-memory, pure-Rust stand-in for a browser DOM.
///
/// It is deliberately small: enough to verify the sync logic of a UI
/// (create elements, set text/classes, remove stale nodes) without a browser.
pub mod dom {
    /// A single DOM node in the stub.
    #[derive(Debug, Clone, PartialEq)]
    pub struct DomElement {
        pub id: String,
        pub tag: String,
        pub text_content: String,
        pub class: String,
        pub children: Vec<String>,
    }

    /// Errors produced by the stub DOM.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum DomError {
        /// An element with this id already exists.
        ElementExists(String),
        /// No element with this id exists.
        ElementNotFound(String),
    }

    /// The stub document: an ordered list of elements addressed by id.
    #[derive(Debug, Clone, Default)]
    pub struct Dom {
        elements: Vec<DomElement>,
    }

    impl Dom {
        /// Creates an empty document.
        pub fn new() -> Self {
            Self::default()
        }

        /// Number of elements currently in the document.
        pub fn element_count(&self) -> usize {
            self.elements.len()
        }

        /// Looks an element up by id.
        pub fn get_element(&self, id: &str) -> Option<&DomElement> {
            self.elements.iter().find(|e| e.id == id)
        }

        /// All element ids, in document order.
        pub fn get_all_ids(&self) -> Vec<String> {
            self.elements.iter().map(|e| e.id.clone()).collect()
        }

        /// Creates a new element; fails if the id is already taken.
        pub fn create_element(&mut self, tag: &str, id: &str) -> Result<(), DomError> {
            // TODO(module-082): push a new `DomElement` (empty text/class,
            // no children) and return `Ok(())`, or `ElementExists` if the id
            // is already present.
            panic!("TODO(module-082): implement Dom::create_element (tag = {tag}, id = {id})");
        }

        /// Sets an element's text content.
        pub fn set_text(&mut self, id: &str, text: &str) -> Result<(), DomError> {
            // TODO(module-082): update the element's `text_content`, or
            // return `ElementNotFound` if the id is missing.
            panic!("TODO(module-082): implement Dom::set_text (id = {id}, text = {text})");
        }

        /// Sets an element's class attribute.
        pub fn set_class(&mut self, id: &str, class: &str) -> Result<(), DomError> {
            // TODO(module-082): update the element's `class`, or return
            // `ElementNotFound` if the id is missing.
            panic!("TODO(module-082): implement Dom::set_class (id = {id}, class = {class})");
        }

        /// Removes an element by id.
        pub fn remove_element(&mut self, id: &str) -> Result<(), DomError> {
            // TODO(module-082): drop the element with this id, or return
            // `ElementNotFound` if it is missing.
            panic!("TODO(module-082): implement Dom::remove_element (id = {id})");
        }
    }
}

pub use dom::{Dom, DomElement, DomError};

/// Core application state — the pure logic that both the host stub and the
/// wasm bindings drive. This is what your UI code *is*, minus the DOM glue.
pub mod todos {
    /// One todo item.
    #[derive(Debug, Clone, PartialEq)]
    pub struct Todo {
        pub id: u32,
        pub text: String,
        pub done: bool,
    }

    /// Errors produced by todo operations.
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum TodoError {
        /// No todo with this id exists.
        NotFound(u32),
    }

    /// The todo list. Ids are assigned sequentially and never reused.
    #[derive(Debug, Clone, Default)]
    pub struct TodoList {
        items: Vec<Todo>,
        next_id: u32,
    }

    impl TodoList {
        /// Creates an empty list.
        pub fn new() -> Self {
            Self::default()
        }

        /// Adds a todo and returns its new id.
        pub fn add(&mut self, text: &str) -> u32 {
            // TODO(module-082): append a not-done `Todo` with `next_id` as
            // its id, bump `next_id`, and return the id.
            panic!(
                "TODO(module-082): implement TodoList::add (text = {text}, next_id = {})",
                self.next_id
            );
        }

        /// Flips a todo's `done` flag.
        pub fn toggle(&mut self, id: u32) -> Result<(), TodoError> {
            // TODO(module-082): toggle `done` on the todo with this id, or
            // return `TodoError::NotFound(id)` if it doesn't exist.
            panic!("TODO(module-082): implement TodoList::toggle (id = {id})");
        }

        /// Removes a todo and returns it.
        pub fn remove(&mut self, id: u32) -> Result<Todo, TodoError> {
            // TODO(module-082): remove and return the todo with this id, or
            // return `TodoError::NotFound(id)` if it doesn't exist.
            panic!("TODO(module-082): implement TodoList::remove (id = {id})");
        }

        /// Number of not-done todos.
        pub fn remaining(&self) -> usize {
            self.items.iter().filter(|t| !t.done).count()
        }

        /// All todos, in insertion order.
        pub fn items(&self) -> &[Todo] {
            &self.items
        }

        /// Looks a todo up by id.
        pub fn get(&self, id: u32) -> Option<&Todo> {
            self.items.iter().find(|t| t.id == id)
        }
    }
}

pub use todos::{Todo, TodoError, TodoList};

/// Syncs a `TodoList` into the stub DOM:
///
/// - a `<ul id="todo-list">` container, created on first render,
/// - one `<li id="todo-<id>">` per item with the text as content,
/// - class `"done"` on completed items,
/// - stale `<li>` elements (for todos that were removed) are deleted.
///
/// Rendering is idempotent: rendering twice changes nothing.
pub fn render_todos(dom: &mut Dom, list: &TodoList) -> Result<(), DomError> {
    // TODO(module-082): implement the sync logic described in the doc
    // comment. Use `Dom::create_element`, `Dom::set_text`,
    // `Dom::set_class` and `Dom::remove_element`.
    panic!(
        "TODO(module-082): implement render_todos ({} todos to render, {} elements in stub dom)",
        list.items().len(),
        dom.element_count()
    );
}

/// The interop surface.
///
/// On wasm this module is compiled with `#[wasm_bindgen]` attributes and
/// touches the real DOM via `web-sys`. On the host it is a pure-Rust stub
/// with the same function names and signatures, backed by the in-memory
/// `Dom` and a process-global `TodoList`.
pub mod bindings {
    #[cfg(target_arch = "wasm32")]
    mod wasm_impl {
        use std::sync::Mutex;

        use super::super::TodoList;
        use wasm_bindgen::prelude::*;

        static TODOS: Mutex<Option<TodoList>> = Mutex::new(None);

        fn with_todos<R>(f: impl FnOnce(&mut TodoList) -> R) -> R {
            let mut guard = TODOS.lock().unwrap();
            let todos = guard.get_or_insert_with(TodoList::new);
            f(todos)
        }

        #[wasm_bindgen]
        pub fn reset() {
            *TODOS.lock().unwrap() = Some(TodoList::new());
        }

        #[wasm_bindgen]
        pub fn todo_add(text: &str) -> u32 {
            with_todos(|todos| todos.add(text))
        }

        #[wasm_bindgen]
        pub fn todo_toggle(id: u32) -> bool {
            with_todos(|todos| todos.toggle(id).is_ok())
        }

        #[wasm_bindgen]
        pub fn todo_remove(id: u32) -> bool {
            with_todos(|todos| todos.remove(id).is_ok())
        }

        #[wasm_bindgen]
        pub fn todo_count() -> u32 {
            with_todos(|todos| todos.items().len() as u32)
        }

        #[wasm_bindgen]
        pub fn todo_remaining() -> u32 {
            with_todos(|todos| todos.remaining() as u32)
        }

        #[wasm_bindgen]
        pub fn todo_text(id: u32) -> Option<String> {
            with_todos(|todos| todos.get(id).map(|t| t.text.clone()))
        }

        #[wasm_bindgen]
        pub fn render_todos_into_dom() -> Result<(), JsValue> {
            let window = web_sys::window().ok_or("no window")?;
            let document = window.document().ok_or("no document")?;

            let list_el = match document.get_element_by_id("todo-list") {
                Some(el) => el,
                None => {
                    let ul = document.create_element("ul")?;
                    ul.set_id("todo-list");
                    document.body().ok_or("no body")?.append_child(&ul)?;
                    ul
                }
            };

            while let Some(child) = list_el.last_child() {
                list_el.remove_child(&child)?;
            }

            with_todos(|todos| {
                for todo in todos.items() {
                    let li = document.create_element("li")?;
                    li.set_text_content(Some(&todo.text));
                    if todo.done {
                        li.set_attribute("class", "done")?;
                    }
                    list_el.append_child(&li)?;
                }
                Ok(())
            })
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub use wasm_impl::*;

    #[cfg(not(target_arch = "wasm32"))]
    mod host_impl {
        use std::sync::Mutex;

        use super::super::TodoList;

        static TODOS: Mutex<Option<TodoList>> = Mutex::new(None);

        /// Resets the process-global todo list (mirrors the wasm `reset`).
        pub fn reset() {
            *TODOS.lock().unwrap() = Some(TodoList::new());
        }

        /// Adds a todo; returns its new id.
        pub fn todo_add(text: &str) -> u32 {
            let mut guard = TODOS.lock().unwrap();
            guard.get_or_insert_with(TodoList::new).add(text)
        }

        /// Toggles a todo; `false` if the id doesn't exist.
        pub fn todo_toggle(id: u32) -> bool {
            let mut guard = TODOS.lock().unwrap();
            guard.get_or_insert_with(TodoList::new).toggle(id).is_ok()
        }

        /// Removes a todo; `false` if the id doesn't exist.
        pub fn todo_remove(id: u32) -> bool {
            let mut guard = TODOS.lock().unwrap();
            guard.get_or_insert_with(TodoList::new).remove(id).is_ok()
        }

        /// Number of todos currently stored.
        pub fn todo_count() -> u32 {
            let mut guard = TODOS.lock().unwrap();
            guard.get_or_insert_with(TodoList::new).items().len() as u32
        }

        /// Number of not-done todos currently stored.
        pub fn todo_remaining() -> u32 {
            let mut guard = TODOS.lock().unwrap();
            guard.get_or_insert_with(TodoList::new).remaining() as u32
        }

        /// Text of the todo with `id`, if present.
        pub fn todo_text(id: u32) -> Option<String> {
            let mut guard = TODOS.lock().unwrap();
            guard
                .get_or_insert_with(TodoList::new)
                .get(id)
                .map(|t| t.text.clone())
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub use host_impl::*;
}

#[cfg(test)]
mod tests {
    // Integration tests live in `tests/` for this module.
}
