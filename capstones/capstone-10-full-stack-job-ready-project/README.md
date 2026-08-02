# Capstone 10: Full-Stack Rust URL Shortener with Analytics

**Portfolio-grade project combining Axum backend + HTML/CSS frontend with tests, CI, Docker, and analytics.**

## Project Brief

Build a URL shortener service with click analytics — a real-world, production-style application demonstrating full-stack Rust proficiency. The backend uses the Axum web framework with SQLite persistence via sqlx. The frontend is a single-page HTML/CSS/JS dashboard served directly from the Rust server. This project shows you can deliver a complete web application end-to-end in Rust.

## Requirements

### Functional
- [x] Shorten URLs: `POST /api/links` accepts `{ "url": "..." }` and returns a short code
- [x] Redirect: `GET /{code}` redirects to the original URL and records a click
- [x] List links: `GET /api/links` returns all shortened URLs with click counts
- [x] Link details: `GET /api/links/{code}` returns a single link
- [x] Click analytics: `GET /api/links/{code}/stats` returns click events and hourly aggregation
- [x] Delete: `DELETE /api/links/{code}` removes a link and its click history
- [x] Health check: `GET /api/health` returns `{ "status": "ok" }`
- [x] Dashboard: `GET /` serves an interactive HTML dashboard (create links, copy, view stats, delete)

### Technical
- **Backend**: Axum 0.7 with Tower middleware (CORS, tracing)
- **Database**: SQLite via sqlx 0.8 with programmatic migrations
- **Short codes**: 8-character nanoid (URL-safe, collision-resistant)
- **IDs**: UUID v4 for all records
- **Tracing**: Structured logging via tracing-subscriber
- **Serialization**: serde + serde_json for all API payloads

### Testing
- 10 integration tests covering all endpoints and edge cases
- Tests run against an in-memory SQLite database with a real Axum server on a random port
- Tests verify: CRUD operations, redirects, click tracking, stats aggregation, HTML dashboard, error handling

### DevOps
- **Dockerfile**: Multi-stage build (rust:1.83-slim-bookworm → debian:bookworm-slim)
- **CI-ready**: All tests pass with `cargo test`, lint clean with `cargo clippy`
- **Environment config**: `DATABASE_URL` and `BASE_URL` via env vars with sensible defaults

## Architecture

```
POST   /api/links        → create_link        (validate URL → insert → return code)
GET    /api/links        → list_links         (SELECT * ORDER BY created_at DESC)
GET    /api/links/:code  → get_link           (SELECT by code → 200 or 404)
GET    /api/links/:code/stats → get_link_stats  (link + events + hourly aggregation)
DELETE /api/links/:code  → delete_link        (delete events → delete link)
GET    /:code            → redirect_to_original (lookup → record click → 307 redirect)
GET    /                 → serve_dashboard    (static HTML with embedded JS)
GET    /api/health       → health_check       ({ status: "ok" })
```

### Data Model

**short_links**
| Column       | Type    | Notes                    |
|-------------|---------|--------------------------|
| id          | TEXT PK | UUID v4                  |
| short_code  | TEXT UNIQUE | 8-char nanoid         |
| original_url| TEXT    |                          |
| created_at  | TEXT    | RFC 3339                 |
| click_count | INTEGER | Default 0                |

**click_events**
| Column     | Type    | Notes                |
|-----------|---------|----------------------|
| id        | TEXT PK | UUID v4              |
| short_code| TEXT    | FK → short_links     |
| timestamp | TEXT    | RFC 3339             |
| user_agent| TEXT?   | From request headers |

## Project Structure

```
capstone-10-full-stack-job-ready-project/
├── README.md               ← You are here
├── Dockerfile              ← Multi-stage production build
├── starter/
│   ├── Cargo.toml          ← Dependencies pre-configured
│   ├── src/
│   │   ├── main.rs         ← Server entry point (complete)
│   │   └── lib.rs          ← Scaffolded implementation with TODOs
│   └── tests/
│       └── capstone_10.rs  ← Integration tests (run against your implementation)
└── solution/
    ├── Cargo.toml
    ├── src/
    │   ├── main.rs
    │   └── lib.rs          ← Full working implementation
    └── tests/
        └── capstone_10.rs  ← All 10 tests passing
```

## Getting Started

### Run the solution

```bash
cd /home/enjin/Projects/RUST.STACK
cargo run -p capstone-10-solution
# Open http://localhost:3000 in a browser
```

### Run tests

```bash
cargo test -p capstone-10-solution
```

### Build Docker image

```bash
docker build -f capstones/capstone-10-full-stack-job-ready-project/Dockerfile \
             -t url-shortener \
             /home/enjin/Projects/RUST.STACK
docker run -p 3000:3000 url-shortener
```

### Starter exercise

1. Open `starter/src/lib.rs`
2. Work through each `TODO` marker implementing the handlers
3. Run `cargo test -p capstone-10-starter` to verify each test passing
4. Compare with `solution/src/lib.rs` for reference

## Stretch Goals

- [ ] Add authentication (API keys or JWT) so users own their links
- [ ] Add a `ttl` (time-to-live) field with automatic link expiration
- [ ] Add rate limiting (tower-governor or custom middleware)
- [ ] Store click geo-location via an IP-to-location service
- [ ] Add Prometheus metrics endpoint
- [ ] Build a Leptos/WASM frontend instead of plain HTML
- [ ] Deploy to Fly.io or Shuttle with a public domain

## Acceptance Criteria

1. All 10 integration tests pass: `cargo test -p capstone-10-solution`
2. No clippy warnings: `cargo clippy -p capstone-10-solution -- -D warnings`
3. Code is formatted: `cargo fmt`
4. Docker image builds and serves the application
5. README includes Job Readiness Checklist (below)

## Design Notes

- **No framework on the frontend**: Plain HTML/CSS/JS keeps it testable and avoids WASM compilation complexity. The dashboard is embedded as a Rust string constant.
- **In-memory SQLite for tests**: Every test gets a fresh `:memory:` database — no cleanup needed, tests run in parallel.
- **307 Temporary Redirect**: Uses `TEMPORARY_REDIRECT` so browsers don't cache the redirect and we always capture clicks.
- **Nanoid for codes**: 8-character alphanumeric codes using the A-Za-z0-9_ alphabet gives ~2×10¹⁴ combinations — collision-resistant for practical use.
- **Tower middleware**: TraceLayer logs every request; CorsLayer is permissive for development (restrict in production).

---

## Job Readiness Checklist

Use this checklist to confirm you're ready to apply for Rust jobs. Modeled after Module 097 portfolio guidance.

### Module 091–099 Progress
- [ ] **Module 091**: Advanced Ownership Patterns — interior mutability, Rc/Arc, Cow
- [ ] **Module 092**: SIMD and Low-Level Optimizations — target_feature, intrinsics
- [ ] **Module 093**: Rust in IoT and Resource-Constrained Devices — `#![no_std]`, embedded
- [ ] **Module 094**: High-Performance Rust Tuning — flamegraphs, criterion, allocation profiling
- [ ] **Module 095**: Rust in Cloud-Native Environments — containers, Kubernetes, observability
- [ ] **Module 096**: Open-Source Contribution — PR merged or issue resolved
- [ ] **Module 097**: Building Your Portfolio — case studies, README polish, GitHub profile
- [ ] **Module 098**: The Rust Job Market and Application Strategy — target companies, resume tailoring
- [ ] **Module 099**: Behavioral and System-Design Interviews — STAR method, architecture diagrams
- [ ] **Module 100**: Final Capstone Support — peer review, mentor feedback

### Portfolio Polish
- [ ] At least 3 capstone projects with clean READMEs and test suites
- [ ] GitHub profile: pinned repos, contribution graph, README profile
- [ ] Personal site or dev.to/blog posts about your Rust projects
- [ ] Code is linted (`clippy`), formatted (`fmt`), and documented (doc comments)

### Resume Ready
- [ ] Resume tailored for Rust roles with project highlights
- [ ] Keywords: async/await, tokio, sqlx, Axum, WASM, systems programming
- [ ] Quantified achievements (e.g., "Reduced latency by 40% using...")
- [ ] GitHub, LinkedIn, and portfolio links prominent

### Mock Interview Practice
- [ ] Completed 3+ mock technical interviews (Rust-specific)
- [ ] Practiced explaining ownership, lifetimes, async runtime internals
- [ ] Can whiteboard a system design (e.g., "Design a URL shortener")
- [ ] Can walk through behaviors in STAR format (Situation, Task, Action, Result)

### Specialization Chosen
- [ ] **Backend/API Engineering**: Axum, Actix, database design, API patterns
- [ ] **Systems/Embedded**: `#![no_std]`, RTIC, Embassy, hardware interfaces
- [ ] **Cloud/DevOps**: Containers, CI/CD, infrastructure-as-code, observability
- [ ] **Blockchain/Web3**: Solana, NEAR, Polkadot, smart contracts (Rust-based)
- [ ] **Game/Graphics**: Bevy, wgpu, Vulkan bindings, game loops
- [ ] **Tooling/CLI**: clap, indicatif, cross-compilation, developer tools

### Open-Source Contribution
- [ ] At least 1 merged PR to a Rust project
- [ ] Issue triage or documentation contribution
- [ ] Understands open-source workflow (fork, branch, PR, review cycle)

### This Capstone Demonstrates
- [x] Full-stack Rust: Axum backend with production-grade patterns
- [x] Database integration: sqlx with SQLite, migrations, query patterns
- [x] Testing: Integration tests against a real HTTP server
- [x] DevOps: Multi-stage Dockerfile, environment configuration
- [x] Clean code: Structured with separation of concerns (models, handlers, router)
- [x] Documentation: Comprehensive README with architecture diagrams

---

**Built for Capstone 10. Covers Modules 091–100.**
