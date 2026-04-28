use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::error::CodefartError;

/// Available built-in sound themes.
pub const BUILTIN_THEMES: &[(&str, &str)] = &[
    ("classic", "The signature CodeFart sound"),
    ("wet", "A wetter, more dramatic variant"),
    ("tiny", "A small, polite notification fart"),
    ("squeaky", "High-pitched, brief"),
    ("thunder", "For those long CI runs"),
];

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Config {
    /// Currently selected built-in theme (e.g. "classic", "wet")
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theme: Option<String>,
    /// Path to a custom sound file (overrides theme)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_sound: Option<String>,
}

impl Config {
    /// Path to the managed sounds directory: ~/.config/codefart/sounds/
    pub fn sounds_dir() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".config").join("codefart").join("sounds")
    }

    /// Path to the config file: ~/.config/codefart/config.toml
    pub fn config_path() -> PathBuf {
        let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("."));
        home.join(".config").join("codefart").join("config.toml")
    }

    /// Load config from disk. Returns default if file doesn't exist.
    pub fn load() -> Result<Config, CodefartError> {
        let path = Self::config_path();
        match std::fs::read_to_string(&path) {
            Ok(content) => toml::from_str(&content).map_err(CodefartError::ConfigParse),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(Config::default()),
            Err(e) => Err(CodefartError::ConfigRead(e)),
        }
    }

    /// Save config to disk. Creates parent directories if needed.
    pub fn save(&self) -> Result<(), CodefartError> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent).map_err(CodefartError::ConfigWrite)?;
        }
        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content).map_err(CodefartError::ConfigWrite)
    }

    /// Get the active theme name. If not set, defaults to "classic".
    pub fn active_theme(&self) -> &str {
        self.theme.as_deref().unwrap_or("classic")
    }

    /// Returns true if the given theme name is valid.
    pub fn is_valid_theme(name: &str) -> bool {
        BUILTIN_THEMES.iter().any(|(n, _)| *n == name)
    }
}
