use std::path::PathBuf;

use crate::error::CodefartError;

/// Path to Claude Code's global settings file.
fn claude_settings_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude")
        .join("settings.json")
}

/// The hook entry to inject for CodeFart.
/// Matches all Stop events (matcher: "") and runs `codefart play`.
fn codefart_hook_entry() -> serde_json::Value {
    serde_json::json!({
        "matcher": "",
        "hooks": [{
            "type": "command",
            "command": "codefart play"
        }]
    })
}

/// Check whether CodeFart's hook is installed in Claude settings.
pub fn check_hook_installed() -> Result<bool, CodefartError> {
    let path = claude_settings_path();
    if !path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(&path).map_err(CodefartError::ClaudeSettingsRead)?;
    let settings: serde_json::Value =
        serde_json::from_str(&content).map_err(CodefartError::ClaudeSettingsParse)?;
    Ok(has_codefart_hook(&settings))
}

/// Check whether CodeFart's hook is already present in the settings.
pub fn has_codefart_hook(settings: &serde_json::Value) -> bool {
    settings
        .get("hooks")
        .and_then(|h| h.get("Stop"))
        .and_then(|arr| arr.as_array())
        .map(|arr| {
            arr.iter().any(|entry| {
                entry
                    .get("hooks")
                    .and_then(|h| h.as_array())
                    .map(|hooks| {
                        hooks.iter().any(|h| {
                            h.get("command").and_then(|c| c.as_str()) == Some("codefart play")
                        })
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Inject the CodeFart Stop hook into Claude's settings.json.
///
/// Returns Ok(true) if hook was added, Ok(false) if already present.
pub fn install_hook() -> Result<bool, CodefartError> {
    let path = claude_settings_path();

    // Read existing settings (or start fresh)
    let mut settings: serde_json::Value = if path.exists() {
        let content = std::fs::read_to_string(&path).map_err(CodefartError::ClaudeSettingsRead)?;
        serde_json::from_str(&content).map_err(CodefartError::ClaudeSettingsParse)?
    } else {
        serde_json::json!({})
    };

    // Check if already installed
    if has_codefart_hook(&settings) {
        return Ok(false);
    }

    // Ensure hooks.Stop array exists
    let hooks = settings
        .as_object_mut()
        .ok_or_else(|| CodefartError::Other("invalid claude settings format".into()))?
        .entry("hooks")
        .or_insert_with(|| serde_json::json!({}));

    let stop_arr = hooks
        .as_object_mut()
        .ok_or_else(|| CodefartError::Other("invalid hooks format".into()))?
        .entry("Stop")
        .or_insert_with(|| serde_json::json!([]));

    // Add our hook entry
    if let Some(arr) = stop_arr.as_array_mut() {
        arr.push(codefart_hook_entry());
    } else {
        return Err(CodefartError::Other("hooks.Stop is not an array".into()));
    }

    // Write back
    let content = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&path, content).map_err(CodefartError::ClaudeSettingsWrite)?;

    Ok(true)
}
