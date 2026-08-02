//! Module 064: Database integration with sqlx.
//!
//! Everything here uses sqlx's *runtime* API (`sqlx::query` /
//! `sqlx::query_as`) — no compile-time macros — so nothing needs a
//! `DATABASE_URL` at build time. Tests run against an in-memory SQLite
//! database, so no server is required to run this module.

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
        let todo = sqlx::query_as::<_, Todo>(
            "INSERT INTO todos (title, completed) VALUES (?, 0) RETURNING id, title, completed",
        )
        .bind(title)
        .fetch_one(&self.pool)
        .await?;
        Ok(todo)
    }

    /// Lists all todos in insertion order.
    pub async fn list_todos(&self) -> Result<Vec<Todo>, sqlx::Error> {
        let todos = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos ORDER BY id")
            .fetch_all(&self.pool)
            .await?;
        Ok(todos)
    }

    /// Fetches one todo by id, or `None` if it doesn't exist.
    pub async fn get_todo(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        let todo = sqlx::query_as::<_, Todo>("SELECT id, title, completed FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(todo)
    }

    /// Flips `completed` for one todo and returns the updated row.
    pub async fn toggle_todo(&self, id: i64) -> Result<Option<Todo>, sqlx::Error> {
        sqlx::query("UPDATE todos SET completed = NOT completed WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        self.get_todo(id).await
    }

    /// Deletes one todo; returns whether a row was actually removed.
    pub async fn delete_todo(&self, id: i64) -> Result<bool, sqlx::Error> {
        let result = sqlx::query("DELETE FROM todos WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected() > 0)
    }
}
