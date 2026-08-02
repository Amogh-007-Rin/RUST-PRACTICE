# Module 009: Modules, Crates & Packages

**Block:** Block A — Foundations I
**Estimated time:** 60–120 min
**Prerequisites:** Module 008 (enums, matching); Module 001 (crates, `cargo`)

## Learning Objectives

- You will be able to explain the difference between a *package*, a *crate*, and a *module* — the three levels of Rust's code organization.
- You will be able to declare child modules with `mod`, both inline and in separate files.
- You will be able to control visibility with `pub` and know the default (private) rule.
- You will be able to import items with `use` and paths (`crate::`, `super::`, `self::`).
- You will be able to explain what a workspace is and how this repo itself is organized as one.

## Why This Matters

Real Rust projects are never one file. A backend service has `handlers/`, `models/`, `db/`, `errors/`; a library has a public API surface and private internals. Module 009 is where you learn the actual syntax of that organization — `mod`, `pub`, `use` — and it's also the *last* syntax module of Block A: everything from here on (collections, errors, traits, lifetimes) assumes you can navigate a multi-file crate. Capstone 01 puts it to work immediately: a lib crate with a `main.rs` binary on top.

## Concept

### The three levels of organization

Rust organizes code at three levels, each *contained* in the previous:

```text
package            a folder with a Cargo.toml        e.g. module-009-exercises/
  └── crate        a unit of compilation             one lib (src/lib.rs)
  │                                                 (+ optional binaries src/main.rs)
  │    └── module  a named namespace of items        math, utils, ...
  │         └── item                                 fn add, struct Book, enum Command
```

- **Package** — the folder with a `Cargo.toml`. One package can hold several crates (a library plus any number of binaries).
- **Crate** — a unit of compilation: `src/lib.rs` (library crate) or `src/main.rs` (binary crate). The crate root is where the compiler starts.
- **Module** — a named section inside a crate: `mod math { ... }`. Modules give items a path (`math::add`) and a privacy boundary.

When you `cargo build`, the compiler compiles the package's crates; each crate's *crate root* (its `lib.rs` or `main.rs`) is where its module tree starts.

### Declaring modules

A module is declared with `mod`, either inline or in its own file. Inline, a module is just a named block:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

fn main() {
    println!("{}", math::add(2, 3));
}
```

But modules really shine in separate files. The convention: `mod math;` in `lib.rs` tells the compiler "the module `math` lives in `src/math.rs`":

```text
src/
├── lib.rs      declares:  mod math;  mod utils;
├── math.rs     defines everything in the `math` module
└── utils.rs    defines everything in the `utils` module
```

which in `lib.rs` looks like two plain lines: `pub mod math;` and `pub mod utils;`. (The `pub` marks the modules as reachable from outside the crate root — the exercise crate relies on that for its tests.) Bigger projects nest modules in folders (`src/handlers/auth.rs` declared as `mod handlers { pub mod auth; }` in a `handlers/mod.rs`), but the rule is always the same: **`mod name;` connects a file to a module path.**

### Privacy: everything is private by default

This is the rule that makes modules a *security boundary*, not just a folder structure: an item is visible only inside its module **unless** marked `pub`. The whole point is that a library exposes a small public API and keeps everything else internal — you can change internals freely without breaking users.

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn secret_helper() -> i32 {
        42
    } // NOT pub: only visible inside `math`
}

fn main() {
    // println!("{}", math::secret_helper()); // compile error: private function
    println!("{}", math::add(2, 3)); // fine: `pub`
}
```

Privacy is per-module, not per-file: a private item is visible to its own module and its descendants, not to siblings or parents. The compiler message for a privacy violation tells you exactly which path is off-limits.

### `pub(crate)` and graded visibility

`pub` has gradations. `pub(crate)` makes an item visible anywhere in the crate but *not* outside it — the right level for internal helpers a library wants to share across its own files without exposing. `pub(super)` narrows visibility to the parent module only. The rule of thumb: start private, widen only as far as needed. Everything at the crate boundary with `pub` is your public API — in real libraries, changing it is a semver event, so Rust code keeps the boundary as small as possible.

### Paths and `use`

Every item has a **path**. From inside the crate, items are reached with `crate::` (the crate root), `self::` (the current module), or `super::` (the parent module):

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod calculator {
    pub fn add_three(a: i32, b: i32, c: i32) -> i32 {
        super::math::add(super::math::add(a, b), c)
        // ^ parent module's items via `super::`
    }
}

fn main() {
    // from the crate root, the full path starts at `crate::`
    let sum = crate::math::add(2, 3);
    println!("{sum}");
}
```

Typing `crate::math::add` every time gets old — that's what `use` is for. `use` imports a path so you can use a short name:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

use math::add;

fn main() {
    println!("{}", add(2, 3)); // `add` now in scope
}
```

Common idioms you'll meet constantly:

```rust
use std::collections::HashMap;   // importing an item into scope

mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

use math::add;                   // importing an item into scope

fn main() {
    println!("{}", add(2, 3)); // `add` now in scope
    let _map = HashMap::<u32, u32>::new();
}
```

Importing several items at once works too: `use math::{add, sub};`. A glob import (`use math::*;`) pulls everything in — handy in small projects, but it erases "where did this come from", so use it sparingly. And note `use` only *aliases* — it doesn't move or copy anything.

The `super::` idiom appears when one module uses another's items:

```rust
mod math {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}

mod utils {
    pub fn shout(s: &str) -> String {
        format!("{}!", s.to_uppercase())
    }
}

fn main() {
    let sum = math::add(2, 3); // sibling modules are reached from the crate root
    println!("{}", utils::shout(&sum.to_string()));
}
```

### Libraries vs binaries: the layering pattern

A package can have both a library crate and a binary crate. The library holds the logic (with `pub` marking the API); the binary is a thin shell that calls it. Capstone 01 uses exactly this pattern: `src/lib.rs` (the `ContactBook`) and `src/main.rs` (argument parsing + printing). The binary imports the library with `use capstone_01_starter::...` — the package name with hyphens translated to underscores.

Why split at all? Because *tests* import the library (that's what this repo's `tests/` files do), because other crates can reuse it, and because the logic becomes testable without spawning a process. "Thin binary, fat library" is the rule of thumb in professional Rust.

### Workspaces: many packages, one build

The last layer: a **workspace** groups several packages so they share one lockfile, one `target/` directory, and one command (`cargo test --workspace`). This repo *is* a workspace — the root `Cargo.toml` lists every module's `exercises/` and `solutions/` (plus capstone crates) as members. That's why you can run `cargo test -p module-009-exercises` from the repo root: `-p` targets one package inside the workspace. You'll build and manage workspaces yourself in Module 039's cargo deep dive; for now, knowing that this layout is *a workspace* explains the `-p` flags in every command in this course.

### The exercise: a three-file crate

`exercises/src/` has three files. Your TODOs are deliberately spread across them so you practice the whole picture:

1. `math.rs` — implement `add`, `sub`, `mul` (these are `pub`; siblings stay private).
2. `utils.rs` — implement `shout` and `is_blank`.
3. `lib.rs` — implement `shout_sum`, which *imports* `utils::shout` via `use` and calls `math::add`.

The integration tests call `math::add(2, 3)`, `utils::shout("hi")`, and `shout_sum(2, 3)` — notice the paths in the tests are exactly the module paths you built.

## Common Pitfalls

- **Forgetting `pub`.** Everything is private by default. If a test can't see your function, the fix is `pub`, not rearranging files.
- **Declaring `mod math;` but naming the file wrong.** The file must be `src/math.rs` (or `src/math/mod.rs` for nested). A missing file is a compile error at the `mod` declaration.
- **`use` inside the wrong module.** `use` brings names into *that module's* scope. `use utils::shout;` in `lib.rs` doesn't make `shout` visible in `math.rs`.
- **Confusing `crate` with `cargo`.** `crate::` is the path prefix for the crate root; `cargo` is the build tool. Different words, different jobs.
- **Glob imports everywhere.** `use x::*;` erases the "where did this come from" information — import items explicitly.

## Key Terms

- **package:** a folder with a `Cargo.toml`; may contain several crates.
- **crate:** a unit of compilation (`lib.rs` or `main.rs`); the crate root starts its module tree.
- **module:** a named namespace and privacy boundary inside a crate (`mod math`).
- **crate root:** the top-level module of a crate, in `lib.rs` or `main.rs`.
- **`pub`:** makes an item visible outside its module; the default is private.
- **path:** how you name an item — `crate::math::add`, `super::add`.
- **`use`:** imports a path into scope so you can use a short name.
- **workspace:** a set of packages sharing one build (`cargo test --workspace`).

## Exercise

In `exercises/`:

1. Open the crate and note the file layout: `lib.rs` declares `pub mod math; pub mod utils;`, and each module lives in its own file.
2. In `src/math.rs` implement `add`, `sub`, `mul`.
3. In `src/utils.rs` implement `shout` and `is_blank`.
4. In `src/lib.rs` implement `shout_sum` — add the `use utils::shout;` import yourself, then combine `math::add` with `shout`.
5. Run `cargo test -p module-009-exercises` until all 7 tests pass.
6. Compare with `solutions/` afterwards — note the identical file layout.

## Further Reading

- [The Rust Book, Chapter 7: Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html) — the definitive chapter on all three levels.
- [The Rust Book, Chapter 7: Bringing Paths into Scope with `use`](https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html) — `use`, aliases, re-exports.
- [The Cargo Book: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) — how this repo's root `Cargo.toml` ties the modules together.
