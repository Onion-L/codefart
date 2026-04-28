$repo = "Onion-L/codefart"
$binName = "codefart.exe"
$installDir = "$env:LOCALAPPDATA\codefart"

# Detect architecture
switch ($env:PROCESSOR_ARCHITECTURE) {
    "AMD64" { $target = "x86_64-pc-windows-msvc" }
    "ARM64" { $target = "aarch64-pc-windows-msvc" }
    default {
        Write-Error "Error: unsupported architecture: $env:PROCESSOR_ARCHITECTURE"
        exit 1
    }
}

Write-Host "Fetching latest release..."
$apiUrl = "https://api.github.com/repos/$repo/releases/latest"
try {
    $release = Invoke-RestMethod -Uri $apiUrl -Headers @{ "User-Agent" = "codefart" }
} catch {
    Write-Error "Failed to fetch release info: $_"
    exit 1
}

$asset = $release.assets | Where-Object { $_.name -eq "codefart-$target.zip" }

if (-not $asset) {
    Write-Error "Error: no release found for $target"
    exit 1
}

# Download
Write-Host "Downloading CodeFart..."
$tmpDir = New-Item -ItemType Directory -Path (Join-Path $env:TEMP ([System.Guid]::NewGuid().ToString()))
$zipPath = Join-Path $tmpDir "codefart.zip"
try {
    Invoke-WebRequest -Uri $asset.browser_download_url -OutFile $zipPath -UseBasicParsing
} catch {
    Write-Error "Download failed: $_"
    exit 1
}

# Extract
Expand-Archive -Path $zipPath -DestinationPath $tmpDir -Force

# Install
Write-Host "Installing to $installDir..."
New-Item -ItemType Directory -Path $installDir -Force | Out-Null
$sourcePath = Join-Path $tmpDir $binName
if (-not (Test-Path $sourcePath)) {
    Write-Error "Downloaded archive does not contain codefart.exe"
    exit 1
}
Move-Item -Path $sourcePath -Destination (Join-Path $installDir $binName) -Force

# Add to PATH
$userPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($userPath -notlike "*$installDir*") {
    [Environment]::SetEnvironmentVariable("Path", "$userPath;$installDir", "User")
    Write-Host "Added $installDir to your PATH. Restart your terminal to use codefart." -ForegroundColor Yellow
}

# Cleanup
Remove-Item -Recurse -Force $tmpDir

Write-Host ""
Write-Host "CodeFart installed successfully!" -ForegroundColor Green
Write-Host ""
Write-Host "To enable Claude notifications, run:"
Write-Host "  codefart setup"
