<div align="center">

# 🛡️ sys-shred

**Forensic-Grade Data Sanitization & Anti-Forensics Utility**

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg?style=flat-square)](https://github.com/v1lleneuve/sys-shred)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![Status: Production Stable](https://img.shields.io/badge/status-production_stable-brightgreen.svg?style=flat-square)](#)

*A high-performance, multi-threaded Rust engine engineered for the irreversible destruction of sensitive file data.*

</div>

---

> [!CAUTION]  
> **Irreversible Operation**  
> Data processed by `sys-shred` is physically overwritten at the hardware level and **cannot be recovered** by any known forensic software or hardware analysis. Use this utility with extreme caution, particularly when employing recursive targeting.

## 📖 Overview

`sys-shred` is an enterprise-ready command-line utility designed to provide absolute certainty in data destruction. Moving beyond simple unlinking, `sys-shred` bypasses standard operating system caching mechanisms to ensure that data is physically committed to the storage medium. It employs multiple cryptographic passes, scrubs metadata footprints, mitigates Flash/SSD wear-leveling artifacts via TRIM, and provides verifiable compliance through detailed audit logging.

Built with Rust and powered by a highly optimized, lock-free parallel execution engine, it handles millions of files with a near-zero memory footprint.

## ✨ Core Capabilities

### ⚡ Uncompromising Performance
- **Massive Parallelism**: Powered by a `rayon`-backed work-stealing thread pool, recursive directory destruction utilizes all available CPU cores, delivering up to 10x throughput scaling on modern hardware.
- **Stream-Based Architecture**: Iterates file systems lazily, completely eliminating RAM bloat regardless of directory size or depth.

### 🔐 Forensic Reliability
- **Industry Standard Algorithms**: Natively supports internationally recognized erasure standards:
  - `zero`: Single-pass zero-fill for rapid sanitization.
  - `random`: (Default) High-entropy cryptographic overwrite using `StdRng`.
  - `dod`: US Department of Defense 5220.22-M standard (3 passes).
  - `gutmann`: The rigorous Peter Gutmann method (35 passes) for legacy magnetic media.
- **Hardware-Level Sync**: Enforces strict `sync_all` (`fsync`/`FlushFileBuffers`) calls after every single write pass, bypassing volatile OS write-behind caches to guarantee physical platter/cell modification.
- **SSD TRIM/Discard Support**: Mitigates "wear-leveling" data shadows on Flash-based media by dispatching hardware-level block deallocation commands (`FALLOC_FL_PUNCH_HOLE` on Linux, `FSCTL_SET_ZERO_DATA` on Windows).
- **Metadata Scrubbing**: Eliminates forensic breadcrumbs by randomizing filenames (16-character alphanumeric strings) and truncating file allocations to zero bytes before final unlinking.

### 📋 Enterprise Compliance & Safety
- **Verifiable Audit Logging**: Generates comprehensive, cryptographic-proof destruction reports (`--audit-log`) in both human-readable `Text` and machine-readable `JSON` formats for regulatory compliance (GDPR, HIPAA).
- **Read-Back Verification**: Optional empirical validation (`--verify`) that reads the physical blocks back into memory to assert successful overwriting against the expected byte pattern.
- **Symlink Protection**: Intelligently identifies and isolates symbolic links, unlinking the reference without traversing or destroying the external target data.
- **Interactive Safeguards**: Professional confirmation prompts prevent accidental recursive disasters. Can be bypassed for automation via the `--force` flag.
- **Dry-Run Mode**: Safely simulate complex exclusion (`--exclude`) and recursive operations without modifying the filesystem.

---

## 🚀 Installation

Ensure you have the latest stable [Rust toolchain](https://rustup.rs/) installed.

### Arch Linux (AUR)
For Arch Linux and its derivatives (Manjaro, EndeavourOS), you can install the package directly from the AUR using your preferred helper:
```bash
yay -S sys-shred
```

### Crates.io (Recommended for other distros)
The easiest way to install `sys-shred` across platforms is via Cargo:
```bash
cargo install sys-shred
```

### Automated Scripts
Fastest method for standard environments.

**Linux / macOS:**
```bash
bash scripts/install.sh
```

**Windows (PowerShell):**
```powershell
.\scripts\install.ps1
```

### Manual Compilation
For environments requiring strict source compilation.
```bash
git clone https://github.com/v1lleneuve/sys-shred.git
cd sys-shred
cargo build --release --locked
```
The optimized executable will be located at `target/release/sys-shred`.

---

## 💻 Usage Guide

> [!TIP]  
> Execute `sys-shred --help` for the complete schema of flags and arguments.

### Foundational Operations

**Standard erasure (Cryptographic Random, 3 passes):**
```bash
sys-shred confidential.pdf
```

**Military-grade erasure (DoD 5220.22-M):**
```bash
sys-shred sensitive_data.bin --method dod
```

### Advanced Targeting

**Recursive multi-threaded destruction with hardware verification:**
```bash
sys-shred ./classified_folder --recursive --verify
```

**Safe simulation (Dry-Run) with glob-pattern exclusions:**
```bash
sys-shred ./project_root --recursive --dry-run --exclude "*.git/*" --exclude "*.log"
```

### Enterprise Audit & SSD Optimization

**Compliance-ready JSON audit report generation:**
```bash
sys-shred ./financial_records -r --audit-log ./compliance_report.json --audit-format json
```

**Modern SSD sanitization (Zero-fill + TRIM command):**
```bash
sys-shred ./nvme_target --method zero --trim
```

---

## 🏛️ Security Lifecycle Architecture

When `sys-shred` targets a file, it executes the following atomic sequence:

1. **Access Acquisition**: Secures read/write handles to the target.
2. **Algorithmic Overwrite**: Writes data streams (Zeros, Random, DoD, Gutmann) over the logical boundaries of the file.
3. **Hardware Flush (`fsync`)**: Commands the storage controller to commit the write cache to non-volatile memory.
4. **Empirical Verification** *(Optional)*: Reads the committed sectors back into RAM and asserts byte-for-byte equality against the expected overwrite pattern.
5. **Metadata Obfuscation**: Renames the file to a random string to obscure its original identity in the Master File Table (MFT) or inode structure.
6. **Block Deallocation** *(Optional)*: Issues a TRIM command to instruct the SSD controller to drop the physical mapping.
7. **Truncation**: Sets the file size to 0 bytes.
8. **Unlinking**: Severs the file from the directory tree, concluding the destruction.

---

## ⚖️ License

This software is distributed under the **MIT License**. See the `LICENSE` file for details.

*Engineered for reliability. Use responsibly.*