# Module 072: Building CLI Tools II — Config Files, Error UX, and Polish

**Block:** Block H — CLI, Networking & Distributed Systems
**Estimated time:** 60–90 min
**Prerequisites:** Module 071 (Building CLI Tools I — clap deep dive)

## Learning Objectives
- Load configuration from TOML and JSON files using `serde`
- Implement config precedence: CLI args > config file > defaults
- Add human-readable error messages with `anyhow` context chains
- Use colored output conditionally (only when stderr is a TTY)
- Display progress bars with `indicatif` (and hide them in tests)

## Why This Matters
Production CLIs don't just parse arguments — they read config files, give helpful error messages when things go wrong, and show progress for long-running operations. `ripgrep` reads `.ripgreprc`, `cargo` reads `config.toml`, and `rustfmt` reads `rustfmt.toml`. This module teaches you to build CLIs that feel polished and professional.

## Concept

You built a CLI with subcommands and validation in Module 071. Now let's make it production-ready: config files, better errors, colored output, and progress indicators.

### Config files with serde

Most CLIs let users put settings in a config file so they don't have to type the same flags every time. `serde` makes this trivial: define a struct, derive `Deserialize`, and load it from TOML, JSON, or any other format `serde` supports.

```rust
use serde::Deserialize;
use std::fs;

#[derive(Deserialize, Debug)]
struct Config {
    verbose: bool,
    output_dir: String,
    max_retries: u32,
}

fn load_config_toml(path: &str) -> anyhow::Result<Config> {
    let content = fs::read_to_string(path)?;
    let config: Config = toml::from_str(&content)?;
    Ok(config)
}
```

The `?` operator propagates errors, and `anyhow` wraps them with context. If the file doesn't exist, you get `No such file or directory`. If the TOML is malformed, you get a parse error. `anyhow` makes all of this ergonomic.

### Config precedence

The standard pattern: **CLI args override config file values, which override defaults**. You implement this by:

1. Start with a `Config` struct with default values
2. Load the config file (if it exists) and merge it on top
3. Parse CLI args and merge them on top

```rust
#[derive(Default)]
struct Config {
    verbose: bool,
    output_dir: String,
    max_retries: u32,
}

impl Config {
    fn load(path: Option<&str>) -> anyhow::Result<Self> {
        let mut config = Config::default();
        
        if let Some(path) = path {
            if let Ok(content) = std::fs::read_to_string(path) {
                if let Ok(file_config) = toml::from_str::<ConfigFile>(&content) {
                    if let Some(v) = file_config.verbose {
                        config.verbose = v;
                    }
                    if let Some(dir) = file_config.output_dir {
                        config.output_dir = dir;
                    }
                    if let Some(retries) = file_config.max_retries {
                        config.max_retries = retries;
                    }
                }
            }
        }
        
        Ok(config)
    }
}
```

The config file uses `Option<T>` for each field so you can distinguish "not set" from "set to default". Then you only override if the field is `Some`.

### Error UX with anyhow

`anyhow` is for application-level error handling (not library code — that's `thiserror` from Module 014). It lets you attach context to errors:

```rust
fn load_config(path: &str) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {path}"))?;
    let config: Config = toml::from_str(&content)
        .with_context(|| format!("failed to parse config file: {path}"))?;
    Ok(config)
}
```

Now when something goes wrong, the error message tells the user *what* failed and *why*:

```
Error: failed to parse config file: config.toml

Caused by:
    TOML parse error at line 3, column 1
      |
    3 | max_retries = "five"
      |               ^^^^^^
    invalid type: string "five", expected u32
```

### Colored output (conditionally)

Colored error messages and status output look nice, but they break tests and piped output. The rule: **only colorize when stderr is a TTY**.

```rust
use std::io::IsTerminal;

fn print_error(msg: &str) {
    if std::io::stderr().is_terminal() {
        eprintln!("\x1b[31merror\x1b[0m: {msg}");
    } else {
        eprintln!("error: {msg}");
    }
}
```

The `is_terminal()` method (from the `is-terminal` crate, or `std::io::IsTerminal` in Rust 1.70+) checks if stderr is connected to a terminal. If it's piped to a file or another process, skip the ANSI codes.

For tests, this means you can assert on the error message text without worrying about ANSI escape sequences.

### Progress bars with indicatif

`indicatif` gives you spinners, progress bars, and multi-progress displays. For a simple progress bar:

```rust
use indicatif::ProgressBar;

fn process_items(items: &[String]) {
    let pb = ProgressBar::new(items.len() as u64);
    for item in items {
        // process item
        pb.inc(1);
    }
    pb.finish_with_message("done");
}
```

But progress bars write to stderr and use ANSI codes, which breaks tests. The fix: **hide the progress bar in tests** or make it optional:

```rust
fn process_items(items: &[String], show_progress: bool) {
    let pb = if show_progress {
        Some(ProgressBar::new(items.len() as u64))
    } else {
        None
    };
    
    for item in items {
        // process item
        if let Some(pb) = &pb {
            pb.inc(1);
        }
    }
    
    if let Some(pb) = pb {
        pb.finish_with_message("done");
    }
}
```

In tests, pass `show_progress = false`. In the real CLI, pass `true` (or check `is_terminal()`).

### Putting it together

A polished CLI:

1. Parses args with `clap`
2. Loads config from file (with precedence over defaults)
3. Merges CLI args on top of config
4. Runs the main logic with progress bars (if TTY)
5. Prints colored errors (if TTY)
6. Returns a clean exit code

```rust
fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    
    let config = Config::load(cli.config.as_deref())
        .context("failed to load configuration")?;
    
    let final_config = merge_config(config, &cli);
    
    run(&final_config, std::io::stderr().is_terminal())
        .context("operation failed")?;
    
    Ok(())
}
```

### Testing config loading

You can test config loading without touching the filesystem by using `serde` to deserialize from a string:

```rust
#[test]
fn parses_toml_config() {
    let toml = r#"
        verbose = true
        output_dir = "/tmp/out"
        max_retries = 5
    "#;
    let config: Config = toml::from_str(toml).unwrap();
    assert!(config.verbose);
    assert_eq!(config.output_dir, "/tmp/out");
    assert_eq!(config.max_retries, 5);
}
```

For precedence tests, create a config, apply overrides, and check the result:

```rust
#[test]
fn cli_overrides_config_file() {
    let config = Config {
        verbose: false,
        output_dir: "/default".to_string(),
        max_retries: 3,
    };
    let overrides = CliOverrides {
        verbose: Some(true),
        output_dir: None,
        max_retries: None,
    };
    let final_config = merge_config(config, overrides);
    assert!(final_config.verbose);
    assert_eq!(final_config.output_dir, "/default");
}
```

## Common Pitfalls
- **Not using `Option<T>` in the config file struct** — you can't distinguish "not set" from "set to default", so you can't implement precedence correctly.
- **Coloring output unconditionally** — breaks tests and piped output. Always check `is_terminal()`.
- **Using `unwrap()` in config loading** — use `anyhow::Context` to give the user a helpful error message.
- **Showing progress bars in tests** — they write to stderr and use ANSI codes. Hide them or make them optional.
- **Forgetting to handle missing config files gracefully** — if the user doesn't pass `--config`, don't error; just use defaults.

## Key Terms
- **Config precedence**: CLI args > config file > defaults
- **`anyhow::Context`**: attaches a message to an error for better diagnostics
- **`is_terminal()`**: checks if a file descriptor is connected to a TTY
- **`indicatif`**: a crate for progress bars and spinners
- **`Option<T>` in config**: distinguishes "not set" from "set to default"

## Exercise

In `exercises/`, you'll build a CLI tool that:

1. Loads a config from a TOML file (or uses defaults if no file is provided)
2. Merges CLI argument overrides on top of the config
3. Returns human-readable errors with `anyhow` context
4. Implements a `run` function that processes items (stub implementation is fine)

Fill in the `TODO(module-072)` markers to:

1. Define the `Config` struct with `serde::Deserialize`
2. Implement `load_config` to parse TOML
3. Implement `merge_config` to apply CLI overrides
4. Add `anyhow::Context` to error messages

The integration tests verify config loading, precedence, and error messages.

## Further Reading
- [serde documentation](https://serde.rs/)
- [anyhow documentation](https://docs.rs/anyhow/latest/anyhow/)
- [indicatif documentation](https://docs.rs/indicatif/latest/indicatif/)
- [The Rust Book: Error handling](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)

## Running This Module's Tests

All tests run with `cargo test -p module-072-exercises` and `cargo test -p module-072-solutions`. No special features or external services required.
