# sys-shred installation script for Windows

$ErrorActionPreference = "Stop"

Write-Host "Starting sys-shred installation..." -ForegroundColor Cyan

# Check for Cargo/Rust installation
if (!(Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Error "Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
}

# Build the project in release mode
Write-Host "Building sys-shred in release mode..." -ForegroundColor Cyan
cargo build --release

# Define installation path (User's Local AppData)
$InstallDir = "$env:LOCALAPPDATA\Programs\sys-shred"
if (!(Test-Path $InstallDir)) {
    Write-Host "Creating installation directory..."
    New-Item -ItemType Directory -Path $InstallDir | Out-Null
}

# Copy binary
Write-Host "Copying binary to $InstallDir..."
Copy-Item "target\release\sys-shred.exe" "$InstallDir\sys-shred.exe" -Force

# Add to User PATH if not already present
$UserPath = [Environment]::GetEnvironmentVariable("Path", "User")
if ($UserPath -notlike "*$InstallDir*") {
    Write-Host "Adding $InstallDir to User PATH..."
    [Environment]::SetEnvironmentVariable("Path", "$UserPath;$InstallDir", "User")
    Write-Host "Please restart your terminal for PATH changes to take effect." -ForegroundColor Yellow
}

Write-Host "Installation complete." -ForegroundColor Green
