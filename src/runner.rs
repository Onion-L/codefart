use std::process::{self, ExitStatus};

use crate::error::CodefartError;

/// Execute a command, wait for it to finish, then return its exit status.
/// stdin/stdout/stderr are all inherited so interactive programs work.
pub fn run_command(cmd: &str, args: &[String]) -> Result<ExitStatus, CodefartError> {
    let status = process::Command::new(cmd)
        .args(args)
        .stdin(process::Stdio::inherit())
        .stdout(process::Stdio::inherit())
        .stderr(process::Stdio::inherit())
        .status()
        .map_err(|e| CodefartError::Other(format!("failed to run '{}': {}", cmd, e)))?;

    Ok(status)
}

/// Convert an ExitStatus to an exit code:
/// - Normal exit → exit code as-is
/// - Signal termination → 128 + signal number
pub fn status_to_code(status: ExitStatus) -> i32 {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(signal) = status.signal() {
            return 128 + signal;
        }
    }
    status.code().unwrap_or(1)
}
