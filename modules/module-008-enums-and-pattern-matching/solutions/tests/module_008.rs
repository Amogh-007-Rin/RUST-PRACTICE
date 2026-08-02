use module_008_solutions::{bump, convert, describe, parse_command, Command, TemperatureUnit};

#[test]
fn parse_add() {
    assert_eq!(
        parse_command("add buy milk"),
        Some(Command::Add("buy milk".to_string()))
    );
}

#[test]
fn parse_list() {
    assert_eq!(parse_command("list"), Some(Command::List));
}

#[test]
fn parse_remove() {
    assert_eq!(parse_command("remove 3"), Some(Command::Remove(3)));
}

#[test]
fn parse_search() {
    assert_eq!(
        parse_command("search milk"),
        Some(Command::Search("milk".to_string()))
    );
}

#[test]
fn parse_unknown_is_none() {
    assert_eq!(parse_command("explode"), None);
    assert_eq!(parse_command(""), None);
}

#[test]
fn convert_celsius_to_fahrenheit() {
    assert_eq!(
        convert(0.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
        32.0
    );
    assert_eq!(
        convert(100.0, TemperatureUnit::Celsius, TemperatureUnit::Fahrenheit),
        212.0
    );
}

#[test]
fn convert_fahrenheit_to_celsius() {
    assert_eq!(
        convert(32.0, TemperatureUnit::Fahrenheit, TemperatureUnit::Celsius),
        0.0
    );
}

#[test]
fn convert_same_unit_identity() {
    assert_eq!(
        convert(21.5, TemperatureUnit::Celsius, TemperatureUnit::Celsius),
        21.5
    );
}

#[test]
fn describe_some_and_none() {
    assert_eq!(describe(Some(7)), "some(7)");
    assert_eq!(describe(None), "none");
}

#[test]
fn bump_increments() {
    assert_eq!(bump(Some(1)), Some(2));
    assert_eq!(bump(None), None);
}
