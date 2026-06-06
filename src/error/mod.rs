//! # Security-First Error Handling
//!
//! This module defines the robust error infrastructure for `sys-shred`.
//! It utilizes `thiserror` to provide descriptive, context-aware error messages
//! while maintaining strict type safety across the application.

use thiserror::Error;

/// The primary error taxonomy for the `sys-shred` ecosystem.
///
/// Every potential failure point, from low-level filesystem I/O to high-level
/// cryptographic entropy exhaustion, is represented here.
#[derive(Debug, Error)]
pub enum ShredError {
    /// Errors originating from the underlying Operating System's file operations.
    /// This includes permission issues, missing files, or hardware failures.
    #[error("System I/O Failure: {0}")]
    Io(#[from] std::io::Error),

    /// Errors encountered during the parsing or validation of Command Line Arguments.
    #[error("CLI Configuration Error: {0}")]
    Cli(String),

    /// Errors related to the failure of cryptographic random number generation.
    /// Critical for ensuring the integrity of the overwriting process.
    #[error("Cryptographic Entropy Error: {0}")]
    Entropy(String),

    /// Errors triggered when a target path is logically invalid (e.g., pointing to a directory
    /// when a file is expected) or otherwise inaccessible.
    #[error("Inaccessible Path Error: {0}")]
    InvalidPath(String),

    /// Errors occurring during the filename randomization or metadata scrubbing phase.
    #[error("Metadata Obfuscation Failure: {0}")]
    Obfuscation(String),
}

/// A specialized `Result` type alias for all `sys-shred` operations.
///
/// This type is used consistently across the library to enforce explicit
/// error propagation and handling.
pub type ShredResult<T> = Result<T, ShredError>;
