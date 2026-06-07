//! # `sys-shred` CLI Application
//!
//! The main entry point for the `sys-shred` command-line utility.
//! Handles high-level initialization, logging, and error reporting.

use clap::Parser;
use std::process;
use sys_shred::cli::Args;
use sys_shred::core::Shredder;

/// The execution entry point for the `sys-shred` binary.
fn main() {
    // 1. Initialize global logging subsystem
    env_logger::init();

    // 2. Parse and validate CLI arguments
    let args = Args::parse();

    // 3. Initialize the destruction engine
    let shredder = match Shredder::new(
        args.method,
        args.passes,
        args.dry_run,
        args.verify,
        &args.exclude,
        !args.verbose,
    ) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("\x1b[31m[ERROR]\x1b[0m Configuration failed: {}", e);
            process::exit(1);
        }
    };

    // 4. Execute the shredding lifecycle
    if let Err(e) = shredder.shred(&args.path, args.recursive, args.keep) {
        eprintln!("\n\x1b[31m[CRITICAL ERROR]\x1b[0m {}", e);
        process::exit(1);
    }

    println!("\x1b[32m[SUCCESS]\x1b[0m File destroyed beyond recovery.");
}
