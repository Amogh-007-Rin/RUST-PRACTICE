# Rust.Stack

![CI](https://github.com/OWNER/Rust.Stack/actions/workflows/ci.yml/badge.svg)

**A self-contained, offline-friendly, 0-to-100 modular Rust learning sandbox.**

Rust.Stack takes you from zero Rust knowledge to job-ready across every major
Rust specialization — backend, async/infra, systems/embedded, CLI/networking,
WASM/frontend, game dev, blockchain — and finishes with interview and
portfolio readiness. It's for programmers who are comfortable with general
programming concepts in *some* language (Python/JS/Java/C++/etc.) but have
zero Rust experience. No website, no build step: `git clone` and go.

---

## Prerequisites

- Rust via **rustup** (stable toolchain — pinned by `rust-toolchain.toml`).
- An editor with rust-analyzer (recommended, not required).
- Git.

Everything installs from the internet once; after that, all content, examples,
and exercises are fully offline. **See [docs/SETUP.md](docs/SETUP.md).**

## How to Use This Repo

The curriculum is strictly linear: **Module 000 → Module 100**, with a
**capstone project** at the end of every 10-module block. There are no
branching tracks — each specialization gets a full block of modules in
sequence.

Every module is hands-on: a `README.md` teaches the concept from scratch, an
`exercises/` crate contains the broken/incomplete code you fix until
`cargo test -p module-XXX-exercises` passes, and a `solutions/` crate holds
the reference implementation (always visible — attempt first, peek when
stuck). Capstones work the same way, just bigger. Three short scripts help:

```bash
./scripts/verify_module.sh 042   # fmt + clippy + test for one module
./scripts/check_progress.sh      # reports completion from the checklist below
```

**Full walkthrough, module anatomy, and FAQ: [docs/HOW_TO_USE.md](docs/HOW_TO_USE.md).**

---

## Curriculum Map

Tick the boxes below as you complete modules and capstones, then run
`./scripts/check_progress.sh` to see your progress.

### Module 000 — Orientation

- [ ] [Module 000 — Orientation](modules/module-000-orientation/README.md)

### Block A — Foundations I (Modules 001–010) → Capstone 01

- [ ] [Module 001 — Toolchain & Your First Program](modules/module-001-toolchain-and-first-program/README.md)
- [ ] [Module 002 — Variables, Mutability & Data Types](modules/module-002-variables-mutability-and-data-types/README.md)
- [ ] [Module 003 — Functions & Control Flow](modules/module-003-functions-and-control-flow/README.md)
- [ ] [Module 004 — Ownership Part 1](modules/module-004-ownership-part-1/README.md)
- [ ] [Module 005 — Ownership Part 2: Borrowing & References](modules/module-005-ownership-part-2-borrowing-and-references/README.md)
- [ ] [Module 006 — Slices & Strings](modules/module-006-slices-and-strings/README.md)
- [ ] [Module 007 — Structs](modules/module-007-structs/README.md)
- [ ] [Module 008 — Enums & Pattern Matching](modules/module-008-enums-and-pattern-matching/README.md)
- [ ] [Module 009 — Modules, Crates & Packages](modules/module-009-modules-crates-and-packages/README.md)
- [ ] [Capstone 01 — Contact Book CLI](capstones/capstone-01-contact-book-cli/README.md)

### Block B — Foundations II (Modules 011–020) → Capstone 02

- [ ] [Module 011 — Common Collections I: `Vec<T>`](modules/module-011-common-collections-1-vec/README.md)
- [ ] [Module 012 — Common Collections II: `HashMap` & `HashSet`](modules/module-012-common-collections-2-hashmap-hashset/README.md)
- [ ] [Module 013 — Error Handling I: `panic!` & `Result`](modules/module-013-error-handling-1-panic-and-result/README.md)
- [ ] [Module 014 — Error Handling II: `?`, Custom Errors & `thiserror`](modules/module-014-error-handling-2-the-question-operator-and-thiserror/README.md)
- [ ] [Module 015 — Generics](modules/module-015-generics/README.md)
- [ ] [Module 016 — Traits I: Defining & Implementing](modules/module-016-traits-1-defining-and-implementing/README.md)
- [ ] [Module 017 — Traits II: Bounds & Trait Objects](modules/module-017-traits-2-bounds-and-trait-objects/README.md)
- [ ] [Module 018 — Lifetimes](modules/module-018-lifetimes/README.md)
- [ ] [Module 019 — Testing in Rust](modules/module-019-testing-in-rust/README.md)
- [ ] [Capstone 02 — Inventory Management CLI](capstones/capstone-02-inventory-management-cli/README.md)

### Block C — Intermediate Rust I (Modules 021–030) → Capstone 03

- [ ] [Module 021 — Closures](modules/module-021-closures/README.md)
- [ ] [Module 022 — Iterators I: The `Iterator` Trait](modules/module-022-iterators-1-the-iterator-trait/README.md)
- [ ] [Module 023 — Iterators II: Combinators](modules/module-023-iterators-2-combinators/README.md)
- [ ] [Module 024 — Advanced Pattern Matching](modules/module-024-advanced-pattern-matching/README.md)
- [ ] [Module 025 — Advanced Traits](modules/module-025-advanced-traits/README.md)
- [ ] [Module 026 — Trait Objects & Dynamic Dispatch](modules/module-026-trait-objects-and-dynamic-dispatch/README.md)
- [ ] [Module 027 — Design Patterns in Rust](modules/module-027-design-patterns-in-rust/README.md)
- [ ] [Module 028 — Smart Pointers I: `Box`, `Deref`, `Drop`](modules/module-028-smart-pointers-1-box-deref-drop/README.md)
- [ ] [Module 029 — Smart Pointers II: `Rc` & `RefCell`](modules/module-029-smart-pointers-2-rc-refcell/README.md)
- [ ] [Capstone 03 — In-Memory Graph Library](capstones/capstone-03-in-memory-graph-library/README.md)

### Block D — Intermediate Rust II: Concurrency, Unsafe & Macros (Modules 031–040) → Capstone 04

- [ ] [Module 031 — Concurrency I: Threads](modules/module-031-concurrency-1-threads/README.md)
- [ ] [Module 032 — Concurrency II: `Mutex` & `Arc`](modules/module-032-concurrency-2-mutex-arc/README.md)
- [ ] [Module 033 — Concurrency III: Channels](modules/module-033-concurrency-3-channels/README.md)
- [ ] [Module 034 — Concurrency IV: `Send`/`Sync` & Atomics](modules/module-034-concurrency-4-send-sync-atomics/README.md)
- [ ] [Module 035 — Unsafe Rust I](modules/module-035-unsafe-rust-1/README.md)
- [ ] [Module 036 — Unsafe Rust II](modules/module-036-unsafe-rust-2/README.md)
- [ ] [Module 037 — Macros I: Declarative](modules/module-037-macros-1-declarative/README.md)
- [ ] [Module 038 — Macros II: Procedural](modules/module-038-macros-2-procedural/README.md)
- [ ] [Module 039 — Cargo Deep Dive](modules/module-039-cargo-deep-dive/README.md)
- [ ] [Capstone 04 — Multithreaded Log Processor](capstones/capstone-04-multithreaded-log-processor/README.md)

### Block E — Async Rust (Modules 041–050) → Capstone 05

- [ ] [Module 041 — Async Fundamentals](modules/module-041-async-fundamentals/README.md)
- [ ] [Module 042 — The Tokio Runtime](modules/module-042-the-tokio-runtime/README.md)
- [ ] [Module 043 — Async I/O](modules/module-043-async-io/README.md)
- [ ] [Module 044 — Async Synchronization](modules/module-044-async-synchronization/README.md)
- [ ] [Module 045 — Streams](modules/module-045-streams/README.md)
- [ ] [Module 046 — Pinning & Async Internals](modules/module-046-pinning-and-async-internals/README.md)
- [ ] [Module 047 — Structured Concurrency & Cancellation](modules/module-047-structured-concurrency-and-cancellation/README.md)
- [ ] [Module 048 — Error Handling in Async Code](modules/module-048-error-handling-in-async-code/README.md)
- [ ] [Module 049 — Async Patterns & Pitfalls](modules/module-049-async-patterns-and-pitfalls/README.md)
- [ ] [Capstone 05 — Concurrent Rate-Limited Web Crawler](capstones/capstone-05-concurrent-web-crawler/README.md)

### Block F — Systems Programming & Performance (Modules 051–060) → Capstone 06

- [ ] [Module 051 — Memory Layout Deep Dive](modules/module-051-memory-layout-deep-dive/README.md)
- [ ] [Module 052 — FFI I: Calling C from Rust](modules/module-052-ffi-1-calling-c-from-rust/README.md)
- [ ] [Module 053 — FFI II: Calling Rust from C](modules/module-053-ffi-2-calling-rust-from-c/README.md)
- [ ] [Module 054 — Performance Profiling](modules/module-054-performance-profiling/README.md)
- [ ] [Module 055 — Benchmarking with Criterion](modules/module-055-benchmarking-with-criterion/README.md)
- [ ] [Module 056 — Zero-Cost Abstractions & Optimization](modules/module-056-zero-cost-abstractions-and-optimization/README.md)
- [ ] [Module 057 — SIMD & Low-Level Optimization](modules/module-057-simd-and-low-level-optimization/README.md)
- [ ] [Module 058 — Introduction to Embedded Rust](modules/module-058-introduction-to-embedded-rust/README.md)
- [ ] [Module 059 — Embedded Rust Hands-On](modules/module-059-embedded-rust-hands-on/README.md)
- [ ] [Capstone 06 — Benchmarked Data Processing Library](capstones/capstone-06-benchmarked-data-processing-library/README.md)

### Block G — Backend Web Development (Modules 061–070) → Capstone 07

- [ ] [Module 061 — HTTP & Web Fundamentals in Rust](modules/module-061-http-and-web-fundamentals/README.md)
- [ ] [Module 062 — Axum Fundamentals](modules/module-062-axum-fundamentals/README.md)
- [ ] [Module 063 — Building REST APIs with Axum](modules/module-063-building-rest-apis-with-axum/README.md)
- [ ] [Module 064 — Database Integration with `sqlx`](modules/module-064-database-integration-with-sqlx/README.md)
- [ ] [Module 065 — Authentication & Authorization](modules/module-065-authentication-and-authorization/README.md)
- [ ] [Module 066 — Middleware & the Tower Ecosystem](modules/module-066-middleware-and-the-tower-ecosystem/README.md)
- [ ] [Module 067 — Actix-web](modules/module-067-actix-web/README.md)
- [ ] [Module 068 — Testing Web Services](modules/module-068-testing-web-services/README.md)
- [ ] [Module 069 — Deployment & Observability](modules/module-069-deployment-and-observability/README.md)
- [ ] [Capstone 07 — Task Management API](capstones/capstone-07-task-management-api/README.md)

### Block H — CLI, Networking & Distributed Systems (Modules 071–080) → Capstone 08

- [ ] [Module 071 — Building CLI Tools I: `clap`](modules/module-071-building-cli-tools-1-clap/README.md)
- [ ] [Module 072 — Building CLI Tools II: Config, Errors & Polish](modules/module-072-building-cli-tools-2-config-errors-and-polish/README.md)
- [ ] [Module 073 — Terminal UIs with `ratatui`](modules/module-073-terminal-uis-with-ratatui/README.md)
- [ ] [Module 074 — Raw Networking](modules/module-074-raw-networking/README.md)
- [ ] [Module 075 — Serialization Deep Dive](modules/module-075-serialization-deep-dive/README.md)
- [ ] [Module 076 — gRPC with Tonic](modules/module-076-grpc-with-tonic/README.md)
- [ ] [Module 077 — Distributed Systems Concepts I](modules/module-077-distributed-systems-1-concepts/README.md)
- [ ] [Module 078 — Distributed Systems II: Key-Value Store](modules/module-078-distributed-systems-2-key-value-store/README.md)
- [ ] [Module 079 — Message Queues & Event-Driven Systems](modules/module-079-message-queues-and-event-driven/README.md)
- [ ] [Capstone 08 — Distributed Key-Value Store](capstones/capstone-08-distributed-key-value-store/README.md)

### Block I — WASM, Frontend, Game Dev, Embedded & Blockchain (Modules 081–090) → Capstone 09

- [ ] [Module 081 — Introduction to WebAssembly](modules/module-081-introduction-to-webassembly/README.md)
- [ ] [Module 082 — `wasm-bindgen` & JS Interop](modules/module-082-wasm-bindgen-and-js-interop/README.md)
- [ ] [Module 083 — Rust Frontend with Leptos](modules/module-083-rust-frontend-with-leptos/README.md)
- [ ] [Module 084 — WASM Performance Use Cases](modules/module-084-wasm-performance-use-cases/README.md)
- [ ] [Module 085 — Game Development with Bevy](modules/module-085-game-development-with-bevy/README.md)
- [ ] [Module 086 — Bevy Deep Dive: 2D Game](modules/module-086-bevy-deep-dive-2d-game/README.md)
- [ ] [Module 087 — Embedded Rust Revisited](modules/module-087-embedded-rust-revisited/README.md)
- [ ] [Module 088 — Blockchain & Smart Contracts in Rust](modules/module-088-blockchain-and-smart-contracts/README.md)
- [ ] [Module 089 — Comparing Rust Career Specializations](modules/module-089-comparing-rust-career-specializations/README.md)
- [ ] [Capstone 09 — 2D Game Compiled to WebAssembly](capstones/capstone-09-2d-game-in-webassembly/README.md)

### Block J — Interview Prep, DSA & Career Readiness (Modules 091–100) → Capstone 10

- [ ] [Module 091 — Data Structures in Rust I](modules/module-091-data-structures-in-rust-1/README.md)
- [ ] [Module 092 — Data Structures in Rust II](modules/module-092-data-structures-in-rust-2/README.md)
- [ ] [Module 093 — Algorithms in Rust](modules/module-093-algorithms-in-rust/README.md)
- [ ] [Module 094 — Rust-Specific Interview Questions](modules/module-094-rust-specific-interview-questions/README.md)
- [ ] [Module 095 — System Design with Rust](modules/module-095-system-design-with-rust/README.md)
- [ ] [Module 096 — Open Source Contribution](modules/module-096-open-source-contribution/README.md)
- [ ] [Module 097 — Building Your Portfolio](modules/module-097-building-your-portfolio/README.md)
- [ ] [Module 098 — Mock Interview & Code Review Practice](modules/module-098-mock-interview-and-code-review-practice/README.md)
- [ ] [Module 099 — Advanced Topics & Staying Current](modules/module-099-advanced-topics-and-staying-current/README.md)
- [ ] [Module 100 — Final Capstone Support](modules/module-100-final-capstone-support/README.md)
- [ ] [Capstone 10 — Full-Stack Rust Job-Ready Project](capstones/capstone-10-full-stack-job-ready-project/README.md)

---

## Progress Tracking

Tick the checkboxes above and run:

```bash
./scripts/check_progress.sh   # → "NN/101 modules complete, MM/10 capstones complete."
```

## License & Contributing

- **License:** [MIT](LICENSE)
- **Contributing:** see [CONTRIBUTING.md](CONTRIBUTING.md) — required folder
  shapes, content guidelines, and the acceptance bar (fmt, clippy, tests
  clean, always).

## CI

Every push and PR to `main` runs `cargo fmt --check`, `cargo clippy --workspace -- -D warnings`,
and `cargo test --workspace` via [GitHub Actions](.github/workflows/ci.yml).
