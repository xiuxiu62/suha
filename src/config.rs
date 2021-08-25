use std::io::Read;

use serde::Deserialize;
use toml;

#[derive(Deserialize, Debug, Clone)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub show_hidden: bool,
    pub show_icons: bool,
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
}

impl Config {
    pub fn load(filename: &str) -> Self {
        match read_config(filename) {
            Ok(config) => config,
            Err(_) => Config::default(),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            show_hidden: false,
            show_icons: false,
        }
    }
}

fn read_config(filename: &str) -> Result<Config, ConfigError> {
    let mut file = std::fs::File::open(filename)?;
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;

    let config = toml::from_str(&buf)?;
    Ok(config)
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err)
    }
}
