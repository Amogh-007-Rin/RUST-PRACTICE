# Module 039: Cargo Deep Dive — Workspaces, Features, Profiles, Publishing

**Block:** Block D — Intermediate Rust II: Concurrency, Unsafe & Macros
**Estimated time:** 90–120 min
**Prerequisites:** Module 009 (crates/packages/workspaces), Module 013 (error handling)

## Learning Objectives

- Explain how the workspace in this repo's root `Cargo.toml` glues hundreds of crates into one build graph.
- Define Cargo **features** in a crate's own `Cargo.toml`, and gate code behind them with `#[cfg(feature = "...")]`.
- Write platform-conditional code with `#[cfg(unix)]` / `#[cfg(not(unix))]` and read compile-time values via `env!`.
- Distinguish `dev`/`release` build profiles and know where profile settings live.
- Summarize the crates.io publishing flow and the semantic versioning contract (`major.minor.patch`).

## Why This Matters

By now you've run `cargo test` hundreds of times; this module makes Cargo itself a tool you understand instead of a black box. Features are how the entire ecosystem does optional functionality — `tokio`'s `full` feature, `serde`'s `derive`, your company's internal feature flags — and `cfg` gating is how one codebase compiles for servers, phones, and WASM. Debugging a real build almost always means reading a `Cargo.toml` and a `#[cfg]`: knowing where features live (each crate's own manifest, not the root workspace) and how they interact is a daily job skill. And the repo you're in right now — a workspace of ~200 crates — only builds because of the mechanics this module explains.

## Concept

### The workspace: one manifest to rule them all

This repo's root `Cargo.toml` is a `[workspace]` table whose `members` are globs over every module's `exercises` and `solutions` crates. Two consequences:

1. **One lockfile, one build graph.** All member crates share `Cargo.lock` and the `target/` directory. Cargo resolves every crate's dependencies once, so `cargo test -p module-039-exercises` doesn't rebuild your whole toolchain.
2. **`-p` selects, workspace-wide commands build everything.** `cargo test --workspace` iterates all members; `cargo test -p <name>` narrows to one. During development with many concurrent agents (like this repo's generation), you always use `-p` — never the workspace-wide command.

Features **never** live in the root manifest. Each crate declares its own `[features]` table:

```toml
[package]
name = "module-039-exercises"
version = "0.1.0"
edition = "2021"

[features]
default = []
demo = []
```

`demo` here is a pure flag feature — it enables nothing extra, it just exists so code can ask "was this feature turned on?" A real feature can also pull in dependencies or enable *other* features:

```toml
[features]
default = ["std"]
std = []
web = ["dep:hyper", "std"]   # enables a dependency and another feature
```

### Gating code with `#[cfg(feature = "...")]`

The feature flag only matters when code consults it. That's `#[cfg(...)]` — a conditional-compilation attribute evaluated before code generation:

```rust
#[cfg(feature = "demo")]
pub fn build_tag() -> &'static str {
    "demo-feature-enabled"
}

#[cfg(not(feature = "demo"))]
pub fn build_tag() -> &'static str {
    "no-demo-feature"
}
```

Exactly one of those functions exists after expansion — `cfg` literally deletes code from the compilation. Callers use whichever is present; the compiler never sees both. That's the whole trick of features: **feature flags are compile-time booleans that turn items on and off.**

### Platform conditionals: `cfg(unix)` and friends

The same mechanism handles the platform. `cfg(unix)` is true on Linux/macOS/BSD, false on Windows; `cfg(windows)` is the inverse; `target_os = "linux"` is exact. A common pattern — a function whose *body* differs per platform — uses block attributes:

```rust
pub fn platform_tag() -> &'static str {
    #[cfg(unix)]
    {
        "unix"
    }
    #[cfg(not(unix))]
    {
        "non-unix"
    }
}
```

And `cfg!(...)` is the *runtime-queryable* version — it compiles to a constant boolean you can use in `if` expressions and test assertions. If code can't even exist on a platform, use `#[cfg]`; if it just needs to *behave* differently, prefer `cfg!`.

### Compile-time environment: `env!`

Cargo sets a pile of environment variables at compile time, and `env!` bakes them in as literals — a warning-proof way to surface crate metadata in code:

```rust
pub const CRATE_NAME: &str = env!("CARGO_PKG_NAME");
pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");
// CARGO_PKG_NAME = "module-039-exercises" (differs per crate!)
// CARGO_PKG_VERSION = "0.1.0"
```

`CARGO_PKG_NAME`, `CARGO_PKG_VERSION`, `CARGO_MANIFEST_DIR`, `PROFILE` ("debug"/"release"), `CARGO_CFG_*` — this is how tools embed their own version into `--version` output and how build scripts behave differently per profile.

### Build profiles

Profiles are collections of compiler settings. The two you know: `dev` (default for `cargo build`/`cargo test`: fast to compile, zero optimization, debug assertions on) and `release` (`cargo build --release`: optimized, slower to compile, no debug assertions). Profile overrides live in a `Cargo.toml`'s `[profile.*]` tables — the root manifest of this repo, for example, turns on LTO for release builds. When you benchmark (Capstone 06) you always benchmark `--release`; when you debug panics you want `dev`.

### Publishing and semver

`cargo publish` uploads a crate to crates.io. Prerequisites: a package `description`, `license`, and `repository` in `Cargo.toml`, a version that isn't taken, and `cargo package`-compatible contents (no absolute paths, no broken `[dependencies]`). Publishing is largely irreversible for a given version, which is why the **semantic versioning** contract matters:

- **patch** (`0.1.0 → 0.1.1`): bug fixes, no API change. Consumers update freely.
- **minor** (`0.1.0 → 0.2.0`): new backward-compatible functionality.
- **major** (`1.0.0 → 2.0.0`): breaking changes.

Cargo's version requirement `"^1.2"` means "any `1.x >= 1.2`" — so a semver-violating `1.2.3 → 1.3.0` patch that removes a public function breaks every consumer's build. In practice: never break the API in a minor/patch bump, keep `0.x` versions honest (in `0.x`, minors break), and read the release notes before upgrading a major.

## Common Pitfalls

- **Putting features in the root workspace manifest.** Features are per-crate; `[features]` belongs in each package's own `Cargo.toml` (see this module's crate). The root only declares workspace members and shared profile settings.
- **Using `#[cfg]` where `cfg!` belongs (or vice versa).** `#[cfg]` removes items at compile time — unusable inside function bodies on the wrong platform; `cfg!` is a runtime-visible boolean. For "does this function exist" use `#[cfg]`; for "which branch at runtime" use `cfg!`.
- **Testing a feature-gated API without the feature.** `cargo test` enables no features by default (besides `default`), so `demo`-gated functions don't exist. This module's exercise is intentionally tested with `cargo test -p module-039-exercises --features demo` — forgetting the flag produces confusing "cannot find function" errors.
- **Counting on `cfg!(feature = "default")`.** `default` is a real feature that is enabled by default, even when it's empty. Check the features you actually defined, not `default`.
- **Benchmarking in the `dev` profile.** Zero optimization makes benchmarks meaningless. Always `--release` (or a custom profile) for numbers you'd show anyone.

## Key Terms

- **workspace:** a set of crates sharing one manifest, lockfile, and target directory.
- **feature:** a compile-time toggle declared in `[features]`; code asks about it via `#[cfg(feature = "...")]`.
- **`#[cfg(...)]`:** conditional compilation — the attribute deletes code before the compiler sees it.
- **`cfg!(...)`:** compile-time boolean form of `cfg`, usable in expressions.
- **`env!("...")`:** embed a compile-time environment value (Cargo metadata) as a literal.
- **profile:** a named bundle of compiler settings (`dev`, `release`, custom).
- **semver:** the `major.minor.patch` versioning contract Cargo resolves against.
- **crates.io:** the public package registry; `cargo publish` uploads to it.

## Exercise

Open `exercises/` and fill in the `// TODO(module-039)` comments in `src/lib.rs`. The crate demonstrates features, `cfg`, and Cargo-provided metadata — no dependencies needed:

1. `CRATE_NAME` and `CRATE_VERSION` — already implemented with `env!`; notice the tests verify them against what Cargo knows.
2. `platform_tag()` — already implemented; it shows the `#[cfg(unix)]` / `#[cfg(not(unix))]` pair.
3. `build_tag()` — the demo-feature version must return `"demo-feature-enabled"` (the `#[cfg(not(feature = "demo"))]` fallback is complete).
4. `demo_sum` and `demo_square` — only compiled when the `demo` feature is on; make them return the real results.

The feature lives in **this crate's** `Cargo.toml` (not the root workspace manifest). Run the tests *with the feature enabled*:

```bash
cargo test -p module-039-exercises --features demo
```

Without `--features demo`, the demo-gated tests are compiled out (a demonstration of exactly what the README above explains). When you're done, compare with `solutions/`.

## Further Reading

- The Cargo Book, [Features](https://doc.rust-lang.org/cargo/reference/features.html)
- The Cargo Book, [Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) and [Profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- The Rust Reference, [Conditional Compilation (`cfg`)](https://doc.rust-lang.org/reference/conditional-compilation.html)
- The Cargo Book, [Publishing](https://doc.rust-lang.org/cargo/reference/publishing.html) and [SemVer compatibility](https://doc.rust-lang.org/cargo/reference/semver.html)
