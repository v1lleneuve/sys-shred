//! # Low-Level Overwrite Engine
//!
//! This module implements the core byte-level overwriting logic. It is designed
//! to be hardware-aware, utilizing `sync_all` to bypass Operating System
//! write buffers and ensure data reaches the physical storage medium.

use crate::error::ShredResult;
use rand::{rngs::StdRng, RngCore, SeedableRng};
use std::fs::File;
use std::io::{Seek, SeekFrom, Write};

/// A specialized engine for performing secure cryptographic overwrites on a file.
///
/// The `Overwriter` maintains a handle to the target file and performs
/// sequential byte-level destruction using a buffered writing approach.
pub struct Overwriter<'a> {
    /// Mutable reference to the file being shredded.
    file: &'a mut File,
    /// The size of the memory buffer used for writing chunks.
    buffer_size: usize,
}

impl<'a> Overwriter<'a> {
    /// Creates a new `Overwriter` instance for the specified file.
    ///
    /// Defaults to a 64KB buffer size, which provides a balance between
    /// memory efficiency and I/O throughput.
    ///
    /// # Arguments
    ///
    /// * `file` - A mutable reference to the target `std::fs::File`.
    pub fn new(file: &'a mut File) -> Self {
        Self {
            file,
            buffer_size: 64 * 1024,
        }
    }

    /// Executes a single complete pass of cryptographic data overwriting.
    ///
    /// This method resets the file pointer to the beginning, generates
    /// random bytes using a cryptographically secure RNG (`StdRng`),
    /// writes them across the entire span of the file, and forces a hardware flush.
    ///
    /// # Errors
    ///
    /// Returns `ShredError::Io` if any filesystem operation fails during the pass.
    ///
    /// # Security
    ///
    /// Employs `file.sync_all()` at the end of the pass to ensure that OS-level
    /// write-behind caching is bypassed, forcing the data to the physical disk.
    pub fn run_pass(&mut self) -> ShredResult<()> {
        let file_size = self.file.metadata()?.len();
        self.file.seek(SeekFrom::Start(0))?;

        let mut rng = StdRng::from_entropy();
        let mut buffer = vec![0u8; self.buffer_size];
        let mut total_written: u64 = 0;

        while total_written < file_size {
            let remaining = file_size - total_written;
            let current_chunk = std::cmp::min(self.buffer_size as u64, remaining) as usize;

            // Generate cryptographically secure random bytes
            rng.fill_bytes(&mut buffer[..current_chunk]);

            // Perform the write operation
            self.file.write_all(&buffer[..current_chunk])?;
            total_written += current_chunk as u64;
        }

        // Critical: Bypass OS caching and force persistence to hardware.
        self.file.sync_all()?;

        Ok(())
    }
}
