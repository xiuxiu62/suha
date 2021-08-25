use std::path::PathBuf;

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
    Utf8(std::string::FromUtf8Error),
    Parse(toml::de::Error),
}

impl Config {
    pub fn load(path: PathBuf) -> Self {
        match read_config(path) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("{:?}", e);
                Config::default()
            }
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

fn read_config(path: PathBuf) -> Result<Config, ConfigError> {
    let raw = std::fs::read(path)?;
    let buf = String::from_utf8(raw)?;
    let config = toml::from_str(&buf)?;
    Ok(config)
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err)
    }
}

impl From<std::string::FromUtf8Error> for ConfigError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        ConfigError::Utf8(err)
    }
}

impl From<toml::de::Error> for ConfigError {
    fn from(err: toml::de::Error) -> Self {
        ConfigError::Parse(err)
    }
}
