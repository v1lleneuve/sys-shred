//! # Shredding Orchestration Layer
//!
//! This module coordinates the various components of the shredding engine
//! to provide a high-level API for secure file erasure.

pub mod metadata;
pub mod overwrite;
pub mod unlink;

use crate::cli::args::ShredMethod;
use crate::error::{ShredError, ShredResult};
use crate::ui::ProgressReporter;
use globset::{Glob, GlobSet, GlobSetBuilder};
use metadata::MetadataHandler;
use overwrite::Overwriter;
use rayon::prelude::*;
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use unlink::Unlinker;
use walkdir::WalkDir;

/// The primary coordinator for the file destruction lifecycle.
pub struct Shredder {
    method: ShredMethod,
    passes: u32,
    dry_run: bool,
    verify: bool,
    exclude: GlobSet,
    progress: Option<ProgressReporter>,
}

impl Shredder {
    /// Initializes a new `Shredder` with the specified destruction policy.
    ///
    /// # Arguments
    /// * `method` - The erasure algorithm to utilize.
    /// * `passes` - Number of overwrite passes (if applicable to the method).
    /// * `dry_run` - If true, no I/O operations will be performed.
    /// * `verify` - If true, data is read back after each pass to verify destruction.
    /// * `exclude_patterns` - Glob patterns for files to skip.
    /// * `show_progress` - Whether to display a terminal progress bar.
    pub fn new(
        method: ShredMethod,
        passes: u32,
        dry_run: bool,
        verify: bool,
        exclude_patterns: &[String],
        show_progress: bool,
    ) -> ShredResult<Self> {
        let mut builder = GlobSetBuilder::new();
        for pattern in exclude_patterns {
            builder.add(Glob::new(pattern).map_err(|e| ShredError::Cli(e.to_string()))?);
        }

        Ok(Self {
            method,
            passes,
            dry_run,
            verify,
            exclude: builder
                .build()
                .map_err(|e| ShredError::Cli(e.to_string()))?,
            progress: if show_progress {
                Some(ProgressReporter::new())
            } else {
                None
            },
        })
    }

    fn should_exclude(&self, path: &Path) -> bool {
        self.exclude.is_match(path)
    }

    fn shred_file(&self, path: &Path, keep: bool) -> ShredResult<()> {
        if self.should_exclude(path) {
            if self.dry_run {
                println!("\x1b[33m[SKIP]\x1b[0m (Excluded): {:?}", path);
            }
            return Ok(());
        }

        if self.dry_run {
            println!("\x1b[32m[DRY-RUN]\x1b[0m Would shred: {:?}", path);
            return Ok(());
        }

        let mut file = OpenOptions::new().read(true).write(true).open(path)?;

        let mut overwriter = Overwriter::new(&mut file, self.verify);
        overwriter.execute(self.method.clone(), self.passes)?;

        drop(file);

        let obfuscated_path = MetadataHandler::obfuscate_filename(path)?;
        MetadataHandler::truncate(&obfuscated_path)?;

        if !keep {
            Unlinker::unlink(&obfuscated_path)?;
        }

        if let Some(ref pr) = self.progress {
            pr.inc_file_complete();
        }

        Ok(())
    }

    /// Entry point for the shredding operation.
    ///
    /// Handles both individual files and recursive directory traversal.
    pub fn shred(&self, path: &Path, recursive: bool, keep: bool) -> ShredResult<()> {
        if !path.exists() {
            return Err(ShredError::InvalidPath(format!(
                "Path does not exist: {:?}",
                path
            )));
        }

        if path.is_file() {
            if let Some(ref pr) = self.progress {
                pr.start_files(1);
            }
            self.shred_file(path, keep)
        } else if path.is_dir() {
            if !recursive {
                return Err(ShredError::InvalidPath(format!(
                    "Target {:?} is a directory. Use --recursive to destroy it.",
                    path
                )));
            }

            let mut entries = Vec::new();
            for entry in WalkDir::new(path) {
                match entry {
                    Ok(e) => entries.push(e.into_path()),
                    Err(e) => {
                        return Err(ShredError::Io(std::io::Error::other(format!(
                            "Failed to access directory entry: {}",
                            e
                        ))))
                    }
                }
            }

            let files: Vec<PathBuf> = entries.iter().filter(|p| p.is_file()).cloned().collect();

            if let Some(ref pr) = self.progress {
                pr.start_files(files.len() as u64);
            }

            // Parallel file shredding
            files
                .par_iter()
                .try_for_each(|f| self.shred_file(f, keep))?;

            if !keep && !self.dry_run {
                let mut dirs: Vec<_> = entries.iter().filter(|p| p.is_dir()).collect();
                dirs.sort_by_key(|b| std::cmp::Reverse(b.as_os_str().len()));
                for dir in dirs {
                    if dir.exists() {
                        // We ignore errors during directory removal (e.g. if a directory
                        // is not empty due to exclusions).
                        let _ = fs::remove_dir(dir);
                    }
                }
            } else if self.dry_run {
                println!(
                    "\x1b[32m[DRY-RUN]\x1b[0m Would remove directories in: {:?}",
                    path
                );
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
