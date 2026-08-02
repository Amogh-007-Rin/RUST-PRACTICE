//! Module 008: solution — the reference implementation.

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
    let parts: Vec<&str> = line.split_whitespace().collect();
    let first = parts.first()?;
    match *first {
        "add" => Some(Command::Add(parts[1..].join(" "))),
        "list" => Some(Command::List),
        "remove" => {
            let id = parts.get(1)?.parse().ok()?;
            Some(Command::Remove(id))
        }
        "search" => Some(Command::Search(parts[1..].join(" "))),
        _ => None,
    }
}

/// A temperature scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TemperatureUnit {
    Celsius,
    Fahrenheit,
}

/// Converts `value` between scales; same-scale conversions return `value`.
pub fn convert(value: f64, from: TemperatureUnit, to: TemperatureUnit) -> f64 {
    match (from, to) {
        (TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit) => value * 9.0 / 5.0 + 32.0,
        (TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius) => (value - 32.0) * 5.0 / 9.0,
        _ => value,
    }
}

/// Describes an `Option<i32>`: `Some(7)` -> `"some(7)"`, `None` -> `"none"`.
pub fn describe(o: Option<i32>) -> String {
    match o {
        Some(n) => format!("some({n})"),
        None => "none".to_string(),
    }
}

/// Adds `1` to the inner value, using `if let`; `None` stays `None`.
pub fn bump(o: Option<i32>) -> Option<i32> {
    if let Some(n) = o {
        let next = n + 1;
        Some(next)
    } else {
        None
    }
}
