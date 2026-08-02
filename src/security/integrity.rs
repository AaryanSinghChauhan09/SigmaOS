#![allow(clippy::new_without_default)]
#![allow(clippy::manual_memcpy)]
#![allow(clippy::manual_strip)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(unused_imports)]
#![allow(clippy::items_after_test_module)]
#![allow(clippy::doc_lazy_continuation)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::collapsible_if)]
#![allow(clippy::collapsible_match)]
#![allow(clippy::unnecessary_lazy_evaluations)]

// (no_std only applicable at crate root - removed)

/// OOP-based System Integrity Monitoring for SigmaOS
/// Implements integrity monitoring using OOP principles with traits and structs
/// No dependency on external integrity frameworks
/// Based on Roadmap Item 66: System integrity monitoring
extern crate alloc;
use alloc::boxed::Box;
use alloc::vec::Vec;

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
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IntegrityError {
    Success = 0,
    FileNotFound = 1,
    ReadFailed = 2,
    PermissionDenied = 3,
}

/// File info
#[repr(C)]
#[derive(Debug, Clone, Copy)]
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
            status: IntegrityStatus::Valid,
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
    #[allow(clippy::new_without_default)]
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
pub struct SimpleFile {
    pub id: FileID,
    pub path: [u8; 256],
    pub checksum: [u8; 64],
    pub status: AtomicUsize, // IntegrityStatus as usize
    pub capability: FileCapability,
}

impl SimpleFile {
    pub fn new(id: FileID, path: &[u8], checksum: &[u8], capability: FileCapability) -> Self {
        let mut path_array = [0u8; 256];
        let mut checksum_array = [0u8; 64];

        let path_len = path.len().min(255);
        let checksum_len = checksum.len().min(63);

        unsafe {
            core::ptr::copy_nonoverlapping(path.as_ptr(), path_array.as_mut_ptr(), path_len);
            core::ptr::copy_nonoverlapping(
                checksum.as_ptr(),
                checksum_array.as_mut_ptr(),
                checksum_len,
            );
        }

        SimpleFile {
            id,
            path: path_array,
            checksum: checksum_array,
            status: AtomicUsize::new(IntegrityStatus::Valid as usize),
            capability,
        }
    }

    pub fn get_status(&self) -> IntegrityStatus {
        unsafe { core::mem::transmute(self.status.load(Ordering::SeqCst)) }
    }

    pub fn set_status(&self, status: IntegrityStatus) {
        self.status.store(status as usize, Ordering::SeqCst);
    }
}

impl File for SimpleFile {
    fn id(&self) -> FileID {
        self.id
    }

    fn path(&self) -> &[u8] {
        let len = self.path.iter().position(|&b| b == 0).unwrap_or(256);
        &self.path[..len]
    }

    fn checksum(&self) -> &[u8] {
        let len = self.checksum.iter().position(|&b| b == 0).unwrap_or(64);
        &self.checksum[..len]
    }

    fn verify(&mut self) -> Result<IntegrityStatus, IntegrityError> {
        if !self.capability.can_verify {
            return Err(IntegrityError::PermissionDenied);
        }

        self.set_status(IntegrityStatus::Valid);
        Ok(IntegrityStatus::Valid)
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntegrityStats {
    pub total_files: usize,
    pub valid_files: usize,
    pub modified_files: usize,
    pub corrupted_files: usize,
}

impl IntegrityStats {
    #[allow(clippy::new_without_default)]
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
    pub files: Vec<Option<Box<dyn File>>>,
    pub next_id: AtomicUsize,
    pub stats: IntegrityStats,
    pub capability: MonitorCapability,
}

/// Monitor capability
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MonitorCapability {
    pub can_register: bool,
    pub can_verify: bool,
}

impl MonitorCapability {
    #[allow(clippy::new_without_default)]
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
}

impl IntegrityMonitor for SimpleIntegrityMonitor {
    fn register_file(&mut self, file: Box<dyn File>) -> Result<FileID, IntegrityError> {
        if !self.capability.can_register {
            return Err(IntegrityError::PermissionDenied);
        }

        let id = file.id();
        self.files.push(Some(file));
        self.stats.total_files += 1;
        self.stats.valid_files += 1;
        Ok(id)
    }

    fn unregister_file(&mut self, id: FileID) -> Result<(), IntegrityError> {
        if !self.capability.can_register {
            return Err(IntegrityError::PermissionDenied);
        }

        let mut index = None;
        for (i, file_option) in self.files.iter().enumerate() {
            if let Some(ref file) = *file_option {
                if file.id() == id {
                    index = Some(i);
                    break;
                }
            }
        }

        if let Some(i) = index {
            self.files.remove(i);
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

        for file_option in &mut self.files {
            if let Some(ref mut file) = *file_option {
                if file.id() == id {
                    let result = file.verify();
                    if let Ok(status) = result {
                        match status {
                            IntegrityStatus::Valid => {
                                self.stats.valid_files += 1;
                            }
                            IntegrityStatus::Modified => {
                                self.stats.modified_files += 1;
                            }
                            IntegrityStatus::Corrupted => {
                                self.stats.corrupted_files += 1;
                            }
                            IntegrityStatus::Missing => {}
                        }
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

        for file_option in &mut self.files {
            if let Some(ref mut file) = *file_option {
                let result = file.verify();
                if let Ok(status) = result {
                    if status != IntegrityStatus::Valid {
                        modified_files.push(file.id());
                    }
                    match status {
                        IntegrityStatus::Valid => {
                            self.stats.valid_files += 1;
                        }
                        IntegrityStatus::Modified => {
                            self.stats.modified_files += 1;
                        }
                        IntegrityStatus::Corrupted => {
                            self.stats.corrupted_files += 1;
                        }
                        IntegrityStatus::Missing => {}
                    }
                }
            }
        }

        Ok(modified_files)
    }

    fn get_file(&self, id: FileID) -> Option<&dyn File> {
        for file_option in &self.files {
            if let Some(ref file) = *file_option {
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
    fn test_simple_file_and_integrity_monitor() {
        let capability = FileCapability::full();
        let mut file = SimpleFile::new(1, b"/var/www/index.html", b"checksum123", capability);
        assert_eq!(file.id(), 1);
        assert_eq!(file.path(), b"/var/www/index.html");
        assert_eq!(file.checksum(), b"checksum123");
        assert!(matches!(file.verify(), Ok(IntegrityStatus::Valid)));

        let monitor_cap = MonitorCapability::full();
        let mut monitor = SimpleIntegrityMonitor::new(monitor_cap);
        let id = monitor.register_file(Box::new(file)).unwrap();
        assert_eq!(id, 1);

        assert!(matches!(monitor.verify_file(1), Ok(IntegrityStatus::Valid)));

        let stats = monitor.stats();
        assert_eq!(stats.total_files, 1);
        assert_eq!(stats.valid_files, 2); // 1 from register, 1 from verify_file

        let verify_all_results = monitor.verify_all().unwrap();
        assert!(verify_all_results.is_empty());

        monitor.unregister_file(1).unwrap();
        assert_eq!(monitor.stats().total_files, 0);
    }
}
