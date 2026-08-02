# Contributing

Thanks for wanting to improve Rust.Stack. This is a teaching repo, which
means the bar for changes is a little different from a typical open-source
project: the curriculum is strictly linear, content must be self-contained,
and — above all — **everything must compile and pass tests** at all times.

## What we accept

- **New or improved module content** — README prose, exercise scaffolding, or
  solution implementations.
- **Bug fixes** — a test that doesn't fail against the scaffold, a solution
  that doesn't pass, a clippy warning, a broken link.
- **New capstones or exercises** that fit the existing curriculum.

What we do *not* accept: branching "choose your track" content (the
curriculum is linear by design), hidden solutions (solutions are always
visible next to exercises), or build steps that require more than the pinned
toolchain.

## Module structure (required)

Every module follows this shape exactly:

```
modules/module-XXX-slug/
├── README.md                    # Required sections, in order:
│                                #   Block, Estimated time, Prerequisites,
│                                #   Learning Objectives, Why This Matters,
│                                #   Concept (800–1500 words), Common Pitfalls,
│                                #   Key Terms, Exercise, Further Reading
├── exercises/
│   ├── Cargo.toml               # name = "module-XXX-exercises", edition 2021
│   ├── src/lib.rs               # scaffolding + TODO(module-XXX) comments
│   └── tests/module_XXX.rs      # tests that fail on the scaffold, pass on the solution
└── solutions/
    ├── Cargo.toml               # name = "module-XXX-solutions"
    ├── src/lib.rs               # complete reference implementation
    └── tests/module_XXX.rs      # identical tests, all passing
```

Capstones follow the same pattern with `capstones/capstone-NN-slug/` and
`starter/` + `solution/` crates (package names `capstone-NN-starter` and
`capstone-NN-solution`). Worksheet-style modules (conceptual/career content)
use `exercises/WORKSHEET.md` + `solutions/EXAMPLE_ANSWERS.md` instead of
crates — nothing else does.

Naming: module folders are zero-padded to 3 digits, capstone folders to 2
digits. Never introduce a module outside `modules/` or a crate that isn't a
workspace member.

**Tip:** `./scripts/new_module.sh <number> <slug>` scaffolds a correct module
folder from the template.

## Content guidelines

- Concept sections are self-contained prose (800–1500 words) — a learner
  should not need external docs to complete the exercise, though further
  reading links are encouraged.
- Every code example compiles. Deliberately-broken examples are labeled
  ```rust,ignore with a "this will not compile" callout and the fix shown.
- No `todo!()` / `unimplemented!()` in committed code — clippy flags them and
  the workspace builds with `-D warnings`. Use panic-based stubs or
  placeholder return values in scaffolds.
- Exercises compile but fail their tests; solutions pass. Never commit a
  scaffold that fails to compile (except the explicitly-flagged
  "make it compile" exercises in the earliest modules).

## The acceptance bar for every PR

Anything you touch must be clean:

```bash
cargo fmt --check
cargo clippy --workspace -- -D warnings
cargo test --workspace
```

For a single module, `./scripts/verify_module.sh <XXX>` covers all three
scoped to that module's crates.

## Process

1. Open an issue or PR against the repo describing your change.
2. Add or update tests and solutions for whatever you change.
3. Run the three commands above; CI runs them again on push.
4. If your PR touches the curriculum map or adds a module, update the root
   `README.md` curriculum map and checkbox list so every link resolves.

Keep PRs focused: one module (or one kind of fix) per PR makes review
possible. Large multi-block additions should be proposed as a plan first.
