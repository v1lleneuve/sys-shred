//! # CLI Argument Schema
//!
//! Defines the structured inputs for the `sys-shred` application using `clap`.
//! This module ensures that all user inputs are validated and mapped to
//! actionable internal configurations.

use clap::Parser;
use std::path::PathBuf;

/// Secure File Erasure Utility (Anti-Forensics)
///
/// `sys-shred` is a specialized tool designed to irreversibly destroy file data.
/// It works by performing multiple cryptographic overwrite passes, randomizing
/// file metadata (name and timestamps), and finally unlinking the file from
/// the filesystem to prevent forensic recovery.
#[derive(Parser, Debug)]
#[command(
    author = "V1lleneuve",
    version = "0.1.0",
    about = "Securely shreds files using cryptographic data and metadata obfuscation",
    long_about = "A high-integrity secure deletion tool that bypasses OS file-system caching to ensure hardware-level data destruction."
)]
pub struct Args {
    /// The absolute or relative path to the target file.
    #[arg(value_name = "FILE_PATH", help = "Path to the file to be destroyed")]
    pub path: PathBuf,

    /// The number of times to overwrite the file with random data.
    #[arg(
        short,
        long,
        default_value_t = 3,
        help = "Number of cryptographic overwrite passes (Default: 3)"
    )]
    pub passes: u32,

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
