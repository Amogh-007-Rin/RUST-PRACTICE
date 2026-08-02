use std::sync::Mutex;

use module_082_exercises::bindings;
use module_082_exercises::{render_todos, Dom, DomError, TodoError, TodoList};

/// Serializes tests that drive the process-global todo list in `bindings`,
/// so parallel test threads never interleave on the shared state.
static TEST_LOCK: Mutex<()> = Mutex::new(());

// --- core logic (TodoList) -------------------------------------------------

#[test]
fn add_returns_incrementing_ids() {
    let mut list = TodoList::new();
    assert_eq!(list.add("buy milk"), 0);
    assert_eq!(list.add("ship the PR"), 1);
    assert_eq!(list.items().len(), 2);
    assert_eq!(list.items()[0].text, "buy milk");
    assert!(!list.items()[0].done);
}

#[test]
fn toggle_flips_done_and_tracks_remaining() {
    let mut list = TodoList::new();
    list.add("a");
    list.add("b");
    list.add("c");
    assert_eq!(list.remaining(), 3);
    list.toggle(0).unwrap();
    list.toggle(2).unwrap();
    assert!(list.get(0).unwrap().done);
    assert_eq!(list.remaining(), 1);
    list.toggle(0).unwrap();
    assert_eq!(list.remaining(), 2);
}

#[test]
fn toggle_unknown_id_errors() {
    let mut list = TodoList::new();
    assert_eq!(list.toggle(99), Err(TodoError::NotFound(99)));
}

#[test]
fn remove_deletes_and_returns_the_todo() {
    let mut list = TodoList::new();
    list.add("doomed");
    list.add("kept");
    let removed = list.remove(0).unwrap();
    assert_eq!(removed.text, "doomed");
    assert_eq!(list.items().len(), 1);
    assert_eq!(list.items()[0].text, "kept");
}

#[test]
fn removing_twice_errors() {
    let mut list = TodoList::new();
    list.add("x");
    list.remove(0).unwrap();
    assert_eq!(list.remove(0), Err(TodoError::NotFound(0)));
}

#[test]
fn ids_are_never_reused() {
    let mut list = TodoList::new();
    list.add("one");
    list.add("two");
    list.remove(0).unwrap();
    list.add("three");
    assert_eq!(list.get(2).unwrap().text, "three");
    assert!(list.get(0).is_none());
}

// --- stub DOM --------------------------------------------------------------

#[test]
fn dom_creates_elements_with_metadata() {
    let mut dom = Dom::new();
    dom.create_element("ul", "todo-list").unwrap();
    dom.create_element("li", "todo-0").unwrap();
    assert_eq!(dom.element_count(), 2);
    let li = dom.get_element("todo-0").unwrap();
    assert_eq!(li.tag, "li");
    assert!(li.text_content.is_empty());
    assert!(li.class.is_empty());
}

#[test]
fn dom_rejects_duplicate_ids() {
    let mut dom = Dom::new();
    dom.create_element("li", "todo-0").unwrap();
    assert_eq!(
        dom.create_element("li", "todo-0"),
        Err(DomError::ElementExists("todo-0".to_string()))
    );
}

#[test]
fn dom_set_text_and_class() {
    let mut dom = Dom::new();
    dom.create_element("li", "todo-0").unwrap();
    dom.set_text("todo-0", "buy milk").unwrap();
    dom.set_class("todo-0", "done").unwrap();
    let li = dom.get_element("todo-0").unwrap();
    assert_eq!(li.text_content, "buy milk");
    assert_eq!(li.class, "done");
}

#[test]
fn dom_operations_on_missing_ids_error() {
    let mut dom = Dom::new();
    assert_eq!(
        dom.set_text("ghost", "x"),
        Err(DomError::ElementNotFound("ghost".to_string()))
    );
    assert_eq!(
        dom.set_class("ghost", "x"),
        Err(DomError::ElementNotFound("ghost".to_string()))
    );
    assert_eq!(
        dom.remove_element("ghost"),
        Err(DomError::ElementNotFound("ghost".to_string()))
    );
}

// --- render_todos (the JS↔Rust sync logic, host-testable) -------------------

#[test]
fn render_creates_container_and_items() {
    let mut list = TodoList::new();
    list.add("buy milk");
    list.add("water plants");

    let mut dom = Dom::new();
    render_todos(&mut dom, &list).unwrap();

    assert_eq!(dom.element_count(), 3);
    assert_eq!(dom.get_element("todo-list").unwrap().tag, "ul");
    assert_eq!(dom.get_element("todo-0").unwrap().text_content, "buy milk");
    assert_eq!(
        dom.get_element("todo-1").unwrap().text_content,
        "water plants"
    );
}

#[test]
fn render_marks_done_items_with_class() {
    let mut list = TodoList::new();
    list.add("done thing");
    list.add("pending thing");
    list.toggle(0).unwrap();

    let mut dom = Dom::new();
    render_todos(&mut dom, &list).unwrap();

    assert_eq!(dom.get_element("todo-0").unwrap().class, "done");
    assert_eq!(dom.get_element("todo-1").unwrap().class, "");
}

#[test]
fn render_is_idempotent() {
    let mut list = TodoList::new();
    list.add("a");
    list.add("b");

    let mut dom = Dom::new();
    render_todos(&mut dom, &list).unwrap();
    let count = dom.element_count();
    render_todos(&mut dom, &list).unwrap();
    assert_eq!(dom.element_count(), count);
    assert_eq!(dom.element_count(), 3);
}

#[test]
fn render_removes_stale_items_after_removal() {
    let mut list = TodoList::new();
    list.add("keep");
    list.add("delete me");

    let mut dom = Dom::new();
    render_todos(&mut dom, &list).unwrap();
    assert!(dom.get_element("todo-1").is_some());

    list.remove(1).unwrap();
    render_todos(&mut dom, &list).unwrap();

    assert!(dom.get_element("todo-1").is_none());
    assert!(dom.get_element("todo-0").is_some());
    assert_eq!(dom.element_count(), 2);
}

#[test]
fn render_updates_existing_items_in_place() {
    let mut list = TodoList::new();
    list.add("before");

    let mut dom = Dom::new();
    render_todos(&mut dom, &list).unwrap();
    assert_eq!(dom.element_count(), 2);

    list.add("after");
    list.toggle(0).unwrap();
    render_todos(&mut dom, &list).unwrap();

    assert_eq!(dom.element_count(), 3);
    assert_eq!(dom.get_element("todo-0").unwrap().text_content, "before");
    assert_eq!(dom.get_element("todo-0").unwrap().class, "done");
    assert_eq!(dom.get_element("todo-1").unwrap().text_content, "after");
}

// --- bindings (the wasm-exported API, exercised via the host stub) ----------

#[test]
fn bindings_roundtrip_through_shared_state() {
    let _guard = TEST_LOCK.lock().unwrap();
    bindings::reset();

    let first = bindings::todo_add("write tests");
    let second = bindings::todo_add("push to github");
    assert_eq!(bindings::todo_count(), 2);
    assert_eq!(bindings::todo_text(first).as_deref(), Some("write tests"));
    assert_eq!(
        bindings::todo_text(second).as_deref(),
        Some("push to github")
    );
    assert_eq!(bindings::todo_remaining(), 2);
}

#[test]
fn bindings_toggle_and_remaining() {
    let _guard = TEST_LOCK.lock().unwrap();
    bindings::reset();

    let id = bindings::todo_add("a");
    let other = bindings::todo_add("b");
    assert!(bindings::todo_toggle(id));
    assert_eq!(bindings::todo_remaining(), 1);
    assert!(bindings::todo_toggle(id));
    assert_eq!(bindings::todo_remaining(), 2);
    assert!(!bindings::todo_toggle(999));
    assert_eq!(bindings::todo_text(other).as_deref(), Some("b"));
}

#[test]
fn bindings_remove() {
    let _guard = TEST_LOCK.lock().unwrap();
    bindings::reset();

    let id = bindings::todo_add("x");
    assert!(bindings::todo_remove(id));
    assert_eq!(bindings::todo_count(), 0);
    assert!(!bindings::todo_remove(id));
    assert_eq!(bindings::todo_text(id), None);
}

#[test]
fn bindings_empty_list_defaults() {
    let _guard = TEST_LOCK.lock().unwrap();
    bindings::reset();
    assert_eq!(bindings::todo_count(), 0);
    assert_eq!(bindings::todo_remaining(), 0);
    assert_eq!(bindings::todo_text(0), None);
}
