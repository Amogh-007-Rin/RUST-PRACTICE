//! Module 072: Building CLI Tools II — config files, error UX, and polish — reference solution.

use anyhow::{Context, Result};
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
pub fn load_config(path: &str) -> Result<Config> {
    let content = std::fs::read_to_string(path)
        .with_context(|| format!("failed to read config file: {path}"))?;
    let file_config: ConfigFile =
        toml::from_str(&content).with_context(|| format!("failed to parse config file: {path}"))?;

    let mut config = Config::default();
    if let Some(v) = file_config.verbose {
        config.verbose = v;
    }
    if let Some(dir) = file_config.output_dir {
        config.output_dir = dir;
    }
    if let Some(retries) = file_config.max_retries {
        config.max_retries = retries;
    }
    Ok(config)
}

/// Merge CLI overrides on top of a config.
pub fn merge_config(mut config: Config, overrides: CliOverrides) -> Config {
    if let Some(v) = overrides.verbose {
        config.verbose = v;
    }
    if let Some(dir) = overrides.output_dir {
        config.output_dir = dir;
    }
    if let Some(retries) = overrides.max_retries {
        config.max_retries = retries;
    }
    config
}
