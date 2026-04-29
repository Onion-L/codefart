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

    /// Set the active built-in theme.
    pub fn set_theme(&mut self, name: &str) -> Result<(), CodefartError> {
        if !Self::is_valid_theme(name) {
            return Err(CodefartError::UnknownTheme(name.to_string()));
        }

        self.theme = Some(name.to_string());
        Ok(())
    }

    /// Copy a custom sound into CodeFart's managed config directory.
    pub fn set_custom_sound_from_path(&mut self, path: &str) -> Result<(), CodefartError> {
        let expanded = shellexpand::tilde(path);
        let source = std::path::Path::new(expanded.as_ref());

        if !source.exists() {
            return Err(CodefartError::SoundFileNotFound(path.to_string()));
        }

        let sounds_dir = Self::sounds_dir();
        std::fs::create_dir_all(&sounds_dir)
            .map_err(|e| CodefartError::Other(format!("cannot create sounds dir: {}", e)))?;

        let filename = source
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("custom");
        let dest = sounds_dir.join(filename);
        std::fs::copy(source, &dest)
            .map_err(|e| CodefartError::Other(format!("cannot copy sound file: {}", e)))?;

        self.custom_sound = Some(dest.to_string_lossy().to_string());
        Ok(())
    }

    /// Clear custom sound config and remove managed custom sounds.
    pub fn clear_custom_sound(&mut self) {
        self.custom_sound = None;

        let sounds_dir = Self::sounds_dir();
        if sounds_dir.exists() {
            let _ = std::fs::remove_dir_all(&sounds_dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[test]
    fn set_theme_rejects_unknown_theme() {
        let mut config = Config::default();

        let result = config.set_theme("missing");

        assert!(result.is_err());
        assert_eq!(config.theme, None);
    }

    #[test]
    fn set_theme_stores_valid_theme() {
        let mut config = Config::default();

        config.set_theme("wet").unwrap();

        assert_eq!(config.theme.as_deref(), Some("wet"));
    }
}
