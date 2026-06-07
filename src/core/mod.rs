pub mod metadata;
pub mod overwrite;
pub mod report;
pub mod unlink;

use crate::cli::args::ShredMethod;
use crate::error::{ShredError, ShredResult};
use crate::ui::ProgressReporter;
use chrono::Utc;
use globset::{Glob, GlobSet, GlobSetBuilder};
use metadata::MetadataHandler;
use overwrite::Overwriter;
use rayon::prelude::*;
use report::{ShredEvent, ShredReport};
use std::fs::{self, OpenOptions};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use unlink::Unlinker;
use walkdir::WalkDir;

/// The primary coordinator for the file destruction lifecycle.
pub struct Shredder {
    method: ShredMethod,
    passes: u32,
    dry_run: bool,
    verify: bool,
    trim: bool,
    exclude: GlobSet,
    progress: Option<ProgressReporter>,
    cancelled: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<ShredEvent>>>,
}

impl Shredder {
    /// Initializes a new `Shredder` with the specified destruction policy.
    ///
    /// # Arguments
    /// * `method` - The erasure algorithm to utilize.
    /// * `passes` - Number of overwrite passes (if applicable to the method).
    /// * `dry_run` - If true, no I/O operations will be performed.
    /// * `verify` - If true, data is read back after each pass to verify destruction.
    /// * `trim` - If true, sends a TRIM command to the SSD after shredding.
    /// * `exclude_patterns` - Glob patterns for files to skip.
    /// * `show_progress` - Whether to display a terminal progress bar.
    pub fn new(
        method: ShredMethod,
        passes: u32,
        dry_run: bool,
        verify: bool,
        trim: bool,
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
            trim,
            exclude: builder
                .build()
                .map_err(|e| ShredError::Cli(e.to_string()))?,
            progress: if show_progress {
                Some(ProgressReporter::new())
            } else {
                None
            },
            cancelled: Arc::new(AtomicBool::new(false)),
            events: Arc::new(Mutex::new(Vec::new())),
        })
    }

    /// Signals the shredder to stop current operations as soon as possible.
    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Relaxed)
    }

    /// Returns the accumulated audit report.
    pub fn generate_report(&self) -> ShredReport {
        let events = self.events.lock().unwrap().clone();
        ShredReport::new(events)
    }

    fn record_event(&self, event: ShredEvent) {
        if let Ok(mut events) = self.events.lock() {
            events.push(event);
        }
    }

    fn should_exclude(&self, path: &Path) -> bool {
        self.exclude.is_match(path)
    }

    fn shred_file(&self, path: &Path, keep: bool) -> ShredResult<()> {
        if self.is_cancelled() {
            return Ok(());
        }

        if self.should_exclude(path) {
            if self.dry_run {
                println!("\x1b[33m[SKIP]\x1b[0m (Excluded): {:?}", path);
            }
            return Ok(());
        }

        if self.dry_run {
            println!("\x1b[32m[DRY-RUN]\x1b[0m Would shred: {:?}", path);
            self.record_event(ShredEvent {
                path: path.to_path_buf(),
                timestamp: Utc::now(),
                method: self.method.clone(),
                success: true,
                error: None,
            });
            return Ok(());
        }

        let res = (|| -> ShredResult<()> {
            let mut file = OpenOptions::new().read(true).write(true).open(path)?;

            let mut overwriter =
                Overwriter::new(&mut file, self.verify, Arc::clone(&self.cancelled));
            overwriter.execute(self.method.clone(), self.passes)?;

            drop(file);

            let obfuscated_path = MetadataHandler::obfuscate_filename(path)?;

            if self.trim {
                let _ = MetadataHandler::trim(&obfuscated_path);
            }

            MetadataHandler::truncate(&obfuscated_path)?;

            if !keep {
                Unlinker::unlink(&obfuscated_path)?;
            }
            Ok(())
        })();

        self.record_event(ShredEvent {
            path: path.to_path_buf(),
            timestamp: Utc::now(),
            method: self.method.clone(),
            success: res.is_ok(),
            error: res.as_ref().err().map(|e| e.to_string()),
        });

        if res.is_ok() {
            if let Some(ref pr) = self.progress {
                pr.inc_file_complete();
            }
        }

        res
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
                if self.is_cancelled() {
                    break;
                }
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

            if self.is_cancelled() {
                return Ok(());
            }

            let files: Vec<PathBuf> = entries.iter().filter(|p| p.is_file()).cloned().collect();

            if let Some(ref pr) = self.progress {
                pr.start_files(files.len() as u64);
            }

            // Parallel file shredding
            files.par_iter().try_for_each(|f| {
                if self.is_cancelled() {
                    return Ok(());
                }
                self.shred_file(f, keep)
            })?;

            if !keep && !self.dry_run && !self.is_cancelled() {
                let mut dirs: Vec<_> = entries.iter().filter(|p| p.is_dir()).collect();
                dirs.sort_by_key(|b| std::cmp::Reverse(b.as_os_str().len()));
                for dir in dirs {
                    if self.is_cancelled() {
                        break;
                    }
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
