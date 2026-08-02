# How to Use This Repo

Rust.Stack is a strictly linear curriculum: **Module 000 → Module 100**, with a
capstone project landing at the end of every 10-module block. There are no
branching tracks — depth on each specialization comes from dedicated blocks,
not learner choice. You can work through everything with `git clone`, a text
editor, and a Rust toolchain. No website, no build step, no sign-up.

---

## The Module Pattern

Every module folder (`modules/module-XXX-slug/`) has the same shape:

```
module-XXX-slug/
├── README.md            # The lesson: concept, pitfalls, exercise instructions
├── exercises/           # The exercise crate — broken/incomplete on purpose
│   ├── Cargo.toml
│   ├── src/lib.rs       # Scaffolding with TODO comments
│   └── tests/module_XXX.rs  # Integration tests that define "done"
└── solutions/           # The reference implementation — always visible
    ├── Cargo.toml
    ├── src/lib.rs       # Fully working code
    └── tests/module_XXX.rs  # Same tests, all passing
```

### Working through a module

1. Read the module's `README.md` **top to bottom**. The Concept section is
   self-contained — you shouldn't need the Rust Book to do the exercise
   (though further-reading links are provided).
2. Open `exercises/` in your editor and find the `// TODO(module-XXX)` comments.
3. Implement the TODOs until the tests pass.
4. When you're done, check your work:

   ```bash
   cargo test -p module-XXX-exercises
   ```

5. Compare against `solutions/` only after you've made a genuine attempt —
   that's where the learning happens. The solution is the same code with the
   TODOs filled in.

> **About the solutions folder:** it's sitting right next to the exercise,
> not hidden. We trust you to use it responsibly. Attempt first; peek when
> stuck; read it after finishing to see a reference approach. There is no
> cheating in a sandbox.

### Worksheet-style modules

A few conceptual/career modules (000, 089, 096, 097) swap the Cargo crate for
a guided written exercise: `exercises/WORKSHEET.md` with prompts, and
`solutions/EXAMPLE_ANSWERS.md` with model answers. Work through the prompts in
writing, then compare.

---

## Capstones

At the end of every block (after modules 010, 020, … 100) there's a larger
project in `capstones/capstone-NN-slug/`:

```
capstone-NN-slug/
├── README.md       # Project brief, requirements, acceptance criteria
├── starter/        # Scaffolding with TODOs — start here
└── solution/       # Reference implementation
```

Capstones integrate everything from the preceding 10 modules. Treat them like
take-home assignments: read the brief, implement in `starter/` until the
acceptance criteria pass, then compare with `solution/`.

```bash
cargo test -p capstone-NN-starter
```

---

## The Testing Workflow

Every exercise defines "done" as a passing integration test. The tests
**must fail** against the unmodified scaffold (that's the point — you make
them pass), and they must pass against the solution.

Common commands:

| Command | What it does |
|---|---|
| `cargo test -p module-XXX-exercises` | Run one module's exercise tests |
| `cargo test -p module-XXX-solutions` | Run one module's solution tests |
| `cargo test -p capstone-NN-starter` | Run one capstone's starter tests |
| `./scripts/verify_module.sh XXX` | fmt + clippy + test for one module (both crates) |
| `./scripts/check_progress.sh` | Parse the root README checklist and report progress |
| `cargo test --workspace` | Run everything (all modules + capstones) |

The whole repo is kept `cargo fmt`- and `cargo clippy`-clean with warnings as
errors. Before you submit a PR or finish a big chunk of work, run:

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

---

## FAQ

**Do I need to install anything besides Rust?** No. The pinned toolchain
(`rustup` + stable) is the only requirement. The handful of modules that can't
be exercised purely with `cargo test` (WASM, some embedded/wasm targets)
document their special commands in their own README — everything else runs on
a stock toolchain.

**I'm stuck on a module. What should I do?** 1) Re-read the Concept section.
2) Look at the specific TODO comment in context — the function signature and
the tests tell you almost everything. 3) Read the tests in
`exercises/tests/module_XXX.rs` — they specify the expected behavior exactly.
4) Look at the solutions. In that order.

**What if a module's tests pass but I didn't write idiomatic code?** The
exercise tests define correctness, not style. Compare your implementation with
the solution and try to internalize differences. Every exercise crate is kept
clippy-clean, so run `cargo clippy -p module-XXX-exercises -- -D warnings` on
your code too — a clean clippy pass is part of "done" in the real world.

**Can I skip the capstones?** Technically yes; they're all extra work. But
they exist because they're where the blocks of knowledge stick together. The
final capstone (Capstone 10) is explicitly the portfolio piece you point to in
job applications.

**How do I track my progress?** Check off modules in the root `README.md`
curriculum map, then run `./scripts/check_progress.sh` to get a completion
percentage.

**Is there a website or an interactive environment?** No — by design. This is
a plain git repository: `git clone` and go.
