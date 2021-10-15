use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Default, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub show_hidden: bool,
    pub show_icons: bool,
}

impl Config {
    // Trys to load a config, returning default if none are found
    pub fn try_load() -> Self {
        if let Some(home_dir) = home::home_dir() {
            let canonicalize = |partial| home_dir.clone().join(partial);
            let possible_partials = [
                ".config/suha.toml",
                ".config/suha/config.toml",
                ".config/suha.d/config.toml",
            ];

            for partial in possible_partials {
                match Config::read(canonicalize(partial)) {
                    Ok(v) => return v,
                    Err(_) => continue,
                }
            }
        };

        Config::default()
    }

    fn read(path: PathBuf) -> Result<Config, ConfigError> {
        let buf = fs::read_to_string(path)?;
        Ok(toml::from_str(&buf)?)
    }
}

#[derive(Debug)]
pub enum ConfigError {
    Io(std::io::Error),
    Parse(toml::de::Error),
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
