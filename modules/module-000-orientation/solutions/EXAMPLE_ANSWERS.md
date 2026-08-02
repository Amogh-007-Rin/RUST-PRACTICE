# Module 000 Example Answers

> Model answers for every prompt in `exercises/WORKSHEET.md`. For the
> run-command prompts you'll see the expected output *shape* — your actual
> versions/numbers will differ. For the reflection prompts you'll see a good
> example answer — yours should be your own, but this is the standard of
> specificity to aim for.

---

## Prompt 1: Enter the repository and orient

At the top level you should see: `README.md` (the front door — pitch,
curriculum map with checkboxes, progress instructions), `Cargo.toml` (the
workspace manifest), `rust-toolchain.toml` (pins the stable toolchain),
`rustfmt.toml`/`clippy.toml` (style tooling config), `docs/` (setup and
how-to guides), `scripts/` (the helper scripts), `modules/` (one folder per
module, zero-padded: `module-000` … `module-100`), and `capstones/` (ten
capstone projects). In `modules/` you'll see the zero-padding convention in
action — `module-001`, `module-002`, … `module-100` — which is what keeps
plain alphabetical listing in the correct numeric order. Some modules may
still be scaffold placeholders at the time you're reading this; that's the
repo mid-build, and it doesn't block working through the finished ones.

## Prompt 2: Run `rustup show`

You should see something shaped like this:

```
Default host: x86_64-unknown-linux-gnu
rustup home:  /home/<you>/.rustup

installed toolchains
--------------------
stable-x86_64-unknown-linux-gnu (default)

active toolchain
----------------
stable-x86_64-unknown-linux-gnu (default)
rustc 1.XX.0 (abcdef012345 2026-XX-XX)
```

The key line is **active toolchain**: it's pinned by the `rust-toolchain.toml`
in this repository (`channel = "stable"` plus the `rustfmt` and `clippy`
components). That means every Rust command you run inside this repo uses the
same toolchain as the CI — you don't have to think about versions again.

## Prompt 3: Inspect the workspace manifest

The `members` globs are `modules/*/exercises`, `modules/*/solutions`,
`capstones/*/starter`, and `capstones/*/solution` — so every module's two
crates and every capstone's two crates are workspace members automatically.
The `exclude` list names the worksheet-style modules (000, 089, 096, 097) and
`module-100-final-capstone-support`. They're excluded because they have no
crate folders at all: `exercises/` holds a markdown worksheet, not a
`Cargo.toml`, and the globs would try to parse their folders as packages and
fail. Excluding them keeps `cargo test --workspace` valid. You'll also see
`edition = "2021"` inherited by every crate via `[workspace.package]`.

## Prompt 4: Dissect a module README

Working top to bottom through `modules/module-001-toolchain-and-first-program/README.md`
you should be able to identify every required element of the §3 template:
the `# Module 001:` title, `**Block:**` (Block A — Foundations I), `**Estimated
time:**`, `**Prerequisites:**`, then the sections Learning Objectives, Why
This Matters, Concept, Common Pitfalls, Key Terms, Exercise, and Further
Reading — in exactly that order. (If the README you opened still says
"under construction," you've correctly identified the template skeleton —
the section headers are the anatomy.) Every module 001–099 follows this same
skeleton, so once you know it you can navigate any module in the repo blind.

## Prompt 5: Find the TODO convention in Module 004's scaffold

The scaffold in `modules/module-004-ownership-part-1/exercises/src/lib.rs`
opens with the doc comment: *"Fill in the TODOs below so the integration
tests in `tests/` pass."* That's the convention: finished modules carry
`// TODO(module-XXX): <specific instruction>` comments at every point you
must implement, and the integration test file `tests/module_004.rs` is what
defines "done" — in the completed version of this module those tests will
fail against the unfinished scaffold and pass once you fill in the TODOs.
The point to internalize: **you always know what to do next (the TODO) and
when you're finished (the tests).** If a module's scaffold is still a
placeholder when you reach it, the same rule applies once its content lands.

## Prompt 6: Run the Module 004 solution tests

The command compiles the `module-004-solutions` crate and its test targets,
then runs the tests. You should see `Compiling module-004-solutions ...` and
a final line like:

```
test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

(`cargo test -p module-004-solutions` passes because the solutions crate is
the reference implementation — this is the state you're aiming to reach with
the exercise crate in every module.)

## Prompt 7: Run the per-module verification script

`./scripts/verify_module.sh 004` runs, in order:

1. `cargo fmt --check` for both crates
2. `cargo clippy -p module-004-exercises -p module-004-solutions -- -D warnings`
3. `cargo test -p module-004-exercises`
4. `cargo test -p module-004-solutions`

and finishes with the line `module-004: all checks passed.` That's the full
acceptance bar for a module, scoped to just those two crates — use it after
finishing any module's exercise. (For worksheet modules like 000 it prints a
"no crate packages found — nothing to verify" message instead, which is
expected, not an error.)

## Prompt 8: Check your progress

You should see:

```
0/101 modules complete, 0/10 capstones complete.
```

The script counts checked boxes (`- [x]`) in the root `README.md` curriculum
map, and you haven't checked any yet — correct. The reason to tick nothing
yet: the checkbox is a *record of completed work*, not a to-do item, and
`check_progress.sh` is only honest if the boxes reflect reality. Module 000
isn't done until you've written genuine answers to all 12 prompts — including
the reflections in Part 2. When you finish this worksheet, tick Module 000,
and let the number grow as your competence grows.

## Prompt 9: Define memory safety in your own words

A good answer captures *the property*, not just the bug list — for example:

> Memory safety means a program can only access memory it is currently
> entitled to access: every read and write lands inside a live, properly
> sized allocation, every reference points at valid data, memory is freed
> exactly once, and concurrent access to shared memory is synchronized.
> A memory-safe language makes violating these properties impossible in
> ordinary code.

If your definition names use-after-free, buffer overflows, and data races as
*examples* of unsafe code while stating the underlying property, you've got
it — that's the formulation the rest of this curriculum builds on.

## Prompt 10: List three things you can build with Rust

Good answers pair the *what* with a *why Rust specifically*. For example:

1. **CLI tools** — Rust produces binaries that start instantly with no
   runtime to boot, so tools like `ripgrep` feel native even in scripts.
2. **Backend services** — Rust's memory safety and lack of GC mean services
   with many concurrent connections use less memory and have no GC pauses,
   and `axum`/`tokio` give you a full production stack.
3. **Embedded firmware** — Rust's `no_std` mode runs on microcontrollers with
   no operating system, and the borrow checker catches memory bugs that C
   firmware discovers only in the field, where they're unreachable.

Any three of the list work — the point is you can now say *why* Rust is the
right tool for each, which is the difference between listing a language's
use cases and understanding them.

## Prompt 11: Pick your likely specialization

There's no wrong pick at this stage — this is a first hypothesis, not a
contract. A good answer reasons from your background, for example:

> I'm leaning toward **backend services**. I already know HTTP and REST from
> building APIs in another language, so Block G builds on what I know, and
> the payoff is tangible: a typed, fast, memory-safe API server is a
> concrete thing I can show in a portfolio. The async block before it (Block
> E) also feeds directly into backend work, so the sequence feels efficient.

Notice the structure: name the track, connect it to your existing
background, and justify it with the curriculum's shape. If you picked
embedded or game dev, do the same. Revisit in Module 089, where the choice
is made deliberately.

## Prompt 12: `cargo build` vs. `cargo test`

A strong paragraph covers both what each compiles and what each runs:

> `cargo build` compiles your crate and its dependencies and links them into
> a binary or library — it validates that the code is *well-formed*, but
> never executes it. `cargo test` compiles your crate plus its test targets
> (unit tests, integration tests, doc tests) into test binaries and actually
> *runs* them, reporting a pass/fail count per test. So `cargo build` answers
> "does this compile?", while `cargo test` answers "does this behave as the
> tests say it should?" — you can have a green build with failing tests, and
> a test run is what the exercises in this repo define as "done." In
> practice, run tests early and often, and keep the build green as a
> baseline sanity check.

---

**Done?** If your answers are comparable in substance (not identical in
wording), you've completed Module 000. Tick the Module 000 box in the root
`README.md` and continue to Module 001 — where you'll write your first real
Rust program.
