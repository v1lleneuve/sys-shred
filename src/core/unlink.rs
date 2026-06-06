//! # Secure Unlinking
//!
//! Handles the final removal of the file entry from the filesystem's
//! directory structure.

use crate::error::ShredResult;
use std::fs;
use std::path::Path;

/// A utility for performing secure file unlinking (deletion).
pub struct Unlinker;

impl Unlinker {
    /// Permanently removes the file entry from the filesystem.
    ///
    /// This is the final step in the shredding process, performed only after
    /// the data has been overwritten and the metadata has been obfuscated.
    ///
    /// # Arguments
    ///
    /// * `path` - The `Path` to the file to be unlinked.
    ///
    /// # Errors
    ///
    /// Returns `ShredError::Io` if the file cannot be removed (e.g., due to
    /// permission constraints).
    pub fn unlink(path: &Path) -> ShredResult<()> {
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}
