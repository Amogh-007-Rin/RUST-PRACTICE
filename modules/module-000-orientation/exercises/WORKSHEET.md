# Module 000 Worksheet: Repo Tour & Reflection

> No code to write here. This module teaches orientation, and the exercise is
> to *do* the orientation: walk the repository, run the tooling, and write
> down what you see and what you think. Answer every prompt in your own words
> before looking at `solutions/EXAMPLE_ANSWERS.md` — then compare.
>
> **Before you start:** you should already have a Rust toolchain installed
> (`rustup` + stable, per `docs/SETUP.md`). Everything in Part 1 happens
> inside the repository root.

---

## Part 1 — Repo tour

### Prompt 1: Enter the repository and orient

`cd` into the repository root (`Rust.Stack/`) and run `ls -F` to see the top
level, then `ls modules/ | head -20` to sample the module folders. Write down
what you see and what you think each top-level item is for.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 2: Run `rustup show`

Run `rustup show` in the repository root. Note the active toolchain and the
fact that it is picked up from `rust-toolchain.toml` in this repository.
Record the output shape (you don't need to copy it all).

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 3: Inspect the workspace manifest

Open `Cargo.toml` at the repository root. Identify the workspace `members`
globs and the `exclude` list. Why do you think the worksheet-style modules
(like this one, `module-000-orientation`) are *excluded* rather than listed
as members?

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 4: Dissect a module README

Open `modules/module-001-toolchain-and-first-program/README.md`. The
project spec (§3) defines a required README template: Title, **Block:**
(letter + theme), **Estimated time:**, **Prerequisites:**, then the sections
Learning Objectives, Why This Matters, Concept, Common Pitfalls, Key Terms,
Exercise, and Further Reading — in that order. List which of these required
sections you can identify in the Module 001 README, in the order they appear.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 5: Find the TODO convention in Module 004's scaffold

Open `modules/module-004-ownership-part-1/exercises/src/lib.rs` and
`modules/module-004-ownership-part-1/exercises/tests/module_004.rs`. Find
where the scaffold tells you what to do (the doc comment / TODO instructions),
and read the test file. Quote the instruction you find, and explain in one
sentence what the test file defines for this module.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 6: Run the Module 004 solution tests

Run `cargo test -p module-004-solutions` from the repository root. Record the
final `test result:` line. Then explain in one sentence what this command
actually did.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 7: Run the per-module verification script

Run `./scripts/verify_module.sh 004`. Write down the numbered checks the
script performs, in order, and the final line it prints.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 8: Check your progress

Run `./scripts/check_progress.sh`. Record its output. The root `README.md`
has a curriculum map of checkboxes — inspect it, and explain why you should
**not** tick the Module 000 box (or any box) before finishing this worksheet.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

## Part 2 — Written reflections

### Prompt 9: Define memory safety in your own words

You read about use-after-free, buffer overflows, and data races. Without
looking back at the README, write your own one-to-two-sentence definition of
"memory safety" — the property, not just a list of bugs.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 10: List three things you can build with Rust

Pick three things from the "what you can build" list in the README (CLIs,
backend services, embedded firmware, WASM/frontend, blockchain programs, game
engines, OS components). For each, write one sentence on *why Rust specifically*
is a good fit — not just what it is.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 11: Pick your likely specialization

The curriculum is strictly linear: no branching. But you will spend a whole
10-module block on one specialization, and Module 089 will revisit the choice.
Pick the one you lean toward *today* — backend, async/infra, systems/embedded,
CLI/networking, WASM/frontend, game dev, or blockchain — and write 2–3
sentences on why, drawing on your background and what you read in this module.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

### Prompt 12: `cargo build` vs. `cargo test`

Write a short paragraph (3–5 sentences) explaining the difference between
`cargo build` and `cargo test` — what each compiles, what each runs, and what
"passing" means for each.

Your answer:

<details><summary>Space for your answer</summary>



</details>

---

**Done?** Compare your answers with `solutions/EXAMPLE_ANSWERS.md`. When your
answers are genuine, tick the Module 000 box in the root `README.md` and move
on to Module 001.
