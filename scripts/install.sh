#!/bin/bash
# sys-shred installation script for Unix-like systems (Linux and macOS)

set -e

echo "Starting sys-shred installation..."

# Check for Cargo/Rust installation
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo is not installed. Please install Rust from https://rustup.rs/"
    exit 1
fi

# Build the project in release mode
echo "Building sys-shred in release mode..."
cargo build --release

# Determine the installation directory
INSTALL_DIR="/usr/local/bin"
if [ ! -d "$INSTALL_DIR" ]; then
    echo "Creating installation directory $INSTALL_DIR..."
    sudo mkdir -p "$INSTALL_DIR"
fi

# Install the binary
echo "Installing binary to $INSTALL_DIR/sys-shred..."
sudo cp target/release/sys-shred "$INSTALL_DIR/sys-shred"
sudo chmod +x "$INSTALL_DIR/sys-shred"

echo "Installation complete. You can now use 'sys-shred' from your terminal."
