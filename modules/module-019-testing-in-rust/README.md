# Module 019: Testing in Rust

**Block:** Block B — Foundations II
**Estimated time:** 60–90 min
**Prerequisites:** Module 013 (`Result`), Module 015 (generics), Module 016 (traits)

## Learning Objectives

- Write unit tests with `#[test]` and assert behavior with `assert!`, `assert_eq!`, and `assert_ne!`.
- Distinguish unit tests (in `src`, testing private functions) from integration tests (in `tests/`, testing the public API).
- Run and filter tests with `cargo test`: by name, with `-- --nocapture`, and understand the pass/fail output format.
- Use `#[should_panic]`, `#[ignore]`, and `#[cfg(test)]` correctly.
- Structure test code with helper functions and `mod tests` — the pattern you'll see in every real crate.

## Why This Matters

This is the module that makes everything else *verifiable* — and it's the last piece of the exercise machinery you've been using since Module 001. Every `cargo test -p module-XXX-exercises` you've run has been Rust's built-in test harness. From here on, you'll write tests as part of every module, and Capstone 02's acceptance criteria are a test suite. In industry, Rust's standard-library test harness is a big selling point: no separate test framework needed, tests live next to the code, and CI runs them everywhere (this repo's own CI runs `cargo test --workspace`).

## Concept

### What `cargo test` does

When you run `cargo test`, cargo builds your crate in test mode and runs three kinds of things:

1. **Unit tests** — `#[test]` functions inside `src/` (usually in a `#[cfg(test)] mod tests`).
2. **Integration tests** — every file in `tests/`, compiled as its own crate against your public API.
3. **Doc tests** — code blocks in `///` doc comments (they're compiled and run as small programs).

Each `#[test]` function runs in its own thread. A test passes if it returns normally; it fails if it panics (an assertion failing *is* a panic — you met the mechanics in Module 013). The harness reports a summary like:

```
running 12 tests
test fahrenheit_freezing_point_is_zero_celsius ... ok
test is_palindrome_rejects_non_palindromes ... ok
...
test result: ok. 12 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

A failing test shows the assertion that broke and the file/line:

```
thread 'word_count_counts_whitespace_separated_words' panicked at tests/module_019.rs:6:5:
assertion `left == right` failed
  left: 11
 right: 2
```

`left`/`right` are the two arguments of `assert_eq!` — this is the "test defines done" loop you'll use for the rest of the repo.

### Writing your first tests

```rust
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn adds_positive_numbers() {
        assert_eq!(add(2, 3), 5);
    }

    #[test]
    fn adds_negatives() {
        assert_eq!(add(-1, -1), -2);
    }
}
```

Three things to unpack:

- `#[test]` marks a function as a test. Any function with this attribute is collected by the harness.
- `use super::*;` brings the parent module's items (including private ones) into the test module — unit tests can reach private internals, which is their superpower.
- `#[cfg(test)]` makes the module compile *only* under `cargo test` — so test code never ships in a release build.

### The assertion macros

```rust
assert!(condition);              // true or fail (no value shown)
assert_eq!(left, right);         // left == right, prints both on failure
assert_ne!(left, right);         // left != right
```

Variants with messages: `assert!(x, "x must be positive, was {x}")` — the message is a format string, handy for debugging output. Assertions are just a `panic!` with a nice message, so "tests fail when a panic occurs" and "tests fail when an assertion breaks" are the same mechanism.

### Unit vs integration tests

```
crate root:  src/lib.rs
+-------------------------------+
| pub fn parse_port(s) ...      |   <- public API
| #[cfg(test)] mod tests {      |   <- UNIT tests: same crate, can touch
|     use super::*;             |      private items directly
|     #[test] fn ...            |
| }                             |
+-------------------------------+

tests/module_019.rs              <- INTEGRATION tests: separate crate,
+-------------------------------+    links the library, only public API
| use module_019_exercises::... |
| #[test] fn ...                |
+-------------------------------+
```

Integration tests can only call `pub` items — they treat your library like any external user would. That's why this repo's exercises put the spec in `tests/module_XXX.rs`: it checks the *public contract*. Unit tests live in `src` and are for internals. Rule of thumb: test the internals with unit tests, test the public behavior with integration tests, and don't duplicate the same assertions in both.

### Controlling the run

```bash
cargo test                      # everything (unit + integration + docs)
cargo test -p module-019-exercises   # just this package (workspace-aware)
cargo test word_count           # only tests with "word_count" in the name
cargo test -- --nocapture       # show println! output from tests
cargo test -- --ignored         # only the #[ignore]d tests
```

One-off and slow tests get `#[ignore]`:

```rust
#[test]
#[ignore = "runs a live network request; use with --ignored"]
fn contacts_the_api() { /* ... */ }
```

### Testing failures: `#[should_panic]`

Sometimes the correct behavior *is* to panic — a precondition violation, an invariant (Module 013). `#[should_panic]` asserts that:

```rust
fn first_element(items: &[i32]) -> &i32 {
    assert!(!items.is_empty(), "empty slice has no first element");
    &items[0]
}

#[test]
#[should_panic(expected = "empty slice has no first element")]
fn first_element_rejects_empty_slices() {
    let _ = first_element(&[]);
}
```

The test passes if the function panics (with a message containing `expected`). The library version of this is `Result::unwrap` on an `Err` — panic is a behavior, and behaviors get tested.

### Test organization that scales

Real crates follow a small set of patterns:

- One `#[cfg(test)] mod tests` per source module, or one big `mod tests` at the bottom of a small file.
- Helper functions shared across tests (like a `build_fixture()` that constructs a populated `HashMap`), defined in the test module — they don't count as tests, they just don't have `#[test]`.
- Integration test files grouped by area: `tests/crud.rs`, `tests/persistence.rs`.
- Shared test utilities in `tests/common/mod.rs` (a directory module is *not* compiled as a test file — only `tests/common.rs` would be).

## Common Pitfalls

- **Naming the test file `common.rs` when you mean a helper.** Any file directly in `tests/` is treated as a test crate. Fix: put shared helpers in `tests/common/mod.rs` (a directory).
- **Testing only the happy path.** A function that passes every test but panics on empty input is untested. Fix: add edge-case tests (empty, zero, boundaries).
- **`assert_eq!` on `f64`.** `assert_eq!(mean(&[1.0, 2.0]), 1.5)` — floating point rarely equals exactly. Fix: `assert!((a - b).abs() < 1e-9)`, or compare with a tolerance helper.
- **Forgetting `#[cfg(test)]`.** Test modules without it get compiled into your shipped library. Fix: always gate `mod tests` with `#[cfg(test)]`.
- **Tests that depend on each other.** Test order is not guaranteed (they run in parallel threads). Fix: each test sets up its own data — never share mutable global state.
- **Debug-print debugging inside tests.** `println!` output is hidden by default. Fix: run with `-- --nocapture`, or better, assert what you're inspecting.

## Key Terms

- **Test harness:** the built-in runner that `cargo test` invokes.
- **Unit test:** a `#[test]` in `src`, able to reach private items.
- **Integration test:** a `#[test]` in `tests/`, linked against the public API only.
- **Doc test:** a runnable code block in a `///` comment.
- **`#[cfg(test)]`:** compile only when testing.
- **`#[should_panic]`:** asserts that a test's code panics.
- **`#[ignore]`:** skip by default; run with `-- --ignored`.
- **Fixture:** setup code shared by several tests.

## Exercise

Open `exercises/src/lib.rs`. Every function has a deliberate bug, and the integration tests in `tests/module_019.rs` spell out the correct behavior:

1. Fix `fahrenheit_to_celsius`, `word_count`, `is_palindrome`, `fibonacci`, and `valid_grade`.
2. Then write **unit tests** in the `#[cfg(test)] mod tests` block — the integration tests define correctness, but this is your chance to practice the unit-test pattern from the Concept section.

The tests in `tests/module_019.rs` define "done":

```bash
cargo test -p module-019-exercises
```

Compare with `solutions/` only after you've made a genuine attempt — the solution shows a set of unit tests for comparison.

## Further Reading

- [The Rust Book, Chapter 11 — Writing Automated Tests](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [cargo book — the `cargo test` reference](https://doc.rust-lang.org/cargo/commands/cargo-test.html)
- [Rust Reference — the `test` attribute and test harness](https://doc.rust-lang.org/reference/attributes/testing.html)
- [std::assert — the assertion macro family](https://doc.rust-lang.org/std/macro.assert.html)
