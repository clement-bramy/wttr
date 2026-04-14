use std::fs;

use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub city: Option<String>,
    pub unit: Unit,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Unit {
    #[serde(alias = "Celsius")]
    Celsius,

    #[serde(alias = "Fahrenheit")]
    Fahrenheit,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            city: None,
            unit: Unit::Celsius,
        }
    }
}

pub fn load_config_or_default() -> Config {
    directories::ProjectDirs::from("io.github", "clement-bramy", "wttr")
        .map(|dirs| dirs.config_dir().join("config.toml"))
        .and_then(|file| fs::read_to_string(&file).ok())
        .and_then(|raw| toml::from_str::<Config>(&raw).ok())
        .unwrap_or_default()
}
