# Changelog

All notable changes to this project will be documented in this file.

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
