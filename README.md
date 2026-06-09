# sys-shred

`sys-shred` is a multi-threaded, forensic-grade Rust library and CLI for the irreversible destruction of sensitive files.

> [!CAUTION]
> **IRREVERSIBLE DATA DESTRUCTION NOTICE**
>
> Data processed by `sys-shred` is physically overwritten at the hardware level. Please verify your target paths carefully before execution. Once processed, data cannot be recovered by forensic software.

---

## Disclaimer

This project is not affiliated with any government or military entity. It is provided for legitimate data sanitization, privacy protection, and compliance purposes. Users are responsible for using this tool legally and ethically. The maintainers discourage any malicious or unauthorized usage and are not liable for any misuse.

## Key Features

- **Hardware-Level Sync:** Uses `fsync` and `sync_all` to bypass volatile OS caches and ensure data is written to physical media.
- **Memory Efficient:** A stream-based architecture that maintains a low RAM footprint even when processing millions of files.
- **SSD Optimization:** Supports hardware `TRIM` commands to mitigate wear-leveling artifacts on modern flash storage.
- **Parallel Performance:** Powered by `rayon` for massive parallel execution and high throughput.

> [!IMPORTANT]
> This is the official repository for `sys-shred`. Please report bugs and request features via [GitHub Issues](https://github.com/v1lleneuve/sys-shred/issues).

---

## Quick Start

For a detailed list of options, run `sys-shred --help`.

To perform a standard cryptographic overwrite on a file:
```bash
sys-shred target_file.txt
```

### Installation

**Via Cargo (Recommended):**
```bash
cargo install sys-shred
```

**Via AUR (Arch Linux):**
```bash
yay -S sys-shred
```

---

## Documentation Index

- [Erasure Algorithms](#erasure-algorithms)
  - [Standard Cryptographic](#standard-cryptographic)
  - [Military Grade (DoD)](#military-grade-dod)
  - [Maximum Security (Gutmann)](#maximum-security-gutmann)
- [Advanced Targeting](#advanced-targeting)
  - [Recursive Destruction](#recursive-destruction)
  - [Glob Exclusions](#glob-exclusions)
  - [Dry-Run Simulation](#dry-run-simulation)
- [Enterprise Features](#enterprise-features)
  - [SSD TRIM / Discard](#ssd-trim--discard)
  - [JSON Audit Logging](#json-audit-logging)
  - [Hardware Read-Back Verification](#hardware-read-back-verification)
- [Safety Guards](#safety-guards)

---

## Erasure Algorithms

### Standard Cryptographic
Overwrites data using three passes of cryptographically secure random entropy (default).
```bash
sys-shred confidential.pdf
```

### Military Grade (DoD)
Implements the US Department of Defense 5220.22-M standard (Pass 1: Zeros, Pass 2: Ones, Pass 3: Random).
```bash
sys-shred sensitive_data.bin --method dod
```

### Maximum Security (Gutmann)
The rigorous 35-pass Gutmann algorithm, designed for older magnetic media.
```bash
sys-shred classified_archive.tar.gz --method gutmann
```

---

## Advanced Targeting

### Recursive Destruction
Destroy entire directory trees using a highly optimized, lock-free parallel execution engine.
```bash
sys-shred ./project_folder --recursive
```

### Glob Exclusions
Exclude specific files or directories using wildcard patterns.
```bash
sys-shred ./server_logs --recursive --exclude "*.git/*"
```

### Dry-Run Simulation
Preview which files will be targeted without modifying the filesystem.
```bash
sys-shred ./directory --recursive --dry-run
```

---

## Enterprise Features

### SSD TRIM / Discard
Dispatches hardware-level block deallocation commands (`FALLOC_FL_PUNCH_HOLE` on Linux, `FSCTL_SET_ZERO_DATA` on Windows) to handle SSD wear-leveling.
```bash
sys-shred ./nvme_drive --method zero --trim
```

### JSON Audit Logging
Generate verifiable destruction reports for GDPR/HIPAA compliance.
```bash
sys-shred ./financials -r --audit-log ./report.json --audit-format json
```

### Hardware Verification
Validates destruction by reading physical blocks back into memory to ensure they were correctly overwritten.
```bash
sys-shred ./target --verify
```

---

## Safety Guards

- **Symlink Protection:** Isolates symbolic links, unlinking the reference without traversing or destroying the external target.
- **Interactive Prompts:** Confirmation prompts help prevent accidental recursive destruction. Use `--force` to bypass.

---

## Links

- [GitHub Repository](https://github.com/v1lleneuve/sys-shred)
- [Crates.io](https://crates.io/crates/sys-shred)
- [AUR Package](https://aur.archlinux.org/packages/sys-shred)

---

## License

Copyright (c) 2026 V1lleneuve. Licensed under the [MIT License](LICENSE).
