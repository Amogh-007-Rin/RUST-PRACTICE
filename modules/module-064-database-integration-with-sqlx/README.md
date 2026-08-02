# Module 064: Database Integration with sqlx

**Block:** Block G — Backend Web Development
**Estimated time:** 60–90 min
**Prerequisites:** Module 063 (CRUD with an in-memory store — this module replaces that store with a real database).

## Learning Objectives

- Connect to a SQL database with `sqlx`, using connection pooling.
- Run schema migrations to create and evolve database tables.
- Write parameterized SQL queries to prevent SQL injection.
- Use `query_as` with `FromRow` to map database rows directly to Rust structs.
- Handle query results: `fetch_one`, `fetch_all`, `fetch_optional`, and `execute`.

## Why This Matters

In-memory stores are fine for learning, but real services persist data. `sqlx` is the dominant async SQL library in the Rust ecosystem — it works with PostgreSQL, MySQL, and SQLite using almost identical code. The skills in this module (migrations, parameterized queries, row mapping) are what you'll use every day building backends, whether the database is Postgres in production or SQLite for testing and edge deployments.

## Concept

### Why sqlx?

Rust has two families of SQL libraries: compile-time verified (diesel, sqlx's compile-time macros) and runtime (sqlx's runtime API, rusqlite). This module uses sqlx's *runtime* API — you write plain SQL strings and sqlx executes them, mapping results to Rust types at runtime. The trade-offs:

| Approach | Builds without DB | Type-checked SQL | Best for |
|---|---|---|---|
| sqlx runtime | Yes | At test time | Development speed, dynamic queries |
| sqlx compile-time macros | No (needs `DATABASE_URL`) | At compile time | Production safety guarantees |
| diesel | No | At compile time | Large, schema-stable projects |

The runtime approach has one huge practical advantage: **you don't need a database running to compile**. An in-memory SQLite database starts fresh for every test run, so `cargo test` works on any machine with zero setup. This is also why the module uses SQLite — it's the simplest database to get running, and the same sqlx patterns transfer directly to Postgres.

### Connection pooling

Every database interaction opens *one* pooled connection, runs a query, and returns the connection to the pool. The pool handles concurrent access and connection lifecycle:

```rust
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use std::str::FromStr;

let options = SqliteConnectOptions::from_str("sqlite::memory:")?;
let pool = SqlitePoolOptions::new()
    .max_connections(1)  // SQLite in-memory: must be 1
    .connect_with(options)
    .await?;
```

The pool is cheap to clone (it's `Arc` inside), so you can store it in your application state and pass `State<SqlitePool>` to axum handlers — same pattern as `Arc<Mutex<HashMap>>` from Module 063, but now backed by a real database.

For SQLite's in-memory databases, `max_connections(1)` is required because each connection gets its own isolated in-memory database. For file-backed SQLite or Postgres, you'd use higher numbers for concurrency.

### Migrations

Migrations are versioned SQL files that define your schema. They live in a `migrations/` directory next to `src/`:

```
migrations/
└── 0001_create_todos.sql
```

The file contains plain SQL:

```sql
CREATE TABLE todos (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    title TEXT NOT NULL,
    completed INTEGER NOT NULL DEFAULT 0
);
```

Applying migrations is one line:

```rust
sqlx::migrate!("./migrations").run(&pool).await?;
```

The `migrate!` macro embeds the migration files into the binary at compile time — the directory path is relative to the crate's `Cargo.toml`. On first run, sqlx creates a `_sqlx_migrations` table in the database to track which migrations have been applied, so subsequent runs are a no-op. This is idempotent: you can call `init_schema` on every startup and it's safe.

### Parameterized queries

The most important security concept in this module: **never concatenate user input into SQL strings**. String interpolation creates SQL injection vulnerabilities:

```rust
// NEVER DO THIS
let query = format!("SELECT * FROM todos WHERE id = {}", user_input);
```

Instead, use parameterized queries with placeholders (`?` for SQLite and most databases, `$1` for PostgreSQL) and `.bind()`:

```rust
sqlx::query("SELECT id, title, completed FROM todos WHERE id = ?")
    .bind(id)
    .fetch_optional(&pool)
    .await?
```

sqlx sends the query and the parameter *separately* to the database. The database compiles the query once, then substitutes each `?` with the bound value — treating it as data, never as executable SQL. This makes injection impossible regardless of what the user types.

### Mapping rows to structs

`FromRow` is a derive macro that tells sqlx how to map column names to struct fields:

```rust
#[derive(Clone, Debug, PartialEq, FromRow)]
pub struct Todo {
    pub id: i64,
    pub title: String,
    pub completed: bool,
}
```

With that, `query_as::<_, Todo>("SELECT ...")` automatically maps each returned row to a `Todo`. The column names in the query must match the struct field names (case-insensitive by default). SQLite stores booleans as integers (0/1), but `FromRow` handles the conversion transparently.

### Query result shapes

sqlx provides four ways to consume a query result, each for a different scenario:

| Method | Returns | Use when |
|---|---|---|
| `fetch_one` | One row (error if zero or multiple) | Primary key lookup, RETURNING clauses |
| `fetch_optional` | `Option<Row>` — `None` if zero rows | "Find by id, but allow not found" |
| `fetch_all` | `Vec<Row>` | List endpoints, bulk reads |
| `execute` | `QueryResult` (rows_affected, last_insert_id) | INSERT/UPDATE/DELETE where you don't need the row back |

A common pattern for inserts: use `RETURNING` to get the full row back in the same query:

```rust
let todo = sqlx::query_as::<_, Todo>(
    "INSERT INTO todos (title, completed) VALUES (?, 0) RETURNING id, title, completed"
)
.bind(title)
.fetch_one(&pool)
.await?;
```

`RETURNING` works in SQLite 3.35+ and PostgreSQL. It's more efficient than INSERT + separate SELECT: one round trip instead of two.

### The toggle pattern

Updating a boolean flag is a common operation. SQL has a direct way to do it without round-tripping:

```sql
UPDATE todos SET completed = NOT completed WHERE id = ?
```

Then re-fetch with `get_todo(id)` to return the updated row. The exercise combines `sqlx::query(...).execute(...)` for the UPDATE with `fetch_optional` for the re-fetch — two queries, but the first is a single write and the second is a consistent read from the same pool connection.

### The full lifecycle

A typical database-backed request:

```
Request: POST /todos {"title": "buy milk"}
    │
    ▼
Handler extracts Json<NewTodo>
    │
    ▼
Store::create_todo("buy milk")
    │
    ▼
sqlx::query_as("INSERT ... RETURNING ...")
    │
    ▼
Pool leases a connection → query executes → connection returns to pool
    │
    ▼
Returns Todo { id: 1, title: "buy milk", completed: false }
    │
    ▼
Handler converts to Json<Todo>, returns 201 Created
```

## Common Pitfalls

- **Forgetting to run migrations.** Calling `create_todo` without `init_schema` first fails because the `todos` table doesn't exist. Always apply migrations at startup in the `connect` function.
- **String interpolation instead of `bind`.** `format!("... WHERE id = {id}")` opens an injection hole. Always use `?` placeholders and `.bind(value)`.
- **Using `fetch_one` for "maybe missing" lookups.** `fetch_one` panics or returns an error on zero rows — use `fetch_optional` when a missing id is a normal outcome (it returns `None`).
- **Assuming SQLite booleans are `true`/`false` in SQL.** SQLite has no boolean type; store them as integers (0/1). sqlx's `FromRow` handles the conversion when the Rust field is `bool`, but raw SQL must use `0`/`1` or `NOT completed` for toggling.
- **Using `max_connections(1)` with file-backed SQLite in production.** This is only for in-memory databases. File-backed SQLite supports WAL mode and multiple readers — use a larger pool size.

## Key Terms

- **Connection pool:** A set of open database connections that handlers borrow and return, avoiding the cost of opening a new connection per request.
- **Migration:** A versioned SQL file that defines or changes the database schema, applied once and tracked in a metadata table.
- **Parameterized query:** A SQL statement with `?` placeholders, with values supplied separately via `.bind()`, preventing injection.
- **`FromRow`:** A derive macro that maps database columns to struct fields.
- **`RETURNING`:** A SQL clause that returns the inserted/updated/deleted row(s) — avoiding a separate SELECT.
- **Injection:** A security vulnerability where user input is treated as SQL code, prevented by parameterization.

## Exercise

Open `exercises/src/lib.rs`. The `TodoStore` struct, connection logic, and `init_schema` are complete. Five data-access methods contain `// TODO(module-064)` stubs:

1. **`create_todo`** — INSERT a todo with `completed = 0`, return the row via `RETURNING id, title, completed`. Use `query_as::<_, Todo>` with `.bind(title)` and `.fetch_one`.

2. **`list_todos`** — SELECT all todos ordered by id, return as `Vec<Todo>` with `query_as` and `.fetch_all`.

3. **`get_todo`** — SELECT by id using `WHERE id = ?`, `.bind(id)`, and `.fetch_optional` (returns `Option<Todo>`).

4. **`toggle_todo`** — UPDATE with `SET completed = NOT completed WHERE id = ?` via `query(...).execute(...)`, then re-fetch with `self.get_todo(id).await`.

5. **`delete_todo`** — DELETE by id, return `result.rows_affected() > 0`.

The tests in `tests/module_064.rs` create an in-memory SQLite database per test, exercise every operation, and include a file-backed database test. Run:

```bash
cargo test -p module-064-exercises
```

Compare with `solutions/` when all tests pass.

## Further Reading

- [sqlx documentation](https://docs.rs/sqlx) — the `query`, `query_as`, connection pool, and migration docs
- [SQL injection prevention cheat sheet](https://cheatsheetseries.owasp.org/cheatsheets/SQL_Injection_Prevention_Cheat_Sheet.html)
- [SQLite documentation](https://sqlite.org/docs.html)
- [Module 063: Building REST APIs with Axum](modules/module-063-building-rest-apis-with-axum/README.md)
