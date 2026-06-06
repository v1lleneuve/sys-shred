//! # Metadata & Filename Obfuscation
//!
//! Handles the transformation of file metadata to prevent path-based recovery
//! and information leakage through filenames.

use crate::error::ShredResult;
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::path::{Path, PathBuf};

/// A utility for scrubbing and randomizing file metadata.
///
/// This component focuses on the anti-forensic aspects of shredding that go
/// beyond raw data destruction.
pub struct MetadataHandler;

impl MetadataHandler {
    /// Renames a file to a randomly generated alphanumeric string.
    ///
    /// This prevents an attacker from identifying what the file originally was
    /// based on its name or extension.
    ///
    /// # Arguments
    ///
    /// * `path` - The current `Path` to the file.
    ///
    /// # Returns
    ///
    /// A `ShredResult` containing the new `PathBuf` pointing to the renamed file.
    ///
    /// # Errors
    ///
    /// Returns `ShredError::Io` if the rename operation is rejected by the OS.
    pub fn obfuscate_filename(path: &Path) -> ShredResult<PathBuf> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));

        // Generate a 16-character random alphanumeric string
        let random_name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(16)
            .map(char::from)
            .collect();

        let new_path = parent.join(random_name);
        fs::rename(path, &new_path)?;

        Ok(new_path)
    }

    /// Truncates a file to zero bytes and flushes the change.
    ///
    /// This serves as a final software-level signal that the data is gone,
    /// and ensures the filesystem entry reports a size of 0.
    ///
    /// # Arguments
    ///
    /// * `path` - The `Path` to the file to truncate.
    ///
    /// # Errors
    ///
    /// Returns `ShredError::Io` if truncation or synchronization fails.
    pub fn truncate(path: &Path) -> ShredResult<()> {
        let file = fs::OpenOptions::new().write(true).open(path)?;
        file.set_len(0)?;
        file.sync_all()?;
        Ok(())
    }
}
