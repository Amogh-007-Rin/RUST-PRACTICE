//! Module 008: exercise scaffold.
//!
//! Fill in the TODOs below so the integration tests in `tests/` pass.

/// A command the user typed — the same shape Capstone 01's CLI uses.
#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Add(String),
    List,
    Remove(usize),
    Search(String),
}

/// Parses one command line like `"add buy milk"`, `"list"`, `"remove 3"`, or
/// `"search milk"`. Returns `None` for anything unrecognized.
pub fn parse_command(line: &str) -> Option<Command> {
    // TODO(module-008): split the line with `line.split_whitespace().collect()`
    // into a `Vec<&str>`, then match on the first word:
    //   "add <rest>"    -> Some(Command::Add(rest joined with " "))
    //   "list"          -> Some(Command::List)
    //   "remove <n>"    -> Some(Command::Remove(n))   (`n.parse().ok()?`)
    //   "search <rest>" -> Some(Command::Search(rest joined with " "))
    //   _               -> None
    let _ = line;
    panic!("TODO(module-008): implement parse_command")
}

/// A temperature scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// Converts `value` between scales; same-scale conversions return `value`.
pub fn convert(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    // TODO(module-008): `match (from, to)`:
    //   Celsius -> Fahrenheit : value * 9.0 / 5.0 + 32.0
    //   Fahrenheit -> Celsius : (value - 32.0) * 5.0 / 9.0
    //   same scale            : value
    let _ = (value, from, to);
    panic!("TODO(module-008): implement convert")
}

/// Describes an `Option<i32>`: `Some(7)` -> `"some(7)"`, `None` -> `"none"`.
pub fn describe(o: Option<i32>) -> String {
    // TODO(module-008): `match o { Some(n) => format!(...), None => ... }`.
    let _ = o;
    panic!("TODO(module-008): implement describe")
}

/// Adds `1` to the inner value, using `if let`; `None` stays `None`.
pub fn bump(o: Option<i32>) -> Option<i32> {
    // TODO(module-008): `if let Some(n) = o { ... } else { None }`. Note: for
    // the one-liner shape clippy suggests `o.map(|n| n + 1)` (closures come in
    // Module 021) — build `next` in a local binding first to keep clippy quiet.
    let _ = o;
    panic!("TODO(module-008): implement bump")
}
