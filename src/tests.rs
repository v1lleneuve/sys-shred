//! # Behavioral Validation Suite
//!
//! Contains integration tests to ensure the functional integrity of the
//! shredding lifecycle under various scenarios.

use crate::cli::args::{AuditFormat, ShredMethod};
use crate::core::Shredder;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

#[test]
fn test_audit_logging() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("audit_target.txt");
    let log_path = dir.path().join("audit.json");

    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"AUDIT ME").unwrap();
    }

    let shredder = Shredder::new(
        ShredMethod::Random,
        1,
        false,
        false,
        false,
        true,
        &[],
        false,
    )
    .unwrap();
    shredder.shred(&file_path, false, false).unwrap();

    let report = shredder.generate_report();
    report
        .save(&log_path, AuditFormat::Json)
        .expect("Failed to save audit log");

    assert!(log_path.exists(), "Audit log file was not created");

    let content = std::fs::read_to_string(&log_path).unwrap();
    assert!(
        content.contains("audit_target.txt"),
        "Audit log does not contain the file path"
    );
    assert!(
        content.contains("\"success\": true"),
        "Audit log does not record success correctly"
    );
}

/// Verifies that a standard shredding operation successfully destroys the file
/// and unlinks it from the filesystem.
#[test]
fn test_standard_shredding_lifecycle() {
    let dir = tempdir().expect("Failed to create temporary directory for testing");
    let file_path = dir.path().join("forensic_target.bin");

    // Prepare a file with simulated sensitive data
    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(b"CONFIDENTIAL DATA STREAM")
            .expect("Failed to write test data");
    }

    // Initialize shredder with 1 pass for speed in tests
    let shredder = Shredder::new(
        ShredMethod::Random,
        1,
        false,
        false,
        false,
        true,
        &[],
        false,
    )
    .unwrap();

    // Execute shredding (non-recursive)
    shredder
        .shred(&file_path, false, false)
        .expect("Shredding operation failed prematurely");

    // The original file path should no longer exist
    assert!(
        !file_path.exists(),
        "Security Breach: File still exists at the original path after shredding"
    );
}

#[test]
fn test_dry_run_mode() {
    let dir = tempdir().unwrap();
    let file_path = dir.path().join("dry_run_target.txt");
    {
        let mut file = File::create(&file_path).unwrap();
        file.write_all(b"STAY ALIVE").unwrap();
    }

    let shredder =
        Shredder::new(ShredMethod::Random, 1, true, false, false, true, &[], false).unwrap();
    shredder.shred(&file_path, false, false).unwrap();

    // In dry-run, the file MUST still exist
    assert!(file_path.exists(), "Dry-run modified the filesystem!");
}

#[test]
fn test_exclude_patterns() {
    let dir = tempdir().unwrap();
    let file1 = dir.path().join("shred_me.txt");
    let file2 = dir.path().join("keep_me.log");

    File::create(&file1).unwrap();
    File::create(&file2).unwrap();

    let shredder = Shredder::new(
        ShredMethod::Random,
        1,
        false,
        false,
        false,
        true,
        &["*.log".to_string()],
        false,
    )
    .unwrap();

    shredder.shred(dir.path(), true, false).unwrap();

    assert!(!file1.exists(), "shred_me.txt should be gone");
    assert!(file2.exists(), "keep_me.log should have been excluded");
}
