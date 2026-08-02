# Module 089: Comparing Rust Career Specializations

**Block:** Block I — WASM, Frontend, Game Dev, Embedded & Blockchain
**Estimated time:** 60–90 min (reading and reflection)
**Prerequisites:** Modules 001–088. This module is a synthesis exercise drawing on everything in Block I.

## Learning Objectives

- You will be able to compare and contrast five Rust career specializations: backend web, embedded systems, blockchain, game development, and WASM/frontend.
- You will be able to identify the typical tech stack, hiring companies, and market shape for each specialization.
- You will be able to design a focused 3-month learning plan for your chosen specialization, building on the Rust.Stack modules you have already completed.
- You will be able to identify open-source projects in your area and plan a contribution strategy.

## Why This Matters

Module 0 presented Rust as a language with many career paths. By now — after 88 modules — you have written backend APIs, async crawlers, CLI tools, embedded firmware, blockchain data structures, a reactive runtime, and an ECS game engine. You have enough breadth to make an informed choice about depth. This module is the inflection point: it asks you to commit to a specialization, research the job market honestly, and design a concrete plan. The worksheet exercise replaces code with strategy — the deliverable is a 3-month study plan and a list of projects you will target, not a passing test suite. This mirrors what you will actually do when job hunting: you do not collect languages and frameworks like trading cards; you go deep enough in one area that an employer trusts you to ship.

## Concept

### The five paths through Rust

Rust is unusual among languages in that it spans the full stack from microcontrollers to web servers to browsers. No one person masters all of them — the engineers who write STM32 firmware are not the same people who write Leptos components, even though both use the same compiler. Here is a grounded comparison of the five specializations covered across Blocks E through I of this curriculum.

#### 1. Backend Web Development (Blocks E + G)

**What you build:** REST APIs, gRPC services, event-driven workers, database-backed applications.

**Typical stack:** Axum or Actix-web, `sqlx` or Diesel, Postgres, Redis, `serde`/`tower`/`tracing`, Docker, Kubernetes. Sometimes `tonic` for gRPC or `lapin` for RabbitMQ/AMQP.

**Who hires:** Cloud infrastructure companies (AWS, Cloudflare, Fly.io), fintech (Stripe, Monzo, Kraken), developer-tooling startups (Railway, Turso, Convex), and increasingly general SaaS companies adopting Rust for performance-critical services.

**Market shape:** The largest Rust job market by volume. Almost every company running Rust in production has a backend service written in it. Many roles are "backend engineer" with Rust as one of several languages, not "Rust developer" exclusively. Rust backend roles typically pay at the top of the backend market because the developer pool is smaller and the use cases (high-throughput, low-latency, correctness-critical) are more demanding.

**Key differentiators from other backend stacks:** No GC pauses make Rust backends predictable under load. The type system catches entire classes of bugs (null pointer exceptions, data races, invalid state transitions) that plague Node.js and Go services in production. The tradeoff is slower iteration speed — Rust compiles more slowly than Go and has a steeper learning curve.

#### 2. Embedded Systems (Modules 058-059 + 087)

**What you build:** Firmware for microcontrollers, sensor drivers, real-time control systems, IoT devices.

**Typical stack:** `#![no_std]`, `embedded-hal`, `cortex-m` or `esp-hal`, `defmt` (logging), `probe-rs` (debugging), often no operating system at all. Embassy brings async to embedded for cooperative multitasking on bare metal.

**Who hires:** Semiconductor companies (STM, Espressif, Nordic Semi), automotive (Bosch, Tesla, Volvo for safety-critical ECUs), industrial automation (Siemens, ABB), consumer electronics (Apple, Google hardware teams), defense contractors.

**Market shape:** Smaller than backend but growing rapidly as software-defined vehicles and IoT regulation (EU Cyber Resilience Act) push for memory-safe firmware. Embedded Rust roles are typically pure Rust — the candidate needs embedded systems experience (timers, interrupts, peripherals, protocols) and Rust fluency, not general web development skills. These roles value electrical engineering fundamentals alongside Rust knowledge.

**Key differentiators from C/C++ embedded:** No NULL pointer dereferences, buffer overflows, or data races — all compile-time guarantees. `embedded-hal` traits make drivers portable across MCU families. Async/await on bare metal (Embassy) is a Rust innovation with no equivalent in the traditional embedded world.

#### 3. Blockchain & Smart Contracts (Module 088)

**What you build:** On-chain programs (Solana), Substrate pallets (Polkadot), CosmWasm contracts (Cosmos), Move-language contracts (Aptos, Sui), blockchain infrastructure (indexers, validators, relayers).

**Typical stack:** `solana-program` / `anchor` (Solana), `frame` / `substrate` (Polkadot), `cosmwasm-std` (Cosmos), `sui-sdk` (Sui). Off-chain tooling uses `ethers-rs`, `alloy`, `reth` (Ethereum ecosystem). Heavy use of `serde`, `borsh`, `parity-scale-codec`, and cryptographic crates (`ed25519-dalek`, `sha2`, `blake3`).

**Who hires:** Layer-1 and Layer-2 protocols (Solana Labs, Parity, Mysten Labs, Aptos Labs), DeFi protocols (every major DEX and lending protocol has Rust tooling), blockchain infrastructure companies (QuickNode, Helius, Syndica), crypto exchanges building internal tools. Web3-native companies dominate, but traditional fintech exploring tokenization also hires.

**Market shape:** Volatile and highly cyclical — hiring surges during bull markets and contracts during bear markets. Compensation is typically high (often token-inclusive) but job security is lower than other specializations. The technical bar is high: you need Rust fluency, understanding of consensus algorithms, cryptographic primitives, and often economic mechanism design. This is the specialization with the highest variance — a few engineers make life-changing money; many burn out or leave after a cycle.

**Key differentiators from other Rust paths:** On-chain execution is deterministic — no `std::time`, no random, no filesystem I/O. Resource constraints are severe (Solana: 200k compute units per transaction). The code you write handles real money, so security review (formal verification, fuzzing, invariant testing) is not optional — it is the job.

#### 4. Game Development (Modules 085-086)

**What you build:** 2D and 3D games, game engines, rendering pipelines, game tools (level editors, asset pipelines), real-time simulations.

**Typical stack:** Bevy (the dominant Rust game engine), `wgpu` (cross-platform graphics), `rapier` or `xpbd` (physics), `winit` (windowing), `kira` or `rodio` (audio). For commercial games: custom engines built on `wgpu` or `ash` (Vulkan bindings) with ECS libraries like `hecs` or `legion`.

**Who hires:** Indie game studios adopting Rust (mostly for performance and iteration safety), tools teams at larger studios (Embark Studios, Treyarch, Ubisoft exploring Rust for pipelines), simulation companies (Roblox engine work, simulation training software), and GPU companies (NVIDIA, AMD tooling teams).

**Market shape:** Smallest of the five specializations in terms of pure Rust roles. Most game studios still use C++ (Unreal Engine) or C# (Unity). Rust game roles are concentrated in the indie scene and engine/tools roles at larger studios. The Bevy ecosystem is growing fast but is not yet at 1.0. This path requires the most patience — you may need to build portfolio projects for years before landing a role that is primarily Rust game dev.

**Key differentiators from other Rust paths:** Real-time constraints (16ms frame budget). Heavy use of ECS patterns you learned in Module 086. SIMD and GPU compute (shaders, ray tracing) become relevant quickly. The rendering pipeline is inherently unsafe Rust territory — GPU memory management and FFI with graphics APIs require `unsafe` blocks.

#### 5. WASM / Frontend (Modules 081-084)

**What you build:** Browser-based UIs (Leptos, Dioxus, Yew), compute-heavy browser applications (image/video processing, scientific visualization), WASM plugins for existing JS ecosystems (Figma's C++-to-ASM.js pipeline, but the Rust equivalent), WASI (WebAssembly System Interface) for edge/server-side WASM.

**Typical stack:** `leptos` or `dioxus` (UI framework), `wasm-bindgen` / `web-sys` (JS interop), `wasm-pack` (build tooling), `trunk` (bundler). For compute: `ndarray`, `image`, custom SIMD via `std::simd`. The Rust side targets `wasm32-unknown-unknown`; the JS side imports the generated glue.

**Who hires:** Figma, Cloudflare (Workers), Shopify (Shopify Functions), Amazon (Prime Video uses WASM for client-side rendering), edge-computing platforms (Fastly Compute@Edge, Fermyon Spin), and any company with a compute-heavy web application looking to replace JavaScript with Rust for hot paths.

**Market shape:** Small but growing fast. WASM is not replacing JS for DOM manipulation — it is supplementing it for compute-heavy work. The roles are typically "frontend engineer with WASM experience" or "browser performance engineer," not exclusively Rust. The WASI direction (server-side WASM with Rust) is adjacent and more systems-oriented. This path rewards deep understanding of both browser internals and Rust performance.

**Key differentiators from other Rust paths:** You live on both sides of the JS boundary. Threading is limited (shared memory is opt-in in browsers). The build pipeline is complex (wasm-pack + webpack/vite/esbuild). Debugging requires browser devtools, not `gdb` or `lldb`. But the payoff is delivering Rust's performance to the browser — tasks that took seconds in JS take milliseconds in WASM.

### Comparison matrix

| Factor | Backend | Embedded | Blockchain | Game Dev | WASM |
|--------|---------|----------|------------|----------|------|
| Job volume | High | Medium | Cyclical | Low | Medium-growing |
| Remote-friendly | Very | Moderate | Very | Moderate | Very |
| Requires domain knowledge | Distributed systems | EE fundamentals | Crypto, consensus | Graphics, physics | Browser internals |
| Safety-critical | Sometimes | Often | Always (money) | Rarely | Rarely |
| Open-source opportunities | Many | Many | Many | Growing | Moderate |
| Typical comp range | High | High | Very high (volatile) | Moderate | High |

### Strategy: how to choose

You have now completed roughly 90% of the Rust.Stack curriculum. You have touched all five domains enough to know which ones interest you. The worksheet exercise asks you to commit to one and design a 90-day plan. Here is how to think about the choice:

1. **Follow the energy.** Which modules did you lose track of time on? Which exercise did you stay up late finishing? That is your real answer — the market will still be there in three months; your enthusiasm needs to last a career.

2. **Consider the ecosystem maturity.** Backend Rust is production-grade with thousands of companies relying on it. Embedded Rust is production-grade in specific MCU families (STM32, nRF, ESP32) but still maturing. Blockchain Rust is production-grade but volatile. Game-dev Rust is pre-1.0 in ecosystem terms but the engine (Bevy) is advancing fast. WASM Rust is production-grade for compute; the UI frameworks are maturing.

3. **Look at open-source activity.** Pick a specialization and look at the GitHub repositories behind its key crates (`axum`, `bevy`, `solana-program`, `embassy`, `leptos`). How many contributors? How fast are issues being closed? A healthy, active community means you will find mentors and collaborators.

4. **Build a specialization project, not a generic one.** A Rust backend engineer with five generic CRUD apps is not competitive. A Rust backend engineer who built a rate-limited concurrent web crawler (Capstone 05) *and* a task management API (Capstone 07) *and* a personal project in their chosen domain — that candidate stands out. The capstones in this curriculum are designed to be the foundation; now add one personal project that screams "I ship."

### Portfolio strategy per specialization

**Backend:** Write a service that solves a real problem with performance requirements. A link shortener that handles 100k RPS. An image processing pipeline with Axum + `image` crate. A websocket chat server with rooms and persistence. Then Dockerize it, add `tracing` spans, and write a case-study README.

**Embedded:** Build a physical (or simulated) project with at least two peripherals communicating via interrupts. An I2C sensor reader that logs to a UART console. A PWM motor controller with timer interrupts. Then post the schematic, the firmware, and a video of it running.

**Blockchain:** Build and deploy a Solana program that does something nontrivial (a simple escrow, a voting mechanism, a token vesting schedule) with Anchor tests. Write a client that interacts with it from a Rust binary. Document the deployed program address and transaction IDs.

**Game Dev:** Build a complete playable game in Bevy. Even a simple Snake clone or a platformer with physics is enough — the key is that it compiles to WASM and runs in a browser (tying into Capstone 09). Add a README with screenshots and a live link.

**WASM:** Build a compute-heavy browser application — an image filter that runs on a `<canvas>`, a Mandelbrot set explorer, a real-time audio visualizer. The demo should have a side-by-side performance comparison (JS vs. WASM) and a live deployment.

## Common Pitfalls

- **Trying to master all five paths.** The Rust job market does not reward breadth — it rewards depth in one domain. Choose one.
- **Choosing blockchain for the money without interest in the technology.** The cycle will turn, and if you do not care about consensus algorithms, you will hate the work and leave before the next bull market.
- **Over-indexing on current job postings.** The Rust market changes fast. Look at trends over 12 months, not the last month's listings. Companies shipping Rust today (Cloudflare, AWS, Figma, Embark) are better indicators than the 3 random startups that posted yesterday.
- **Building portfolio projects that look like tutorials.** A TODO list app built with Axum in 30 minutes from a blog post does not demonstrate anything. Build something original.
- **Skipping open-source contribution.** Every Rust specialization hires disproportionately from people who contribute to the ecosystem — maintainers notice you, your PRs are public, and your code is reviewed by domain experts before you ever interview.

## Key Terms

- **Specialization:** committing technical depth to one domain (backend, embedded, blockchain, game dev, WASM) rather than maintaining broad shallow knowledge across all five.
- **Portfolio project:** a self-contained, shipped (deployed or published) piece of work that demonstrates your chosen specialization, ideally with documented performance metrics or a live demo.
- **Open-source contribution:** a pull request accepted into a public Rust project related to your specialization, demonstrating ability to read unfamiliar codebases and work with maintainers.

## Exercise

This module uses a worksheet format instead of a Cargo crate. Open `exercises/WORKSHEET.md` and complete the five prompts:

1. List your top 3 Rust specialization interests and explain why each appeals to you.
2. For each specialization, research 3 companies actively hiring or using Rust in that domain and note what they build.
3. Compare the typical tech stacks across the five specializations — what is similar, what is unique.
4. Design a 3-month learning plan for your chosen specialization, referencing specific Rust.Stack modules you will revisit plus new resources you will study.
5. Identify 2 open-source projects in your specialization area that you could contribute to and describe a potential first contribution.

There are no automated tests for this module — the work is the research and the plan. Sample answers are provided in `solutions/EXAMPLE_ANSWERS.md` to give you a sense of what a strong response looks like.

## Further Reading

- [Rust Foundation — "State of Rust 2025"](https://rust-foundation.org) — the annual survey includes job market trends and salary data.
- [Are We Game Yet?](https://arewegameyet.rs) — status of the Rust game-dev ecosystem, updated regularly by the Bevy community.
- [Embedded Rust — "Awesome Embedded Rust"](https://github.com/rust-embedded/awesome-embedded-rust) — a curated list of embedded Rust projects, tools, and companies.
- [Solana Developer Resources](https://solana.com/developers) — guides for writing, testing, and deploying on-chain Rust programs.
- [Module 097 — Building Your Portfolio](../module-097-building-your-portfolio/README.md) — a later module that picks up where this one leaves off, with resume and GitHub profile guidance.
