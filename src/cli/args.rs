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
    version = "0.2.0",
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
        help = "Number of cryptographic overwrite passes (Default: 3)"
    )]
    pub passes: u32,

    /// Recursively shred directories and their contents.
    #[arg(
        short,
        long,
        help = "Recursively destroy directories and their contents"
    )]
    pub recursive: bool,

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
