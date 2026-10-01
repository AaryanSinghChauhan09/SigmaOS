
use std::boxed::Box;
use std::vec::Vec;

use core::sync::atomic::{AtomicUsize, Ordering};

/// File ID
pub type FileID = usize;

/// File integrity status
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityStatus {
    Valid = 0,
    Modified = 1,
    Corrupted = 2,
    Missing = 3,
    Unverified = 4,
    Invalid = usize::MAX,
}

/// File trait (OOP interface)
pub trait File {
    /// Get file ID
    fn id(&self) -> FileID;
    /// Get file path
    fn path(&self) -> &[u8];
    /// Get expected checksum
    fn checksum(&self) -> &[u8];
    /// Verify integrity
    fn verify(&mut self) -> Result<IntegrityStatus, IntegrityError>;
    /// Get file info
    fn info(&self) -> FileInfo;
}

/// Integrity error types
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum IntegrityError {
    Success = 0,
    FileNotFound = 1,
    ReadFailed = 2,
    PermissionDenied = 3,
    VerificationUnavailable = 4,
}

/// File info
#[repr(C)]
pub struct FileInfo {
    pub id: FileID,
    pub path: [u8; 256],
    pub checksum: [u8; 64],
    pub status: IntegrityStatus,
    pub capability: FileCapability,
}

impl FileInfo {
    pub fn new(id: FileID) -> Self {
        FileInfo {
            id,
            path: [0; 256],
            checksum: [0; 64],
            status: IntegrityStatus::Unverified,
            capability: FileCapability::new(),
        }
    }
}

/// File capability
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct FileCapability {
    pub can_verify: bool,
    pub can_modify: bool,
}

impl FileCapability {
    pub fn new() -> Self {
        FileCapability {
            can_verify: false,
            can_modify: false,
        }
    }

    pub fn full() -> Self {
        FileCapability {
            can_verify: true,
            can_modify: true,
        }
    }
}

/// Simple file (OOP: Concrete file class)
#[repr(C)]
pub struct SimpleFile {
    pub id: FileID,
    path: [u8; 256],
    path_len: u16,
    checksum: [u8; 64],
    checksum_len: u8,
    status: IntegrityStatus,
    pub capability: FileCapability,
}

impl SimpleFile {
    pub fn new(id: FileID, path: &[u8], checksum: &[u8], capability: FileCapability) -> Self {
        let mut path_array = [0u8; 256];
        let mut checksum_array = [0u8; 64];

        let path_len = path.len().min(255);
        let checksum_len = checksum.len().min(63);

        path_array[..path_len].copy_from_slice(&path[..path_len]);
        checksum_array[..checksum_len].copy_from_slice(&checksum[..checksum_len]);

        SimpleFile {
            id,
            path: path_array,
            path_len: path_len as u16,
            checksum: checksum_array,
            checksum_len: checksum_len as u8,
            status: IntegrityStatus::Unverified,
            capability,
        }
    }

    pub fn get_status(&self) -> IntegrityStatus {
        self.status
    }
}

impl File for SimpleFile {
    fn id(&self) -> FileID {
        self.id
    }

    fn path(&self) -> &[u8] {
        // O(1) slice lookup using cached path_len, avoiding O(N) zero-byte linear scan (.position(|&b| b == 0))
        &self.path[..self.path_len as usize]
    }

    fn checksum(&self) -> &[u8] {
        // O(1) slice lookup using cached checksum_len, avoiding O(N) zero-byte linear scan (.position(|&b| b == 0))
        &self.checksum[..self.checksum_len as usize]
    }

    fn verify(&mut self) -> Result<IntegrityStatus, IntegrityError> {
        if !self.capability.can_verify {
            return Err(IntegrityError::PermissionDenied);
        }

        // This model has no file reader or trusted checksum implementation.
        // Preserve the unverified state instead of reporting a false success.
        Err(IntegrityError::VerificationUnavailable)
    }

    fn info(&self) -> FileInfo {
        FileInfo {
            id: self.id,
            path: self.path,
            checksum: self.checksum,
            status: self.get_status(),
            capability: self.capability,
        }
    }
}

/// Integrity monitor trait (OOP interface)
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

impl SimpleIntegrityMonitor {
    pub fn new(capability: MonitorCapability) -> Self {
        SimpleIntegrityMonitor {
            files: Vec::new(),
            next_id: AtomicUsize::new(1),
            stats: IntegrityStats::new(),
            capability,
        }
    }

    fn update_stats(&mut self, status: IntegrityStatus) {
        match status {
            IntegrityStatus::Valid => {
                self.stats.valid_files = self.stats.valid_files.saturating_add(1)
            }
            IntegrityStatus::Modified => {
                self.stats.modified_files = self.stats.modified_files.saturating_add(1)
            }
            IntegrityStatus::Corrupted => {
                self.stats.corrupted_files = self.stats.corrupted_files.saturating_add(1)
            }
            IntegrityStatus::Missing | IntegrityStatus::Unverified | IntegrityStatus::Invalid => {}
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
        self.stats.total_files = self.stats.total_files.saturating_add(1);
        Ok(id)
    }

    fn unregister_file(&mut self, id: FileID) -> Result<(), IntegrityError> {
        if !self.capability.can_register {
            return Err(IntegrityError::PermissionDenied);
        }

        let mut index = None;
        for i in 0..self.files.len() {
            if let Some(Some(ref file)) = self.files.get(i) {
                if file.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            if let Some(slot) = self.files.get_mut(i) {
                *slot = None;
            }
            self.stats.total_files = self.stats.total_files.saturating_sub(1);
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
            if let Some(Some(ref mut file)) = self.files.get_mut(i) {
                if file.id() == id {
                    let result = file.verify();
                    if let Ok(status) = result {
                        self.update_stats(status);
                        return Ok(status);
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
            if let Some(Some(ref mut file)) = self.files.get_mut(i) {
                let result = file.verify();
                match result {
                    Ok(status) => {
                        if status != IntegrityStatus::Valid {
                            modified_files.push(file.id());
                        }
                        self.update_stats(status);
                    }
                    Err(error) => return Err(error),
                }
            }
        }

        Ok(modified_files)
    }

    fn get_file(&self, id: FileID) -> Option<&dyn File> {
        for i in 0..self.files.len() {
            if let Some(Some(ref file)) = self.files.get(i) {
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

pub struct IntegrityCheck;
pub struct IntegrityVerifier;

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_file_and_integrity_monitor() {
        let capability = FileCapability::full();
        let mut file = SimpleFile::new(1, b"/var/www/index.html", b"checksum123", capability);
        assert_eq!(file.id(), 1);
        assert_eq!(file.path(), b"/var/www/index.html");
        assert_eq!(file.checksum(), b"checksum123");
        assert!(matches!(
            file.verify(),
            Err(IntegrityError::VerificationUnavailable)
        ));
        assert_eq!(file.get_status(), IntegrityStatus::Unverified);

        let monitor_cap = MonitorCapability::full();
        let mut monitor = SimpleIntegrityMonitor::new(monitor_cap);
        let id = monitor.register_file(Box::new(file)).unwrap();
        assert_eq!(id, 1);

        assert!(matches!(
            monitor.verify_file(1),
            Err(IntegrityError::VerificationUnavailable)
        ));

        let stats = monitor.stats();
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.valid_files, 0);

        assert!(matches!(
            monitor.verify_all(),
            Err(IntegrityError::VerificationUnavailable)
        ));

        monitor.unregister_file(1).unwrap();
        assert_eq!(monitor.stats().total_files, 0);
    }
}
