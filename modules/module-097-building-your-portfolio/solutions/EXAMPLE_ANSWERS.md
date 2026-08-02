# Module 097 Example Answers

These are sample answers — your own profile, projects, and aspirations will generate different responses. Use these as a benchmark for specificity and honesty, not as templates to copy.

---

## Prompt 1: Audit Your Current GitHub Profile

### Sample Audit (for a hypothetical Rust learner, "Alex," mid-way through Rust.Stack)

**Profile README:** I don't have a profile README yet. A hiring manager landing on my profile sees a list of pinned repos and a contribution graph — no context about who I am or what I do. **Action:** Create a profile README that says: "Rust developer | building backend services and CLI tools | currently working through Rust.Stack" with links to my 2 best projects.

**Pinned repositories:** I have 6 pinned repos. Two are Rust (an inventory CLI from Capstone 02, a half-finished chat server). Four are old Python/JS projects from 2022. The signal-to-noise ratio is low. **Action:** Unpin the 4 pre-Rust repos. Pin the inventory CLI, the log processor from Capstone 04, and (once finished) the Capstone 07 Task Management API. Three strong Rust repos beats six mixed-language repos.

**Repository READMEs:**
- Inventory CLI: README has install instructions but no screenshots, no "why," no architecture section. Rating: 2/5.
- Log processor: README is boilerplate from `cargo init`. Rating: 0/5.
- Chat server: Incomplete project, README says "WIP." Rating: 1/5.
**Action:** Write proper case-study READMEs for the inventory CLI and the log processor (see Prompt 2).

**Activity graph:** Last 3 months show 1–2 contributions per week, mostly weekends. Not terrible for someone learning while employed, but could be stronger. **Action:** Set a goal of one commit per day for 30 days, even if it's small — fixing a doc comment, adding a test, or solving a module exercise. Consistent green squares signal discipline.

**Languages:** GitHub says my top language is Python because of those old repos. **Action:** Delete or archive the Python/JS repos that I'm not maintaining. Once Rust repos dominate my profile, the language bar will reflect reality.

**3 high-impact changes this week:**
1. Write and deploy a profile README.
2. Archive all pre-Rust repos.
3. Write a case-study README for the inventory CLI (Prompt 2, done in parallel).

---

## Prompt 2: Write a README Case Study for a Project

### Case Study: `inventory-cli` — A Terminal-First Stock Tracker

**One-sentence pitch:** A fast, JSON-backed inventory manager for tracking stock items, categories, and low-stock alerts — built in Rust as a learning exercise in ownership, error handling, and CLI design.

**The problem:** Small businesses and home labs often track inventory in spreadsheets that grow unwieldy. A terminal-based tool lets you `inventory add "Widget" --quantity 50 --category electronics` without leaving your workflow, and `inventory list --low-stock` surfaces items that need reordering before you run out.

**Architecture overview:** The project is organized into three crates inside a workspace: `inventory-core` (the domain types — `Item`, `Category`, `Inventory` — and the JSON persistence layer), `inventory-cli` (the `clap`-based CLI with subcommands for add/list/search/remove/export), and `inventory-tests` (integration tests that exercise the CLI binary end-to-end). The core library uses `serde` for JSON serialization and `thiserror` for an error enum that covers file-not-found, parse errors, and duplicate-item conflicts. The most interesting design decision: the `Inventory` struct holds a `HashMap<String, Item>` keyed by a slug derived from the item name, avoiding linear scans for lookup. When an item is renamed, the slug updates and the old key is removed — a tradeoff of one extra hash insertion for O(1) lookups everywhere else.

**Code snippet — the slug generation logic:**
```rust
fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}
```

**What I learned:** The first version loaded the entire JSON file on every command — startup took 300ms for a 10K-item inventory. Profiling with `cargo flamegraph` showed that 80% of the time was in `serde_json::from_str`. Switching to a memory-mapped file with `memmap2` dropped startup to 30ms. I also learned that `clap`'s derive API makes subcommand routing almost declarative — the entire CLI is defined in a single enum with `#[derive(Subcommand)]`.

**If I were to rebuild it:** I'd use `sled` or `SQLite` from the start instead of a single JSON file. The JSON approach worked for 1K items but would buckle under concurrent access (two terminals running `inventory` simultaneously could clobber each other's writes). Even a simple file-lock with `fs2` would have been an improvement over no locking at all.

---

## Prompt 3: Draft a Resume Bullet Point for a Rust Project

### Bullet point for the Inventory CLI:

> Built a JSON-backed inventory management CLI in Rust serving 10K+ items with sub-30ms startup, using `clap` for argument parsing, `serde` for persistence, and `memmap2`-backed file I/O that reduced cold-start latency by 10x compared to the initial `serde_json::from_str` approach.

### Alternative (for the log processor from Capstone 04):

> Designed a multi-threaded log aggregation tool in Rust processing 2M lines/sec across 8 cores, using `rayon` for data-parallel parsing, `regex` for pattern extraction, and custom `macro_rules!` macros to eliminate repetitive error-handling boilerplate — replaced a single-threaded Python script that took 45+ seconds per run.

---

## Prompt 4: Design a Portfolio Project Idea

### Project: `taskforge` — A Self-Hosted Task Queue with Observability

**Specialization:** Backend / Systems Infrastructure

**What it does:** `taskforge` is a self-hosted task queue server (think: a minimal, single-binary Sidekiq or Celery, but in Rust). Users submit jobs via a REST API (`POST /jobs` with a JSON payload containing a job type and arguments). Workers poll for jobs, execute them, and report results. The server provides a dashboard (terminal UI via `ratatui` or a simple web UI) showing queue depth, processing rate, failed jobs, and per-job-type latency percentiles. It persists jobs to SQLite by default, with a pluggable storage backend (Postgres, in-memory for testing). Jobs can be scheduled for future execution, retried on failure with exponential backoff, and assigned to named queues with per-queue concurrency limits.

**Why it's impressive:** Most "portfolio projects" are CRUD APIs with a database. A task queue demonstrates deeper systems thinking: you have to think about at-least-once delivery semantics, worker lifecycle management (spawn, health-check, reap dead workers), concurrency limits, backpressure (what happens when 100K jobs arrive in 5 seconds?), and observability (how do you know a job is stuck?). It also demonstrates that you can read and implement a real distributed-systems pattern, not just glue libraries together.

**Technical challenges:**
1. **At-least-once delivery:** Jobs must not be lost on crash. This requires an atomic "claim" operation: mark a job as `processing` AND record the worker's ID in a single transaction. If the worker dies, a reaper goroutine (task) unclaims jobs stuck in `processing` after a timeout.
2. **Backpressure:** When the queue depth exceeds a configurable threshold, the enqueue endpoint returns 503 with a `Retry-After` header. This requires tracking queue depth atomically — a simple `AtomicU64` counter with `fetch_add`/`fetch_sub`.
3. **Pluggable storage:** The `Storage` trait must support SQLite and Postgres with the same interface. This is where Module 095's trait-based design pattern pays off.

**Dependency choices:**
- Use: `axum` for the HTTP layer, `sqlx` for database, `tokio` for async runtime, `serde`/`serde_json` for serialization, `tracing` for structured logging, `clap` for the server binary.
- Implement myself: the job state machine (Pending → Claimed → Running → Completed/Failed), the worker health-check / reaper loop, the backpressure mechanism (no crate for this), the pluggable storage trait.

**Deliverables:**
- `taskforge` binary on crates.io with a README case study.
- Integration tests using an in-memory storage backend.
- A `docker-compose.yml` showing `taskforge` + Postgres + a worker.
- A 5-minute demo video or GIF showing jobs flowing through the system.
- A blog post explaining the at-least-once delivery design and the tradeoffs vs. exactly-once.

---

## Prompt 5: List 5 Interview Talking Points From Your Portfolio

### Talking Point 1: The Token Bucket Rate Limiter

**What I'd say:** "In my rate limiter, I inject time as a `now_ms: u64` parameter rather than calling `SystemTime::now()` inside the algorithm. This means tests advance the clock by hand — I can test '100ms passed' without actually sleeping. The tricky part was the refill logic: you have to advance `last_refill_ms` by `added * interval`, not `now_ms`, or you silently lose up to one interval of refill time on every check."

**Demonstrates:** Testable design, algorithmic correctness, awareness of time-injection as an interview pattern.

**Follow-up Q:** "What production rate limiter would you use?" → "`governor` crate for in-process limiting, or a Redis-based approach with sorted sets for distributed limiting. The implementation exercise taught me what to audit in a rate limiter — particularly how it handles clock skew and burst capacity."

---

### Talking Point 2: The Async Crawler's Backpressure Strategy

**What I'd say:** "My Capstone 05 web crawler uses a `Semaphore` from Tokio to cap concurrency at `N` in-flight requests. When the semaphore is exhausted, new URL fetches block at `.acquire().await` — natural backpressure without explicit queue management. The cancellation story uses `select!` with a `CancellationToken`, so shutting down drains in-flight requests gracefully rather than aborting mid-fetch."

**Demonstrates:** Async patterns, structured concurrency, backpressure design.

**Follow-up Q:** "How would you make this distributed across multiple machines?" → "Replace the in-memory URL queue with Redis lists. Each worker would `LPOP` a URL, fetch it, and `RPUSH` discovered URLs. The semaphore becomes per-worker, and the global concurrency limit becomes a separate concern — you'd track active workers in Redis with a TTL-based heartbeat."

---

### Talking Point 3: The `macro_rules!` Log Parser

**What I'd say:** "In my log processor, I had 12 different log formats to parse, each with its own regex and extraction logic. I wrote a `macro_rules! pattern!` macro that takes a regex pattern and field names, and generates a parser function with zero boilerplate — the regex is compiled once at startup, and the macro expands to a strongly-typed struct with named fields. This cut 200 lines of repetitive code down to 40."

**Demonstrates:** Metaprogramming, DRY principles, performance awareness (regex compilation).

**Follow-up Q:** "Why not use a procedural macro instead?" → "`macro_rules!` was sufficient — all patterns were known at compile time, and declarative macros compile faster than proc macros. I'd use a proc macro if I needed to derive parsers from a struct definition with attributes, but for this use case the simpler tool was the right one."

---

### Talking Point 4: The `Storage` Trait in the URL Shortener

**What I'd say:** "My URL shortener defines a `Storage` trait with `get` and `insert` methods. The `Shortener` struct is generic over `S: Storage`. For tests, I use `HashMapStorage`, which is deterministic and instant. For production, I'd swap in `PostgresStorage` implementing the same trait. The shortener's core logic — encoding, URL validation, collision handling — has no idea what the storage backend is. This made it trivially testable."

**Demonstrates:** Trait-based design, dependency inversion, testability patterns.

**Follow-up Q:** "Why generics instead of trait objects?" → "Static dispatch — the storage backend is chosen at compile time and never changes at runtime, so there's no need for dynamic dispatch overhead. If the backend were configurable at runtime (e.g. a config flag choosing SQLite vs. Postgres), I'd use `Box<dyn Storage>` instead."

---

### Talking Point 5: The Portfolio Itself — Process and Polish

**What I'd say:** "I spent a week auditing my GitHub profile: I archived pre-Rust repos, wrote case-study READMEs for my three strongest projects, pinned them, and created a profile README that communicates what I build and what I'm learning. The inventory CLI repo went from 'cargo init boilerplate' to a README with architecture diagrams, a code snippet, and a 'what I learned' section. GitHub isn't just code storage — it's the first thing a hiring manager sees, and I wanted it to say 'this person writes production-worthy Rust.'"

**Demonstrates:** Professionalism, intentionality, understanding of hiring dynamics.

**Follow-up Q:** "What's missing from your portfolio?" → "A production deployment story. All my projects are local. I'm working on a Capstone 10 project that I'll deploy to a \$5 VPS with Docker, a health-check endpoint, and a real domain. Having something running that I can point to in an interview — even a tiny service — closes the gap between 'portfolio project' and 'production experience.'"
