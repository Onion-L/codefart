use std::env;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;

use crate::error::CodefartError;

const REPO: &str = "Onion-L/codefart";

#[derive(Debug, Clone, serde::Serialize, PartialEq, Eq)]
pub struct DesktopUpdateInfo {
    pub current_version: String,
    pub latest_version: String,
    pub update_available: bool,
    pub release_url: String,
    pub download_url: Option<String>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct LatestRelease {
    tag_name: String,
    html_url: String,
    assets: Vec<ReleaseAsset>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct ReleaseAsset {
    name: String,
    browser_download_url: String,
}

/// Self-update codefart by downloading the latest release binary.
pub fn update() -> Result<String, CodefartError> {
    let current_exe = env::current_exe()
        .map_err(|e| CodefartError::Other(format!("cannot find current binary: {}", e)))?;

    let target = detect_target();
    let release_url = get_latest_release_url(target)?;

    eprintln!("Downloading {}...", release_url);

    let tmp_dir = tempfile::tempdir()
        .map_err(|e| CodefartError::Other(format!("cannot create temp dir: {}", e)))?;

    let archive_path = tmp_dir.path().join(archive_name(target));
    download_file(&release_url, &archive_path)?;

    // Extract
    extract_archive(&archive_path, tmp_dir.path())?;

    let new_binary = tmp_dir.path().join(binary_name());
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
    #[cfg(unix)]
    {
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
    }

    #[cfg(windows)]
    {
        let old_exe = current_exe.with_extension("exe.old");
        let _ = fs::remove_file(&old_exe);
        fs::rename(&current_exe, &old_exe)
            .map_err(|e| CodefartError::Other(format!("cannot rename current exe: {}", e)))?;
        if let Err(e) = fs::rename(&new_binary, &current_exe) {
            let _ = fs::rename(&old_exe, &current_exe);
            return Err(CodefartError::Other(format!(
                "cannot replace {}: {}",
                current_exe.display(),
                e
            )));
        }
        let _ = fs::remove_file(&old_exe);
    }

    Ok(current_exe.display().to_string())
}

/// Check whether a newer desktop release is available.
pub fn check_desktop_update(current_version: &str) -> Result<DesktopUpdateInfo, CodefartError> {
    let release = fetch_latest_release()?;
    Ok(desktop_update_info(current_version, release))
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
    #[cfg(all(target_os = "windows", target_arch = "x86_64"))]
    {
        "x86_64-pc-windows-msvc"
    }
    #[cfg(not(any(
        all(target_os = "macos", target_arch = "aarch64"),
        all(target_os = "macos", target_arch = "x86_64"),
        all(target_os = "linux", target_arch = "aarch64"),
        all(target_os = "linux", target_arch = "x86_64"),
        all(target_os = "windows", target_arch = "x86_64")
    )))]
    {
        "unsupported"
    }
}

fn archive_name(target: &str) -> String {
    if target.contains("windows") {
        "codefart.zip".into()
    } else {
        "codefart.tar.gz".into()
    }
}

fn binary_name() -> &'static str {
    #[cfg(windows)]
    {
        "codefart.exe"
    }
    #[cfg(not(windows))]
    {
        "codefart"
    }
}

fn extract_archive(archive: &std::path::Path, dest: &std::path::Path) -> Result<(), CodefartError> {
    #[cfg(unix)]
    {
        let status = Command::new("tar")
            .arg("xzf")
            .arg(archive)
            .arg("-C")
            .arg(dest)
            .status()
            .map_err(|e| CodefartError::Other(format!("tar failed: {}", e)))?;
        if !status.success() {
            return Err(CodefartError::Other("tar extraction failed".into()));
        }
    }

    #[cfg(windows)]
    {
        let status = Command::new("powershell")
            .arg("-Command")
            .arg(format!(
                "Expand-Archive -Path '{}' -DestinationPath '{}' -Force",
                archive.display(),
                dest.display()
            ))
            .status()
            .map_err(|e| CodefartError::Other(format!("Expand-Archive failed: {}", e)))?;
        if !status.success() {
            return Err(CodefartError::Other("zip extraction failed".into()));
        }
    }

    Ok(())
}

fn get_latest_release_url(target: &str) -> Result<String, CodefartError> {
    let release = fetch_latest_release()?;

    let filename = if target.contains("windows") {
        format!("codefart-{}.zip", target)
    } else {
        format!("codefart-{}.tar.gz", target)
    };

    release
        .assets
        .iter()
        .find(|asset| asset.name == filename)
        .map(|asset| asset.browser_download_url.clone())
        .ok_or_else(|| CodefartError::Other(format!("no release asset found for {}", target)))
}

fn fetch_latest_release() -> Result<LatestRelease, CodefartError> {
    let api_url = format!("https://api.github.com/repos/{}/releases/latest", REPO);

    let response = ureq::get(&api_url)
        .header("User-Agent", "codefart")
        .header("Accept", "application/json")
        .call()
        .map_err(|e| CodefartError::Other(format!("API request failed: {}", e)))?;

    let body = response
        .into_body()
        .read_to_string()
        .map_err(|e| CodefartError::Other(format!("read failed: {}", e)))?;

    serde_json::from_str(&body).map_err(|e| CodefartError::Other(format!("invalid JSON: {}", e)))
}

fn desktop_update_info(current_version: &str, release: LatestRelease) -> DesktopUpdateInfo {
    let latest_version = normalize_version(&release.tag_name);
    let asset_name = desktop_dmg_asset_name(&latest_version);
    let download_url = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .map(|asset| asset.browser_download_url.clone());

    DesktopUpdateInfo {
        current_version: normalize_version(current_version),
        latest_version: latest_version.clone(),
        update_available: is_newer_version(&latest_version, current_version),
        release_url: release.html_url,
        download_url,
    }
}

fn normalize_version(version: &str) -> String {
    version.trim().trim_start_matches('v').to_string()
}

fn is_newer_version(latest: &str, current: &str) -> bool {
    parse_version(latest) > parse_version(current)
}

fn parse_version(version: &str) -> Vec<u64> {
    normalize_version(version)
        .split('.')
        .map(|part| {
            part.chars()
                .take_while(|c| c.is_ascii_digit())
                .collect::<String>()
                .parse::<u64>()
                .unwrap_or(0)
        })
        .collect()
}

fn desktop_dmg_asset_name(version: &str) -> String {
    format!(
        "CodeFart_{}_{}.dmg",
        normalize_version(version),
        desktop_arch()
    )
}

fn desktop_arch() -> &'static str {
    #[cfg(target_arch = "aarch64")]
    {
        "aarch64"
    }
    #[cfg(target_arch = "x86_64")]
    {
        "x64"
    }
    #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
    {
        "unsupported"
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_update_info_reports_available_update_and_matching_dmg() {
        let release = LatestRelease {
            tag_name: "v0.2.23".to_string(),
            html_url: "https://github.com/Onion-L/codefart/releases/tag/v0.2.23".to_string(),
            assets: vec![
                ReleaseAsset {
                    name: "codefart-aarch64-apple-darwin.tar.gz".to_string(),
                    browser_download_url: "https://example.com/cli.tar.gz".to_string(),
                },
                ReleaseAsset {
                    name: desktop_dmg_asset_name("0.2.23").to_string(),
                    browser_download_url: "https://example.com/desktop.dmg".to_string(),
                },
            ],
        };

        let info = desktop_update_info("0.2.22", release);

        assert!(info.update_available);
        assert_eq!(info.current_version, "0.2.22");
        assert_eq!(info.latest_version, "0.2.23");
        assert_eq!(
            info.release_url,
            "https://github.com/Onion-L/codefart/releases/tag/v0.2.23"
        );
        assert_eq!(
            info.download_url.as_deref(),
            Some("https://example.com/desktop.dmg")
        );
    }

    #[test]
    fn desktop_update_info_handles_current_version() {
        let release = LatestRelease {
            tag_name: "v0.2.22".to_string(),
            html_url: "https://github.com/Onion-L/codefart/releases/tag/v0.2.22".to_string(),
            assets: vec![],
        };

        let info = desktop_update_info("0.2.22", release);

        assert!(!info.update_available);
        assert_eq!(info.latest_version, "0.2.22");
        assert_eq!(info.download_url, None);
    }
}
