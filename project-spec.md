# Rust.Stack — Project Specification

**Repository:** `Rust.Stack`
**Purpose:** A self-contained, offline-friendly, 0-to-100 modular Rust learning sandbox, taking a learner from zero Rust knowledge to job-ready across every major Rust specialization (backend, async/infra, systems/embedded, CLI/networking, WASM/frontend, game dev, blockchain), finishing with interview and portfolio readiness.
**Format:** Plain GitHub repository. No website, no build step to view content — everything is markdown + runnable Cargo crates. `git clone` and go.
**Audience:** Learners who are comfortable with general programming concepts in *some* language (Python/JS/Java/C++/etc.) but have zero Rust experience. Module 0 briefly bridges programming fundamentals before assuming general dev literacy from Module 1 onward.

This document is the complete build spec. It is meant to be handed to a coding agent that will scaffold and generate the entire repository. Read it fully before generating anything — later sections (especially §7 the Full Curriculum Map, and §11 the Execution Plan) depend on conventions defined earlier.

---

## 1. Guiding Principles

1. **Strictly linear.** Module 0 → Module 100, in order, no branching paths, no "choose your track." Every specialization (backend, async, embedded, WASM, blockchain, game dev, networking) gets its own dedicated block of modules placed in sequence. Depth is achieved by giving each specialization a full 10-module block, not by branching.
2. **Every module is hands-on.** Every module ships with a small, focused exercise (a broken/incomplete Cargo crate the learner fixes until `cargo test` passes). Every 10th module (10, 20, 30 … 100) is a larger **capstone project** that integrates everything from the preceding block.
3. **Solutions are visible.** Every module and capstone has a `solutions/` (or `solution/`) folder with a fully working reference implementation, sitting alongside (not hidden from) the exercise. Learners are trusted to use it responsibly — the value is in the README's guidance to attempt first.
4. **Idiomatic, current, production-grade Rust.** No teaching of outdated patterns "for simplicity." Use `?`, iterators, `thiserror`/`anyhow`, current edition idioms throughout. `rustfmt` and `clippy` clean at all times.
5. **Everything compiles and every test passes.** The single most important acceptance criterion for the finished repo: `cargo test --workspace` and `cargo clippy --workspace -- -D warnings` both succeed from the repo root, on a clean clone, with only `rustup` and the pinned toolchain installed.
6. **Self-contained explanations.** Each module's README teaches the concept from scratch with prose + code examples — it should not require the learner to consult The Rust Book or external docs to complete the exercise, though further reading links are always provided.

---

## 2. Repository Structure

```
Rust.Stack/
├── README.md                     # Landing page, pitch, curriculum map, progress tracker
├── CONTRIBUTING.md
├── LICENSE                       # MIT
├── rust-toolchain.toml           # Pins stable toolchain version
├── rustfmt.toml
├── clippy.toml
├── .gitignore
├── Cargo.toml                    # Root workspace manifest (glob members, see §9)
├── .github/
│   └── workflows/
│       └── ci.yml                # Runs fmt check, clippy, and test across full workspace
├── docs/
│   ├── HOW_TO_USE.md              # How to work through a module, how testing works, FAQ
│   └── SETUP.md                   # Installing Rust/rustup/cargo/rust-analyzer, editor setup
├── scripts/
│   ├── new_module.sh              # Scaffolds a new module folder from the template
│   ├── check_progress.sh          # Parses README progress table, reports completion %
│   └── verify_module.sh           # Runs fmt/clippy/test for a single module by number
├── modules/
│   ├── module-000-orientation/
│   ├── module-001-toolchain-and-first-program/
│   ├── module-002-.../
│   ├── ...
│   └── module-100-final-capstone-support/     # (see note in §7 — 100 is a capstone; module folder holds only the README pointer)
└── capstones/
    ├── capstone-01-.../   (lands after module 010)
    ├── capstone-02-.../   (lands after module 020)
    ├── ...
    └── capstone-10-.../   (lands after module 100 — the final job-ready project)
```

### Naming convention

- Module folders: `module-XXX-kebab-case-slug`, where `XXX` is **zero-padded to 3 digits** (`module-000`, `module-007`, `module-042`, `module-100`). This is a deliberate deviation from saying "module-0"/"module-100" in conversation — zero-padding is required so that `ls`/GitHub's file listing sorts modules in correct numeric order instead of `module-1, module-10, module-100, module-11...`.
- Capstone folders: `capstone-NN-kebab-case-slug`, zero-padded to 2 digits (`capstone-01` … `capstone-10`).
- Every module number referenced in this spec, README tables, and cross-links must use the zero-padded form consistently.

---

## 3. Module Anatomy (template — every module 001–099 follows this exactly)

```
modules/module-XXX-slug/
├── README.md
├── exercises/
│   ├── Cargo.toml
│   ├── src/
│   │   └── lib.rs        # or main.rs where a binary makes more sense (e.g. CLI modules)
│   └── tests/
│       └── module_XXX.rs # integration tests the learner must make pass
└── solutions/
    ├── Cargo.toml
    ├── src/
    │   └── lib.rs         # fully working reference implementation
    └── tests/
        └── module_XXX.rs   # identical tests, all passing
```

**Modules that are primarily conceptual/career content** (e.g. Module 089 "Comparing Rust Career Specializations", Module 096 "Open Source Contribution") keep the same folder shape, but `exercises/` may contain a guided written exercise (a markdown worksheet with prompts) instead of a Cargo crate, e.g. `exercises/WORKSHEET.md` + a small `solutions/EXAMPLE_ANSWERS.md`. This is the only permitted exception to the crate-based exercise format, and it must be used sparingly — see §7 notes for exactly which modules qualify.

### `README.md` template (required sections, in order)

```markdown
# Module XXX: <Title>

**Block:** <Block letter/name> — <Block theme>
**Estimated time:** <e.g. 45–90 min>
**Prerequisites:** Module XXX-1 (and any specific earlier modules if a hard dependency)

## Learning Objectives
- Bullet list of 3-6 concrete, testable objectives ("You will be able to...")

## Why This Matters
2-4 sentences connecting the concept to real Rust jobs/codebases — not generic motivation, specific ("This is the pattern every `axum` handler uses under the hood").

## Concept
800-1500 words of self-contained prose teaching the concept from first principles, with inline code examples (using ```rust fenced blocks, all of which must compile — verify with `rustdoc --test` or by embedding in the exercise crate as doctests where practical). Must include at least one diagram-in-text (ASCII or a clear step-by-step breakdown) for any concept involving memory layout, ownership transfer, or control flow.

## Common Pitfalls
Bullet list of 2-5 mistakes beginners make with this concept, each with a one-line fix/explanation.

## Key Terms
Short glossary of any new vocabulary introduced (term: one-line definition).

## Exercise
What the learner needs to do in `exercises/`, phrased as a task list. Point to the specific TODOs in the code.

## Further Reading
2-4 links: relevant Rust Book chapter, relevant std docs page, one blog/RFC if applicable.
```

### Exercise crate conventions

- `exercises/src/lib.rs` (or `main.rs`) contains working scaffolding plus `todo!()` macros or intentionally incomplete function bodies at every point the learner must fill in. Each TODO has a `// TODO(module-XXX): <specific instruction>` comment.
- `exercises/tests/module_XXX.rs` contains the integration tests that define "done." Tests must be meaningful (not trivial) and must fail against the unmodified scaffold and pass against the reference solution.
- Every exercise crate's `Cargo.toml` uses `edition = "2021"` (or the current stable edition at time of generation) and pins only the minimum dependencies needed for that module's concept — do not bring in `tokio` for a Module 007 pattern-matching exercise, for example.
- `cargo test -p module-XXX-exercises` must fail cleanly (compile, but test-fail — never a compile error) against the unmodified scaffold, unless the module is Rust syntax itself is the point (e.g. Module 002 may have a compile-error-driven exercise around mutability — acceptable only in the very earliest modules, and must be clearly flagged in the README as a "make it compile" exercise).
- `cargo test -p module-XXX-solutions` must always pass.

---

## 4. Capstone Anatomy (template — capstone-01 through capstone-10)

```
capstones/capstone-NN-slug/
├── README.md
├── starter/
│   ├── Cargo.toml
│   └── src/...            # scaffolding with TODOs, may be multi-file (main.rs + modules)
└── solution/
    ├── Cargo.toml
    └── src/...             # complete reference implementation
```

### Capstone `README.md` required sections

```markdown
# Capstone NN: <Title>

**Covers modules:** XXX–YYY
**Estimated time:** <e.g. 4-8 hours>

## Project Brief
A paragraph describing the thing being built and why it's a realistic artifact (not a toy) — frame it the way a take-home assignment or a real small tool would be framed.

## Requirements
Numbered, testable list of functional requirements.

## Stretch Goals
2-4 optional extensions for learners who want to go further (not required to "complete" the capstone).

## Acceptance Criteria
Checklist mirroring what the provided tests check, plus any manual/behavioral checks a test can't express (e.g. "CLI prints a usage message when run with --help").

## Design Notes / Hints
Optional guidance on structuring the solution without giving it away — pointers to which modules' concepts apply where.
```

Capstones should have real automated tests in `starter/tests/` (learner must pass them) mirrored in `solution/tests/`, exactly like a normal module, just larger in scope.

---

## 5. Root `Cargo.toml` (Workspace)

```toml
[workspace]
resolver = "2"
members = [
    "modules/*/exercises",
    "modules/*/solutions",
    "capstones/*/starter",
    "capstones/*/solution",
]

[workspace.package]
edition = "2021"
```

Note: some later-block modules (embedded/no_std, WASM) cannot always live in the default workspace target triple cleanly — where a module's crate requires a different target (e.g. `wasm32-unknown-unknown`, or a `no_std` embedded target), that crate must still be a workspace member for `cargo fmt`/`cargo clippy` purposes, but its `cargo test` behavior should be documented in that module's README (e.g. "run `wasm-pack test --headless --chrome` instead of `cargo test` for this module") and excluded from the default `cargo test --workspace` run via `[package.metadata]` notes or a `default-members` adjustment, so CI doesn't break on machines without the relevant target/toolchain installed. Document any such exception explicitly in that module's README under a new "Running This Module's Tests" subsection.

---

## 6. Tooling Files

- **`rust-toolchain.toml`**: pin to current stable at generation time, e.g.:
  ```toml
  [toolchain]
  channel = "stable"
  components = ["rustfmt", "clippy"]
  ```
- **`rustfmt.toml`**: sensible defaults, `edition = "2021"`, otherwise leave at rustfmt defaults unless there's a strong reason to deviate — this is a teaching repo, code should look like idiomatic community-standard Rust.
- **`clippy.toml`**: default/empty unless a specific lint threshold is needed.
- **`.github/workflows/ci.yml`**: on push/PR to main, run (in order) `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`. Use a matrix only if needed; otherwise ubuntu-latest + the pinned toolchain is sufficient.
- **`scripts/new_module.sh`**: takes a module number and slug, copies the module template (see §3), fills in the number in file paths and README front matter.
- **`scripts/verify_module.sh`**: takes a module number, runs fmt/clippy/test scoped to just that module's two crates — lets a learner (or the generating agent) validate one module without running the whole workspace.
- **`scripts/check_progress.sh`**: parses the checkbox table in root `README.md` and prints "NN/101 modules complete, MM/10 capstones complete."

---

## 7. Full Curriculum Map

101 modules (000–100) + 10 capstones. Ten thematic blocks of 10 modules each (A–J), preceded by Module 0. A capstone lands at the end of every block.

Depth allocation rationale: since the format must stay linear (no branching tracks), depth on each specialization is achieved by dedicating an entire 10-module block to it — Async gets its own block, Backend Web gets its own block, CLI/Networking/Distributed Systems gets its own block, and WASM/Frontend/Game/Embedded/Blockchain share a block covering each in real depth (2–4 modules apiece) rather than one shallow "misc" module per topic.

### Module 000 — Orientation

| # | Title | Covers |
|---|-------|--------|
| 000 | Welcome to Rust.Stack | What Rust is and its history (Mozilla origins, Graydon Hoare, 1.0 in 2015, Foundation today); why Rust exists as a reaction to memory-safety CVEs in C/C++; the "systems programming without a GC" pitch. What you can build with Rust today: CLIs, backend services, embedded firmware, WASM/frontend, blockchain programs, game engines, OS components. High-level compiler architecture (rustc, borrow checker, MIR, LLVM backend) — enough to understand *why* the compiler behaves the way it does later. What memory safety means concretely (use-after-free, data races, buffer overflows) and why it's an industry-wide hiring driver (mention the general trend of memory-safe-language mandates without overclaiming specific current employer figures). A one-paragraph preview of concurrency and async Rust, revisited properly in Blocks D and E. What a "high-paying Rust developer job" actually looks for (systems thinking, ownership fluency, one specialization track). How to use this repo: module anatomy, exercises vs solutions, capstone cadence, testing workflow. |

### Block A — Foundations I (Modules 001–010) → Capstone 01

| # | Title |
|---|-------|
| 001 | Toolchain & Your First Program — rustup, cargo new/build/run/check, anatomy of `main.rs`, `println!` |
| 002 | Variables, Mutability & Data Types — `let`, `mut`, `const`, scalar/compound types, shadowing |
| 003 | Functions & Control Flow — `fn`, expressions vs statements, `if`/`else`, `loop`/`while`/`for` |
| 004 | Ownership Part 1 — stack vs heap, the three ownership rules, move semantics, `.clone()` |
| 005 | Ownership Part 2: Borrowing & References — `&`, `&mut`, the borrow checker's rules, dangling references |
| 006 | Slices & Strings — `String` vs `&str`, the slice type, common string methods, UTF-8 gotchas |
| 007 | Structs — defining, instantiating, methods, associated functions, tuple/unit structs |
| 008 | Enums & Pattern Matching — enum definitions, `Option<T>`, `match`, `if let`/`while let` |
| 009 | Modules, Crates & Packages — `mod`, `pub`, `use`, crate layout, intro to workspaces |
| 010 | *(Capstone 01)* |

**Capstone 01 — "Contact Book CLI"**: A command-line contact manager (add/list/search/remove contacts) exercising structs, enums, ownership/borrowing, and module organization, storing data in memory for the session.

### Block B — Foundations II (Modules 011–020) → Capstone 02

| # | Title |
|---|-------|
| 011 | Common Collections I — `Vec<T>` |
| 012 | Common Collections II — `HashMap<K,V>`, `HashSet<T>` |
| 013 | Error Handling I — `panic!`, `Result<T, E>`, `.unwrap()`/`.expect()` and when each is appropriate |
| 014 | Error Handling II — the `?` operator, custom error types, `thiserror` |
| 015 | Generics — generic functions, structs, enums; a plain-language take on monomorphization |
| 016 | Traits I — defining and implementing traits, default methods |
| 017 | Traits II — trait bounds, `where` clauses, intro to `dyn Trait` |
| 018 | Lifetimes — annotations, elision rules, structs holding references |
| 019 | Testing in Rust — `#[test]`, `assert!`/`assert_eq!`, unit vs integration tests, test organization |
| 020 | *(Capstone 02)* |

**Capstone 02 — "Inventory Management CLI"**: Tracks stock items with categories, quantities, and low-stock alerts; persists to a JSON file. Exercises collections, custom errors, generics/traits, and a real test suite.

### Block C — Intermediate Rust I (Modules 021–030) → Capstone 03

| # | Title |
|---|-------|
| 021 | Closures — `Fn`/`FnMut`/`FnOnce`, capturing environment, closures as params/return values |
| 022 | Iterators I — the `Iterator` trait, `next()`, what `for` desugars to |
| 023 | Iterators II — `map`/`filter`/`fold`/`collect` and friends, writing a custom iterator |
| 024 | Advanced Pattern Matching — match guards, `@` bindings, nested destructuring |
| 025 | Advanced Traits — associated types, operator overloading, supertraits, the newtype pattern |
| 026 | Trait Objects & Dynamic Dispatch — `dyn Trait` deep dive, object safety, static vs dynamic dispatch tradeoffs |
| 027 | Common Design Patterns in Rust — builder, typestate, RAII, visitor |
| 028 | Smart Pointers I — `Box<T>`, `Deref`, `Drop` |
| 029 | Smart Pointers II — `Rc<T>`, `RefCell<T>`, interior mutability, `Rc<RefCell<T>>` |
| 030 | *(Capstone 03)* |

**Capstone 03 — "In-Memory Graph Library"**: A small graph data structure crate (nodes/edges, traversal, cycle detection) exposed as a reusable library with a documented public API, exercising trait objects, iterators, and smart pointers.

### Block D — Intermediate Rust II: Concurrency, Unsafe & Macros (Modules 031–040) → Capstone 04

| # | Title |
|---|-------|
| 031 | Concurrency I — `std::thread::spawn`, `join`, `move` closures |
| 032 | Concurrency II — `Mutex<T>`, `Arc<T>`, shared state across threads |
| 033 | Concurrency III — channels (`mpsc`), message-passing patterns |
| 034 | Concurrency IV — `Send`/`Sync` explained, data-race prevention, atomics |
| 035 | Unsafe Rust I — why unsafe exists, raw pointers, `unsafe fn`, dereferencing |
| 036 | Unsafe Rust II — FFI preview, mutable statics, unions, upholding safety invariants |
| 037 | Macros I — `macro_rules!`, reducing duplication with declarative macros |
| 038 | Macros II — procedural macro overview, how `#[derive(...)]` works conceptually |
| 039 | Cargo Deep Dive — workspaces, features, build profiles, publishing to crates.io, semver |
| 040 | *(Capstone 04)* |

**Capstone 04 — "Multi-threaded Log Processor"**: Reads multiple log files concurrently, parses/aggregates stats (error counts, latency percentiles) using threads + channels, with a small `macro_rules!`-based helper for repetitive parsing code.

### Block E — Async Rust (Modules 041–050) → Capstone 05

| # | Title |
|---|-------|
| 041 | Async Fundamentals — `async`/`.await`, the `Future` trait, why async exists alongside threads |
| 042 | The Tokio Runtime — `#[tokio::main]`, spawning tasks, runtime flavors (current-thread vs multi-thread) |
| 043 | Async I/O — `tokio::fs`, `tokio::net`, reading files/sockets asynchronously |
| 044 | Async Synchronization — `tokio::sync::{Mutex, RwLock}`, `mpsc`/`oneshot`/`broadcast` channels |
| 045 | Streams — the `Stream` trait, async iteration, stream combinators |
| 046 | Pinning & Async Internals — `Pin<T>`, `Unpin`, what the compiler generates for an `async fn` |
| 047 | Structured Concurrency & Cancellation — `select!`, timeouts, cancellation tokens, `JoinSet` |
| 048 | Error Handling in Async Code — `anyhow` in async contexts, retry/backoff patterns |
| 049 | Async Patterns & Pitfalls — blocking-in-async, `spawn_blocking`, common async deadlocks |
| 050 | *(Capstone 05)* |

**Capstone 05 — "Concurrent Rate-Limited Web Crawler"**: An async crawler that fetches a set of URLs concurrently with a configurable concurrency limit and per-domain rate limiting, aggregating results, using Tokio tasks, channels, and `select!`-based cancellation.

### Block F — Systems Programming & Performance (Modules 051–060) → Capstone 06

| # | Title |
|---|-------|
| 051 | Memory Layout Deep Dive — struct layout, alignment, padding, `size_of`/`align_of` |
| 052 | FFI I — `extern "C"`, calling C from Rust, `bindgen` basics |
| 053 | FFI II — calling Rust from C, `cbindgen`, building a C-ABI-compatible library |
| 054 | Performance Profiling — `cargo flamegraph`, reading a flamegraph, finding real bottlenecks |
| 055 | Benchmarking — `criterion`, writing meaningful benchmarks, avoiding benchmark traps |
| 056 | Zero-Cost Abstractions & Optimization — inlining, const generics, `Cow<T>`, avoiding needless allocation |
| 057 | SIMD & Low-Level Optimization — `std::simd`/portable SIMD concepts, when vectorization helps |
| 058 | Introduction to Embedded Rust — `#![no_std]`, `embedded-hal`, target architectures overview |
| 059 | Embedded Rust Hands-On — a simulated "blink an LED" / read-a-sensor project (QEMU or a simulator such as Wokwi/renode), real-time constraints |
| 060 | *(Capstone 06)* |

**Capstone 06 — "Benchmarked Data Processing Library"**: A CSV/log parsing-and-aggregation library with a `criterion` benchmark suite, optimized across at least two documented iterations (e.g. reducing allocations, adding SIMD-friendly summation), with before/after numbers recorded in the README.

### Block G — Backend Web Development (Modules 061–070) → Capstone 07

| # | Title |
|---|-------|
| 061 | HTTP & Web Fundamentals in Rust — `hyper` basics, how a web framework is built on top of it |
| 062 | Axum Fundamentals — routing, handlers, extractors, shared state |
| 063 | Building REST APIs with Axum — CRUD endpoints, JSON via `serde`, request validation |
| 064 | Database Integration — `sqlx`, connecting to Postgres, migrations, queries |
| 065 | Authentication & Authorization — JWTs, sessions, middleware, password hashing with `argon2` |
| 066 | Middleware & the Tower Ecosystem — `tower` layers, request tracing/logging, rate limiting, CORS |
| 067 | Actix-web — the actor-model-flavored alternative, comparing it to Axum |
| 068 | Testing Web Services — integration tests for APIs, mocking, ephemeral test databases |
| 069 | Deployment & Observability — Dockerizing a Rust service, structured logging with `tracing`, health checks, config management |
| 070 | *(Capstone 07)* |

**Capstone 07 — "Task Management API"**: A full CRUD backend service (Axum + `sqlx` + Postgres) with JWT auth, request validation, integration tests against a test database, structured logging, and a Dockerfile.

### Block H — CLI, Networking & Distributed Systems (Modules 071–080) → Capstone 08

| # | Title |
|---|-------|
| 071 | Building CLI Tools I — `clap` deep dive, subcommands, argument parsing |
| 072 | Building CLI Tools II — config files, error UX, colored output, progress bars (`indicatif`) |
| 073 | Terminal UIs — `ratatui` basics, building an interactive TUI |
| 074 | Raw Networking — TCP/UDP sockets (std and Tokio), designing a tiny custom protocol |
| 075 | Serialization Deep Dive — advanced `serde`, `bincode`, a Protocol Buffers primer |
| 076 | gRPC with Tonic — defining `.proto` services, implementing client and server |
| 077 | Distributed Systems Concepts I — CAP theorem, consensus basics, leader election, in plain terms |
| 078 | Distributed Systems Concepts II — building a minimal distributed key-value store (single-leader replication) |
| 079 | Message Queues & Event-Driven Systems — pub/sub from Rust (e.g. against Redis), event-driven design |
| 080 | *(Capstone 08)* |

**Capstone 08 — "Distributed Key-Value Store"**: A gRPC-based key-value store with a leader node and one or more replica nodes, a CLI client (`clap`), and a basic replication protocol — pulling together Blocks H's networking, serialization, and distributed-systems modules.

### Block I — WASM, Frontend, Game Dev, Embedded & Blockchain (Modules 081–090) → Capstone 09

| # | Title |
|---|-------|
| 081 | Introduction to WebAssembly — what WASM is, why Rust targets it well, `wasm-pack` setup |
| 082 | `wasm-bindgen` & JS Interop — calling JS from Rust and vice versa, basic DOM manipulation |
| 083 | Building a Rust Frontend App — component/state basics with Leptos (or Yew) |
| 084 | WASM Performance Use Cases — running a compute-heavy task (e.g. image filter) in-browser via WASM, benchmarking vs. JS |
| 085 | Introduction to Game Development in Rust — Bevy setup, the ECS (Entity-Component-System) pattern explained |
| 086 | Bevy Deep Dive — systems, components, resources; building a simple 2D game |
| 087 | Embedded Rust Revisited — a fuller embedded project: interrupts, peripherals, timers, building on Module 059 |
| 088 | Blockchain & Smart Contracts in Rust — why Rust dominates blockchain tooling; a Solana program (or Substrate pallet) walkthrough |
| 089 | Comparing Rust Career Specializations — backend vs. embedded vs. blockchain vs. game dev vs. WASM: job market shape, typical stacks, and portfolio strategy per track *(worksheet-style exercise — see §3 exception)* |
| 090 | *(Capstone 09)* |

**Capstone 09 — "2D Game Compiled to WebAssembly"**: A small playable 2D game built with Bevy, compiled to WASM via `wasm-pack`, and run in a plain HTML page — directly combining the game-dev and WASM modules from this block into one deliverable, kept as a single fixed project (no branching) per the repo's linear-format rule.

### Block J — Interview Prep, DSA & Career Readiness (Modules 091–100)

| # | Title |
|---|-------|
| 091 | Data Structures in Rust I — linked lists, stacks/queues; why linked lists are famously awkward in Rust (`Box`/`Rc`/index-based alternatives) |
| 092 | Data Structures in Rust II — trees, graphs, hash maps and heaps built from scratch |
| 093 | Algorithms in Rust — sorting, searching, complexity analysis, idiomatic-Rust takes on common interview patterns |
| 094 | Rust-Specific Interview Questions — ownership/borrowing trick questions, classic gotchas, whiteboard-style prompts (with worked answers) |
| 095 | System Design with Rust — how to talk through architecture in an interview; worked case studies (rate limiter, URL shortener) in Rust-flavored terms |
| 096 | Open Source Contribution — finding good-first-issues, reading unfamiliar codebases, the PR workflow *(worksheet-style exercise)* |
| 097 | Building Your Portfolio — structuring a GitHub profile, writing project READMEs/case studies, resume framing for Rust roles *(worksheet-style exercise)* |
| 098 | Mock Interview & Code Review Practice — a structured self/peer review checklist, worked example of reviewing a flawed Rust PR |
| 099 | Advanced Topics & Staying Current — const generics deep dive, GATs, the state of async traits, reading Rust release notes/RFCs, community resources |
| 100 | *(Capstone 10 — Final)* |

**Capstone 10 — "Full-Stack Rust Job-Ready Project" (Final Capstone)**: A polished, portfolio-grade project combining at least two specialization blocks — e.g. an Axum backend + Leptos/WASM frontend for a small real app (such as a link-shortener-with-analytics or a personal task tracker), with tests, CI, a Dockerfile, and a written README case study following the Module 097 portfolio guidance. This is explicitly meant to be the piece a learner points to in job applications. Its README should also include a short "Job Readiness Checklist" (portfolio, resume, mock interview practice, specialization chosen, open-source contribution made) referencing Modules 091–099.

---

## 8. Content Depth & Style Guidelines

- **Word count per Concept section**: 800–1500 words. Conceptual/career modules (089, 096, 097, 099) may run longer since they're prose-heavy and code-light.
- **Every code example must compile.** No pseudo-Rust. Where a snippet demonstrates a compile error on purpose (e.g. teaching the borrow checker), label it clearly as such (` ```rust,ignore ` or an explicit "this will not compile" callout) and show the fix immediately after.
- **Consistent voice**: direct, plain language, second person ("you"), no filler ("Let's dive in!"). Assume an intelligent adult who already knows how to program.
- **No hand-waved concurrency/unsafe/async explanations.** These are exactly the topics that make Rust developers valuable — they deserve the most rigor, not the least.
- **Terminology introduced once, used consistently** thereafter (track this across modules; e.g. don't call it "borrow checking" in Module 005 and "reference validation" in Module 018).
- All modules run through `cargo fmt` and `cargo clippy` clean, including exercise scaffolds (the scaffold's *unfinished* parts should still be clippy-clean — `todo!()` is fine, half-written broken syntax is not, except where §3's compile-error exception applies).

---

## 9. Root `README.md` Requirements

The root README is the front door and must contain, in order:

1. **Pitch** — one paragraph: what Rust.Stack is, who it's for.
2. **Prerequisites** — link to `docs/SETUP.md` for installing `rustup`, a stable toolchain, and an editor (VS Code + rust-analyzer recommended, not required).
3. **How to Use This Repo** — link to `docs/HOW_TO_USE.md`; briefly explain the module/exercise/solution/capstone pattern and how to run tests (`cargo test -p module-XXX-exercises`, or the `scripts/verify_module.sh XXX` shortcut).
4. **Curriculum Map** — a full table of all 101 modules + 10 capstones with checkboxes, e.g.:

   ```markdown
   - [ ] [Module 000 — Orientation](modules/module-000-orientation/README.md)
   - [ ] [Module 001 — Toolchain & Your First Program](modules/module-001-toolchain-and-first-program/README.md)
   ...
   - [ ] [Capstone 01 — Contact Book CLI](capstones/capstone-01-contact-book-cli/README.md)
   ...
   ```

   Grouped under block headers (Block A — Foundations I, Block B — Foundations II, …) matching §7 exactly.
5. **Progress tracking** — a one-line pointer to `scripts/check_progress.sh`.
6. **License & Contributing** — links to `LICENSE` and `CONTRIBUTING.md`.
7. **CI badge** pointing at the GitHub Actions workflow status.

---

## 10. License & Contribution

- **License**: MIT, root `LICENSE` file, standard boilerplate.
- **`CONTRIBUTING.md`**: how to propose a new/improved module, the required folder shape (link back to §3/§4), the requirement that any PR pass `cargo fmt --check`, `cargo clippy -- -D warnings`, and `cargo test` for anything touched.

---

## 11. Execution Plan for the Coding Agent

Given the scope here (101 modules + 10 capstones, each with a README, an exercise crate, and a solution crate — realistically 700+ files), this is not a single-pass, single-file generation task. Work through it in this order, committing after each numbered step so progress is checkpointed:

1. **Scaffold the repo skeleton**: all files in §2 except `modules/` and `capstones/` contents — root configs, CI, scripts, docs, empty `modules/`/`capstones/` directories.
2. **Generate `scripts/new_module.sh`** and use it (or its equivalent logic) to stamp out the folder structure for all 101 module folders and 10 capstone folders up front, each with a placeholder README (title + "under construction") and a minimal-but-compiling `exercises`/`solutions` crate pair (empty `lib.rs`, passing trivial test) — this makes the workspace `Cargo.toml` valid immediately and lets `cargo test --workspace` succeed at every intermediate step of the build, rather than only at the very end.
3. **Write the root `README.md`** with the full curriculum map from §7 (this can be done as soon as step 2's folder names are finalized).
4. **Generate modules in strict numeric order, one block at a time** (Module 000, then Block A's 001–010 + Capstone 01, then Block B's 011–020 + Capstone 02, and so on through Block J + Capstone 10). For each module: write the README per the §3 template, write the exercise crate with real TODOs and real failing-until-fixed tests, write the matching solution crate, then run `cargo fmt`, `cargo clippy`, and `cargo test` scoped to that module (`scripts/verify_module.sh XXX`) before moving to the next module. Do not proceed to the next block until the current block's capstone also passes verification.
5. **After all 101 modules + 10 capstones are generated**, run the full workspace check (`cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, `cargo test --workspace`) and fix anything that fails at scale (e.g. a naming collision between two crates).
6. **Final pass**: verify every internal README link resolves, every checkbox in the root README's curriculum map corresponds to a real folder, and the CI workflow runs green on a fresh clone.

### Definition of Done (per module)

A module is complete when:
- [ ] README has all required sections from §3, Concept section is 800–1500 words, at least one code example, at least one pitfall listed.
- [ ] `exercises/` compiles; its tests fail against the unfinished scaffold (unless it's a compile-error-teaching module) and pass once correctly filled in.
- [ ] `solutions/` compiles and all tests pass.
- [ ] `cargo fmt --check` and `cargo clippy -- -D warnings` are clean for both crates.
- [ ] Root README's checklist entry links correctly to the module.

### Definition of Done (whole project)

- [ ] All 101 modules + 10 capstones meet the per-module DoD above.
- [ ] `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`, and `cargo test --workspace` all pass from a clean clone with only the pinned toolchain installed (modulo the documented WASM/embedded exceptions from §5).
- [ ] CI is green on the default branch.
- [ ] Root README's curriculum map is complete and every link resolves.

---

## 12. Explicit Non-Goals

- No hosted website, no mdBook/Docusaurus build step — this is a plain git repo by design.
- No hidden/branch-gated solutions — solutions are always visible in-repo.
- No branching "choose your track" content — the curriculum is strictly linear; specialization depth comes from dedicated blocks (§7), not learner choice.
- No gamification/progress-tracking service — `scripts/check_progress.sh` is a local, offline convenience only.