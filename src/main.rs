//! # `sys-shred` CLI Application
//!
//! The main entry point for the `sys-shred` command-line utility.
//! Handles high-level initialization, logging, and error reporting.

use clap::Parser;
use std::process;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use sys_shred::cli::Args;
use sys_shred::core::Shredder;
use sys_shred::ui::UI;

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
            UI.error(format!("Configuration failed: {}", e));
            process::exit(1);
        }
    };

    // 4. Setup Signal Handling for graceful interruption
    let s_clone = Arc::clone(&shredder);
    let handler_count = Arc::new(std::sync::atomic::AtomicU8::new(0));
    if let Err(e) = ctrlc::set_handler(move || {
        let count = handler_count.fetch_add(1, Ordering::SeqCst);
        if count == 0 {
            eprintln!();
            UI.warn("Interruption signal received. Cleaning up safely...");
            UI.info("Hint", "Press Ctrl+C again to force immediate exit");
            s_clone.cancel();
        } else {
            UI.error("Force exiting...");
            process::exit(130); // 130 is standard for SIGINT
        }
    }) {
        UI.warn(format!("Failed to set signal handler: {}", e));
    }

    // 5. Execute the shredding lifecycle
    let start_time = std::time::Instant::now();
    let shred_res = shredder.shred(&args.path, args.recursive, args.keep);

    // 6. Generate and save Audit Report if requested
    if let Some(log_path) = args.audit_log {
        let report = shredder.generate_report();
        if let Err(e) = report.save(&log_path, args.audit_format) {
            UI.warn(format!("Failed to save audit log: {}", e));
        } else {
            UI.info(
                "Report",
                format!("Forensic audit log generated at: {:?}", log_path),
            );
        }
    }

    if let Err(e) = shred_res {
        eprintln!();
        UI.error(format!("{}", e));
        process::exit(1);
    }

    let duration = start_time.elapsed();
    UI.finished(format!("shredding completed in {:.2?}", duration));
}
