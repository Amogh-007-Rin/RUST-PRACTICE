# Module 001: Toolchain & Your First Program

**Block:** Block A — Foundations I
**Estimated time:** 45–90 min
**Prerequisites:** Module 000 (orientation); a working Rust toolchain per `docs/SETUP.md`

## Learning Objectives

- You will be able to install and update the Rust toolchain with `rustup` and name what each component (`rustc`, `cargo`, `rustfmt`, `clippy`) does.
- You will be able to create a new crate with `cargo new` and build, run, and check it with `cargo build`, `cargo run`, and `cargo check`.
- You will be able to explain the anatomy of a `main.rs` and why `fn main()` is the entry point.
- You will be able to print to the terminal with `println!` and format values with `{}` placeholders.
- You will be able to tell a binary crate apart from a library crate and say when you'd use each.

## Why This Matters

Every Rust developer's day starts and ends with cargo: you create crates, build them, run tests, lint, and ship with the same tool. In any real Rust job you'll be expected to understand `cargo build` vs `cargo check`, what a workspace is, and what the error messages coming out of `rustc` actually mean. Getting the toolchain mental model right now makes every later module — especially Module 039's cargo deep dive — dramatically easier.

## Concept

### The three tools you need to know

The Rust toolchain is really three programs working together:

- **`rustup`** — the *installer and version manager*. It installs Rust, keeps it updated (`rustup update`), and lets you pin a specific version per project via a `rust-toolchain.toml` file. This repo pins a stable channel that way, so you can clone it and `cargo` just works.
- **`rustc`** — the *compiler*. It translates Rust source into machine code. You will almost never call it directly; cargo drives it for you.
- **`cargo`** — the *build system and package manager*. It resolves dependencies, invokes `rustc`, runs tests, builds docs, and can publish crates. It is to Rust what `npm` is to JS and what `pip` is to Python, except it does building too.

Two more tools that matter from day one: **`rustfmt`** (the standard code formatter — this repo keeps every crate formatted, and so should you) and **`clippy`** (the linter — it catches non-idiomatic code and common mistakes with hundreds of extra checks beyond the compiler's).

### Your first crate

Create a new project and look at what you get:

```text
cargo new hello                # creates the folder hello/
cd hello
cargo run                      # builds and runs it
```

`cargo new` generated this structure:

```text
hello/
├── Cargo.toml    # the package manifest: name, version, dependencies
└── src/
    └── main.rs   # the source code, including the entry point
```

Open `src/main.rs`. It contains the smallest possible complete Rust program:

```rust
fn main() {
    println!("Hello, world!");
}
```

`fn main()` is the entry point: when you run the compiled program, the OS calls `main` first. Whatever you want your program to do, you do it from `main` — or more realistically, you call other functions *from* `main`. Everything in `main.rs` that matters lives inside that function or in helper functions it calls.

### Anatomy of the `println!` macro

`println!` — note the exclamation mark — is a **macro**, not a function. Macros generate code at compile time; for now, treat `println!` as "print a line to the terminal". The important skill is formatting. Placeholders in the string get filled by the arguments that follow:

```rust
fn main() {
    let name = "Ada";
    println!("Hello, {}!", name);          // positional placeholder
    println!("Hello, {name}!");            // named placeholder (same result)
    println!("Pi is roughly {:.2}", 3.14159); // formatting spec: 2 decimals
}
```

The named form (`{name}`) is the modern idiom and is used throughout this repo. `{:.2}` is a formatting spec — you'll meet more of these in later modules; for now know that placeholders exist and accept values of any printable type.

### Building, running, checking

Three commands you'll use thousands of times:

| Command | What it does |
|---|---|
| `cargo build` | Compiles a debug binary into `target/debug/` |
| `cargo run` | Builds (if needed) and then *runs* the binary |
| `cargo check` | Checks the code compiles **without** producing a binary — much faster, used constantly while editing |

You also have `cargo fmt` (format your code), `cargo clippy` (lint it), and `cargo test` (run tests — the next section), which will be your feedback loop for the entire course.

### Where does the compiled code come from?

It helps to have a mental picture of what happens between `cargo run` and a running program:

```text
  main.rs + library crates
            │
            ▼
        ┌─────────┐      │
        │  rustc  │      │ front end: parse + type check + borrow check
        └─────────┘      │ (this is where most error messages are born)
            │
            ▼
   middle representation (MIR)
            │
            ▼
        ┌─────────┐
        │ LLVM    │   machine-code generation + optimization
        └─────────┘
            │
            ▼
   native binary  ──►  ./target/debug/hello
```

You don't need to control any of this to use Rust — but knowing that the *borrow checker* runs during the front end is why you'll see some errors mention ownership rules (Module 004) and why the same logic can behave differently under optimization. Cargo orchestrates this pipeline; `cargo check` stops after the front end, which is why it's so fast.

### Binary crates vs library crates

A crate is the unit of compilation: one crate = one library or one binary. `cargo new hello` makes a **binary crate** (it has `src/main.rs`, an entry point, and produces an executable). But most real code lives in **library crates** (`src/lib.rs`, no entry point) that other code imports and uses. Libraries are how Rust code is shared and reused; binaries are thin layers that glue libraries together and talk to the user.

This course's exercise crates are libraries with integration tests in `tests/` — but this module's exercise crate has *both*: a `src/lib.rs` with the functions, and a `src/main.rs` binary that calls them, so you can watch a real library being used by a real binary.

### The exercise crate's manifest

Look at `exercises/Cargo.toml`:

```toml
[package]
name = "module-001-exercises"
version = "0.1.0"
edition = "2021"
```

`name` is the package name (this repo's convention is `module-XXX-exercises` for exercises, `module-XXX-solutions` for solutions — cargo translates the hyphens to underscores when you `use` them in code). `edition` picks the language edition, the collection of rules that defines how your code parses; 2021 is the current stable one and the whole repo pins it. `[dependencies]` lists external crates — this module needs none, and neither does anything in Block A.

### What this module's exercise asks of you

The exercise crate has two tiny functions in `src/lib.rs` — `greet` and `message_length` — each with a `// TODO(module-001)` comment and a `panic!` placeholder. Replace the placeholder with a real implementation, then run the tests:

```text
cargo test -p module-001-exercises
```

When the tests pass, run the binary too:

```text
cargo run -p module-001-exercises
```

Two things worth noticing: `panic!` is a valid way to scaffold an unimplemented function (it compiles, and it makes tests fail loudly until you replace it — the pattern used all through this repo), and the same `greet` function is being used both by the tests *and* by the binary. That is exactly the binary-crate-uses-library-crate relationship you'll see in every capstone.

## Common Pitfalls

- **Forgetting `cargo run` needs a manifest.** Running `cargo run` from a subfolder of the workspace can target the wrong crate. Use the scoped form `cargo run -p module-001-exercises` or make sure your current directory is the crate folder.
- **Confusing `cargo build` with `cargo check`.** `build` produces a binary; `check` only verifies compilation. Use `check` while editing, `run` when you want to see output.
- **Writing `println(name)` instead of `println!("{name}")`.** `println!` is a macro that requires a *format string* first; `println(name)` will not compile. The `!` and the string are not optional.
- **Treating `fn main` as a regular function you can call.** It's the entry point the runtime invokes; call your own helper functions from it instead.

## Key Terms

- **crate:** the unit of compilation — one library or one binary.
- **package:** a folder with a `Cargo.toml` that can contain one or more crates (usually one lib + one bin).
- **binary crate:** a crate with an entry point (`fn main`) that produces an executable.
- **library crate:** a crate without an entry point, meant to be imported by other crates.
- **macro:** code that generates code at compile time; `println!` is your first one (the `!` marks it).
- **toolchain:** the set of compiler + build tool versions managed by `rustup`.

## Exercise

In `exercises/`:

1. Open `src/lib.rs` and find the two `// TODO(module-001)` comments.
2. Implement `greet(name)` so it returns a `String` greeting using `format!`.
3. Implement `message_length(message)` so it returns the byte length of the input.
4. Run `cargo test -p module-001-exercises` until all five tests pass.
5. Run `cargo run -p module-001-exercises` to see your library in action from the binary.
6. Optionally compare with `solutions/` afterwards to see the reference implementation.

## Further Reading

- [The Rust Book, Chapter 1: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html) — the classic first-chapter walkthrough.
- [std `println!` documentation](https://doc.rust-lang.org/std/macro.println.html) — format syntax details.
- [The Cargo Book: Getting Started](https://doc.rust-lang.org/cargo/getting-started/index.html) — what cargo does and how.
- [rustup documentation](https://rust-lang.github.io/rustup/) — how `rustup` installs and manages toolchains.
