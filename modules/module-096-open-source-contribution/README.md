# Module 096: Open Source Contribution

**Block:** Block J — Interview Prep, DSA & Career Readiness
**Estimated time:** 60–90 min
**Prerequisites:** Module 095 (System Design with Rust), Module 094 (Rust-Specific Interview Questions)

## Learning Objectives
- You will be able to find "good first issue" candidates in real Rust projects on GitHub using label filters and topic searches.
- You will be able to write an approach plan for a contribution that covers codebase navigation, reproduction, implementation, and testing.
- You will be able to write a PR description that follows the conventions of mature Rust projects, including issue references, checklists, and testing instructions.
- You will be able to identify the 5 repository conventions (CONTRIBUTING.md, CI, code style, issue triage, testing standards) that determine whether a PR gets accepted or rejected.
- You will be able to read an unfamiliar Rust crate's `lib.rs` and summarize its public API structure in 250–400 words.

## Why This Matters

Open source contributions are the strongest signal on a Rust developer's resume that no side project can replicate. A merged PR to `tokio`, `clap`, `serde`, or `rust-lang/rust` tells a hiring manager three things: you can read and navigate a large codebase, you can follow an unfamiliar project's conventions and CI pipeline, and you can communicate with maintainers professionally. These are exactly the skills you use in your first month on any Rust team. The barrier to entry is lower than most learners think — many projects explicitly label issues as "good first issue" and provide mentoring — but the process is unfamiliar, and this module walks you through it.

## Concept

### Why Open Source Contributions Matter for Rust Jobs

Rust hiring is unusual among software engineering disciplines. Most languages have large pools of experienced developers; Rust's pool is smaller, and companies compensate by looking for deeper signals. A candidate with 3 merged PRs to respected Rust crates is often preferred over a candidate with 5 years of Python experience and "learning Rust on the side." The reason: merged PRs are verifiable. Anyone can read the diff, the review comments, and the test results. They show real code, real collaboration, and real problem-solving — not abstract claims.

But the contribution itself is only half the signal. The other half is in the *process*: did you discuss the approach with maintainers before coding? Did you write tests that cover edge cases? Did your PR pass CI on the first push? Did you respond to review feedback constructively? These process signals are what senior engineers evaluate during hiring — and they're the same signals you produce during an open source contribution.

### How to Find "Good First Issues"

The phrase "good first issue" is a conventional label across GitHub, but each Rust community has its own labeling system. Here is how to search effectively:

**The Rust compiler and standard library:**
```text
github.com/rust-lang/rust/issues
  → label:E-easy          (good first issues in the compiler)
  → label:E-mentor         (issues with a mentor assigned)
  → label:A-diagnostics    (improving error messages — a common entry point)
```

The compiler is intimidating, but diagnostics-only issues are well-documented in the `rustc-dev-guide` and are isolated from the borrow checker and codegen internals. They're also high-impact: every Rust user sees compiler error messages, and improving one is visible to millions of developers.

**Tokio (async runtime):**
```text
github.com/tokio-rs/tokio/labels/good%20first%20issue
```
Tokio issues tend to be well-specified and actively mentored. Common categories: adding missing convenience methods, improving doc examples, and fixing race conditions in test helpers. Tokio's CI is thorough (tests on Linux, macOS, Windows, multiple feature flag combinations), so a successful PR teaches real CI discipline.

**Bevy (game engine):**
```text
github.com/bevyengine/bevy/labels/good%20first%20issue
```
Bevy's first-issue labels include `D-Good-First-Issue`, `S-Needs-RFC` (design discussion needed), and `A-ECS` (entity-component-system). Bevy is a large, actively developed project, and its community is unusually welcoming to newcomers. The challenge: Bevy moves fast, so an issue you pick up today might be solved by someone else tomorrow. Claim it first (comment on the issue).

**General approach:**
```text
github.com/topics/rust → filter by "good first issue"
```

This returns issues across all repositories tagged with the `rust` topic. Apply additional filters: language=Rust, sort by recently updated, label=good-first-issue. Skim the issue titles; open 5–10 that sound interesting. Read the full description and the most recent comments to check if anyone is already working on it.

When evaluating an issue, ask three questions:
1. **Do I understand the problem?** If the issue description uses jargon you don't recognize, bookmark it and come back in a month. Pick something you can explain to a colleague in one sentence.
2. **Can I reproduce it?** If it's a bug, the issue should include steps to reproduce. If you can't reproduce it on your machine, you can't verify your fix.
3. **Is the scope manageable?** A "good first issue" should be completable in 1–3 evenings. Adding a single method with 20 lines of code and 40 lines of tests is ideal. Refactoring an entire module is not.

### Reading an Unfamiliar Codebase

You've just cloned a large Rust project. Where do you start? The order that works:

1. **Read `Cargo.toml`** — What dependencies does it have? Is it a library or a binary? Are there feature flags? Understanding the dependency tree tells you what the project *uses*, which hints at what it *does*.
2. **Read `src/lib.rs`** (for libraries) or `src/main.rs` (for binaries) — The top-level source file is the table of contents. It declares the module tree, re-exports the public API, and often contains the most important types.
3. **Skim `tests/`** — Integration tests are executable documentation. They show you real usage patterns without the implementation guts. If you're fixing a bug, find the test that should catch it.
4. **Use `rg` (ripgrep) heavily** — `rg "fn recv"` to find where a method is defined, `rg "TryRecvError"` to find all uses of a type, `rg "// TODO"` to find incomplete work. Navigating a codebase by text search is faster than reading files sequentially.
5. **Read the CI config** — `.github/workflows/ci.yml` tells you what the project considers "correct": which Rust version is used, which lints are enforced, which feature flags are tested.

### The PR Workflow

Every established Rust project follows roughly the same PR workflow. The specific tools vary (some use `bors` for merging, some use GitHub's merge queue), but the shape is consistent:

```
1. Find an issue → comment "I'd like to work on this"
2. Fork the repo → clone your fork → create a branch
3. Write the code + tests → cargo fmt + cargo clippy + cargo test
4. Push → open a PR against the upstream main branch
5. CI runs → fix any failures → respond to review comments
6. Maintainer approves → squash-merge → your commit is in main
```

Step 1 is the most commonly skipped step — and the most common reason a PR is rejected. If you don't comment on the issue before starting, someone else might submit a PR simultaneously, or the maintainer might have already decided the issue isn't a good fit. Always comment first.

Step 3 deserves emphasis: run the exact commands the CI runs *before* pushing. Most projects have a contributing guide that lists them. Common commands:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-features
```

If any of these fail locally, they'll fail in CI. Fix them before opening the PR — maintainers appreciate PRs that pass CI on the first commit.

### Communication Norms

Rust's community has strong, evolving norms around communication. The Rust Code of Conduct applies to all official Rust spaces (repositories, Discord, Zulip, forums), and most third-party crates adopt it. In practice, this means:

- **Be specific and technical.** "This doesn't work" is unhelpful. "When I call `foo()` with an empty string, it panics at `src/foo.rs:42` because `bar.unwrap()` fails" is actionable.
- **Assume good intent.** The maintainer who wrote the code you're fixing isn't incompetent — they made a tradeoff you might not see. Ask before asserting.
- **Respond to reviews promptly but not instantly.** Within 24–48 hours is the norm. If a reviewer asks for changes and you respond in 5 minutes with a rushed fix, they'll ask for more changes. Take the time to get it right.
- **Close the loop.** If you decide not to finish a contribution, comment on the issue saying so. Abandoned issues prevent others from picking up the work.

The Rust Zulip (`rust-lang.zulipchat.com`) is the working communication platform for the Rust project itself. Each team (compiler, lang, library, infrastructure) has its own stream. If you're contributing to `rust-lang/rust`, Zulip is where you ask questions — not the issue tracker, which is for design discussion and tracking.

### Async-First Development

"Async-first" doesn't refer to `async`/`.await` in this context — it refers to the workflow pattern. Open source contributions are async by nature: you submit a PR, a maintainer reviews it days later, you respond, they review again. This is normal. The round-trip time can be days or weeks. During that time:

- **Work on something else.** Don't block your learning on one PR. Start another issue. Build a different project. The Rust.Stack curriculum is designed to give you parallel tracks of progress.
- **Don't ping excessively.** One follow-up comment after 7–10 days of silence is reasonable. Daily "any update?" comments will annoy maintainers.
- **Expect multiple review rounds.** The first review almost always finds something. A PR that's approved on the first pass is rare — it usually means the change was trivial or the reviewer was lenient.

### Practical Contribution Categories for Learners

Not all contributions need to be code. The entry points ranked by difficulty:

1. **Documentation fixes** (easiest): Fix a typo, add a missing example, clarify an ambiguous sentence. Many projects label these `documentation` or `good first issue`. They teach the PR workflow without requiring domain knowledge.

2. **Test additions** (easy-medium): Add a test for an untested edge case. You don't change any production code, but you improve coverage and learn how the project's test infrastructure works.

3. **Small feature additions** (medium): Add a convenience method that mirrors an existing API. Adding `try_recv` to a channel type that already has `recv` is an example — the pattern is established, and the work is in implementation and testing.

4. **Bug fixes** (medium-hard): Fix a reported issue. Requires reproducing the bug, tracing through unfamiliar code, and ensuring the fix doesn't regress other behavior.

5. **Performance improvements** (hard): Optimize a hot path. Requires profiling, benchmarking, and a PR body that includes before/after numbers. Always discuss with maintainers before attempting — some performance tradeoffs are intentional.

Start with category 1 or 2. The confidence from shipping a small fix carries forward. A typo fix in `serde`'s docs is still a contribution to `serde` — the strongest signal for a job interview.

### What Maintainers Look For

From the maintainer's perspective, reviewing a PR from a first-time contributor answers these questions:

- **Is this safe?** No undefined behavior, no unsound `unsafe`, no panics on valid input.
- **Is this tested?** Are there tests for the happy path AND the error path? Do the tests exercise edge cases?
- **Is this consistent?** Does it match the existing code style, naming conventions, and API design patterns?
- **Is this documented?** Are new public items documented with doc comments that include examples? Do the examples compile?
- **Is this complete?** Does the change touch all the places it needs to (e.g. a new error variant needs a `Display` impl, a re-export in `lib.rs`, and a test)?

A "no" to any of these means changes requested. Address all five, and your PR has a high chance of being merged.

## Common Pitfalls
- **Opening a PR without commenting on the issue first.** Maintainers may reject it outright or ask you to close it and comment instead. Always express interest before coding.
- **Running only `cargo test` without `cargo fmt` and `cargo clippy`.** CI enforces all three. Run them locally in the same order CI does.
- **Picking an issue that's too large.** A "good first issue" should be a single function or method. If it touches 5+ files or requires refactoring an existing abstraction, it's not a good first issue — it's a feature request.
- **Getting discouraged by delays.** Maintainers are volunteers or have other work obligations. A PR sitting unreviewed for a week is normal. Follow up politely after 7–10 days.
- **Interpreting review feedback as personal criticism.** Reviews are about the code, not the author. "This approach has a soundness hole" means "let's find a different approach," not "you're bad at Rust."

## Key Terms
- **Good first issue:** an issue labeled by maintainers as suitable for new contributors, typically small in scope and well-specified.
- **PR (Pull Request):** a proposed set of changes submitted for review, consisting of a branch, a diff, a description, and a discussion thread.
- **CI (Continuous Integration):** automated checks that run on every push and PR — formatting, linting, compiling, and testing.
- **Upstream:** the original repository you forked from. "Upstream/main" refers to the main branch of the project you're contributing to, as opposed to "origin/main," which is your fork.
- **Review round:** one cycle of review feedback and author response. Complex PRs may go through 3–5 review rounds before merging.
- **Squash-merge:** combining all commits in a PR into a single commit on the main branch, keeping the git history clean. Most Rust projects require this.
- **Zulip:** the chat platform used by the Rust project for working-group discussions. Zulip's threaded model is designed for async, long-form technical discussion — unlike Discord, which is more real-time.

## Exercise

This module uses a worksheet format. Open `exercises/WORKSHEET.md` and work through the five prompts:

1. **Find 3 "Good First Issue" Candidates** — Search GitHub for Rust projects with beginner-friendly labels. Record the repo, issue title, summary, and why you picked it.
2. **Write an Approach Plan** — For one issue, write a detailed plan covering codebase navigation, reproduction, implementation strategy, testing, and potential blockers.
3. **Write a Sample PR Description** — Draft a full PR body following the conventions of mature Rust projects, with summary, changes, issue reference, testing instructions, and a checklist.
4. **List 5 Repository Conventions** — Identify the conventions (CONTRIBUTING.md, CI, code style, issue triage, testing) you must check before contributing, and explain why each matters.
5. **Summarize a Crate's Public API** — Pick a Rust crate on crates.io, read its `lib.rs`, and write a 250–400 word summary of its structure, key types, entry point, and one design decision.

There is no code to compile or tests to run. Write your answers in a separate file — these exercises are for you to practice the planning and communication skills that open source contribution requires. Sample answers are in `solutions/EXAMPLE_ANSWERS.md` for comparison.

## Further Reading
- [How to Contribute to Rust (rustc-dev-guide)](https://rustc-dev-guide.rust-lang.org/) — if you're considering contributing to the compiler.
- [Open Source Guide: How to Contribute](https://opensource.guide/how-to-contribute/) — a language-agnostic primer on the contribution workflow.
- [Google Engineering Practices: Code Review](https://google.github.io/eng-practices/review/) — what reviewers look for, from the reviewer's perspective.
- [The Rust Community Discord](https://discord.gg/rust-lang-community) — for real-time questions; the `#beginners` and `#contribute` channels are good places to ask.
