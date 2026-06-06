//! # Shredding Orchestration Layer
//!
//! This module coordinates the various components of the shredding engine
//! to provide a high-level API for secure file erasure.

pub mod metadata;
pub mod overwrite;
pub mod unlink;

use crate::error::{ShredError, ShredResult};
use crate::ui::ProgressReporter;
use metadata::MetadataHandler;
use overwrite::Overwriter;
use std::fs::OpenOptions;
use std::path::Path;
use unlink::Unlinker;

/// The primary coordinator for the file destruction lifecycle.
///
/// The `Shredder` manages the sequence of operations required to securely
/// erase a file: data overwriting, name obfuscation, truncation, and unlinking.
pub struct Shredder {
    /// Total number of cryptographic overwrite passes to execute.
    passes: u32,
    /// Optional progress reporting component for interactive feedback.
    progress: Option<ProgressReporter>,
}

impl Shredder {
    /// Initializes a new `Shredder` with the specified configuration.
    ///
    /// # Arguments
    ///
    /// * `passes` - How many times to overwrite the data.
    /// * `show_progress` - Whether to initialize the terminal progress bar.
    pub fn new(passes: u32, show_progress: bool) -> Self {
        Self {
            passes,
            progress: if show_progress {
                Some(ProgressReporter::new())
            } else {
                None
            },
        }
    }

    /// Executes the full secure erasure sequence on the target file.
    ///
    /// # Lifecycle Stages:
    /// 1. **Validation**: Ensures the target is a valid file.
    /// 2. **Overwriting**: Performs N passes of cryptographic random data writing.
    /// 3. **Metadata Scrubbing**: Renames the file to a random string.
    /// 4. **Truncation**: Sets the file size to 0 bytes.
    /// 5. **Unlinking**: Removes the file entry from the filesystem (optional).
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to be destroyed.
    /// * `keep` - If `true`, skips the final `unlink` step.
    ///
    /// # Errors
    ///
    /// Returns `ShredError` if any stage of the lifecycle fails.
    pub fn shred(&mut self, path: &Path, keep: bool) -> ShredResult<()> {
        if !path.is_file() {
            return Err(ShredError::InvalidPath(format!(
                "Target is not a file or is inaccessible: {:?}",
                path
            )));
        }

        // Stage 1: Data Destruction
        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        if let Some(ref pr) = self.progress {
            pr.start_overwrite(self.passes);
        }

        let mut overwriter = Overwriter::new(&mut file);
        for i in 0..self.passes {
            overwriter.run_pass()?;
            if let Some(ref pr) = self.progress {
                pr.inc_overwrite(1, format!("Pass {}/{}", i + 1, self.passes));
            }
        }

        // Close file handle before renaming/truncating
        drop(file);

        if let Some(ref pr) = self.progress {
            pr.finish_overwrite();
            pr.start_metadata();
        }

        // Stage 2: Metadata & Name Obfuscation
        let obfuscated_path = MetadataHandler::obfuscate_filename(path)?;
        MetadataHandler::truncate(&obfuscated_path)?;

        // Stage 3: Final Removal
        if !keep {
            Unlinker::unlink(&obfuscated_path)?;
        }

        if let Some(ref pr) = self.progress {
            pr.finish_metadata();
        }

        Ok(())
    }
}
