use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodefartError {
    #[error("failed to read config: {0}")]
    ConfigRead(#[source] std::io::Error),

    #[error("failed to parse config: {0}")]
    ConfigParse(#[from] toml::de::Error),

    #[error("failed to serialize config: {0}")]
    ConfigSerialize(#[from] toml::ser::Error),

    #[error("failed to write config: {0}")]
    ConfigWrite(#[source] std::io::Error),

    #[error("unknown theme '{0}'. run `codefart list` to see available themes")]
    UnknownTheme(String),

    #[error("custom sound file not found: {0}")]
    SoundFileNotFound(String),

    #[error("failed to play audio: {0}")]
    AudioPlayback(String),

    #[error("failed to read claude settings: {0}")]
    ClaudeSettingsRead(#[source] std::io::Error),

    #[error("failed to parse claude settings: {0}")]
    ClaudeSettingsParse(#[from] serde_json::Error),

    #[error("failed to write claude settings: {0}")]
    ClaudeSettingsWrite(#[source] std::io::Error),

    #[error("{0}")]
    Other(String),
}

/// Errors that should be silent (don't print, just exit 0).
/// Audio failures should never interrupt the calling process.
impl CodefartError {
    pub fn is_silent(&self) -> bool {
        matches!(
            self,
            CodefartError::SoundFileNotFound(_) | CodefartError::AudioPlayback(_)
        )
    }
}
