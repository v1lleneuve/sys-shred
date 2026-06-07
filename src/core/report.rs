//! # Forensic Audit Reporting
//!
//! Provides structures and logic for capturing and serializing the results
//! of shredding operations for audit and compliance purposes.

use crate::cli::args::{AuditFormat, ShredMethod};
use chrono::{DateTime, Utc};
use serde::Serialize;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

/// Represents a single file destruction event.
#[derive(Debug, Serialize, Clone)]
pub struct ShredEvent {
    /// The path to the file that was targeted.
    pub path: PathBuf,
    /// Timestamp of when the operation completed.
    pub timestamp: DateTime<Utc>,
    /// The algorithm used for destruction.
    pub method: ShredMethod,
    /// Whether the operation succeeded or failed.
    pub success: bool,
    /// Detailed error message if the operation failed.
    pub error: Option<String>,
}

/// A comprehensive report of a shredding session.
#[derive(Debug, Serialize, Clone)]
pub struct ShredReport {
    /// Timestamp of when the report was generated.
    pub generated_at: DateTime<Utc>,
    /// Total number of files targeted in this session.
    pub total_files: usize,
    /// Number of successful destructions.
    pub success_count: usize,
    /// Number of failed destructions.
    pub failure_count: usize,
    /// Individual event logs.
    pub events: Vec<ShredEvent>,
}

impl ShredReport {
    /// Creates a new `ShredReport` from a collection of events.
    pub fn new(events: Vec<ShredEvent>) -> Self {
        let total = events.len();
        let success_count = events.iter().filter(|e| e.success).count();
        let failure_count = total - success_count;

        Self {
            generated_at: Utc::now(),
            total_files: total,
            success_count,
            failure_count,
            events,
        }
    }

    /// Saves the report to the specified path in the given format.
    pub fn save(&self, path: &Path, format: AuditFormat) -> std::io::Result<()> {
        let content = match format {
            AuditFormat::Json => {
                serde_json::to_string_pretty(self).map_err(std::io::Error::other)?
            }
            AuditFormat::Text => self.to_text(),
        };

        let mut file = File::create(path)?;
        file.write_all(content.as_bytes())?;
        Ok(())
    }

    fn to_text(&self) -> String {
        let mut text = String::new();
        text.push_str("=== sys-shred Forensic Audit Report ===\n");
        text.push_str(&format!("Generated at: {}\n", self.generated_at));
        text.push_str(&format!("Total Files:  {}\n", self.total_files));
        text.push_str(&format!("Successes:    {}\n", self.success_count));
        text.push_str(&format!("Failures:     {}\n", self.failure_count));
        text.push_str("========================================\n\n");

        for event in &self.events {
            let status = if event.success { "SUCCESS" } else { "FAILURE" };
            text.push_str(&format!(
                "[{}] {:?} - {} ({:?})\n",
                event.timestamp.format("%Y-%m-%d %H:%M:%S"),
                event.path,
                status,
                event.method
            ));
            if let Some(ref err) = event.error {
                text.push_str(&format!("  Error: {}\n", err));
            }
        }

        text
    }
}
