use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use crate::error::CodefartError;

const REPO: &str = "Onion-L/codefart";

/// Self-update codefart by downloading the latest release binary.
pub fn update() -> Result<String, CodefartError> {
    let current_exe = env::current_exe()
        .map_err(|e| CodefartError::Other(format!("cannot find current binary: {}", e)))?;

    let target = detect_target();
    let release_url = get_latest_release_url(target)?;

    eprintln!("Downloading {}...", release_url);

    let tmp_dir = tempfile::tempdir()
        .map_err(|e| CodefartError::Other(format!("cannot create temp dir: {}", e)))?;

    let tarball_path = tmp_dir.path().join("codefart.tar.gz");
    download_file(&release_url, &tarball_path)?;

    // Extract
    let status = Command::new("tar")
        .arg("xzf")
        .arg(&tarball_path)
        .arg("-C")
        .arg(tmp_dir.path())
        .status()
        .map_err(|e| CodefartError::Other(format!("tar failed: {}", e)))?;

    if !status.success() {
        return Err(CodefartError::Other("tar extraction failed".into()));
    }

    let new_binary = tmp_dir.path().join("codefart");
    if !new_binary.exists() {
        return Err(CodefartError::Other(
            "downloaded archive does not contain codefart binary".into(),
        ));
    }

    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&new_binary)
            .map_err(|e| CodefartError::Other(format!("stat failed: {}", e)))?
            .permissions();
        perms.set_mode(0o755);
        fs::set_permissions(&new_binary, perms)
            .map_err(|e| CodefartError::Other(format!("chmod failed: {}", e)))?;
    }

    // Replace current binary
    if let Err(e) = fs::rename(&new_binary, &current_exe) {
        if e.kind() == io::ErrorKind::PermissionDenied {
            // Retry with sudo
            let status = Command::new("sudo")
                .arg("mv")
                .arg(&new_binary)
                .arg(&current_exe)
                .status()
                .map_err(|e| CodefartError::Other(format!("sudo failed: {}", e)))?;
            if !status.success() {
                return Err(CodefartError::Other("sudo mv failed".into()));
            }
        } else {
            return Err(CodefartError::Other(format!(
                "cannot replace {}: {}",
                current_exe.display(),
                e
            )));
        }
    }

    Ok(current_exe.display().to_string())
}

fn detect_target() -> &'static str {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    {
        "aarch64-apple-darwin"
    }
    #[cfg(all(target_os = "macos", target_arch = "x86_64"))]
    {
        "x86_64-apple-darwin"
    }
    #[cfg(all(target_os = "linux", target_arch = "aarch64"))]
    {
        "aarch64-unknown-linux-gnu"
    }
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    {
        "x86_64-unknown-linux-gnu"
    }
    #[cfg(not(any(
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "linux", target_arch = "x86_64")
    )))]
    {
        "unsupported"
    }
}

fn get_latest_release_url(target: &str) -> Result<String, CodefartError> {
    let api_url = format!(
        "https://api.github.com/repos/{}/releases/latest",
        REPO
    );

    let response = ureq::get(&api_url)
        .header("User-Agent", "codefart")
        .header("Accept", "application/json")
        .call()
        .map_err(|e| CodefartError::Other(format!("API request failed: {}", e)))?;

    let body = response
        .into_body()
        .read_to_string()
        .map_err(|e| CodefartError::Other(format!("read failed: {}", e)))?;
    let json: serde_json::Value = serde_json::from_str(&body)
        .map_err(|e| CodefartError::Other(format!("invalid JSON: {}", e)))?;

    let assets = json["assets"]
        .as_array()
        .ok_or_else(|| CodefartError::Other("no assets in release".into()))?;

    let filename = format!("codefart-{}.tar.gz", target);
    for asset in assets {
        if asset["name"].as_str() == Some(&filename) {
            return asset["browser_download_url"]
                .as_str()
                .map(|s: &str| s.to_string())
                .ok_or_else(|| CodefartError::Other("no download URL".into()));
        }
    }

    Err(CodefartError::Other(format!(
        "no release asset found for {}",
        target
    )))
}

fn download_file(url: &str, dest: &PathBuf) -> Result<(), CodefartError> {
    let response = ureq::get(url)
        .header("User-Agent", "codefart")
        .call()
        .map_err(|e| CodefartError::Other(format!("download failed: {}", e)))?;

    let mut reader = response.into_body().into_reader();
    let mut file = fs::File::create(dest)
        .map_err(|e| CodefartError::Other(format!("cannot create file: {}", e)))?;
    io::copy(&mut reader, &mut file)
        .map_err(|e| CodefartError::Other(format!("download failed: {}", e)))?;

    Ok(())
}
