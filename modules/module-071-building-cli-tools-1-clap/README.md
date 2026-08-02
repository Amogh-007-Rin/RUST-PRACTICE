# Module 071: Building CLI Tools I — `clap` Deep Dive

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 013 (Error Handling I), Module 014 (Error Handling II)

## Learning Objectives
- Define a CLI with subcommands, positional arguments, and named options using `clap`'s derive API
- Add validation (required vs optional, value ranges, custom parsers)
- Parse arguments into strongly-typed Rust structs
- Test argument parsing without running the binary
- Separate argument parsing from business logic for testability

## Why This Matters
Every serious Rust CLI tool — `ripgrep`, `fd`, `bat`, `cargo` itself — uses `clap`. Understanding its derive API means you can build production-grade CLIs with automatic `--help` generation, shell completions, and type-safe argument handling. This is the foundation for everything in Block H.

## Concept

Command-line argument parsing is the first thing almost every Rust CLI tool gets right, and `clap` is the reason why. You've probably used CLIs that print a wall of text when you pass `--help`, accept both `--verbose` and `-v`, reject bad inputs with a clear message, and organize functionality into subcommands like `git commit` or `cargo build`. `clap` gives you all of that for free once you describe your CLI's shape with Rust types.

### The derive API

The modern way to use `clap` is through its **derive API**: you define a struct (or enum) that describes your CLI, derive `Parser` on it, and `clap` generates the parsing logic, help text, and validation rules from your type definitions.

```rust
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "my-tool", about = "A small CLI tool")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Number of retries
    #[arg(short, long, default_value_t = 3)]
    retries: u32,

    /// Input file
    input: String,
}
```

When you call `Cli::parse()`, `clap` reads `std::env::args`, matches each token against the fields, and produces a populated struct. If the user passes `--help`, `clap` prints an auto-generated help message and exits. If they pass something invalid, it prints an error and exits with a non-zero code.

### Subcommands

Most non-trivial CLIs have subcommands: `git add`, `git commit`, `docker run`. In `clap`, you model these as an enum:

```rust
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "task")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Remove a task by ID
    Remove { id: u64 },
}
```

Each variant becomes a subcommand. Fields in a variant become that subcommand's arguments. `clap` generates help for each one automatically.

### Argument attributes

The `#[arg(...)]` attribute controls how each field maps to CLI arguments:

- `short` — adds a single-letter flag (`-v` for `verbose`)
- `long` — adds a long flag (`--verbose`)
- `default_value_t = expr` — sets a default (the `_t` suffix means the value implements `Display`)
- `required = true` — makes an optional field mandatory
- `value_parser` — constrains the type or adds custom parsing
- `num_args = 1..=5` — accepts a range of values (produces a `Vec`)

```rust
#[derive(Parser, Debug)]
struct Cli {
    /// Port to listen on (1–65535)
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..=65535))]
    port: u16,

    /// Files to process
    #[arg(num_args = 1..)]
    files: Vec<String>,
}
```

### Value parsing and validation

`clap` parses argument strings into Rust types using the `FromStr` trait. If your type implements `FromStr`, `clap` can parse it directly. For custom validation, you can use `value_parser!` with range constraints, or write a custom parser function:

```rust
fn parse_hex(s: &str) -> Result<u32, String> {
    u32::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(|e| format!("invalid hex: {e}"))
}

#[derive(Parser)]
struct Cli {
    #[arg(long, value_parser = parse_hex)]
    color: u32,
}
```

### Testing without running the binary

Here's the key insight for testability: `clap`'s `Parser` trait gives you `try_parse_from`, which takes an iterator of strings instead of reading `std::env::args`. You can also use `CommandFactory::command()` to get the `Command` and call `try_get_matches_from` on it. This means you can test argument parsing in unit tests without spawning a process.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_basic_args() {
        let cli = Cli::try_parse_from(["my-tool", "-v", "--retries", "5", "input.txt"]).unwrap();
        assert!(cli.verbose);
        assert_eq!(cli.retries, 5);
        assert_eq!(cli.input, "input.txt");
    }

    #[test]
    fn rejects_bad_port() {
        let result = Cli::try_parse_from(["my-tool", "--port", "99999"]);
        assert!(result.is_err());
    }
}
```

### Separating parsing from logic

The golden rule: your `lib.rs` should contain the struct definitions and the business logic, while `main.rs` just calls `Cli::parse()` and dispatches. This way, integration tests can import the library and test parsing + logic without touching `main`.

```rust
// lib.rs
pub fn run_add(description: &str) -> String {
    format!("Added task: {description}")
}

// main.rs
fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::Add { description } => println!("{}", run_add(&description)),
        // ...
    }
}
```

### The `CommandFactory` approach

For more control in tests, you can drop down to the `Command` level:

```rust
use clap::{CommandFactory, FromArgMatches, Parser};

let cmd = Cli::command();
let matches = cmd.try_get_matches_from(["tool", "add", "buy milk"])?;
let cli = Cli::from_arg_matches(&matches)?;
```

This is what `try_parse_from` does internally. You rarely need this directly, but it's useful when you want to inspect the raw matches or test error messages.

### Common patterns

**Mutually exclusive flags**: use an enum with `#[command(flatten)]` or validate in a post-parse check.

**Config file + CLI override**: parse CLI args first, then layer config-file values underneath (covered in Module 072).

**Shell completions**: `clap` can generate completions for bash, zsh, fish, and PowerShell from your `Command` definition — no extra work beyond what you've already written.

## Common Pitfalls
- **Forgetting `#[command(subcommand)]`** on the enum field — `clap` won't know it's a subcommand and will treat it as a regular argument.
- **Using `String` where you could use a typed field** — `clap` can parse `u16`, `PathBuf`, `bool`, and anything with `FromStr` directly. Use the type system.
- **Putting business logic in `main.rs`** — makes it untestable. Keep `main` thin: parse, dispatch, print.
- **Using `unwrap()` on `parse()` in tests** — `parse()` calls `std::process::exit` on failure. Use `try_parse_from` in tests.
- **Not setting `default_value_t` correctly** — the `_t` variant is for types that implement `Display`; use `default_value` for string literals.

## Key Terms
- **Parser**: the derive macro that turns a struct into a CLI argument parser
- **Subcommand**: a named variant under a parent command (like `git commit`)
- **`try_parse_from`**: test-friendly parsing from an iterator of strings
- **`value_parser`**: constrains or customizes how a string argument becomes a Rust value
- **`CommandFactory`**: trait that produces a `Command` (the runtime representation of your CLI)
- **`FromArgMatches`**: trait that constructs your struct from parsed `ArgMatches`

## Exercise

In `exercises/`, you'll build a task-manager CLI called `taskr` with three subcommands: `add`, `list`, and `done`. The CLI struct and subcommand enum are partially defined — fill in the `TODO(module-071)` markers to:

1. Define the `Cli` struct with a `#[command(subcommand)]` field
2. Complete the `Commands` enum with the three subcommands and their arguments
3. Implement `execute` that dispatches on the command and returns a `String` result
4. Add proper `#[arg(...)]` attributes for validation (e.g., `done` takes a positive `u64` id)

The integration tests in `tests/module_071.rs` will verify parsing and execution.

## Further Reading
- [The Rust Book: CLI arguments](https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html)
- [clap derive reference](https://docs.rs/clap/latest/clap/_derive/index.html)
- [clap tutorial](https://docs.rs/clap/latest/clap/_tutorial/index.html)
