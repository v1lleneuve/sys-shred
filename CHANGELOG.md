# Changelog

All notable changes to this project will be documented in this file.

## [1.0.0] - 2026-06-07

### Added
- **Official crates.io Release**: `sys-shred` is now officially available on crates.io and can be installed globally via `cargo install sys-shred`.
- **Production Stable**: Achieved v1.0.0 milestone. The tool is now considered production-ready with zero known bugs, zero compiler warnings, and full test coverage.
- **Symlink & Hardlink Safety**: Added safety checks to prevent accidental data destruction when targeting symbolic links. Only the link itself is removed.
- **Interactive Prompts**: Added a confirmation prompt before executing destructive actions. Can be bypassed using the new `--force` or `-f` flag.
- **Memory Efficiency**: Re-engineered the recursive traversal engine to use stream-based iterators (`par_bridge` compatible logic) instead of loading all paths into memory, allowing the tool to safely shred millions of files without RAM bloat.

## [0.4.0] - 2026-06-07

### Added
- **Professional Audit Logging**: Introduced a high-integrity forensic reporting system. Users can now generate detailed session reports using `--audit-log <PATH>`.
- **SSD TRIM/Discard Support**: Added `--trim` flag to send hardware-level deallocation commands (Linux/Windows), significantly improving anti-forensic reliability on Flash-based media.
- **Flexible Log Formats**: Support for both human-readable `Text` and machine-readable `JSON` audit logs via the `--audit-format` flag.
- **Detailed Event Tracking**: Reports capture per-file results including timestamps, methods used, and specific error messages for failed operations.

### Changed
- Refactored core orchestration to collect and summarize destruction events across parallel threads.

## [0.3.1] - 2026-06-07

### Added
- **Graceful Interruption**: Implemented signal handling (Ctrl+C) via `ctrlc` crate. The tool now catches interruption signals and cancels ongoing operations safely without leaving the terminal in an inconsistent state.
- **Cancellation-Aware Engine**: Refactored the core shredding engine and overwriting loops to respond immediately to cancellation signals, ensuring data destruction stops as soon as requested.

### Changed
- Improved robustness of the parallel traversal engine during high-load "chaos" scenarios.

## [0.3.0] - 2026-06-07

### Added
- **Multi-threaded Destruction**: Integrated `rayon` for parallel file shredding, significantly improving performance on multi-core systems.
- **Industry Standard Algorithms**: Added support for Zero-fill, US DoD 5220.22-M, and Gutmann (35-pass) erasure methods.
- **Dry-Run Mode**: Introduced `--dry-run` to simulate destruction sequences without modifying the filesystem.
- **Read-back Verification**: Added `--verify` flag to ensure hardware-level data persistence by reading back and comparing written buffers.
- **Glob Exclusions**: Added `--exclude` support for skipping specific files or directories using pattern matching.

### Changed
- Refactored UI to provide a cleaner aggregate progress view for parallel operations.
- Updated core engine to be thread-safe.
- Overhauled project documentation (README.md) to reflect new v0.3.0 capabilities and forensic standards.

## [0.2.1] - 2026-06-06

### Fixed
- Hardened recursive traversal: Now explicitly handles and reports filesystem errors (e.g., permission issues).
- Zero-panic UI: Implemented safe fallback for terminal progress styles.
- Removed unused imports to ensure clean compilation.

### Added
- Automated dependency management via GitHub Dependabot.
- Strict documentation enforcement with `#![deny(missing_docs)]`.

## [0.2.0] - 2026-06-06

### Added
- Recursive Directory Shredding: Securely destroy entire directory trees using the `-r` or `--recursive` flag.
- Integrated `walkdir` for robust filesystem traversal.
- Enhanced core engine to handle bottom-up directory removal after content destruction.

### Changed
- Updated CLI schema to support both file and directory targets.
- Refactored `Shredder` API to accommodate recursive operations.

## [0.1.0] - 2026-06-06

### Added
- Initial release of sys-shred: a professional secure file erasure utility.
- Modular core engine for cryptographic overwriting and hardware synchronization.
- Metadata obfuscation, filename randomization, and secure unlinking logic.
- Cross-platform installation infrastructure (`scripts/install.sh` and `scripts/install.ps1`).
- Professional CLI interface with interactive progress reporting.
- Formal Security Policy (`SECURITY.md`) and project hygiene configuration (`.editorconfig`).
- Comprehensive technical documentation and integration test suite.
- Continuous Integration workflow via GitHub Actions.
