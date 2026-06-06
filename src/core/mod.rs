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
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use unlink::Unlinker;
use walkdir::WalkDir;

/// The primary coordinator for the file destruction lifecycle.
///
/// The `Shredder` manages the sequence of operations required to securely
/// erase files and directories.
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

    /// Securely destroys a single file.
    fn shred_file(&mut self, path: &Path, keep: bool) -> ShredResult<()> {
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

        drop(file);

        if let Some(ref pr) = self.progress {
            pr.finish_overwrite();
            pr.start_metadata();
        }

        let obfuscated_path = MetadataHandler::obfuscate_filename(path)?;
        MetadataHandler::truncate(&obfuscated_path)?;

        if !keep {
            Unlinker::unlink(&obfuscated_path)?;
        }

        if let Some(ref pr) = self.progress {
            pr.finish_metadata();
        }

        Ok(())
    }

    /// Securely shreds a file or a directory (if recursive is enabled).
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file or directory to be destroyed.
    /// * `recursive` - Whether to destroy directory contents.
    /// * `keep` - If `true`, skips the final `unlink` step for files.
    pub fn shred(&mut self, path: &Path, recursive: bool, keep: bool) -> ShredResult<()> {
        if !path.exists() {
            return Err(ShredError::InvalidPath(format!(
                "Path does not exist: {:?}",
                path
            )));
        }

        if path.is_file() {
            self.shred_file(path, keep)
        } else if path.is_dir() {
            if !recursive {
                return Err(ShredError::InvalidPath(format!(
                    "Target {:?} is a directory. Use --recursive to destroy it.",
                    path
                )));
            }

            // Collect all entries to avoid modification-during-iteration issues
            let entries: Vec<PathBuf> = WalkDir::new(path)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.into_path())
                .collect();

            // Shred files first
            for entry in entries.iter().filter(|p| p.is_file()) {
                self.shred_file(entry, keep)?;
            }

            // Finally, remove the directories (bottom-up)
            if !keep {
                let mut dirs: Vec<_> = entries.iter().filter(|p| p.is_dir()).collect();
                dirs.sort_by_key(|b| std::cmp::Reverse(b.as_os_str().len()));
                for dir in dirs {
                    if dir.exists() {
                        fs::remove_dir(dir)?;
                    }
                }
            }
            Ok(())
        } else {
            Err(ShredError::InvalidPath(format!(
                "Target is not a file or directory: {:?}",
                path
            )))
        }
    }
}
