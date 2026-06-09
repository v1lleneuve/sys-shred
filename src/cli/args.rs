//! # CLI Argument Schema
//!
//! Defines the structured inputs for the `sys-shred` application using `clap`.
//! This module ensures that all user inputs are validated and mapped to
//! actionable internal configurations.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Standard algorithms for secure data erasure.
#[derive(ValueEnum, Clone, Debug, serde::Serialize)]
pub enum ShredMethod {
    /// Overwrite with zero bytes (Fastest).
    Zero,
    /// Overwrite with cryptographically secure random bytes (Balanced).
    Random,
    /// US DoD 5220.22-M (3 passes: 0x00, 0xFF, Random).
    Dod,
    /// Gutmann method (35 passes - Extreme security for older magnetic media).
    Gutmann,
}

/// Output formats for the audit log.
#[derive(ValueEnum, Clone, Debug, Default)]
pub enum AuditFormat {
    /// Human-readable text format.
    #[default]
    Text,
    /// Machine-readable JSON format.
    Json,
}

/// Secure File Erasure Utility (Anti-Forensics)
///
/// `sys-shred` is a specialized tool designed to irreversibly destroy file data.
/// It works by performing multiple cryptographic overwrite passes, randomizing
/// file metadata (name and timestamps), and finally unlinking the file from
/// the filesystem to prevent forensic recovery.
#[derive(Parser, Debug)]
#[command(
    author = "V1lleneuve",
    version = "1.0.1",
    about = "Securely shreds files using cryptographic data and metadata obfuscation",
    long_about = "A high-integrity secure deletion tool that bypasses OS file-system caching to ensure hardware-level data destruction."
)]
pub struct Args {
    /// The absolute or relative path to the target file or directory.
    #[arg(
        value_name = "PATH",
        help = "Path to the file or directory to be destroyed"
    )]
    pub path: PathBuf,

    /// Number of overwrite passes to perform.
    #[arg(
        short,
        long,
        default_value_t = 3,
        help = "Number of cryptographic overwrite passes (Ignored for DoD/Gutmann)"
    )]
    pub passes: u32,

    /// Recursively shred directories and their contents.
    #[arg(
        short,
        long,
        help = "Recursively destroy directories and their contents"
    )]
    pub recursive: bool,

    /// Algorithm to use for data destruction.
    #[arg(
        short,
        long,
        value_enum,
        default_value_t = ShredMethod::Random,
        help = "Erasure method to utilize"
    )]
    pub method: ShredMethod,

    /// Perform a trial run without modifying the filesystem.
    #[arg(
        long,
        help = "Show what would be destroyed without performing actual deletion"
    )]
    pub dry_run: bool,

    /// Verify overwrites by reading back data after writing.
    #[arg(long, help = "Enable read-back verification after each overwrite pass")]
    pub verify: bool,

    /// Exclude files matching specific glob patterns.
    #[arg(
        short,
        long,
        help = "Exclude files matching patterns (e.g. *.log, secret/*)"
    )]
    pub exclude: Vec<String>,

    /// informs the SSD to discard the blocks used by the file (TRIM).
    #[arg(
        long,
        help = "Send a TRIM/Discard command to the SSD after shredding (Linux/Windows only)"
    )]
    pub trim: bool,

    /// Force destruction without interactive confirmation.
    #[arg(short, long, help = "Skip interactive confirmation prompts")]
    pub force: bool,

    /// Path to save the forensic audit log.

    #[arg(
        long,
        value_name = "LOG_PATH",
        help = "Path to generate a forensic audit report"
    )]
    pub audit_log: Option<PathBuf>,

    /// Format of the forensic audit log.
    #[arg(
        long,
        value_enum,
        default_value_t = AuditFormat::Text,
        help = "Format of the audit report (text or json)"
    )]
    pub audit_format: AuditFormat,

    /// Enable verbose logging for granular visibility into the shredding process.
    #[arg(short, long, help = "Enable detailed debug output")]
    pub verbose: bool,

    /// Obfuscate and truncate the file but skip the final unlinking (deletion).
    #[arg(
        long,
        help = "Perform overwriting and obfuscation but do not delete the final file"
    )]
    pub keep: bool,
}
