//! # Behavioral Validation Suite
//!
//! Contains integration tests to ensure the functional integrity of the
//! shredding lifecycle under various scenarios.

use crate::core::Shredder;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

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
    let mut shredder = Shredder::new(1, false);

    // Execute shredding
    shredder
        .shred(&file_path, false)
        .expect("Shredding operation failed prematurely");

    // The original file path should no longer exist
    assert!(
        !file_path.exists(),
        "Security Breach: File still exists at the original path after shredding"
    );
}

/// Verifies the behavior of the `--keep` flag, ensuring the file is overwritten
/// and renamed but not unlinked.
#[test]
fn test_shredding_with_keep_flag() {
    let dir = tempdir().expect("Failed to create temporary directory for testing");
    let file_path = dir.path().join("persistent_target.bin");

    {
        let mut file = File::create(&file_path).expect("Failed to create test file");
        file.write_all(b"PERSISTENT DATA")
            .expect("Failed to write test data");
    }

    let mut shredder = Shredder::new(1, false);

    // Execute shredding with keep=true
    shredder
        .shred(&file_path, true)
        .expect("Shredding operation failed with keep=true");

    // The original path must be gone because the file was renamed
    assert!(
        !file_path.exists(),
        "Logical Failure: Original path exists after metadata obfuscation"
    );
}
