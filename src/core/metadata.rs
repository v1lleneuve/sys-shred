//! # Metadata & Filename Obfuscation
//!
//! Handles the transformation of file metadata to prevent path-based recovery
//! and information leakage through filenames.

use crate::error::{ShredError, ShredResult};
use rand::{distributions::Alphanumeric, Rng};
use std::fs;
use std::path::{Path, PathBuf};

#[cfg(unix)]
use std::os::unix::io::AsRawFd;

/// A utility for scrubbing and randomizing file metadata.
///
/// This component focuses on the anti-forensic aspects of shredding that go
/// beyond raw data destruction.
pub struct MetadataHandler;

impl MetadataHandler {
    /// Renames a file to a randomly generated alphanumeric string.
    pub fn obfuscate_filename(path: &Path) -> ShredResult<PathBuf> {
        let parent = path.parent().unwrap_or_else(|| Path::new("."));

        // Retry logic to handle rare filename collisions
        for _ in 0..5 {
            let random_name: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(16)
                .map(char::from)
                .collect();

            let new_path = parent.join(random_name);
            if !new_path.exists() {
                fs::rename(path, &new_path)?;
                return Ok(new_path);
            }
        }

        Err(ShredError::Obfuscation(
            "Failed to generate a unique random filename after multiple attempts".to_string(),
        ))
    }

    /// Truncates a file to zero bytes and flushes the change.
    pub fn truncate(path: &Path) -> ShredResult<()> {
        let file = fs::OpenOptions::new().write(true).open(path)?;
        file.set_len(0)?;
        file.sync_all()?;
        Ok(())
    }

    /// Informs the OS/Hardware to discard the file's blocks (TRIM).
    ///
    /// On Linux, this uses `fallocate` with `FALLOC_FL_PUNCH_HOLE`.
    /// On Windows, it uses `FSCTL_SET_ZERO_DATA`.
    pub fn trim(path: &Path) -> ShredResult<()> {
        let file = fs::OpenOptions::new().write(true).open(path)?;
        let len = file.metadata()?.len();
        if len == 0 {
            return Ok(());
        }

        #[cfg(target_os = "linux")]
        {
            let fd = file.as_raw_fd();
            let res = unsafe {
                // FALLOC_FL_PUNCH_HOLE (0x02) | FALLOC_FL_KEEP_SIZE (0x01)
                libc::fallocate(fd, 0x01 | 0x02, 0, len as libc::off_t)
            };
            if res != 0 {
                return Err(ShredError::Io(std::io::Error::last_os_error()));
            }
        }

        #[cfg(windows)]
        {
            use std::os::windows::io::AsRawHandle;
            use windows_sys::Win32::Foundation::HANDLE;
            use windows_sys::Win32::Storage::FileSystem::{
                FILE_SET_ZERO_DATA_INFORMATION, FSCTL_SET_ZERO_DATA,
            };
            use windows_sys::Win32::System::IO::DeviceIoControl;

            let handle = file.as_raw_handle() as HANDLE;
            let mut info = FILE_SET_ZERO_DATA_INFORMATION {
                FileOffset: 0,
                BeyondFinalZero: len as i64,
            };
            let mut bytes_returned = 0;

            unsafe {
                DeviceIoControl(
                    handle,
                    FSCTL_SET_ZERO_DATA,
                    &mut info as *mut _ as *mut _,
                    std::mem::size_of::<FILE_SET_ZERO_DATA_INFORMATION>() as u32,
                    std::ptr::null_mut(),
                    0,
                    &mut bytes_returned,
                    std::ptr::null_mut(),
                );
            }
        }

        file.sync_all()?;
        Ok(())
    }
}
