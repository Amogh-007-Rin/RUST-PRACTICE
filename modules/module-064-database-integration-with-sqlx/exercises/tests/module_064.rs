//! Integration tests for module 064. Every test opens its own fresh
//! in-memory SQLite database, so tests are isolated from each other and
//! need no database server.

use module_064_exercises::TodoStore;

#[tokio::test]
async fn fresh_store_starts_empty() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    assert!(store.list_todos().await.unwrap().is_empty());
}

#[tokio::test]
async fn create_todo_assigns_an_id() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    let todo = store.create_todo("buy milk").await.unwrap();
    assert_eq!(todo.id, 1);
    assert_eq!(todo.title, "buy milk");
    assert!(!todo.completed);
}

#[tokio::test]
async fn create_todo_trims_nothing_and_accepts_duplicates() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    store.create_todo("first").await.unwrap();
    store.create_todo("first").await.unwrap();
    assert_eq!(store.list_todos().await.unwrap().len(), 2);
}

#[tokio::test]
async fn list_returns_todos_in_creation_order() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    store.create_todo("first").await.unwrap();
    store.create_todo("second").await.unwrap();
    let todos = store.list_todos().await.unwrap();
    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0].title, "first");
    assert_eq!(todos[0].id, 1);
    assert_eq!(todos[1].title, "second");
    assert_eq!(todos[1].id, 2);
}

#[tokio::test]
async fn get_returns_none_for_missing_todo() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    assert!(store.get_todo(42).await.unwrap().is_none());
}

#[tokio::test]
async fn get_returns_the_stored_todo() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    let created = store.create_todo("rust").await.unwrap();
    let fetched = store.get_todo(created.id).await.unwrap().unwrap();
    assert_eq!(fetched, created);
}

#[tokio::test]
async fn toggle_flips_completed_back_and_forth() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    let created = store.create_todo("rust").await.unwrap();
    assert!(!created.completed);
    let toggled = store.toggle_todo(created.id).await.unwrap().unwrap();
    assert!(toggled.completed);
    let toggled_back = store.toggle_todo(created.id).await.unwrap().unwrap();
    assert!(!toggled_back.completed);
}

#[tokio::test]
async fn toggle_returns_none_for_missing_todo() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    assert!(store.toggle_todo(999).await.unwrap().is_none());
}

#[tokio::test]
async fn delete_removes_the_todo() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    let created = store.create_todo("rust").await.unwrap();
    assert!(store.delete_todo(created.id).await.unwrap());
    assert!(store.get_todo(created.id).await.unwrap().is_none());
    assert!(store.list_todos().await.unwrap().is_empty());
}

#[tokio::test]
async fn delete_missing_todo_returns_false() {
    let store = TodoStore::connect_in_memory().await.unwrap();
    assert!(!store.delete_todo(999).await.unwrap());
}

#[tokio::test]
async fn file_backed_database_works_too() {
    let path = std::env::temp_dir().join(format!(
        "module-064-{}-file-test.sqlite",
        std::process::id()
    ));
    let _ = std::fs::remove_file(&path);
    let url = format!("sqlite://{}", path.display());
    let store = TodoStore::connect(&url).await.unwrap();
    store.create_todo("file-backed").await.unwrap();
    assert_eq!(store.list_todos().await.unwrap().len(), 1);
    let _ = std::fs::remove_file(&path);
}
