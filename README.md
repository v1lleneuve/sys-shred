# sys-shred

A high-integrity command-line utility for secure file erasure and anti-forensics in Rust.

## Overview

sys-shred is designed to irreversibly destroy file data by bypassing standard operating system caching mechanisms. It ensures that data is physically overwritten on the storage medium, metadata is obfuscated, and the file is securely unlinked from the filesystem.

## Features

- Multi-pass cryptographic overwriting using cryptographically secure random numbers.
- Hardware-level persistence enforcement via mandatory file synchronization.
- Filename and metadata obfuscation through random alphanumeric renaming.
- Automated file truncation to zero bytes before deletion.
- Real-time progress reporting for interactive terminal environments.

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

Standard shredding with default settings (3 passes):

```bash
sys-shred path/to/file.txt
```

Recursive directory shredding:

```bash
sys-shred path/to/directory --recursive
```

Customizing the number of overwrite passes:

```bash
sys-shred path/to/file.txt --passes 5
```

Obfuscating data without deleting the final file entry:

```bash
sys-shred path/to/file.txt --keep
```

## Security Model

The tool follows a multi-stage destruction lifecycle:

1. Data Destruction: The file is overwritten N times with random data generated from a secure entropy source.
2. Persistence: The tool calls `sync_all` after every pass to bypass OS write buffers.
3. Obfuscation: The filename is randomized to prevent path-based recovery.
4. Truncation: File length is set to zero to clear filesystem size metadata.
5. Unlinking: The file entry is removed from the directory structure.

## License

This project is licensed under the MIT License.
