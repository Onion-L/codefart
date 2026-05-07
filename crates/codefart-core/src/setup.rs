use std::path::PathBuf;

use crate::error::CodefartError;

const LEGACY_HOOK_COMMAND: &str = "codefart play";
const HOOK_COMMAND: &str = "codefart play; open -gj -a CodeFart --args --codefart-notify-completion >/dev/null 2>&1 || true";

/// Path to Claude Code's global settings file.
fn claude_settings_path() -> PathBuf {
    dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".claude")
        .join("settings.json")
}

/// The hook entry to inject for CodeFart.
/// Matches all Stop events (matcher: "") and plays sound through the CLI.
/// Desktop notification is delegated to CodeFart.app when it is installed.
fn codefart_hook_entry() -> serde_json::Value {
    serde_json::json!({
        "matcher": "",
        "hooks": [{
            "type": "command",
            "command": HOOK_COMMAND
        }]
    })
}

fn is_codefart_hook_command(command: &str) -> bool {
    command == HOOK_COMMAND || command == LEGACY_HOOK_COMMAND
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
                            h.get("command")
                                .and_then(|c| c.as_str())
                                .map(is_codefart_hook_command)
                                .unwrap_or(false)
                        })
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

/// Remove CodeFart's Stop hook from Claude's settings.json.
///
/// Returns Ok(true) if hook was removed, Ok(false) if not present.
pub fn uninstall_hook() -> Result<bool, CodefartError> {
    let path = claude_settings_path();
    if !path.exists() {
        return Ok(false);
    }
    let content = std::fs::read_to_string(&path).map_err(CodefartError::ClaudeSettingsRead)?;
    let mut settings: serde_json::Value =
        serde_json::from_str(&content).map_err(CodefartError::ClaudeSettingsParse)?;

    let hooks_obj = match settings.get_mut("hooks") {
        Some(h) => h,
        None => return Ok(false),
    };
    let stop_arr = match hooks_obj.get_mut("Stop").and_then(|s| s.as_array_mut()) {
        Some(arr) => arr,
        None => return Ok(false),
    };

    let before = stop_arr.len();
    stop_arr.retain(|entry| {
        !entry
            .get("hooks")
            .and_then(|h| h.as_array())
            .map(|hooks| {
                hooks.iter().any(|h| {
                    h.get("command")
                        .and_then(|c| c.as_str())
                        .map(is_codefart_hook_command)
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    });

    if stop_arr.len() == before {
        return Ok(false);
    }

    // Clean up empty Stop / hooks objects
    if stop_arr.is_empty()
        && let Some(obj) = hooks_obj.as_object_mut()
    {
        obj.remove("Stop");
        if obj.is_empty()
            && let Some(root) = settings.as_object_mut()
        {
            root.remove("hooks");
        }
    }

    let content = serde_json::to_string_pretty(&settings)?;
    std::fs::write(&path, content).map_err(CodefartError::ClaudeSettingsWrite)?;
    Ok(true)
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

    if has_current_codefart_hook(&settings) {
        return Ok(false);
    }

    remove_codefart_hooks(&mut settings);

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

fn has_current_codefart_hook(settings: &serde_json::Value) -> bool {
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
                            h.get("command").and_then(|c| c.as_str()) == Some(HOOK_COMMAND)
                        })
                    })
                    .unwrap_or(false)
            })
        })
        .unwrap_or(false)
}

fn remove_codefart_hooks(settings: &mut serde_json::Value) -> bool {
    let hooks_obj = match settings.get_mut("hooks") {
        Some(h) => h,
        None => return false,
    };
    let stop_arr = match hooks_obj.get_mut("Stop").and_then(|s| s.as_array_mut()) {
        Some(arr) => arr,
        None => return false,
    };

    let before = stop_arr.len();
    stop_arr.retain(|entry| {
        !entry
            .get("hooks")
            .and_then(|h| h.as_array())
            .map(|hooks| {
                hooks.iter().any(|h| {
                    h.get("command")
                        .and_then(|c| c.as_str())
                        .map(is_codefart_hook_command)
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false)
    });

    before != stop_arr.len()
}

#[cfg(test)]
mod tests {
    use super::{
        HOOK_COMMAND, LEGACY_HOOK_COMMAND, has_codefart_hook, has_current_codefart_hook,
        remove_codefart_hooks,
    };

    #[test]
    fn detects_current_hook_command() {
        let settings = serde_json::json!({
            "hooks": {
                "Stop": [{
                    "matcher": "",
                    "hooks": [{
                        "type": "command",
                        "command": HOOK_COMMAND
                    }]
                }]
            }
        });

        assert!(has_codefart_hook(&settings));
        assert!(has_current_codefart_hook(&settings));
    }

    #[test]
    fn removes_legacy_hook_command() {
        let mut settings = serde_json::json!({
            "hooks": {
                "Stop": [{
                    "matcher": "",
                    "hooks": [{
                        "type": "command",
                        "command": LEGACY_HOOK_COMMAND
                    }]
                }]
            }
        });

        assert!(has_codefart_hook(&settings));
        assert!(remove_codefart_hooks(&mut settings));
        assert!(!has_codefart_hook(&settings));
    }
}
