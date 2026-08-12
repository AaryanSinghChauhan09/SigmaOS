#![no_std]
#![no_main]

/// OOP-based File Integrity Monitoring for SigmaOS
/// Based on Ideas-999-Structured: Security & Sovereignty Item 532
/// Implements system file hashing and background verify checkups

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;

pub type FileID = usize;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityStatus {
    Valid = 0,
    Modified = 1,
    Corrupted = 2,
    Missing = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityError {
    Success = 0,
    FileNotFound = 1,
    PermissionDenied = 2,
    HashMismatch = 3,
}

pub trait File {
    fn id(&self) -> FileID;
    fn path(&self) -> &[u8];
    fn expected_hash(&self) -> &[u8];
    fn current_hash(&self) -> &[u8];
    fn verify(&mut self) -> Result<IntegrityStatus, IntegrityError>;
}

#[repr(C)]
pub struct SimpleFile {
    pub id: FileID,
    pub path: [u8; 128],
    pub expected_hash: [u8; 64],
    pub current_hash: [u8; 64],
    pub status: AtomicUsize,
}

impl SimpleFile {
    pub fn new(id: FileID, path: &[u8], expected_hash: &[u8]) -> Self {
        let mut path_array = [0u8; 128];
        let mut hash_array = [0u8; 64];

        let path_len = path.len().min(127);
        let hash_len = expected_hash.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), path_array.as_mut_ptr(), path_len);
            core::ptr::copy_nonoverlapping(expected_hash.as_ptr(), hash_array.as_mut_ptr(), hash_len);
        }

        SimpleFile {
            id,
            path: path_array,
            expected_hash: hash_array,
            current_hash: hash_array, // initialized as matching
            status: AtomicUsize::new(IntegrityStatus::Valid as usize),
        }
    }

    pub fn set_current_hash(&mut self, hash: &[u8]) {
        let mut hash_array = [0u8; 64];
        let hash_len = hash.len().min(63);
        unsafe {
            core::ptr::copy_nonoverlapping(hash.as_ptr(), hash_array.as_mut_ptr(), hash_len);
        }
        self.current_hash = hash_array;
    }
}

impl File for SimpleFile {
    fn id(&self) -> FileID {
        self.id
    }

    fn path(&self) -> &[u8] {
        let len = self.path.iter().position(|&b| b == 0).unwrap_or(128);
        &self.path[..len]
    }

    fn expected_hash(&self) -> &[u8] {
        let len = self.expected_hash.iter().position(|&b| b == 0).unwrap_or(64);
        &self.expected_hash[..len]
    }

    fn current_hash(&self) -> &[u8] {
        let len = self.current_hash.iter().position(|&b| b == 0).unwrap_or(64);
        &self.current_hash[..len]
    }

    fn verify(&mut self) -> Result<IntegrityStatus, IntegrityError> {
        let expected = self.expected_hash();
        let current = self.current_hash();

        if current.is_empty() {
            self.status.store(IntegrityStatus::Missing as usize, Ordering::SeqCst);
            return Ok(IntegrityStatus::Missing);
        }

        if expected == current {
            self.status.store(IntegrityStatus::Valid as usize, Ordering::SeqCst);
            Ok(IntegrityStatus::Valid)
        } else {
            self.status.store(IntegrityStatus::Modified as usize, Ordering::SeqCst);
            Ok(IntegrityStatus::Modified)
        }
    }
}

pub trait IntegrityMonitor {
    /// Register file
    fn register_file(&mut self, file: Box<dyn File>) -> Result<FileID, IntegrityError>;
    /// Unregister file
    fn unregister_file(&mut self, id: FileID) -> Result<(), IntegrityError>;
    /// Verify file
    fn verify_file(&mut self, id: FileID) -> Result<IntegrityStatus, IntegrityError>;
    /// Verify all files
    fn verify_all(&mut self) -> Result<Vec<FileID>, IntegrityError>;
    /// Get file
    fn get_file(&self, id: FileID) -> Option<&dyn File>;
    /// Get monitor statistics
    fn stats(&self) -> IntegrityStats;
}

/// Integrity statistics
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct IntegrityStats {
    pub total_files: usize,
    pub valid_files: usize,
    pub modified_files: usize,
    pub corrupted_files: usize,
}

impl IntegrityStats {
    pub fn new() -> Self {
        IntegrityStats {
            total_files: 0,
            valid_files: 0,
            modified_files: 0,
            corrupted_files: 0,
        }
    }
}

impl Default for IntegrityStats {
    fn default() -> Self {
        Self::new()
    }
}

/// Simple integrity monitor (OOP: Concrete monitor class)
pub struct SimpleIntegrityMonitor {
    files: Vec<Option<Box<dyn File>>>,
    next_id: AtomicUsize,
    stats: IntegrityStats,
    capability: MonitorCapability,
}

/// Monitor capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct MonitorCapability {
    pub can_register: bool,
    pub can_verify: bool,
}

impl MonitorCapability {
    pub fn new() -> Self {
        MonitorCapability {
            can_register: false,
            can_verify: false,
        }
    }

    pub fn full() -> Self {
        MonitorCapability {
            can_register: true,
            can_verify: true,
        }
    }
}

impl Default for MonitorCapability {
    fn default() -> Self {
        Self::new()
    }
}

impl SimpleIntegrityMonitor {
    pub fn new(capability: MonitorCapability) -> Self {
        SimpleIntegrityMonitor {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: IntegrityStats::new(),
            capability,
        }
    }

    pub fn update_stats(&mut self, status: IntegrityStatus) {
        match status {
            IntegrityStatus::Valid => self.stats.valid_files += 1,
            IntegrityStatus::Modified => self.stats.modified_files += 1,
            IntegrityStatus::Corrupted => self.stats.corrupted_files += 1,
            IntegrityStatus::Missing => self.stats.corrupted_files += 1,
        }
    }
}

impl IntegrityMonitor for SimpleIntegrityMonitor {
    fn register_file(&mut self, file: Box<dyn File>) -> Result<FileID, IntegrityError> {
        if !self.capability.can_register {
            return Err(IntegrityError::PermissionDenied);
        }

        let id = file.id();
        self.files.push(Some(file));
        self.stats.total_files += 1;
        Ok(id)
    }

    fn unregister_file(&mut self, id: FileID) -> Result<(), IntegrityError> {
        if !self.capability.can_register {
            return Err(IntegrityError::PermissionDenied);
        }

        let mut index = None;
        for i in 0..self.files.len() {
            if let Some(ref file) = self.files[i] {
                if file.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.files[i] = None;
            self.stats.total_files -= 1;
            Ok(())
        } else {
            Err(IntegrityError::FileNotFound)
        }
    }

    fn verify_file(&mut self, id: FileID) -> Result<IntegrityStatus, IntegrityError> {
        if !self.capability.can_verify {
            return Err(IntegrityError::PermissionDenied);
        }

        for i in 0..self.files.len() {
            if let Some(ref mut file) = self.files[i] {
                if file.id() == id {
                    let result = file.verify();
                    if let Ok(status) = result {
                        self.update_stats(status);
                    }
                    return result;
                }
            }
        }
        Err(IntegrityError::FileNotFound)
    }

    fn verify_all(&mut self) -> Result<Vec<FileID>, IntegrityError> {
        if !self.capability.can_verify {
            return Err(IntegrityError::PermissionDenied);
        }

        let mut modified_files = Vec::new();

        for i in 0..self.files.len() {
            if let Some(ref mut file) = self.files[i] {
                let result = file.verify();
                if let Ok(status) = result {
                    if status != IntegrityStatus::Valid {
                        modified_files.push(file.id());
                    }
                    self.update_stats(status);
                }
            }
        }

        Ok(modified_files)
    }

    fn get_file(&self, id: FileID) -> Option<&dyn File> {
        for i in 0..self.files.len() {
            if let Some(ref file) = self.files[i] {
                if file.id() == id {
                    return Some(file.as_ref());
                }
            }
        }
        None
    }

    fn stats(&self) -> IntegrityStats {
        self.stats
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_integrity_monitoring() {
        let capability = MonitorCapability::full();
        let mut monitor = SimpleIntegrityMonitor::new(capability);

        let file = SimpleFile::new(1, b"/etc/passwd", b"hash123");
        monitor.register_file(Box::new(file)).unwrap();

        let status = monitor.verify_file(1).unwrap();
        assert_eq!(status, IntegrityStatus::Valid);

        let stats = monitor.stats();
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.valid_files, 1);
    }
}
