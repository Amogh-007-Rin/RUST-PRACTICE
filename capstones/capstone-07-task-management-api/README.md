# Capstone 07: Task Management API

A full CRUD backend service built with Axum, sqlx, and SQLite featuring JWT authentication, request validation, structured logging, and integration tests.

**Status: complete**

## Overview

Build a RESTful task management API from scratch. You'll implement:

- JWT-based user registration and login
- Full CRUD operations on tasks with ownership scoping
- Query parameter filtering (by status and priority)
- Integration tests using in-memory SQLite
- Structured logging with `tracing`

## What You'll Practice

- Axum 0.7 routing, extractors, and state management
- Custom `FromRequestParts` for JWT auth middleware
- sqlx with SQLite (queries, migrations, `FromRow` derive)
- `thiserror` for typed API errors with `IntoResponse`
- argon2 password hashing
- jsonwebtoken for token generation and verification
- `axum-test` for integration tests against an in-memory database
- `serde` derive for request/response serialization (including enums)
- Docker multi-stage builds

## Project Structure

```
capstone-07-task-management-api/
  starter/           # Scaffold with TODO markers
    Cargo.toml
    src/
      lib.rs         # App library (models, handlers, router)
      main.rs        # Binary entry point
    tests/
      capstone_07.rs # Integration tests
  solution/          # Reference implementation
    Cargo.toml
    src/
      lib.rs
      main.rs
    tests/
      capstone_07.rs
  Dockerfile         # Multi-stage Docker build
  README.md
```

## API Routes

| Method | Path              | Auth | Description              |
|--------|-------------------|------|--------------------------|
| GET    | /api/health       | No   | Health check             |
| POST   | /api/auth/register| No   | Register a new user      |
| POST   | /api/auth/login   | No   | Login, receive JWT       |
| GET    | /api/tasks        | Yes  | List user's tasks        |
| POST   | /api/tasks        | Yes  | Create a task            |
| GET    | /api/tasks/:id    | Yes  | Get a single task        |
| PUT    | /api/tasks/:id    | Yes  | Update a task            |
| DELETE | /api/tasks/:id    | Yes  | Delete a task            |

### Query Parameters for GET /api/tasks

- `status` — filter by status: `todo`, `in_progress`, `done`
- `priority` — filter by priority: `low`, `medium`, `high`

Example: `GET /api/tasks?status=todo&priority=high`

## Getting Started

### Run the starter (with TODO stubs)

```bash
cd capstones/capstone-07-task-management-api/starter
cargo run
```

### Run the tests

```bash
cargo test -p capstone-07-task-management-api-starter
```

Three tests should pass initially (health check and unauthorized access). The remaining tests are marked `#[ignore]` and will pass as you implement the TODO sections.

### Build with Docker

```bash
cd /path/to/RUST.STACK
docker build -f capstones/capstone-07-task-management-api/Dockerfile -t task-api .
docker run -p 3000:3000 task-api
```

## Implementation Order

1. **Models and types** — All types are already defined in the starter.
2. **`make_token` helper** — Generate JWTs using `jsonwebtoken`.
3. **`AuthUser` extractor** — Read `Authorization` header, verify JWT, extract `user_id`.
4. **`register` handler** — Validate input, hash password with argon2, insert user, return JWT.
5. **`login` handler** — Look up user, verify password hash, return JWT.
6. **`create_task` handler** — Validate title, generate UUID, insert into database, return task.
7. **`get_task` handler** — Fetch task by ID scoped to the authenticated user.
8. **`list_tasks` handler** — Query all tasks for the user, with optional status/priority filters.
9. **`update_task` handler** — Fetch existing task, merge changes, update database.
10. **`delete_task` handler** — Delete task scoped to user, return 204 or 404.

## Environment Variables

| Variable       | Default                      | Description          |
|---------------|------------------------------|----------------------|
| DATABASE_URL   | sqlite:task-api.db?mode=rwc  | SQLite database path |
| JWT_SECRET     | dev-secret-change-in…        | JWT signing secret   |
| PORT           | 3000                         | Server port          |

## Dependencies

- **axum** — Web framework
- **sqlx** — Async SQL toolkit (SQLite)
- **argon2** — Password hashing
- **jsonwebtoken** — JWT creation/verification
- **uuid** — ID generation
- **serde / serde_json** — Serialization
- **tower-http** — HTTP middleware (CORS, tracing)
- **tracing / tracing-subscriber** — Structured logging
- **thiserror / anyhow** — Error handling
- **axum-test** — Integration testing (dev dependency)
