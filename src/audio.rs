use std::io::Cursor;
use std::path::Path;

use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use rust_embed::RustEmbed;

use crate::config::Config;
use crate::error::CodefartError;

/// Embedded WAV sound files from the sounds/ directory.
#[derive(RustEmbed)]
#[folder = "sounds/"]
struct SoundAssets;

/// Resolve which sound to play based on config.
/// Priority: custom_sound > theme > default (classic).
fn resolve_sound(config: &Config) -> Result<Vec<u8>, CodefartError> {
    // 1. Custom sound file
    if let Some(ref path_str) = config.custom_sound {
        let expanded = shellexpand::tilde(path_str);
        let path = Path::new(expanded.as_ref());
        if !path.exists() {
            return Err(CodefartError::SoundFileNotFound(path_str.clone()));
        }
        return std::fs::read(path).map_err(|e| {
            CodefartError::AudioPlayback(format!("failed to read {}: {}", path_str, e))
        });
    }

    // 2. Built-in theme
    let theme = config.active_theme();
    let filename = format!("{}.wav", theme);
    SoundAssets::get(&filename)
        .map(|f| f.data.to_vec())
        .ok_or_else(|| CodefartError::UnknownTheme(theme.to_string()))
}

/// Play a specific built-in theme by name (ignores config).
pub fn play_theme(theme: &str) -> Result<(), CodefartError> {
    let filename = format!("{}.wav", theme);
    let audio_data = SoundAssets::get(&filename)
        .map(|f| f.data.to_vec())
        .ok_or_else(|| CodefartError::UnknownTheme(theme.to_string()))?;

    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| {
            CodefartError::AudioPlayback(format!("no audio device: {}", e))
        })?;

    let cursor = Cursor::new(audio_data);
    let source = Decoder::new(cursor).map_err(|e| {
        CodefartError::AudioPlayback(format!("failed to decode audio: {}", e))
    })?;

    let sink = Sink::try_new(&stream_handle).map_err(|e| {
        CodefartError::AudioPlayback(format!("failed to create sink: {}", e))
    })?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}

/// Play the configured sound. Blocks until playback completes.
/// Errors are returned but callers may choose to swallow them (silent fail).
pub fn play_sound(config: &Config) -> Result<(), CodefartError> {
    let audio_data = resolve_sound(config)?;

    // Set up audio output
    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| {
            CodefartError::AudioPlayback(format!("no audio device: {}", e))
        })?;

    // Decode from memory
    let cursor = Cursor::new(audio_data);
    let source = Decoder::new(cursor).map_err(|e| {
        CodefartError::AudioPlayback(format!("failed to decode audio: {}", e))
    })?;

    // Play and wait
    play_to_end(source, &stream_handle)?;

    Ok(())
}

/// Play a source to completion, blocking the current thread.
fn play_to_end(source: Decoder<Cursor<Vec<u8>>>, handle: &OutputStreamHandle) -> Result<(), CodefartError> {
    let sink = Sink::try_new(handle).map_err(|e| {
        CodefartError::AudioPlayback(format!("failed to create sink: {}", e))
    })?;
    sink.append(source);
    sink.sleep_until_end();
    Ok(())
}
