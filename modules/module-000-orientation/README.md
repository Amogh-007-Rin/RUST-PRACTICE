# Module 000: Welcome to Rust.Stack

**Block:** Orientation (introductory module — no block letter)
**Estimated time:** 60–90 min
**Prerequisites:** none

## Learning Objectives

- You will be able to explain what Rust is, where it came from, and why it exists as a reaction to memory-safety failures in C and C++.
- You will be able to define "memory safety" concretely (use-after-free, data races, buffer overflows) and explain why it is an industry-wide hiring driver.
- You will be able to describe the rustc pipeline — borrow checker, MIR, LLVM backend — well enough to understand *why* the compiler behaves the way it does in later modules.
- You will be able to name the main things people build with Rust today and commit to at least one specialization you want to pursue.
- You will be able to navigate this repository: module anatomy, exercises vs. solutions, capstone cadence, and the testing workflow.

## Why This Matters

Every later module in this repository — and every Rust interview you will ever sit — builds on the ideas in this one: memory safety, ownership, and what the compiler is actually doing when it rejects your code. Hiring managers screen for exactly the fluency this module introduces: can you reason about memory and safety at the systems level, and can you talk about it precisely? If you walk away from this module able to explain *why Rust exists*, you have already started the conversation that gets you hired.

## Concept

### What Rust is, and where it came from

Rust is a compiled systems programming language. It began around 2006 as a personal project of Graydon Hoare, a Mozilla employee, who was frustrated by the compromises of existing languages: C and C++ give you total control over memory but constantly shoot you in the foot; garbage-collected languages protect you but take control away. Mozilla began sponsoring the project in 2009, and it was announced publicly in 2010. The first stable release — Rust 1.0 — shipped on May 15, 2015, which mattered enormously: the 1.0 release was a promise of backward compatibility, the point at which companies could start building on the language without fearing constant breakage. Since then Rust has shipped a new stable release every six weeks.

In 2020, Mozilla laid off much of its Rust team as part of a broader restructuring. Rather than dying, the project consolidated into the **Rust Foundation**, an independent non-profit organization founded in 2021 and backed by a consortium of large technology companies (including AWS, Google, Microsoft, and Huawei). Today Rust is developed in the open through an RFC process, governed by the Foundation and elected project teams, and its governance — not any single vendor — is the reason companies trust it as an infrastructure language.

### Why Rust exists: the memory-safety problem

Rust is, at its core, a reaction to a specific class of failure. For forty years, a huge share of the world's critical software — operating systems, browsers, databases, network stacks, cryptographic libraries — has been written in C and C++. Both languages give programmers direct, unchecked access to memory: you allocate a buffer and read and write raw bytes through it, and the language assumes you get it right. When you don't, you get memory-safety bugs:

- **Use-after-free.** You free a block of memory but keep a pointer to it, then read or write through that dangling pointer. The memory may now hold unrelated data — or be in the hands of an attacker.
- **Buffer overflow.** You write past the end of an allocated buffer. Adjacent memory gets corrupted — which in the worst case lets an attacker overwrite control data and execute arbitrary code.
- **Data race.** Two threads access the same memory concurrently, at least one of them writes, and there is no synchronization. The result is nondeterministic and formally undefined.

These are not abstract concerns. The single most famous security incident of the last decade, Heartbleed (2014), was an out-of-bounds read in OpenSSL, a C library — a buffer overrun that let attackers read private memory from servers. Class after class of remotely exploitable vulnerabilities (CVE after CVE) trace back to these same three bug families. Over the last several years this has turned from an engineering annoyance into a policy issue: cybersecurity agencies and standards bodies in the US and EU have published explicit guidance directing organizations to move memory-unsafe code toward memory-safe languages, and major technology companies have internally mandated that new infrastructure code be written in memory-safe languages. The exact employer figures change quarterly, so treat them as noise — the trend is the signal: *memory-safe is no longer a nice-to-have; it is becoming the baseline requirement.*

Rust's answer is that the three bug families above — use-after-free, buffer overflow, data race — are **impossible in safe Rust**. Not unlikely. Not rare. *Impossible*, because the compiler refuses to produce a program that could do them. That is the language's founding premise.

### The pitch: systems programming without a garbage collector

"Systems programming" means working close to the metal: no heavyweight runtime, no automatic memory management, precise control over memory layout, small binaries, and predictable performance. Historically this domain came with a stark tradeoff:

- Use a **garbage-collected** language (Java, Go, Python) and you get memory safety, but you pay for a runtime, GC pauses, larger binaries, and less control over when memory is freed.
- Use **C or C++** and you get control, but you are personally responsible for every allocation and free — and the CVEs above are what that responsibility costs.

Rust's pitch is that this tradeoff is false. It gives you the control of C++ *and* the safety of a GC'd language, by moving the safety check from runtime to **compile time**: the **borrow checker** — a component of the compiler — proves your program is memory-safe before it ever runs, so no garbage collector is needed at runtime. You get deterministic, explicit memory management (memory is freed exactly when ownership ends, with no pauses) and a runtime that is effectively zero. The price is paid once, while compiling, in the form of compiler errors that teach you to think about memory explicitly. That price is exactly what this curriculum is designed to help you pay — painfully at first, then fluently.

To see the compiler's guarantee in action, consider what a use-after-free attempt looks like in Rust. This program does **not** compile:

```rust,ignore
// This does NOT compile — which is the point. Do not copy it.
fn main() {
    let message = String::from("hello");
    let first_byte = &message[..1]; // borrow of `message`
    drop(message);                  // the memory is freed here
    println!("{first_byte}");       // use-after-free attempt
}
```

In C this would compile and silently read freed memory. In Rust the compiler rejects the program before it runs — it can see that you are using a borrow of `message` after `message`'s memory was released. The fix is to use the borrow while the data is still alive:

```rust
fn main() {
    let message = String::from("hello");
    let first_byte = &message[..1]; // borrow of `message`
    println!("{first_byte}");       // used while the data is still alive
    drop(message);                  // freed only after the last use
}
```

You don't need to understand every rule behind this yet — Module 004 and Module 005 teach ownership and borrowing in depth. What you should absorb now is the *shape* of the guarantee: the compiler catches the bug class before deployment, at zero runtime cost.

### What "memory safety" means concretely

To be precise: **memory safety** means a program can only access memory it is currently entitled to access. Every read and write lands within a live, properly sized allocation; every reference points at valid, initialized data; memory is freed exactly once; and concurrent access to shared memory is synchronized. A memory-safe language makes violations of these properties impossible *except* where you explicitly opt out with an `unsafe` block (a narrow, well-understood escape hatch you'll meet in Block D).

### What you can build with Rust today

Rust is no longer a research language — it is production infrastructure. The things people build with it today, and the specializations this curriculum trains you for:

- **CLI tools.** Fast-starting, dependency-light command-line programs: `ripgrep`, `bat`, `fd`, `starship`, and `cargo` itself are all Rust.
- **Backend services.** HTTP APIs and gRPC services (Block G) — the `axum`/`actix-web`/`tonic` stack — plus databases and data pipelines at scale.
- **Embedded firmware.** Code running on microcontrollers with no operating system and no runtime (`no_std`, Block F): sensors, drones, industrial controllers.
- **WASM and frontend.** Rust compiled to WebAssembly running in the browser — both compute-heavy libraries and full frontend frameworks (Leptos, Yew, Block I).
- **Blockchain programs.** Smart contracts (Solana, Polkadot/Substrate) and consensus clients (the Ethereum ecosystem's `lighthouse` and `rust-ethereum`) — Block I.
- **Game engines.** The Bevy engine and friends — Block I.
- **OS components.** The Linux kernel began accepting Rust in 2022; Windows and other projects are doing the same.

Notice the pattern: Rust is strongest where performance, reliability, and memory safety all matter at once. That's a real economic niche, and it's why the language's adopters pay well for genuine fluency.

### How the compiler thinks

When you run `cargo build`, a lot happens before any machine code exists. This pipeline is worth seeing once, because in later modules the compiler will seem to argue with you — and you'll argue back better if you know who you're talking to:

```
  source .rs files
        │
        ▼
  parser ──────────────► AST (abstract syntax tree)
        │
        ▼
  name resolution + type inference ──────► HIR (high-level IR)
        │
        ▼
  BORROW CHECKER ◄────── the part that "argues" with you;
  ownership & lifetimes   and the part that proves memory safety
        │
        ▼
  MIR (mid-level IR) ────► optimizations
        │
        ▼
  LLVM IR ───────────────► LLVM backend ──► machine code
```

The stages, briefly:

- **Parser** turns text into an abstract syntax tree — it only checks that your code is *grammatically* valid Rust.
- **Type checking** then resolves names and infers types, and the **borrow checker** runs its analysis on the program's structure. This is the stage that implements the memory-safety guarantee: it tracks ownership, references, and lifetimes and rejects programs that could free or alias memory unsafely. The famously bossy Rust compiler is largely the borrow checker talking.
- **MIR** (mid-level IR) is an intermediate representation that makes data flow easy to analyze and optimize; many borrow-checker errors are reported against it.
- Finally, rustc lowers to **LLVM IR** and hands off to the **LLVM backend** — a battle-tested optimizer and code generator used by many compilers — which produces the actual machine code.

You will not need to know more than this for a long time. The point of knowing it now: when the compiler rejects your code in Module 005 with a lifetime error, it is not being pedantic — it is a static proof system doing its only job. And when Module 015 mentions monomorphization or Module 056 talks about zero-cost abstractions, those are compiler behaviors that happen between MIR and LLVM.

### Concurrency and async: a preview

Two of the reasons Rust developers are paid well are threads and async — and both are built on the memory-safety foundation above. In most languages, concurrent code is where data races hide. In Rust, safe code cannot produce a data race, because the type system (the `Send`/`Sync` marker traits) refuses to let you share unsynchronized mutable state across threads — the compiler enforces synchronization discipline. **Async Rust** — `async fn` and `.await`, usually on the Tokio runtime — is Rust's model for I/O-heavy workloads: instead of one OS thread per connection, tasks yield at await points and a runtime schedules thousands of them on a few threads. Both are deferred here deliberately: Block D teaches concurrency properly (threads, channels, atomics, `Send`/`Sync`), and Block E is an entire block on async. For now, register that "Rust is good at concurrency" is a *consequence* of memory safety, not a separate feature.

### What a "high-paying Rust developer job" actually looks for

Companies hiring for Rust at real salaries are not looking for someone who knows the syntax. They are looking for three things:

1. **Systems thinking.** The ability to reason about memory, performance, failure modes, and resource management — not just to write code that works on happy paths. This is a habit of mind, and this curriculum builds it block by block.
2. **Ownership fluency.** The ability to talk precisely about ownership, borrowing, and lifetimes — what moves, what borrows, what is `'static`, and why. This is the screening question in most Rust interviews, so Blocks A and B are deliberately thorough.
3. **One specialization track.** Companies hire Rust engineers *for something* — a backend, an embedded device, a blockchain node, a WASM tool. Depth in one domain beats breadth in all of them. This is why Rust.Stack is strictly linear: you build the foundation, then commit to a full 10-module block in a specialization (Module 089 will help you choose deliberately).

### How to use this repository

Every module folder `modules/module-XXX-slug/` follows the same anatomy:

```
module-XXX-slug/
├── README.md              # the lesson: concept, pitfalls, exercise instructions
├── exercises/             # the exercise crate — deliberately incomplete
│   ├── Cargo.toml
│   ├── src/lib.rs         # scaffolding with // TODO(module-XXX) comments
│   └── tests/module_XXX.rs
└── solutions/             # the reference implementation — always visible
```

You read the README, then fill in the `exercises/` crate until its tests pass (`cargo test -p module-XXX-exercises`), then — and only then — compare with `solutions/`. The solutions folder is deliberately not hidden: this is a sandbox, not an exam, but the learning is in the attempt. Every 10th module is a **capstone** (`capstones/capstone-NN-slug/`) that integrates the whole preceding block like a take-home project.

A few operational facts:

- Zero-padded module numbers (`module-007`, `module-042`) exist so folders sort correctly — always use the zero-padded form.
- `./scripts/verify_module.sh XXX` runs formatting, clippy, and tests for one module's two crates.
- `./scripts/check_progress.sh` reads the checkbox list in the root `README.md` and reports your completion percentage — tick boxes as you finish modules.
- Modules 000, 089, 096, and 097 are worksheet modules: they ship a guided written exercise (`exercises/WORKSHEET.md`) instead of a crate, because what they teach is thinking and strategy rather than syntax.

This module is the first of those. Head to the Exercise section and do the repo tour — it's the fastest way to make everything above concrete.

## Common Pitfalls

- **Reading without running.** Rust is learned by fighting the compiler, not by reading about it. Every module in this repo expects you to run commands and make tests pass — do that even when it's slow.
- **Peeking at solutions first.** The solutions are visible, and using them to cheat yourself out of the attempt defeats the entire curriculum. Attempt → get stuck → compare. In that order.
- **Rushing the ownership blocks.** Modules 004–005 are the load-bearing wall of the whole language. Every interview question and every later module assumes you can reason about ownership on demand. Slowing down there is not falling behind.
- **Assuming Rust is "C++ done right."** It's a different model of memory — safety by default, no GC, no inheritance. Approach it on its own terms instead of translating C++ habits.
- **Skipping the worksheet modules.** They look like "soft" content but they are calibration for the job market: they make you articulate the reasoning interviewers actually probe.

## Key Terms

- **Memory safety:** the property that a program can only access memory it is currently entitled to access — no use-after-free, no out-of-bounds access, no data races.
- **CVE (Common Vulnerabilities and Exposures):** the standard catalog of publicly disclosed security vulnerabilities.
- **Garbage collector (GC):** a runtime component that automatically reclaims unused memory, at the cost of runtime overhead and pauses. Rust has none.
- **Borrow checker:** the part of the compiler that statically enforces ownership and borrowing rules, proving memory safety before the program runs.
- **Ownership:** Rust's rule that every value has exactly one owner, who is responsible for freeing it.
- **Borrow:** temporarily using a value through a reference (`&`) without taking ownership.
- **MIR (mid-level IR):** an intermediate representation of your program used for analysis and optimization between type checking and code generation.
- **LLVM backend:** the optimizer and machine-code generator that rustc hands off to at the end of the pipeline.
- **WebAssembly (WASM):** a portable, sandboxed binary format that runs in browsers and other hosts; Rust compiles to it well.
- **`no_std`:** Rust code that runs without the standard library's OS-dependent parts — the normal mode for embedded firmware.
- **Runtime:** whatever code must be present (other than your own) when your program executes. Rust's safe core has effectively no runtime.

## Exercise

This module has no Cargo crate — it is a **worksheet module**. Open `exercises/WORKSHEET.md` and work through all 12 prompts in order:

1. Part 1 is a guided tour of the repository: run `rustup show`, inspect the workspace manifest, open the Module 001 README and identify its required sections, find the TODO convention in Module 004's exercise scaffold, and run the verification and progress scripts.
2. Part 2 is four written reflections — your own definition of memory safety, three things you can build, your specialization pick, and the `cargo build` vs. `cargo test` distinction.

Write your answers in the worksheet file itself. There is nothing to tick in the root `README.md` until you have genuinely completed the work — then compare your answers with `solutions/EXAMPLE_ANSWERS.md`.

## Further Reading

- [The Rust Book, Chapter 1 — Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — the canonical first chapter, including installation and "Hello, World!".
- [The Rust Programming Language homepage](https://www.rust-lang.org/) — official pitch, examples, and language news.
- [Learn Rust](https://www.rust-lang.org/learn) — the official hub of learning resources, including the interactive "Rust in a Browser" playpen.
- [The Rust Reference](https://doc.rust-lang.org/reference/) — the formal specification of the language; skim now, consult when the compiler confuses you.
