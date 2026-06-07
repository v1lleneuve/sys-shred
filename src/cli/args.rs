//! # CLI Argument Schema
//!
//! Defines the structured inputs for the `sys-shred` application using `clap`.
//! This module ensures that all user inputs are validated and mapped to
//! actionable internal configurations.

use clap::{Parser, ValueEnum};
use std::path::PathBuf;

/// Standard algorithms for secure data erasure.
#[derive(ValueEnum, Clone, Debug)]
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

/// Secure File Erasure Utility (Anti-Forensics)
///
/// `sys-shred` is a specialized tool designed to irreversibly destroy file data.
/// It works by performing multiple cryptographic overwrite passes, randomizing
/// file metadata (name and timestamps), and finally unlinking the file from
/// the filesystem to prevent forensic recovery.
#[derive(Parser, Debug)]
#[command(
    author = "V1lleneuve",
    version = "0.3.0",
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
