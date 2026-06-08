//! # `sys-shred` CLI Application
//!
//! The main entry point for the `sys-shred` command-line utility.
//! Handles high-level initialization, logging, and error reporting.

use clap::Parser;
use std::process;
use std::sync::Arc;
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
        args.trim,
        args.force,
        &args.exclude,
        !args.verbose,
    ) {
        Ok(s) => Arc::new(s),
        Err(e) => {
            eprintln!("\x1b[31m[ERROR]\x1b[0m Configuration failed: {}", e);
            process::exit(1);
        }
    };

    // 4. Setup Signal Handling for graceful interruption
    let s_clone = Arc::clone(&shredder);
    if let Err(e) = ctrlc::set_handler(move || {
        eprintln!("\n\x1b[33m[WARN]\x1b[0m Interruption signal received. Cleaning up and exiting safely...");
        s_clone.cancel();
    }) {
        eprintln!("\x1b[33m[WARN]\x1b[0m Failed to set signal handler: {}", e);
    }

    // 5. Execute the shredding lifecycle
    let shred_res = shredder.shred(&args.path, args.recursive, args.keep);

    // 6. Generate and save Audit Report if requested
    if let Some(log_path) = args.audit_log {
        let report = shredder.generate_report();
        if let Err(e) = report.save(&log_path, args.audit_format) {
            eprintln!("\x1b[33m[WARN]\x1b[0m Failed to save audit log: {}", e);
        } else {
            println!(
                "\x1b[34m[INFO]\x1b[0m Forensic audit log generated at: {:?}",
                log_path
            );
        }
    }

    if let Err(e) = shred_res {
        eprintln!("\n\x1b[31m[CRITICAL ERROR]\x1b[0m {}", e);
        process::exit(1);
    }

    println!("\x1b[32m[SUCCESS]\x1b[0m File destruction sequence finalized.");
}
