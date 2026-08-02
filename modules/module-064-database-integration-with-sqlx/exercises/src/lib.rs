//! Module 064: Database integration with sqlx — exercise scaffold.
//!
//! Connection handling, the schema migration, and the `Todo` row type are
//! all complete. Your job is to write the SQL queries in the five data
//! access methods. Everything uses sqlx's runtime API — no compile-time
//! macros — so no `DATABASE_URL` is needed to build.
//!
//! Find the `// TODO(module-064)` comments below and fill them in until
//! `cargo test -p module-064-exercises` passes.

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};
use sqlx::FromRow;
use std::str::FromStr;

/// A todo row as stored in the database.
#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}

/// A handle to a database: a connection pool plus the schema.
#[derive(Clone)]
pub struct TodoStore {
    pool: SqlitePool,
}

impl TodoStore {
    /// Opens a brand-new in-memory SQLite database and applies the schema.
    ///
    /// Note the `max_connections(1)`: an in-memory SQLite database lives in
    /// its connection's memory, so exactly one connection must be used (or
    /// the shared-cache URI from Module 068).
    pub async fn connect_in_memory() -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str("sqlite::memory:")?;
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect_with(options)
            .await?;
        let store = Self { pool };
        store.init_schema().await?;
        Ok(store)
    }

    /// Opens a database at the given URL (`sqlite://path/to.db` locally,
    /// `postgres://...` in production — same code) and applies the schema.
    pub async fn connect(database_url: &str) -> Result<Self, sqlx::Error> {
        let options = SqliteConnectOptions::from_str(database_url)?.create_if_missing(true);
        let pool = SqlitePoolOptions::new().connect_with(options).await?;
        let store = Self { pool };
        store.init_schema().await?;
        Ok(store)
    }

    /// Applies the embedded migrations from `migrations/`.
    pub async fn init_schema(&self) -> Result<(), sqlx::migrate::MigrateError> {
        sqlx::migrate!("./migrations").run(&self.pool).await
    }

    /// Inserts a todo and returns the stored row (id included).
    pub async fn create_todo(&self, title: &str) -> Result<Todo, sqlx::Error> {
        // TODO(module-064): Insert the todo with `completed = 0` and return
        // the stored row. `sqlx::query_as::<_, Todo>("...")` can map a row
        // straight into `Todo`; use `RETURNING id, title, completed` and
        // `.bind(title)`. Fetch with `.fetch_one(&self.pool)`.
        panic!("not implemented: create_todo(title={title:?})")
    }

    /// Lists all todos in insertion order.
    pub async fn list_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        // TODO(module-064): Select all todos ordered by id and fetch all
        // of them as `Vec<Todo>` (`.fetch_all(&self.pool)`).
        panic!("not implemented: list_todos()")
    }

    /// Fetches one todo by id, or `None` if it doesn't exist.
    pub async fn get_todo(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        // TODO(module-064): Select the todo with the given id, using `?`
        // as the placeholder and `.bind(id)`. Use `.fetch_optional(...)`
        // so a missing row comes back as `None` instead of an error.
        panic!("not implemented: get_todo(id={id:?})")
    }

    /// Flips `completed` for one todo and returns the updated row.
    pub async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        // TODO(module-064): Run `UPDATE todos SET completed = NOT completed
        // WHERE id = ?` (plain `sqlx::query(...)` — no row comes back), then
        // return `self.get_todo(id).await` for the fresh row.
        panic!("not implemented: toggle_todo(id={id:?})")
    }

    /// Deletes one todo; returns whether a row was actually removed.
    pub async fn delete_todo(&self, id: i64) -> Result<bool, sqlx::Error> {
        // TODO(module-064): Run `DELETE FROM todos WHERE id = ?` and return
        // `result.rows_affected() > 0` to report whether a row was removed.
        panic!("not implemented: delete_todo(id={id:?})")
    }
}
