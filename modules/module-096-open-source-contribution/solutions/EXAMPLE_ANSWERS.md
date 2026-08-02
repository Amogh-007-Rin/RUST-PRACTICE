# Module 096 Example Answers

These are sample answers — your own responses will differ based on which issues and crates you choose. Use these as a reference for the level of detail expected, not as answers to copy.

---

## Prompt 1: Find 3 "Good First Issue" Candidates

### Candidate 1

**Repository:** `tokio-rs/tokio`
**Issue:** "#6789: Add `try_recv` method to `mpsc::UnboundedReceiver`"
**Labels:** `A-tokio`, `E-easy`, `help wanted`
**Summary:** Tokio's `mpsc` unbounded channel has a blocking `recv` and an async `recv`, but no non-blocking `try_recv` method — unlike `std::sync::mpsc`, which has one. The issue asks for a `try_recv` that returns `Option<T>` immediately (Some if data is available, None if the channel is empty but not closed).
**Why I picked it:** I've used Tokio channels in Capstone 05 and understand the `mpsc` API. This is additive (no existing behavior changes) and small in scope — an ideal first contribution.

### Candidate 2

**Repository:** `rust-lang/rust`
**Issue:** "#101234: Improve error message for E0502 when borrowing struct fields"
**Labels:** `E-easy`, `A-diagnostics`, `T-compiler`
**Summary:** When you borrow two fields of the same struct mutably and immutably, the compiler emits E0502 with a suggestion about splitting borrows — but the suggestion only works for simple cases. This issue asks to improve the diagnostic to detect more patterns (e.g. tuple struct fields and nested field access).
**Why I picked it:** This is in the compiler itself, which is intimidating, but diagnostics-only issues are well-documented and don't require deep knowledge of the borrow checker internals. The `rustc-dev-guide` has a dedicated chapter on diagnostics.

### Candidate 3

**Repository:** `serde-rs/serde`
**Issue:** "#2745: Add example to `serde_json::from_reader` docs showing line-delimited JSON parsing"
**Labels:** `documentation`, `good first issue`
**Summary:** `serde_json::from_reader` parses a single JSON value from a reader. Users often want to parse a file with one JSON object per line (NDJSON), which requires a different approach. The issue asks for a doc example showing how to do this with `BufRead::lines()` and `from_str`.
**Why I picked it:** Documentation-only, no code changes to the library itself. Tests it against `cargo doc` to verify the example compiles. A low-risk way to practice the contribution workflow.

---

## Prompt 2: Write an Approach Plan for One Issue

### Plan for: `try_recv` on `tokio::sync::mpsc::UnboundedReceiver` (Candidate 1)

**Understanding the codebase:**
- Main file: `tokio/src/sync/mpsc/unbounded.rs` (contains `UnboundedReceiver`)
- Relevant trait: the internal `chan::Rx` trait in `tokio/src/sync/mpsc/chan.rs` — this is the shared channel implementation used by both bounded and unbounded receivers.
- Navigation strategy: Start by reading the existing `recv` method on `UnboundedReceiver` to understand how it interacts with the channel. Then look at `std::sync::mpsc::Receiver::try_recv` in the standard library as a reference for the expected semantics.

**Reproducing the issue:**
- Not a bug — a missing feature. The desired behavior: call `rx.try_recv()` and get `Some(value)` if a message is available, or `None` if no message is queued (channel still open). If the channel is closed and empty, return `None` (same as "no message available"), consistent with std's behavior.

**Implementation strategy:**
1. Add a `try_recv` method to the `Rx` trait in `chan.rs` IF the internal recv already supports non-blocking polling (likely it does via `Poll::Ready` / `Poll::Pending`).
2. If the internal recv uses `poll_recv`, then `try_recv` is essentially calling `poll_recv(cx)` with a noop waker and returning immediately.
3. Add the public method:
   ```rust
   pub fn try_recv(&mut self) -> Result<Option<T>, TryRecvError> { ... }
   ```
   Define `TryRecvError` in the `error` module (likely `TryRecvError::Closed` if the channel is closed and empty, and `TryRecvError::Empty` if open but empty — check std's design).
4. Add `#[must_use]` and doc comments matching Tokio's style.

**Testing:**
- Test in `tokio/tests/sync_mpsc.rs` (or wherever unbounded channel tests live):
  - `try_recv` on empty channel → returns `Err(TryRecvError::Empty)` (or `Ok(None)`, depending on the design choice).
  - Send then `try_recv` → `Ok(Some(value))`.
  - Drop sender, `try_recv` on remaining messages → drains them, then returns `Err(TryRecvError::Closed)`.
  - Concurrent test: send from one task, `try_recv` from another — verify no race condition.

**Potential blockers:**
- The internal `chan::Rx` trait may not support a non-blocking recv natively. If it only supports `async fn recv`, I'd need to add a `poll_recv`-style method first, which is a larger change. I would discuss this with maintainers in the issue before coding.
- `TryRecvError` might already exist in a different module — need to check the `error` module to avoid duplication.
- Tokio's MSRV (minimum supported Rust version) policy: any new API must compile on the MSRV. I'd check `tokio/Cargo.toml` for the current MSRV.

---

## Prompt 3: Write a Sample PR Description

### PR Title: `sync: add try_recv method to UnboundedReceiver`

---

#### Summary

This PR adds a `try_recv` method to `tokio::sync::mpsc::UnboundedReceiver`, matching the API provided by `std::sync::mpsc::Receiver`. The method attempts to receive a message without blocking or awaiting — it returns immediately with `Ok(Some(value))` if a message is available, `Ok(None)` if the channel is open but empty, or `Err(TryRecvError::Closed)` if the channel is closed and empty. This is useful for event loops that want to drain a channel opportunistically without suspending.

#### Changes

- Added `TryRecvError` enum to `tokio/src/sync/mpsc/error.rs` with `Empty` and `Closed` variants.
- Added `try_recv(&mut self) -> Result<Option<T>, TryRecvError>` to `UnboundedReceiver` in `tokio/src/sync/mpsc/unbounded.rs`.
- Added internal `try_recv` support to `chan::Rx` trait in `tokio/src/sync/mpsc/chan.rs`.
- Added tests in `tokio/tests/sync_mpsc.rs` covering: empty channel, message available, channel closed with pending messages, channel closed with no messages.
- Updated `tokio/src/sync/mpsc/mod.rs` re-exports to include `TryRecvError`.

#### Issue Reference

Closes #6789

#### Testing

```bash
cargo test -p tokio --test sync_mpsc
cargo test --doc  # doc examples compile
```

All existing tests pass. The new tests exercise all three states (empty, message, closed).

#### Checklist

- [x] `cargo fmt --all` passes
- [x] `cargo clippy --all -- -D warnings` passes
- [x] `cargo test --all` passes
- [x] Documentation updated (doc comments on `try_recv` and `TryRecvError`)
- [x] No MSRV bump required (uses only stable-1.60+ APIs)

---

## Prompt 4: List 5 Repository Conventions to Check Before Contributing

### 1. CONTRIBUTING.md

**Where to find it:** Root of the repository (`github.com/owner/repo/blob/main/CONTRIBUTING.md`).
**What to look for:** Commit message format (e.g. `type(scope): description`), sign-off requirements (DCO vs. CLA), branch naming conventions, and whether the project wants an issue discussion before a PR.
**Why it matters:** Violating the contributing guidelines is the fastest way to get a PR closed without review. Some projects require you to comment `/take` on an issue before submitting a PR to avoid duplicate work.

### 2. CI Configuration

**Where to find it:** `.github/workflows/ci.yml` (or similar) in the repository.
**What to look for:** Which Rust toolchain version is pinned, which lints are enforced, whether `cargo fmt --check` is mandatory, and whether there are platform-specific tests (Windows, macOS, ARM).
**Why it matters:** If the CI uses `cargo clippy -- -D warnings` and your code triggers a clippy warning, your PR fails CI before anyone reviews it. You should run the exact CI commands locally before pushing.

### 3. Code Style Conventions

**Where to find it:** Read a few source files in the crate (start with `src/lib.rs` and one or two submodules). Also check `rustfmt.toml` for non-default formatting rules.
**What to look for:** Naming conventions (snake_case vs. SCREAMING_SNAKE_CASE for consts?), use of `unsafe` (does the project avoid it, or is it common?), import style (nested vs. flat `use` statements), whether `#[derive(...)]` is used or traits are implemented manually.
**Why it matters:** A PR that uses a different naming convention or import style stands out as "outsider code" and creates noise for the reviewer. Matching the existing style signals that you've read the codebase.

### 4. Issue Triage and Assignment

**Where to find it:** The issue tracker itself — look at how recent issues were resolved. Did contributors comment "I'd like to work on this" and get a maintainer's go-ahead?
**What to look for:** Whether the project uses `/take` or assignment bots, whether there's a "help wanted" process described in CONTRIBUTING.md, and whether issues are labeled with difficulty indicators.
**Why it matters:** Working on an issue someone else is already fixing wastes your time and theirs. Most projects expect you to express interest first — a surprise PR on a contested issue may be rejected regardless of quality.

### 5. Testing Standards

**Where to find it:** Look at existing test files (`tests/` directory, `#[cfg(test)] mod tests` blocks in source files). Check if there's a code coverage comment in CONTRIBUTING.md or CI config.
**What to look for:** Do tests use `#[tokio::test]` (async) or plain `#[test]`? Are there integration tests separate from unit tests? Does every public function have a test? Are there `#[should_panic]` tests for error paths?
**Why it matters:** If the project requires a test for every new public API and you don't add one, the PR will be marked "changes requested." Some projects also have coverage badges that drop, alerting maintainers.

---

## Prompt 5: Read a Rust Crate's `lib.rs` and Summarize Its Public API

### Summary of `thiserror` (v1.0, `src/lib.rs`)

**Problem:** `thiserror` solves the boilerplate of implementing `std::error::Error` and `Display` for custom error types. Without it, every error enum requires manual `impl Display` and `impl Error` — dozens of lines of repetitive code per error type.

**Top-level public modules:** `thiserror` is a single-file proc-macro crate. Almost everything lives in `src/lib.rs` as a proc-macro definition. There is no public module tree — users interact with the `#[derive(Error)]` macro, which is the only public API surface.

**Key public types:**
- `#[derive(Error)]` — the proc-macro that generates `Display`, `Error`, and optionally `From` implementations for a struct or enum.
- The `#[error("...")]` attribute — placed on enum variants (or the struct itself) to define the `Display` format string. Supports interpolating fields with `{0}`, `{var}`, or `.named_field`.
- `#[source]` attribute — marks the field that is the underlying error cause, used by `Error::source()`.
- `#[from]` attribute — auto-generates `From<InnerError> for MyError`, typically used on a tuple variant like `Io(#[from] std::io::Error)`.
- `#[error(transparent)]` — for newtype wrappers that delegate `Display` and `source` to the inner error.

**Entry point:** The user adds `use thiserror::Error;` and annotates their error enum with `#[derive(Error)]`. No function call or initialization — the macro generates the implementation at compile time.

**Design decision:** `thiserror` is a proc-macro crate (not a regular library), which means its code runs at compile time inside `rustc`. The actual logic lives in an `impl/` subdirectory containing the token-parsing and code-generation logic. The `src/lib.rs` file itself is thin — it registers the proc-macro with `#[proc_macro_derive(Error, attributes(error))]` and delegates to the implementation module. This separation of "macro registration" from "macro logic" is a common pattern in proc-macro crates (see also `serde_derive`, `derive_more`).

**Why this design matters for contributors:** The `impl/` directory is where real work happens, but it operates on `proc_macro::TokenStream`, not regular Rust types. Contributing to proc-macro crates requires understanding token trees, spans, and error reporting via `syn` and `quote` — a different skill set from contributing to runtime libraries.
