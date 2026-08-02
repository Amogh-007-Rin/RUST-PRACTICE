//! Module 072: Building CLI Tools II — config files, error UX, and polish — exercise scaffold.

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub verbose: bool,
    pub output_dir: String,
    pub max_retries: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            verbose: false,
            output_dir: ".".to_string(),
            max_retries: 3,
        }
    }
}

#[derive(Deserialize, Debug, Default)]
pub struct ConfigFile {
    pub verbose: Option<bool>,
    pub output_dir: Option<String>,
    pub max_retries: Option<u32>,
}

#[derive(Debug, Default)]
pub struct CliOverrides {
    pub verbose: Option<bool>,
    pub output_dir: Option<String>,
    pub max_retries: Option<u32>,
}

/// Load a config from a TOML file.
pub fn load_config(_path: &str) -> Result<Config> {
    // TODO(module-072): read the file, parse TOML, and return a Config.
    // Use `.with_context(...)` on both the file read and the parse to give
    // helpful error messages.
    panic!("TODO(module-072): implement load_config")
}

/// Merge CLI overrides on top of a config.
pub fn merge_config(_config: Config, _overrides: CliOverrides) -> Config {
    // TODO(module-072): if an override field is Some, overwrite the config field.
    panic!("TODO(module-072): implement merge_config")
}
