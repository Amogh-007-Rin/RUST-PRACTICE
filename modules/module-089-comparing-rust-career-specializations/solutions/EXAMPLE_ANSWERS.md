# Module 089 Example Answers

These are sample answers — not "correct" answers. Your responses will differ based on your interests, location, and research date. Use these as a benchmark for specificity and concreteness, not as content to copy.

---

## 1. Your Top 3 Rust Specialization Interests

| # | Specialization | Why It Appeals | Project Idea |
|---|---------------|----------------|-------------|
| 1 | Backend Web | I spent most of Blocks E and G engaged with Axum and sqlx — building APIs that handle real traffic feels more tangible to me than games or firmware. The job market is also the largest, which matters since I am job-hunting within 4 months. | A rate-limited URL shortener with analytics: Axum + sqlx + Redis, Dockerized, with a load test showing 50k RPS on a $20/month VPS. |
| 2 | Embedded Systems | I have an electrical engineering background from university and Module 087's interrupt controller was the most fun I have had in the curriculum. I like that the code runs on physical hardware I can hold. | A CO2 monitor: ESP32-C3 reading an SCD40 sensor over I2C, logging to an e-ink display, with Embassy async tasks for sensor polling and display refresh. |
| 3 | Blockchain | I find consensus algorithms and distributed state machines intellectually compelling. The deterministic execution model (no `std::time`) forces a rigor I respect. The compensation potential is higher than backend, though the volatility concerns me. | An on-chain voting program on Solana: create a proposal, cast votes, tally results — with Anchor tests covering all edge cases. |

---

## 2. Companies Hiring Per Specialization

### Backend Web

| Specialization | Company | What They Build | Hiring? | Why Interesting |
|---------------|---------|----------------|---------|-----------------|
| Backend | Cloudflare | Workers (serverless compute), Pingora (HTTP proxy replacing nginx, written in Rust) | Yes | Pingora is a flagship Rust infrastructure project — open-source and handling a massive share of internet traffic. The engineering blog details performance decisions. |
| Backend | Fly.io | Global application platform — edge compute with Rust-based orchestration | Yes | Fly.io's entire control plane and edge runtime is Rust. They publish "Rust at Fly.io" blog posts and open-source their internal libraries. Small team, high impact per engineer. |
| Backend | Apollo GraphQL | GraphQL router — a high-performance Rust gateway that sits in front of GraphQL servers | Yes | The Apollo Router is Rust compiled to a single binary. It replaced a Node.js implementation and cut p99 latency by 10x. A good example of Rust replacing JS in production. |

### Embedded Systems

| Specialization | Company | What They Build | Hiring? | Why Interesting |
|---------------|---------|----------------|---------|-----------------|
| Embedded | Espressif | ESP32 microcontrollers with first-class Rust support via esp-hal and esp-wifi | Yes | Espressif employs Rust developers to maintain the official Rust HAL for their chips. Rare chance to work on Rust tooling at a semiconductor company. |
| Embedded | Tweede Golf | Safety-critical embedded Rust consulting — medical devices, automotive, industrial | Sometimes | A consulting firm specializing exclusively in Rust for safety-critical systems. Blog posts about certification (IEC 62304, ISO 26262) with Rust are unique in the industry. |
| Embedded | OxidOS | Rust-based automotive OS and ECU platform | Yes | Building an RTOS in Rust for automotive ECUs — directly competing with AUTOSAR. Small startup but targeting a trillion-dollar industry that is mandating memory-safe software. |

### Blockchain

| Specialization | Company | What They Build | Hiring? | Why Interesting |
|---------------|---------|----------------|---------|-----------------|
| Blockchain | Solana Labs | Solana validator client (Rust), Solana Program Library (SPL token, governance) | Yes | The Solana validator is the most performance-critical Rust codebase in blockchain — 400ms block times, tens of thousands of TPS. Working on it means optimizing at the compiler and CPU level. |
| Blockchain | Parity Technologies | Substrate (Polkadot's blockchain framework) written in Rust | Yes | Substrate is a framework for building entire blockchains in Rust. Working here means contributing to the tool that dozens of parachains are built on. |
| Blockchain | Helius | Solana RPC infrastructure and developer tools | Yes | Builds the infrastructure that Solana dApps depend on — geyser plugins, compressed NFTs, webhooks. More platform-engineering than smart-contract work, which suits my backend interests. |

---

## 3. Tech Stack Comparison

The table below fills in the comparison matrix from the worksheet:

| Layer | Backend | Embedded | Blockchain | Game Dev | WASM |
|-------|---------|----------|------------|----------|------|
| Framework | Axum / Actix-web | embedded-hal + PAC | anchor (Solana) / frame (Substrate) | Bevy ECS + wgpu | Leptos / Dioxus |
| Storage | Postgres (sqlx), Redis | On-chip flash / EEPROM | On-chain account data | Scene graph / asset files | IndexedDB (web-sys) |
| Serialization | serde (JSON) | postcard / bincode | borsh / SCALE | ron / custom binary | serde (JSON to JS) |
| Build | cargo | cargo + probe-rs / cargo-embed | cargo-build-sbf (Solana) / cargo-contract | cargo + wasm-pack (for wasm) | wasm-pack / trunk |
| Testing | #[tokio::test], reqwest test client | defmt over RTT / QEMU | Anchor framework / #[solana-program-test] | Bevy app tests | wasm-pack test --headless |
| Deployment | Docker → K8s / fly.io | Firmware flash via SWD/JTAG | solana program deploy / runtime upgrade | Native binary / WASM in browser | Static file server + JS glue |

**Surprising differences:**

- **Serialization is the most fragmented layer across specializations.** Backend uses JSON via serde. Embedded uses postcard (compact binary for no-std) or bincode. Blockchain uses its own encodings (SCALE for Substrate, Borsh for Solana) because deterministic serialization is a consensus requirement — two nodes must produce identical bytes, which serde doesn't guarantee across versions. Game dev serializes scene data with `ron` (Rusty Object Notation) or custom binary formats. WASM serializes to JSON for the JS boundary.
- **Testing is the most environment-dependent layer.** Backend tests run with `cargo test` like any Rust code. Embedded tests require hardware or a simulator (QEMU, Wokwi). Blockchain tests require a local validator. Game tests must run a Bevy `App` inside a test. WASM tests require a headless browser (`wasm-pack test --headless`). This means the same `cargo test` command does wildly different things in each specialization.
- **Deployment targets span four orders of magnitude in resource constraints.** Embedded devices have kilobytes of RAM. Browser WASM has tens to hundreds of megabytes. Backend services have gigabytes. Blockchain validators have hundreds of gigabytes of RAM and terabytes of storage. The same Rust code is running on all of them.

---

## 4. Your 3-Month Learning Plan

**Chosen specialization:** Backend Web Development

| Week | Focus Area | Rust.Stack Modules to Revisit | New Resources to Study | Deliverable |
|------|-----------|------------------------------|----------------------|-------------|
| 1 | Solidify Axum + sqlx | 062-064 (Axum, REST APIs, database) | [Zero to Production in Rust](https://www.zero2prod.com) chapters 1-4 | Working subscription-email API: Axum POST endpoint, sqlx migrations, integration test |
| 2 | Auth & middleware | 065-066 (JWT, Tower middleware) | Tower docs: Service trait, Layer trait | Add JWT auth middleware to the email API; write tests for unauthenticated rejection |
| 3 | Observability | 069 (deployment, tracing) | tracing-subscriber setup, OpenTelemetry exporter | Add structured tracing spans to every endpoint; export to Jaeger running in Docker |
| 4 | Redis & caching | Capstone 05 (rate-limited crawler) | redis-rs docs, cache-aside pattern | Add Redis-backed rate limiting to the API; load-test with `oha` showing limits enforced |
| 5 | Background jobs | 078-079 (distributed KV, message queues) | SQLx listen/notify or `lapper` | Implement email sending as async background job with status endpoint |
| 6 | Capstone 07 rebuild | 070 (Capstone 07: Task Management API) | Pagination patterns, cursor vs offset | Rebuild Capstone 07 from scratch with all patterns learned: paginated list, filtering, bulk operations |
| 7 | Performance profiling | 054-055 (profiling, benchmarking) | `cargo-flamegraph`, `pprof-rs` | Profile the task API under load; find and fix the top bottleneck; document before/after |
| 8 | gRPC service | 076 (gRPC with Tonic) | Tonic interceptor docs, proto3 style guide | Add a gRPC version of the task API alongside the REST endpoints |
| 9 | Docker & CI | 037-039 (cargo features, publish) | Docker multi-stage builds for Rust, GitHub Actions caching | Full Dockerfile (builder + distroless runtime), CI pipeline with fmt/clippy/test/docker-build |
| 10 | Portfolio project: design | — | System design resources, hexagonal architecture in Rust | Design doc for portfolio project: architecture diagram, data model, API spec, testing strategy |
| 11 | Portfolio project: build | — | — | Working MVP: at least 3 endpoints, database, auth, tracing, Docker |
| 12 | Portfolio project: polish | 097 (portfolio, resume) | README template, case study writing | Deployed project with live endpoint, load-test results, architecture diagram, case-study README |

---

## 5. Open-Source Projects to Target

**Project 1: `sqlx`**

- **Name & URL:** sqlx — https://github.com/launchbadge/sqlx
- **What it does:** Async, compile-time-checked SQL toolkit for Rust. Queries are verified against a live database at compile time.
- **Why it matters for my career:** sqlx is the most-used async database crate in Rust. Contributing means my code touches thousands of production services. The maintainers are well-known in the Rust community.
- **Potential contribution:** Improve error messages for compile-time query checking. When a query fails to type-check, the error often spans 200+ lines of proc-macro output. A controlled wrapper that extracts the relevant line and suggests a fix would help every sqlx user.
- **Relevant Rust.Stack modules:** 013-014 (Error Handling), 016-017 (Traits), 038 (Procedural Macros overview), 064 (sqlx usage).

**Project 2: `axum`**

- **Name & URL:** axum — https://github.com/tokio-rs/axum
- **What it does:** Ergonomic and modular web framework built on Tokio, Tower, and Hyper. The most popular Rust web framework by GitHub stars.
- **Why it matters for my career:** Contributing to the framework I use daily demonstrates deep understanding. Even a documentation improvement shows I read the source code of the tools I depend on.
- **Potential contribution:** Improve the extractor documentation. Specifically, document how to write custom extractors with detailed examples (header parsing, query parameter validation, request body limits). The current docs cover `FromRequest` but lack worked examples for common patterns.
- **Relevant Rust.Stack modules:** 062-063 (Axum), 025 (Advanced Traits — `FromRequest` is a trait), 066 (Middleware/Tower ecosystem).
