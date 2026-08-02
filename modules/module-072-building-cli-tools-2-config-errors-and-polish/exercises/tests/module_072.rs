//! Module 072: integration tests.

use module_072_exercises::{load_config, merge_config, CliOverrides, Config, ConfigFile};
use std::io::Write;
use tempfile::NamedTempFile;

#[test]
fn loads_toml_config() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(
        tmp,
        r#"
verbose = true
output_dir = "/tmp/out"
max_retries = 5
"#
    )
    .unwrap();

    let config = load_config(tmp.path().to_str().unwrap()).unwrap();
    assert!(config.verbose);
    assert_eq!(config.output_dir, "/tmp/out");
    assert_eq!(config.max_retries, 5);
}

#[test]
fn missing_config_file_returns_error() {
    let result = load_config("/nonexistent/path/config.toml");
    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("failed to read config file") || msg.contains("No such file"),
        "error should mention file read failure: {msg}"
    );
}

#[test]
fn malformed_toml_returns_error() {
    let mut tmp = NamedTempFile::new().unwrap();
    writeln!(tmp, "this is not valid toml {{{{").unwrap();

    let result = load_config(tmp.path().to_str().unwrap());
    assert!(result.is_err());
    let err = result.unwrap_err();
    let msg = format!("{err:?}");
    assert!(
        msg.contains("failed to parse config file") || msg.contains("TOML"),
        "error should mention parse failure: {msg}"
    );
}

#[test]
fn cli_overrides_verbose() {
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
    let merged = merge_config(config, overrides);
    assert!(merged.verbose);
    assert_eq!(merged.output_dir, "/default");
    assert_eq!(merged.max_retries, 3);
}

#[test]
fn cli_overrides_output_dir() {
    let config = Config::default();
    let overrides = CliOverrides {
        verbose: None,
        output_dir: Some("/custom".to_string()),
        max_retries: None,
    };
    let merged = merge_config(config, overrides);
    assert_eq!(merged.output_dir, "/custom");
}

#[test]
fn cli_overrides_max_retries() {
    let config = Config::default();
    let overrides = CliOverrides {
        verbose: None,
        output_dir: None,
        max_retries: Some(10),
    };
    let merged = merge_config(config, overrides);
    assert_eq!(merged.max_retries, 10);
}

#[test]
fn no_overrides_keeps_config() {
    let config = Config {
        verbose: true,
        output_dir: "/test".to_string(),
        max_retries: 7,
    };
    let overrides = CliOverrides::default();
    let merged = merge_config(config.clone(), overrides);
    assert_eq!(merged, config);
}

#[test]
fn partial_toml_uses_defaults() {
    let toml = "verbose = true\n";
    let file_config: ConfigFile = toml::from_str(toml).unwrap();
    let mut config = Config::default();
    if let Some(v) = file_config.verbose {
        config.verbose = v;
    }
    assert!(config.verbose);
    assert_eq!(config.output_dir, ".");
    assert_eq!(config.max_retries, 3);
}
