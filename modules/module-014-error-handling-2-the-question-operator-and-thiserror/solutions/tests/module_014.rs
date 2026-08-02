use std::path::PathBuf;

use module_014_solutions::{
    find_entry, load_port_config, parse_port, read_config, validate_username, AppError,
};

fn write_temp(contents: &str, tag: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("module_014_{}_{}.txt", std::process::id(), tag));
    std::fs::write(&path, contents).unwrap();
    path
}

#[test]
fn validate_username_accepts_valid_names() {
    assert_eq!(validate_username("alice_01").unwrap(), "alice_01");
    assert_eq!(validate_username("bob").unwrap(), "bob");
}

#[test]
fn validate_username_rejects_short_names() {
    assert!(validate_username("ab").is_err());
}

#[test]
fn validate_username_rejects_too_long_names() {
    assert!(validate_username("a_very_long_username_that_exceeds_limits").is_err());
}

#[test]
fn validate_username_rejects_invalid_characters() {
    assert!(validate_username("a b").is_err());
    assert!(validate_username("a@b").is_err());
    assert!(validate_username("a-b").is_err());
}

#[test]
fn read_config_reads_an_existing_file() {
    let path = write_temp("key = value\n", "config");
    let result = read_config(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result.unwrap(), "key = value\n");
}

#[test]
fn read_config_reports_missing_files() {
    let missing = std::env::temp_dir().join("module_014_missing_xyz.txt");
    let _ = std::fs::remove_file(&missing);
    let result = read_config(&missing);
    assert!(matches!(result, Err(AppError::Io(_))));
}

#[test]
fn find_entry_returns_the_matching_entry() {
    let entries = vec!["alice".to_string(), "bob".to_string()];
    assert_eq!(find_entry(&entries, "bob").unwrap(), "bob");
}

#[test]
fn find_entry_errors_on_a_miss() {
    let entries = vec!["alice".to_string()];
    let result = find_entry(&entries, "nobody");
    assert!(matches!(result, Err(AppError::NotFound(name)) if name == "nobody"));
}

#[test]
fn parse_port_accepts_valid_ports() {
    assert_eq!(parse_port("8080").unwrap(), 8080);
    assert_eq!(parse_port("1").unwrap(), 1);
}

#[test]
fn parse_port_rejects_zero() {
    assert!(matches!(parse_port("0"), Err(AppError::InvalidInput(_))));
}

#[test]
fn parse_port_rejects_garbage() {
    assert!(matches!(parse_port("http"), Err(AppError::ParseInt(_))));
    assert!(matches!(parse_port("99999"), Err(AppError::ParseInt(_))));
}

#[test]
fn load_port_config_reads_and_parses() {
    let path = write_temp("8080\n", "port");
    let result = load_port_config(&path);
    let _ = std::fs::remove_file(&path);
    assert_eq!(result.unwrap(), 8080);
}

#[test]
fn load_port_config_propagates_missing_files() {
    let missing = std::env::temp_dir().join("module_014_port_missing.txt");
    let _ = std::fs::remove_file(&missing);
    let result = load_port_config(&missing);
    assert!(matches!(result, Err(AppError::Io(_))));
}

#[test]
fn load_port_config_propagates_unparseable_content() {
    let path = write_temp("http\n", "badport");
    let result = load_port_config(&path);
    let _ = std::fs::remove_file(&path);
    assert!(matches!(result, Err(AppError::ParseInt(_))));
}
