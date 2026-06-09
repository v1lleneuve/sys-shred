pub mod metadata;
pub mod overwrite;
pub mod report;
pub mod unlink;

use crate::cli::args::ShredMethod;
use crate::error::{ShredError, ShredResult};
use crate::ui::ProgressReporter;
use chrono::Utc;
use dialoguer::Confirm;
use globset::{Glob, GlobSet, GlobSetBuilder};
use metadata::MetadataHandler;
use overwrite::Overwriter;
use rayon::prelude::*;
use report::{ShredEvent, ShredReport};
use std::fs::{self, OpenOptions};
use std::path::Path;
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
    force: bool,
    exclude: GlobSet,
    progress: Option<ProgressReporter>,
    cancelled: Arc<AtomicBool>,
    events: Arc<Mutex<Vec<ShredEvent>>>,
}

impl Shredder {
    /// Initializes a new `Shredder` with the specified destruction policy.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        method: ShredMethod,
        passes: u32,
        dry_run: bool,
        verify: bool,
        trim: bool,
        force: bool,
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
            force,
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

        // Safety: Check if it's a symlink. We only delete the link, NOT the target data.
        let metadata = fs::symlink_metadata(path)?;
        if metadata.file_type().is_symlink() {
            if !self.dry_run && !keep {
                fs::remove_file(path)?;
            }
            return Ok(());
        }

        // Safety: Skip special files (FIFOs, Sockets, etc.) to prevent hanging.
        if !metadata.is_file() && !metadata.is_dir() {
            return Ok(());
        }

        if self.should_exclude(path) {
            return Ok(());
        }

        if self.dry_run {
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
    pub fn shred(&self, path: &Path, recursive: bool, keep: bool) -> ShredResult<()> {
        if !path.exists() {
            return Err(ShredError::InvalidPath(format!(
                "Path does not exist: {:?}",
                path
            )));
        }

        // Professional Guard: Interactive Confirmation
        if !self.force && !self.dry_run {
            let prompt = if path.is_dir() && recursive {
                format!(
                    "Are you sure you want to RECURSIVELY destroy all contents in {:?}?",
                    path
                )
            } else {
                format!("Are you sure you want to permanently destroy {:?}?", path)
            };

            if !Confirm::new()
                .with_prompt(prompt)
                .default(false)
                .interact()
                .unwrap_or(false)
            {
                return Err(ShredError::Cli("Operation cancelled by user".to_string()));
            }
        }

        if path.is_file() {
            if let Some(ref pr) = self.progress {
                pr.start_files(1);
            }
            self.shred_file(path, keep)
        } else if path.is_dir() {
            if !recursive {
                return Err(ShredError::InvalidPath(format!(
                    "Target {:?} is a directory. Use --recursive.",
                    path
                )));
            }

            // For directories, we still need a count for the progress bar
            let mut entries = Vec::new();
            let mut file_count = 0;

            for e in WalkDir::new(path).into_iter().flatten() {
                let entry_path = e.into_path();
                if entry_path.is_file() && !self.should_exclude(&entry_path) {
                    file_count += 1;
                }
                entries.push(entry_path);
            }

            if let Some(ref pr) = self.progress {
                pr.start_files(file_count);
            }

            // Parallel execution using the pre-collected entries.
            entries
                .par_iter()
                .filter(|p| p.is_file() && !self.should_exclude(p))
                .try_for_each(|f| self.shred_file(f, keep))?;

            if !keep && !self.dry_run && !self.is_cancelled() {
                entries.sort_by_key(|b| std::cmp::Reverse(b.as_os_str().len()));
                for dir in entries {
                    if dir.is_dir() && dir.exists() {
                        let _ = fs::remove_dir(dir);
                    }
                }
            }
            Ok(())
        } else {
            Err(ShredError::InvalidPath(format!(
                "Invalid target type: {:?}",
                path
            )))
        }
    }
}
