# sys-shred

A high-integrity command-line utility for secure file erasure and anti-forensics in Rust.

> [!WARNING]  
> **Irreversible Data Destruction**  
> Data processed by `sys-shred` is physically overwritten and cannot be recovered by forensic software. Use this utility with extreme caution, especially when using the recursive flag.

## Overview

`sys-shred` is a forensic-grade tool designed to irreversibly destroy file data by bypassing standard operating system caching mechanisms. It ensures that data is physically committed to the storage medium, metadata is obfuscated, and the file entry is securely unlinked from the filesystem.

## Features

- **Cryptographic Overwriting**: Multi-pass destruction using cryptographically secure random entropy.
- **Recursive Shredding**: Securely destroy entire directory trees and their contents.
- **Hardware Persistence**: Mandatory `sync_all` enforcement to bypass OS write-behind caching.
- **Metadata Scrubbing**: Automated filename randomization and truncation to clear filesystem metadata.
- **Interactive UI**: Real-time progress reporting for granular visibility.

## Installation

Ensure you have the Rust toolchain installed. You can install `sys-shred` using the provided automated scripts or by building it manually.

### Automated Installation

**Linux and macOS:**
```bash
bash scripts/install.sh
```

**Windows (PowerShell):**
```powershell
.\scripts\install.ps1
```

### Manual Build

```bash
git clone https://github.com/v1lleneuve/sys-shred
cd sys-shred
cargo build --release
```
The compiled binary will be located at `target/release/sys-shred`.

## Usage

> [!TIP]  
> Run `sys-shred --help` for a full list of commands and configuration options.

**Standard file shredding (3 passes):**
```bash
sys-shred confidential_document.pdf
```

**Recursive directory shredding:**
```bash
sys-shred ./sensitive_folder --recursive
```

**Overwriting without final deletion:**
```bash
sys-shred verification_target.bin --keep
```

## Security Model

`sys-shred` follows a rigorous multi-stage destruction lifecycle:

1.  **Data Destruction**: The file is overwritten $N$ times with random data generated from a secure entropy source.
2.  **Persistence**: The tool forces a hardware flush after every pass to ensure data bypasses volatile RAM buffers.
3.  **Obfuscation**: The filename is randomized to a 16-character alphanumeric string to prevent path-based recovery.
4.  **Truncation**: File length is set to zero bytes to clear filesystem size metadata.
5.  **Unlinking**: The file entry is removed from the directory structure.

> [!NOTE]  
> **SSD & Flash Storage Note**  
> On modern SSDs and Flash-based media, "wear leveling" and "TRIM" algorithms may move data to different physical blocks. While `sys-shred` attempts to overwrite the mapped logical blocks, hardware-level remapping is a physical characteristic of the drive.

## License

This project is licensed under the MIT License.
