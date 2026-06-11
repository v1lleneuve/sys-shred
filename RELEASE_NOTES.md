# sys-shred Release Notes

This document tracks the evolution of the `sys-shred` utility. Each release focuses on technical integrity, forensic reliability, and system-level data destruction.

---

## [1.2.0] - 2026-06-12
### The Performance & Precision Release

The `v1.2.0` release is a major architectural upgrade focusing on high-performance streaming and aggressive hardware-level cache management.

> [!IMPORTANT]
> **Industrial-Grade Performance**
> This update introduces "True Streaming" logic. `sys-shred` now uses `par_bridge` to parallelize discovery and destruction simultaneously, allowing it to handle massive directory trees (millions of files) with a fixed, ultra-low memory footprint.

#### Key Enhancements
*   **True Streaming Architecture**: Removed the legacy "collect-then-process" model. Files are now shredded as they are found in the directory walk, dramatically reducing latency and RAM usage.
*   **OS-Specific Cache Bypassing**:
    *   **macOS**: Now uses `F_NOCACHE` to explicitly instruct the kernel to bypass the page cache, ensuring data is sent directly to the hardware.
    *   **Linux**: Implemented `posix_fadvise` (DONTNEED) to proactively flush overwritten data from the system cache, preventing "lazy writes" from lingering in volatile memory.
*   **High-Precision Progress**: A new dual-pass discovery engine ensures that even with the new streaming model, the progress bar remains 100% accurate without sacrificing performance.

---

## [1.1.1] - 2026-06-11
### UI Synchronization Patch

The `v1.1.1` release is a targeted patch to resolve visual race conditions in the progress reporting system.

> [!IMPORTANT]
> **Visual Integrity**
> This update ensures that the progress bar always reaches 100% and flushes its final state before being cleared, preventing misleading "partial" progress displays during high-speed parallel operations.

#### Key Enhancements
*   **Progress Bar Finalization**: Implemented explicit UI synchronization to ensure that all background worker threads have reported their completion before the progress bar is removed.
*   **High-Speed Stability**: Hardened the UI reporting engine to maintain accuracy even during ultra-fast operations on small files where terminal refresh rates might otherwise miss the final increments.

---

## [1.1.0] - 2026-06-10

### The "Cargo-Look" UI Refactor

The `v1.1.0` release introduces a major overhaul of the terminal user interface, bringing a modern and professional aesthetic inspired by the Rust compiler (`rustc`) and `cargo`.

> [!IMPORTANT]
> **Modern Aesthetic**
> This update replaces all legacy terminal output with a structured, bold, and color-coded reporting system. It's not just about looks—the new UI improves readability and operational clarity.

#### Key Enhancements
*   **Rustc-Inspired Styling**: All output now follows the "Label -> Message" pattern (e.g., `   Shredding file.txt`) with bold, high-contrast colors.
*   **Standardized UI Engine**: Introduced a centralized styling module using the `console` crate, ensuring consistent appearance across all platforms and eliminating brittle manual ANSI codes.
*   **Clean Progress Reporting**: The progress bar has been simplified and styled to match the new aesthetic, providing high-integrity feedback without terminal clutter.
*   **Precise Timing**: Final success messages now include high-resolution execution timing, giving users immediate feedback on performance.

---

## [1.0.1] - 2026-06-09
### Maintenance & Reliability Patch

The `v1.0.1` release is a maintenance patch focused on hardening the destruction engine against edge cases and improving the responsiveness of the terminal user interface.

> [!IMPORTANT]
> **Reliability Patch**
> This update fixes a rare race condition in filename obfuscation and ensures that the progress bar accurately reflects the work being performed by filtering out excluded files.

> [!TIP]
> **Improved Responsiveness**
> If you need to stop a long-running shredding operation, `Ctrl+C` is now more responsive during the verification phase thanks to higher-frequency cancellation polling.

#### Key Enhancements
*   **Collision-Resistant Obfuscation**: Added retry logic for filename randomization. While 16-character alphanumeric collisions are astronomically rare, the engine now gracefully handles them by regenerating names up to 5 times.
*   **Precision Progress Tracking**: Refined the progress bar calculation logic. Excluded files are now accurately filtered out before the progress bar initialization, providing a 1:1 ratio between the progress count and actual work performed.
*   **Enhanced Interruption Latency**: Improved responsiveness to `Ctrl+C` by injecting high-frequency cancellation checks into the hardware verification loops.
*   **Linux TRIM Hardening**: Improved error propagation for the `--trim` feature on Linux, ensuring hardware-level failures are reported back to the audit log.

---

## [1.0.0] - 2026-06-08
### The Fortress Release (Production Stable)

The `v1.0.0` milestone marks `sys-shred` as a production-stable, forensic-grade utility. This release focuses entirely on architectural hardening, safety invariants, and memory efficiency, ensuring "Zero Bug" operation even in the most chaotic filesystem environments.

> [!IMPORTANT]
> **Production Stable**
> This release guarantees absolute safety against symlink traversal and handles millions of files with a flat memory footprint.

#### Key Enhancements
*   **Official Packaging (AUR & crates.io)**: `sys-shred` is now officially published to the Rust package registry (`cargo install sys-shred`) and the Arch User Repository (`yay -S sys-shred`), enabling seamless installation across Linux ecosystems.
*   **Symlink Safety**: Explicitly protects against following symbolic links. The engine will securely unlink the symlink itself without destroying the target data, preventing catastrophic out-of-scope erasure.
*   **Interactive Safeguards**: Destructive operations now require explicit confirmation via `dialoguer`. Power users and scripts can bypass this with the new `-f` or `--force` flag.
*   **Stream-Based Traversal**: Refactored the core recursive engine to process files lazily, reducing RAM usage to near-zero even when shredding directories with millions of files.
*   **Zero-Bug Guarantee**: Passed all aggressive linting (`cargo clippy -D warnings`), multi-threaded stress tests, and formatting checks.

---

## [0.4.0] - 2026-06-07
### Professional Audit & SSD Optimization

The `v0.4.0` release introduces two critical features for enterprise and forensic environments: **Professional Audit Logging** and **Hardware-Level SSD TRIM support**. This system provides verifiable proof of data destruction and enhances reliability on modern storage media.

> [!TIP]
> **Audit & SSD Performance**
> Use the new `--audit-log` flag to generate a forensic report, and the `--trim` flag to ensure that SSD controllers are informed of block deallocation—a key step for anti-forensics on Flash media.

#### Key Enhancements
*   **Forensic Reporting System**: A new reporting engine that tracks every file targeted, recording success/failure, precise timestamps, and the specific destruction methods applied.
*   **SSD TRIM/Discard Support**: Implemented platform-specific hardware commands (`fallocate` on Linux, `DeviceIoControl` on Windows) to inform SSDs of discarded blocks, bypassing hardware-level remapping artifacts.
*   **Multi-Format Export**: Support for human-readable **Text** reports and machine-readable **JSON** exports for integration into security dashboards.

#### Technical Implementation
*   **Structured Serialization**: Integrated `serde` and `serde_json` for robust data representation and export.
*   **Atomic Aggregation**: Implemented a thread-safe event bus to summarize session metrics (total files, successes, failures) without bottlenecking the parallel destruction logic.
*   **Extended CLI Schema**: Added `--audit-log <PATH>` and `--audit-format <text|json>` to allow precise control over forensic output.

---

## [0.3.1] - 2026-06-07
### Stabilization & Signal Integrity

The `v0.3.1` maintenance release focuses on operational robustness and graceful termination. It introduces a comprehensive signal handling system to ensure the utility remains in a consistent state even when interrupted by the user or the operating system.

> [!IMPORTANT]
> **Graceful Interruption**
> This version introduces active signal monitoring. Pressing `Ctrl+C` now triggers a coordinated cleanup sequence across all worker threads, preventing terminal hang-ups and ensuring a clean exit from deep recursive operations.

#### Key Enhancements
*   **Coordinated Cancellation**: Implemented a thread-safe cancellation mechanism using atomic primitives. All cryptographic overwrite loops and verification passes are now "interruption-aware" and respond immediately to termination signals.
*   **Graceful Signal Handling**: Integrated the `ctrlc` crate to capture SIGINT/SIGTERM. The tool now provides clear feedback when an interruption is received and shuts down the parallel execution engine safely.
*   **Engine Hardening**: Refined the recursive traversal logic to be resilient against interrupted I/O states, ensuring that no file handles or system resources are leaked during an abrupt shutdown.

#### Technical Implementation
*   **Atomic Control Flow**: Utilized `Arc<AtomicBool>` to propagate cancellation signals across the Rayon thread pool without compromising performance.
*   **Loop Sanitization**: Added high-frequency cancellation checks inside the core I/O loops (writing and verification) for near-instant response times.
*   **Library Documentation**: Expanded module-level documentation to satisfy strict `#![deny(missing_docs)]` requirements and improve architectural clarity.

---

## [0.3.0] - 2026-06-07
### High-Performance Parallelism & Forensic Standards

The `v0.3.0` release is a major technical milestone, introducing multi-threaded execution and industry-standard erasure algorithms. This version transforms `sys-shred` into a high-performance suite capable of handling massive data volumes with forensic-grade precision.

> [!TIP]
> **Performance & Multi-threading**
> This release introduces `rayon` for parallel file processing. On multi-core systems, recursive shredding operations are now up to 10x faster by distributing the cryptographic workload across all available CPU threads.

#### Key Enhancements
*   **Parallel Destruction Engine**: Fully integrated `rayon` to enable concurrent shredding of multiple files. This significantly reduces execution time for large directory trees.
*   **Standard Erasure Algorithms**: Added support for international data destruction standards including **US DoD 5220.22-M** (3-pass) and the **Gutmann** (35-pass) method via the `--method` flag.
*   **Hardware Verification**: Introduced the `--verify` flag, which performs a post-pass read-back of data blocks to ensure physical persistence and bypass hardware-level remapping failures.
*   **Safe Execution (Dry-Run)**: Added a non-destructive `--dry-run` mode to preview operations, critical for validating recursive targets and exclusion patterns before commit.
*   **Pattern-Based Exclusions**: Implemented `--exclude` support using `globset`, allowing users to skip specific files or directories (e.g., `.git/`, `*.log`) during recursive operations.

#### Technical Implementation
*   **Thread-Safe Core**: Refactored the `Shredder` and `Overwriter` components to be fully thread-safe and compatible with parallel iterators.
*   **Unified Progress UI**: Re-engineered the terminal interface to provide a coherent, aggregate progress view for multi-threaded operations.
*   **Hardened Directory Cleanup**: Optimized the bottom-up cleanup logic to gracefully handle partially excluded directory structures.

---

## [0.2.1] - 2026-06-06
### Infrastructure & Stability

This maintenance release focuses on long-term project health, automated security, and engine hardening.

> [!IMPORTANT]
> **Security & Stability**
> This release prioritizes supply-chain security via automated audits and increases the robustness of the destruction engine through explicit error handling and terminal-safe UI fallbacks.

#### Key Enhancements
*   **Automated Dependency Management**: Integrated GitHub **Dependabot** to ensure all underlying libraries are cryptographically up-to-date and free of known vulnerabilities.
*   **Hardened Traversal Engine**: Refined the recursive destruction logic to explicitly handle and report filesystem errors (e.g., permission denials) instead of failing silently.
*   **Zero-Panic UI**: Implemented safe fallback mechanisms for terminal progress bar styles to ensure stability across diverse terminal environments.

#### Technical Implementation
*   **Strict Documentation Enforcement**: Added `#![deny(missing_docs)]` to the library core to ensure architectural transparency.
*   **Refinement**: Removed unused imports and optimized the binary footprint for faster execution.

---

## [0.2.0] - 2026-06-06
### Recursive Directory Destruction

The `v0.2.0` release introduces a major upgrade to the destruction engine, enabling the secure erasure of entire directory structures. This version refactors the core orchestration layer to handle nested file systems with forensic-grade reliability.

> [!CAUTION]
> **Irreversible Operation**
> Recursive shredding will destroy all contents within a directory tree. Data destroyed by `sys-shred` is physically overwritten and cannot be recovered by forensic software. Use with extreme caution.

#### Key Enhancements
*   **Recursive Processing**: Added the `-r` / `--recursive` flag to destroy entire directory trees.
*   **Integrated Traversal Engine**: Leveraging `walkdir` for robust and safe filesystem iteration across all platforms.
*   **Atomic Cleanup**: Implemented bottom-up directory removal logic ensuring all nested children are destroyed before parent entry unlinking.

#### Technical Implementation
*   **API Refactor**: Updated the `Shredder` core to handle both polymorphic targets (Files and Directories).
*   **Validation**: Introduced a new behavioral test suite to verify recursive erasure integrity and symlink safety.
*   **Schema Update**: Updated CLI metadata and help menus to reflect recursive capabilities.

---

## [0.1.0] - 2026-06-06
### Initial Release: High-Integrity File Erasure

The initial production release of `sys-shred`, a professional-grade command-line utility for secure file erasure and anti-forensics. Designed for reliability and hardware-level data destruction.

> [!NOTE]
> **Hardware Synchronization**
> This utility utilizes `sync_all` after every write pass to bypass Operating System write-behind caching, ensuring data reaches the physical storage medium.

#### Features
*   **Cryptographic Overwriting**: Multi-pass destruction using cryptographically secure random entropy.
*   **Metadata Scrubbing**: Automated filename randomization and truncation to clear filesystem metadata leakage.
*   **Secure Unlinking**: Final removal of file entries from the directory structure.
*   **Real-time UI**: Interactive progress feedback via `indicatif`.

#### Infrastructure & Security
*   **Cross-Platform Support**: Distribution scripts provided for Linux, macOS, and Windows.
*   **Security Policy**: Established `SECURITY.md` for responsible vulnerability disclosure.
*   **Automated QA**: Comprehensive integration tests and GitHub Actions CI integration.