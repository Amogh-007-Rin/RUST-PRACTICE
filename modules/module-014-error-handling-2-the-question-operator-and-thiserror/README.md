# Module 014: Error Handling II — the `?` Operator, Custom Errors & `thiserror`

**Block:** Block B — Foundations II
**Estimated time:** 60–90 min
**Prerequisites:** Module 013 (`panic!` and `Result`), Module 007 (structs & methods), Module 016 will formalize the traits used here

## Learning Objectives

- Replace nested `match` error handling with the `?` operator and explain what it does to `Ok` and `Err` values.
- Define a custom error `enum` and implement `Display` + `Error` on it.
- Use `thiserror` to derive `Display`, `std::error::Error`, and automatic `From` conversions in one line per variant.
- Compose functions returning different error types (I/O, parsing, validation) into one function with a single error type.
- Know when `thiserror` (library code, typed errors) is the right tool versus `anyhow` (applications, opaque errors).

## Why This Matters

Module 013 gave you `Result` but left you with `match`-everywhere boilerplate: two functions returning different error types, chained with nested matches, is the classic Rust code smell that scares people off. Production Rust solves it in exactly two ways: **`thiserror`** for libraries that need typed, matchable errors (this is what `sqlx`, `serde_json`, and the crates you'll use in Blocks G and H do), and **`anyhow`** for application code that just wants to bubble errors up with context. Every real crate you read from here on — including Capstone 02 in this block — will have a custom error enum and a `?`-heavy style. This module is where error handling becomes pleasant.

## Concept

### The `?` operator: unwrap-or-return

Look at the shape Module 013 left you with:

```rust
use std::fs;

fn read_first_line(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(contents) => match contents.lines().next() {
            Some(line) => Ok(line.to_string()),
            None => Ok(String::new()),
        },
        Err(error) => Err(error),
    }
}
```

The `Ok`-arm hoists the value out; the `Err`-arm returns it unchanged. That exact pattern is so common it got an operator. `expr?` means:

```
expr: Result<T, E>
  |-- Ok(value)  -> the expression evaluates to `value` (type T)
  `-- Err(error) -> return Err(error) from the enclosing function
                    right now, converted if needed
```

So the function above becomes:

```rust
use std::fs;

fn read_first_line(path: &str) -> Result<String, std::io::Error> {
    let contents = fs::read_to_string(path)?;
    Ok(contents.lines().next().unwrap_or("").to_string())
}
```

One crucial rule: **`?` only works in a function whose return type is `Result`** (or `Option`). The compiler enforces it — a `?` inside a function returning `()` is a compile error. That's the point: `?` makes "I cannot continue without this value" explicit in the signature.

### The conversion superpower

`?` doesn't just unwrap — it *converts*. `read_to_string` returns `Result<String, io::Error>`, but if your function returns `Result<String, ConfigError>` and `ConfigError` implements `From<io::Error>`, the `Err(io::Error)` is converted automatically before returning. This is what lets you chain operations with different error types:

```rust
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
struct ConfigError {
    message: String,
}

impl From<std::io::Error> for ConfigError {
    fn from(error: std::io::Error) -> Self {
        ConfigError { message: format!("i/o: {error}") }
    }
}

impl From<ParseIntError> for ConfigError {
    fn from(error: ParseIntError) -> Self {
        ConfigError { message: format!("parse: {error}") }
    }
}

fn load_port(path: &str) -> Result<u16, ConfigError> {
    let contents = fs::read_to_string(path)?; // io::Error -> ConfigError
    let port: u16 = contents.trim().parse()?; // ParseIntError -> ConfigError
    if port == 0 {
        return Err(ConfigError { message: "port must be non-zero".to_string() });
    }
    Ok(port)
}
```

Each `?` jumps through the matching `From` impl and produces your single error type. This is why custom error types exist: they make every error in a library *one type* the caller can match on.

### Writing a custom error type by hand

Before the crates, the hand-rolled way. An error type needs two traits: `Display` (to print it) and `Error` (the standard error trait, enabling `?` conversions via the blanket `From<E: Error> for Box<dyn Error>`, logging, and `source()` chaining):

```rust
use std::fmt;

#[derive(Debug)]
enum ConfigError {
    MissingFile(String),
    InvalidPort(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::MissingFile(path) => write!(f, "config file not found: {path}"),
            ConfigError::InvalidPort(value) => write!(f, "invalid port value: {value}"),
        }
    }
}

impl std::error::Error for ConfigError {}
```

`impl std::error::Error for ConfigError {}` is empty because `Display` + `Debug` are the only requirements — but the trait is what makes `Box<dyn Error>`, `println!("{error}")`, and `?`-into-boxed-errors all work. You'll meet `dyn Error` properly in Module 017.

### `thiserror`: derive the boilerplate

The manual impls above are mechanical, so `thiserror` derives them:

```rust
#[derive(Debug, thiserror::Error)]
enum ConfigError {
    #[error("config file not found: {0}")]
    MissingFile(String),
    #[error("invalid port value: {0}")]
    InvalidPort(String),
    #[error("i/o error: {0}")]
    Io(#[from] std::io::Error),
    #[error("could not parse port: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
}
```

- Each `#[error("...")]` string becomes the `Display` implementation. `{0}` inserts the first field, `{name}` inserts a named field.
- Each `#[from]` generates the `From` impl for that variant's type — so `fs::read_to_string(path)?` and `s.parse()?` convert automatically, as long as your function returns `Result<_, ConfigError>`.
- `#[derive(Debug, thiserror::Error)]` also gives you `impl std::error::Error`, including chained `source()` when a variant wraps another error.

That's the entire boilerplate. One enum, one line per error case, and every `?` in your crate composes against it. This is exactly the pattern used by real crates — `sqlx::Error`, `serde_json::Error`, and tokio's errors are all thiserror-style enums (or hand-written equivalents).

### One more piece: `Box<dyn Error>` and `anyhow`

For *application* code (a CLI's `main`, a script), matching on error variants is usually pointless — you just want "read the file, parse it, if anything fails, tell the user and stop." Two idioms handle that:

```rust
fn read_settings(path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(path)?; // any error type converts
    Ok(contents)
}
```

`Box<dyn Error>` works because `?` converts any `E: Error` into it via the blanket `From`. The `anyhow` crate (later, in the async block) is the ergonomic upgrade: `Result<T, anyhow::Error>` with `.context("...")`. The rule of thumb: **libraries return typed `thiserror` enums so callers can match; applications return `anyhow`/`Box<dyn Error>` because they just report.** Capstone 02 uses the library side of that rule.

## Common Pitfalls

- **Using `?` in a function that returns `()` or `Option<T>` for a `Result` expression.** `?` requires the enclosing function to return a compatible type. Fix: match the `Result` explicitly, or change the return type.
- **Forgetting `#[from]` and getting "`?` conversion failed" errors.** If `?` won't convert, the error type has no `From` impl for it. Fix: add `#[from]` on the variant wrapping that error type.
- **One giant `String`-carrying error enum.** Error variants that lose their structure (`#[error("{0}")]` on a raw `String`) can't be matched meaningfully. Fix: separate variants with `#[from]` wrappers for each error kind.
- **`#[error("...")]` without enough `{0}`s.** Every field you want in the message needs a placeholder, or `thiserror` complains. Named fields need `{name}`.
- **Hand-rolling `Display` when `thiserror` is available.** The manual `impl fmt::Display` is exactly what `#[error("...")]` generates — prefer the derive, and keep the hand-written version as a mental model only.

## Key Terms

- **`?` operator:** unwraps `Ok`, or returns the `Err` (converted via `From`) from the enclosing function.
- **`From` trait:** conversion trait; `From<A> for B` lets `?` turn an `A` error into a `B` error.
- **`std::error::Error`:** the standard error trait; `Display` + `Debug` required, `source()` optional.
- **`#[from]` (thiserror):** generates the `From` impl for a variant's wrapped error type.
- **`#[error("...")]` (thiserror):** the display format for a variant, with `{0}`/`{name}` placeholders.
- **`Box<dyn Error>`:** an owned pointer to any error type; the poor-man's `anyhow`.

## Exercise

Open `exercises/src/lib.rs` and fill in the `TODO(module-014)` bodies. The `AppError` enum is already defined for you — you will *use* it, not write it:

1. `validate_username` — length and character checks, returning `AppError::InvalidInput` with helpful messages.
2. `read_config` — `read_to_string` plus `?` (converts to `AppError::Io` automatically).
3. `find_entry` — `.find(...)` plus `.ok_or_else(...)` naming the missing entry.
4. `parse_port` — `s.parse()?` (converts to `AppError::ParseInt`) plus a non-zero check.
5. `load_port_config` — compose `read_config` and `parse_port` with two `?`s.

The tests in `tests/module_014.rs` define "done":

```bash
cargo test -p module-014-exercises
```

Compare with `solutions/` only after you've made a genuine attempt.

## Further Reading

- [The Rust Book, Chapter 9.2 — Recoverable Errors with Result; 9.3 — To panic or not](https://doc.rust-lang.org/book/ch09-02-recoverable-errors.html)
- [The Rust Book, Chapter 10.2 — trait `From` and `?` conversions](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [thiserror crate documentation](https://docs.rs/thiserror)
- [anyhow crate documentation (the application-side counterpart)](https://docs.rs/anyhow)
