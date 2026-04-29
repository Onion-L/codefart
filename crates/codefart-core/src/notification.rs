use crate::config::Config;
use crate::error::CodefartError;

/// Show the configured completion notification.
///
/// Notifications are currently supported only on macOS. Other platforms are a no-op.
pub fn notify_completion(config: &Config) -> Result<(), CodefartError> {
    if !config.notification_enabled() {
        return Ok(());
    }

    notify(config.notification_title(), config.notification_body())
}

#[cfg(target_os = "macos")]
fn notify(title: &str, body: &str) -> Result<(), CodefartError> {
    let script = format!(
        "display notification \"{}\" with title \"{}\"",
        escape_applescript_string(body),
        escape_applescript_string(title)
    );

    let status = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .status()
        .map_err(|e| CodefartError::Other(format!("notification failed: {}", e)))?;

    if !status.success() {
        return Err(CodefartError::Other("notification failed".into()));
    }

    Ok(())
}

#[cfg(not(target_os = "macos"))]
fn notify(_title: &str, _body: &str) -> Result<(), CodefartError> {
    Ok(())
}

#[cfg(target_os = "macos")]
fn escape_applescript_string(value: &str) -> String {
    value.replace('\\', "\\\\").replace('"', "\\\"")
}

#[cfg(all(test, target_os = "macos"))]
mod tests {
    use super::escape_applescript_string;

    #[test]
    fn escapes_applescript_string_quotes_and_backslashes() {
        assert_eq!(escape_applescript_string(r#"a "b" \ c"#), r#"a \"b\" \\ c"#);
    }
}
