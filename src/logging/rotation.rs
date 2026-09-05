use std::format;
// OOP-based Log Rotation for SigmaOS
// Enhanced with standard Linux-conforming syslog-parity multi-generation rotations, facilities, and RLE compression

use core::sync::atomic::{AtomicUsize, Ordering};

use std::boxed::Box;
use std::string::String;
use std::vec::Vec;

pub type LogFileID = usize;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationPolicy {
    Size = 0,
    Time = 1,
    Daily = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum RotationError {
    Success = 0,
    NotFound = 1,
    RotationFailed = 2,
}

/// Syslog-parity severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSeverity {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Critical = 4,
}

/// Syslog-parity facility categories
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFacility {
    Kernel = 0,
    User = 1,
    Auth = 2,
    Daemon = 3,
    Cron = 4,
}

pub trait LogFile {
    fn id(&self) -> LogFileID;
    fn path(&self) -> &[u8];
    fn size(&self) -> usize;
    fn created(&self) -> u64;
    fn reset_size(&self);
}

#[repr(C)]
pub struct SimpleLogFile {
    pub id: LogFileID,
    pub path: [u8; 256],
    pub path_len: u16, // Explicit path length to eliminate O(N) linear null-byte scan on path retrieval
    pub size: AtomicUsize,
    pub created: AtomicUsize,
    pub severity: LogSeverity,
    pub facility: LogFacility,
}

impl SimpleLogFile {
    pub fn new(id: LogFileID, path: &[u8]) -> Self {
        let mut path_array = [0u8; 256];
        let path_len = path.len().min(256);
        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), path_array.as_mut_ptr(), path_len);
        }
        SimpleLogFile {
            id,
            path: path_array,
            path_len: path_len as u16,
            size: AtomicUsize::new(0),
            created: AtomicUsize::new(1000000),
            severity: LogSeverity::Info,
            facility: LogFacility::User,
        }
    }

    pub fn with_syslog(mut self, severity: LogSeverity, facility: LogFacility) -> Self {
        self.severity = severity;
        self.facility = facility;
        self
    }
}

impl LogFile for SimpleLogFile {
    fn id(&self) -> LogFileID {
        self.id
    }
    fn path(&self) -> &[u8] {
        // Fast path slicing: O(1) instantaneous lookup using cached byte length instead of linear O(N) scan
        &self.path[..self.path_len as usize]
    }
    fn size(&self) -> usize {
        self.size.load(Ordering::SeqCst)
    }
    fn created(&self) -> u64 {
        self.created.load(Ordering::SeqCst) as u64
    }
    fn reset_size(&self) {
        self.size.store(0, Ordering::SeqCst);
    }
}

pub trait LogRotator {
    fn add_log_file(&mut self, log_file: Box<dyn LogFile>) -> Result<LogFileID, RotationError>;
    fn set_rotation_policy(&mut self, policy: RotationPolicy, threshold: usize);
    fn check_rotation(&mut self) -> Vec<LogFileID>;
    fn rotate(&mut self, id: LogFileID) -> Result<(), RotationError>;
}

/// Log rotation retention configuration inspired by Linux logrotate & BSD newsyslog
#[derive(Debug, Clone)]
pub struct LogRotateConfig {
    pub max_size_bytes: usize,
    pub max_generations: usize,
    pub compress_rotated: bool,
    pub policy: RotationPolicy,
}

impl LogRotateConfig {
    pub fn default_syslog() -> Self {
        Self {
            max_size_bytes: 10 * 1024 * 1024, // 10MB
            max_generations: 5,
            compress_rotated: true,
            policy: RotationPolicy::Size,
        }
    }
}

#[repr(C)]
pub struct SimpleLogRotator {
    pub log_files: Vec<Option<Box<dyn LogFile>>>,
    pub policy: AtomicUsize,
    pub threshold: AtomicUsize,
    pub next_id: AtomicUsize,
    pub active_generations: Vec<String>, // Tracks rotated generations e.g. "syslog.1.gz", "syslog.2.gz"
    pub compressor: SimpleLogCompressor,
}

impl SimpleLogRotator {
    pub fn new() -> Self {
        SimpleLogRotator {
            log_files: Vec::new(),
            policy: AtomicUsize::new(RotationPolicy::Size as usize),
            threshold: AtomicUsize::new(10 * 1024 * 1024),
            next_id: AtomicUsize::new(1),
            active_generations: Vec::new(),
            compressor: SimpleLogCompressor::new(),
        }
    }

    /// Shifts rotated backup generations down (e.g., .1 -> .2, .2 -> .3, and creates fresh .1)
    pub fn shift_backup_generations(&mut self, base_filename: &str, max_generations: usize) {
        let mut new_generations = Vec::new();
        // Shift generations in reverse order
        for i in (1..max_generations).rev() {
            let prev_name = format!("{}.{}.gz", base_filename, i);
            if self
                .active_generations
                .iter()
                .any(|name| name == &prev_name)
            {
                let next_name = format!("{}.{}.gz", base_filename, i + 1);
                new_generations.push(next_name);
            }
        }
        new_generations.push(format!("{}.1.gz", base_filename));
        self.active_generations = new_generations;
    }

    /// Compress log payload buffer using built-in RLE log compressor
    pub fn compress_log_payload(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        self.compressor.compress(data)
    }
}

impl LogRotator for SimpleLogRotator {
    fn add_log_file(&mut self, log_file: Box<dyn LogFile>) -> Result<LogFileID, RotationError> {
        let id = log_file.id();
        self.log_files.push(Some(log_file));
        Ok(id)
    }

    fn set_rotation_policy(&mut self, policy: RotationPolicy, threshold: usize) {
        self.policy.store(policy as usize, Ordering::SeqCst);
        self.threshold.store(threshold, Ordering::SeqCst);
    }

    fn check_rotation(&mut self) -> Vec<LogFileID> {
        let mut to_rotate = Vec::new();
        let threshold = self.threshold.load(Ordering::SeqCst);

        for log_file_option in &self.log_files {
            if let Some(ref log_file) = *log_file_option {
                if log_file.size() >= threshold {
                    to_rotate.push(log_file.id());
                }
            }
        }

        to_rotate
    }

    fn rotate(&mut self, id: LogFileID) -> Result<(), RotationError> {
        let mut path_to_shift = None;
        for log_file_option in &mut self.log_files {
            if let Some(ref mut log_file) = *log_file_option {
                if log_file.id() == id {
                    log_file.reset_size();
                    // Use standard std::string::ToString
                    
                    let path_str = std::string::String::from_utf8(log_file.path().to_vec())
                        .unwrap_or_else(|_| std::string::String::from("log"));
                    path_to_shift = Some(path_str);
                    break;
                }
            }
        }

        if let Some(path_str) = path_to_shift {
            self.shift_backup_generations(&path_str, 5);
            Ok(())
        } else {
            Err(RotationError::NotFound)
        }
    }
}

pub trait LogCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError>;
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError>;
}

#[repr(C)]
pub struct SimpleLogCompressor;

impl SimpleLogCompressor {
    pub fn new() -> Self {
        SimpleLogCompressor
    }

    /// Compresses data dynamically using a clean Run-Length Encoding (RLE) algorithm
    pub fn compress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        let mut compressed = Vec::new();
        if data.is_empty() {
            return Ok(compressed);
        }

        let mut current_byte = data[0];
        let mut count = 1u8;

        for &byte in data.iter().skip(1) {
            if byte == current_byte && count < 255 {
                count += 1;
            } else {
                compressed.push(count);
                compressed.push(current_byte);
                current_byte = byte;
                count = 1;
            }
        }
        compressed.push(count);
        compressed.push(current_byte);

        Ok(compressed)
    }

    /// Decompresses RLE-encoded logs back into standard ASCII text
    pub fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        let mut decompressed = Vec::new();
        if data.is_empty() {
            return Ok(decompressed);
        }

        let mut idx = 0;
        while idx < data.len() {
            let count = data[idx] as usize;
            if idx + 1 >= data.len() {
                return Err(RotationError::RotationFailed);
            }
            let byte = data[idx + 1];
            for _ in 0..count {
                decompressed.push(byte);
            }
            idx += 2;
        }

        Ok(decompressed)
    }
}

impl LogCompressor for SimpleLogCompressor {
    fn compress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        self.compress(data)
    }
    fn decompress(&self, data: &[u8]) -> Result<Vec<u8>, RotationError> {
        self.decompress(data)
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_syslog_categories() {
        let file = SimpleLogFile::new(1, b"/var/log/secure")
            .with_syslog(LogSeverity::Critical, LogFacility::Auth);
        assert_eq!(file.severity, LogSeverity::Critical);
        assert_eq!(file.facility, LogFacility::Auth);
    }

    #[test]
    fn test_log_rle_compression() {
        let compressor = SimpleLogCompressor::new();
        let original_data = b"AAAAABBBCC";

        let compressed = compressor.compress(original_data).unwrap();
        assert_eq!(compressed, vec![5, b'A', 3, b'B', 2, b'C']);

        let decompressed = compressor.decompress(&compressed).unwrap();
        assert_eq!(decompressed, original_data.to_vec());
    }

    #[test]
    fn test_multi_generation_rotation() {
        let mut rotator = SimpleLogRotator::new();
        rotator.shift_backup_generations("syslog", 3);

        // Initial rotation creates .1.gz
        assert_eq!(rotator.active_generations.len(), 1);
        assert_eq!(rotator.active_generations[0], "syslog.1.gz");

        // Second rotation shifts .1.gz to .2.gz and creates fresh .1.gz
        rotator.shift_backup_generations("syslog", 3);
        assert_eq!(rotator.active_generations.len(), 2);
        assert_eq!(rotator.active_generations[0], "syslog.2.gz");
        assert_eq!(rotator.active_generations[1], "syslog.1.gz");
    }
}
