# Module 096 Worksheet

These prompts guide you through the process of finding, planning, and practicing an open source contribution in Rust. Write your answers in a separate file or a notebook — there are no code compilation steps in this module.

---

## Prompt 1: Find 3 "Good First Issue" Candidates

Search GitHub for Rust projects with issues labeled "good first issue," "help wanted," or "E-easy" (the Rust compiler convention). For each issue you find, record:

- **Repository name and link**
- **Issue title and label(s)**
- **A 2–3 sentence summary of what the issue asks for**
- **Why you picked it** — does it match your skill level? Is it in a domain you care about?

**Search tips:**
- `github.com/topics/rust` and filter by "good first issue"
- `github.com/rust-lang/rust/issues?q=is%3Aissue+is%3Aopen+label%3AE-easy` (the Rust compiler itself)
- `github.com/tokio-rs/tokio/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22`
- `github.com/bevyengine/bevy/issues?q=is%3Aissue+is%3Aopen+label%3A%22good+first+issue%22`

---

## Prompt 2: Write an Approach Plan for One Issue

Pick one of your three issues and write a plan for how you would fix it. Include:

- **Understanding the codebase:** Which files do you expect to touch? What part of the crate's public API is involved? How would you navigate an unfamiliar codebase — what do you read first?
- **Reproducing the issue:** If it's a bug, how do you reproduce it? If it's a feature addition, what's the desired behavior?
- **Implementation strategy:** What data structures, trait implementations, or function signatures need to change? List the concrete steps in order.
- **Testing:** What tests would you write? Do existing tests need updating?
- **Potential blockers:** What might be harder than it looks? Are there design decisions you'd need to discuss with maintainers before coding?

---

## Prompt 3: Write a Sample PR Description

Write a pull request description for a hypothetical fix to a Rust open source crate. Your description should follow the conventions most Rust projects expect. Include:

- **Summary:** One paragraph explaining what this PR does and why.
- **Changes:** A bullet list of concrete changes (files modified, functions added, tests added).
- **Issue reference:** `Fixes #XXX` or `Closes #XXX`.
- **Testing:** How was this tested? What commands should a reviewer run?
- **Screenshots / output** (if applicable): e.g. CLI output before/after.
- **Checklist:**
  - [ ] `cargo fmt` passes
  - [ ] `cargo clippy` passes with no warnings
  - [ ] `cargo test` passes
  - [ ] Documentation updated (if applicable)

**Tip:** Look at recent merged PRs in popular repositories like `tokio-rs/tokio`, `bevyengine/bevy`, or `rust-lang/rust` for real-world examples of what maintainers expect.

---

## Prompt 4: List 5 Repository Conventions to Check Before Contributing

Before you open a PR on any Rust project, there are conventions you should check. List 5 things you would look for and explain why each matters. Examples to consider:

- `CONTRIBUTING.md` — what does it say about commit message format, branch naming, or required sign-off?
- CI configuration — what checks does the CI pipeline run? Does it enforce a specific `rustfmt` style or `clippy` lint level?
- Code style conventions — does the project use `unsafe`? Are there naming conventions for types, functions, or modules?
- Issue triage — should you comment on an issue before starting work to avoid duplicating effort?
- Testing standards — does the project require tests for every change? Is there a test coverage threshold?

For each convention, describe **how you would find it** (which file to check, which CI log to read) and **why ignoring it could get your PR rejected.**

---

## Prompt 5: Read a Rust Crate's `lib.rs` and Summarize Its Public API

Pick a medium-sized Rust crate on crates.io (suggestions: `serde`, `clap`, `reqwest`, `thiserror`, `axum`, or any crate from your own dependencies). Open its `src/lib.rs` file (on GitHub or locally after `git clone`). Write a 250–400 word summary covering:

- **What problem the crate solves** (one sentence)
- **The top-level public modules** and what each contains
- **The key public types** (structs, enums, traits) and their roles
- **The most important entry point** — which function or macro does a user call first?
- **One design decision you notice** — e.g. is it generic over a trait? Does it use builder patterns? Does it re-export key types?

This exercise builds the skill of reading unfamiliar Rust codebases quickly — the same skill you'll use when you open a real issue on a real crate.
