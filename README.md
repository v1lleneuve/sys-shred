# sys-shred

A high-integrity, multi-threaded command-line utility for secure file erasure and anti-forensics in Rust.

> [!WARNING]  
> **Irreversible Data Destruction**  
> Data processed by `sys-shred` is physically overwritten and cannot be recovered by forensic software. Use this utility with extreme caution, especially when using the recursive flag.

## Overview

`sys-shred` (v0.3.0) is a forensic-grade tool designed to irreversibly destroy file data by bypassing standard operating system caching mechanisms and utilizing parallel processing. It ensures that data is physically committed to the storage medium across multiple cryptographic passes, metadata is obfuscated, and the file entry is securely unlinked from the filesystem.

## Features

- **High-Performance Parallelism**: Powered by `rayon`, recursive operations utilize all available CPU cores for maximum throughput.
- **Industry Standard Algorithms**: Support for US DoD 5220.22-M, Gutmann (35-pass), and Zero-fill erasure methods.
- **Hardware-Level Verification**: Optional read-back verification (`--verify`) to confirm data persistence on the physical medium.
- **Dry-Run Mode**: Safely preview destruction sequences (`--dry-run`) before execution.
- **Hardware Persistence**: Mandatory `sync_all` (fsync) enforcement to bypass volatile OS write-behind caching.
- **Metadata Scrubbing**: Automated path randomization and file truncation to clear filesystem metadata.
- **Advanced Filtering**: Exclude specific files or directories using glob-based patterns (`--exclude`).

## Installation

Ensure you have the Rust toolchain installed.

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
cargo build --release
```
The compiled binary will be located at `target/release/sys-shred`.

## Usage

> [!TIP]  
> Run `sys-shred --help` for a full list of commands and configuration options.

**Standard file shredding (DoD 3-pass):**
```bash
sys-shred confidential.pdf --method dod
```

**Recursive multi-threaded shredding with verification:**
```bash
sys-shred ./sensitive_folder --recursive --verify
```

**Dry-run with exclusions:**
```bash
sys-shred ./project --recursive --dry-run --exclude "*.git/*"
```

**Overwriting without final deletion:**
```bash
sys-shred target.bin --keep --method zero
```

## Security Model

`sys-shred` follows a rigorous multi-stage destruction lifecycle:

1.  **Parallel Data Destruction**: Files are processed concurrently using selected algorithms (Random, DoD, Gutmann, or Zero).
2.  **Persistence Enforcement**: The tool forces a hardware flush (`sync_all`) after every pass to ensure data bypasses RAM buffers.
3.  **Read-back Verification**: (Optional) Data is read from the physical sectors to verify the overwrite was successful.
4.  **Metadata Obfuscation**: The filename is randomized to a 16-character alphanumeric string.
5.  **Truncation & Unlinking**: File length is set to zero bytes before the directory entry is removed.

> [!NOTE]  
> **SSD & Flash Storage Note**  
> On modern SSDs and Flash-based media, "wear leveling" and "TRIM" algorithms may move data to different physical blocks. While `sys-shred` attempts to overwrite the mapped logical blocks, hardware-level remapping is a physical characteristic of the drive. For maximum security on SSDs, the `--verify` flag is highly recommended.

## License

This project is licensed under the MIT License.
