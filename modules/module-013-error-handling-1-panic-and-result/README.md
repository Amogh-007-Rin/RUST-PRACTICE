# Module 013: Error Handling I — `panic!` and `Result<T, E>`

**Block:** Block B — Foundations II
**Estimated time:** 45–90 min
**Prerequisites:** Module 008 (`Option<T>`, `match`), Module 011 (`Vec`), Module 012 (`HashMap`)

## Learning Objectives

- Explain what `panic!` does to the program, including the stack-unwinding mechanism, and when it is the right tool.
- Read and write `Result<T, E>` values: `Ok`, `Err`, `match`, and the convenience methods (`unwrap`, `expect`, `unwrap_or`, `map_err`, `ok_or`, `is_ok`, `is_err`).
- Decide, per failure, whether a panic or a `Result` is the appropriate design.
- Convert `Option`-based lookups into `Result`-based errors with `ok_or` and `ok_or_else`.
- Return real `std::io::Error` values from file operations without panicking.

## Why This Matters

Every program fails sometimes: a file is missing, input is malformed, a network call times out. How a codebase handles failure decides whether bugs surface as loud, debuggable crashes or silent data corruption. `Result<T, E>` is the backbone of every Rust API you will use from here on — `std::fs`, `serde_json`, and later `sqlx` and `axum` all return `Result`s, and Rust's error philosophy (make failures explicit, recoverable errors typed, unrecoverable ones loud) is what makes the error UX of Rust CLIs and services so much better than silent null-pointer land. Module 014 builds `?` and custom error types on top of what you learn here.

## Concept

Rust has two very different answers to "something went wrong":

1. **`panic!`** — something is so broken the program cannot continue; crash loudly.
2. **`Result<T, E>`** — a recoverable failure you can detect, inspect, and respond to.

Choosing between them is a design decision, and the first half of idiomatic Rust error handling is learning to make it correctly.

### `panic!`: crashing on purpose

A panic is Rust's "this is impossible / this invariant was violated" signal. The most common panic you've already met: indexing a `Vec` out of bounds. When a panic happens, the runtime **unwinds the stack**:

```
thread main
+-------------------------+       1. `main` calls `do_work()`
| main                    |       2. `do_work` calls `lookup()`
|   calls do_work()       |       3. `lookup` executes `v[i]` with a
+-------------------------+          bad index -> panic! is raised
| do_work                 |       4. unwinding starts: the runtime walks
|   calls lookup()        |          the stack, running destructors
+-------------------------+          (Drops) of every live variable
| lookup                  |       5. `lookup` frame cleaned up,
|   v[i]  -> PANIC!       |          unwinding continues
+-------------------------+       6. `do_work` frame cleaned up
        ...                |       7. no catch point: thread prints the
+-------------------------+          panic message and aborts
```

Because every local value you created (including your `Vec`'s heap buffer) is a Rust value with a destructor, unwinding **runs `Drop` along the way** — no memory leaks, no orphaned file handles — and then the thread prints the panic message and terminates. That message is gold in debugging:

```
thread 'main' panicked at src/main.rs:4:5:
index out of bounds: the len is 3 but the index is 99
```

You can trigger this deliberately:

```rust
fn reject_empty(items: &[i32]) -> &i32 {
    if items.is_empty() {
        panic!("cannot process an empty list");
    }
    &items[0]
}

let first = reject_empty(&[]); // panics with your custom message
```

Use `panic!` for **invariants and programmer errors** — the empty-list case a caller should never hand you, a `match` arm that is logically unreachable, a pre-condition you promised was true. Do not use it for things that can legitimately happen at runtime, like bad user input or a missing file: those need `Result`.

### `Result<T, E>`: failures as values

`Result` is an enum with two variants:

```rust
enum Result<T, E> {
    Ok(T),   // success, carrying the value
    Err(E),  // failure, carrying the error
}
```

It is a *value* like any other: it can be returned, stored, passed around, and matched. There's nothing magical about it — `main` can't "throw" it, but it makes failure visible in the type system:

```rust
use std::fs;

fn read_config(path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(path)
}

match read_config("config.toml") {
    Ok(contents) => println!("loaded {} bytes", contents.len()),
    Err(error) => eprintln!("could not read config: {error}"),
}
```

The signature says "this might fail, and if it does, here is an `io::Error` describing how." The caller *must* deal with both cases — the compiler won't let you silently ignore the error. That alone fixes an entire class of bugs from languages where forgetting to check a return code is a runtime surprise.

### Matching and the convenience methods

`match` is the honest, always-available way:

```rust
fn describe(result: Result<u32, String>) -> String {
    match result {
        Ok(value) => format!("got {value}"),
        Err(message) => format!("failed: {message}"),
    }
}
```

For the common cases, `Result` ships short-cuts:

```rust
let ok: Result<u32, String> = Ok(42);
let err: Result<u32, String> = Err("nope".to_string());

assert_eq!(ok.unwrap(), 42);          // panics on Err
assert_eq!(err.unwrap_or(0), 0);      // fallback value on Err
assert!(ok.is_ok());
assert!(err.is_err());
```

`unwrap` and `expect` are the "panic on failure" escape hatches:

```rust
let value = ok.expect("the math is guaranteed to succeed here");
```

`expect` panics with your message on `Err`, and `unwrap` panics with the default message. Both are fine when the `Err` case is genuinely impossible by design (an invariant), and they are the idiomatic panic-in-disguise. But on a function that parses user input, `input.parse::<u32>().unwrap()` is a crash waiting for a bad keystroke — that's the exact mistake this module is teaching you to avoid.

### `Option` → `Result` bridges

You'll constantly meet `Option` where an error message would be more useful. Two one-liners convert:

```rust
let items = [10, 20, 30];

let found: Result<&i32, &str> = items.get(1).ok_or("index out of bounds");
let computed: Result<&i32, String> = items.get(5).ok_or_else(|| "too far".to_string());
```

`ok_or` takes a ready-made error; `ok_or_else` takes a closure, so the error is only constructed when it's actually needed (avoids waste in the hot path). `map_err` does the reverse direction — it transforms the error type *inside* the `Err`:

```rust
fn parse_port(s: &str) -> Result<u16, String> {
    s.parse::<u16>().map_err(|_| format!("not a valid port: {s}"))
}

assert_eq!(parse_port("8080"), Ok(8080));
assert!(parse_port("web").is_err());
```

### The decision table

| Situation | Tool |
|---|---|
| Pre-condition violated, impossible input, code bug | `panic!` (or `expect`) |
| Bad user input, missing file, parse failure — *recoverable* | `Result<T, E>` |
| Lookup that may miss (`Vec::get`, `HashMap::get`) | `Option` + `ok_or` to make it a `Result` |
| A function that itself does I/O | Return `std::io::Error` (or `Result` of it) |

The rule of thumb from the standard library: **`panic!` is for bugs, `Result` is for foreseeable failures.** A password that fails validation is foreseeable. A config file with `"lvl": "high"` instead of `"high"` is foreseeable. An empty database cursor when your code guarantees one row is a bug — that one may panic.

## Common Pitfalls

- **Using `unwrap` on user-provided data.** `s.parse::<u32>().unwrap()` crashes on any typo. Fix: return a `Result` and `map_err` the parse error.
- **Panicking when a `Result` would do.** Writing `panic!` for bad input means the whole process dies on something the caller could have handled. Fix: return `Result<T, String>` and let the caller decide.
- **Forgetting that `unwrap` consumes the `Result`.** After `let v = result.unwrap();` the original `Result` is gone — you can't inspect it afterwards. Fix: match or `expect` at the right spot, or clone first.
- **`ok_or` vs `ok_or_else`.** `ok_or(build_error())` builds the error even on the success path. Fix: use `ok_or_else(|| ...)` when building the error is nontrivial.
- **Ignoring `Result` with a discard.** `let _ = fs::write(...)`? No — `let _ = result;` suppresses the `must_use` warning, and a failed write silently does nothing. Fix: match it, `?` it (Module 014), or at least log the `Err`.

## Key Terms

- **Panic:** an unrecoverable error that unwinds the stack, runs destructors, prints a message, and aborts the thread.
- **Unwinding:** the runtime process of walking back up the call stack, running `Drop` for each live value.
- **`Result<T, E>`:** an enum encoding success (`Ok(T)`) or failure (`Err(E)`) as a value.
- **`unwrap` / `expect`:** panic-on-`Err` convenience methods; `expect` adds your message.
- **`ok_or` / `ok_or_else`:** convert `Option<T>` into `Result<T, E>` by supplying the error for the `None` case.
- **`map_err`:** transform the error inside `Err` without touching the `Ok` value.
- **Recoverable failure:** an error the program is designed to handle and continue past.

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-013)` bodies:

1. `check_grade` — return `Ok`/`Err` based on a threshold.
2. `safe_divide` — reject division by zero with an error instead of panicking.
3. `parse_stock_quantity` — `parse` + `map_err` into a helpful `String`.
4. `nth_item` — safe indexing with `get` + `ok_or`.
5. `read_first_line` — wrap `fs::read_to_string`, extract the first line, propagate the `io::Error`.

The tests in `tests/module_013.rs` define "done":

```bash
cargo test -p module-013-exercises
```

Compare with `solutions/` only after you've made a genuine attempt. Note that Module 014 will show you the `?` operator that removes most of the boilerplate you see here.

## Further Reading

- [The Rust Book, Chapter 9 — Error Handling (panic!, Result, unwrap/expect)](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [std::result::Result — the full method list](https://doc.rust-lang.org/std/result/enum.Result.html)
- [Rust RFC 2361 — "panic!" vs "Result" design guidance (unwinding section)](https://rust-lang.github.io/rfcs/2361-relax-struct-unsize.html)
- [Rust: "Error Handling" blog post by Joe Duffy (why Result-style errors win)](https://joeduffyblog.com/2016/02/07/the-error-model/)
