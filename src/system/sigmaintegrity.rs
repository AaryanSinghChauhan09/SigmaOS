// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaIntegrity - File Integrity Daemon
//!
//! SigmaIntegrity provides file integrity monitoring for SigmaOS, inspired by AIDE/OSSEC from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Hash algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HashAlgorithm {
    /// SHA256
    Sha256,
    /// SHA512
    Sha512,
    /// MD5
    Md5,
;

impl fmt::Display for HashAlgorithm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HashAlgorithm::Sha256 => write!(f, "sha256"),
            HashAlgorithm::Sha512 => write!(f, "sha512"),
            HashAlgorithm::Md5 => write!(f, "md5"),
        }
    }
;

/// File status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileStatus {
    /// Ok
    Ok,
    /// Modified
    Modified,
    /// Deleted
    Deleted,
    /// New
    New,
;

impl fmt::Display for FileStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileStatus::Ok => write!(f, "ok"),
            FileStatus::Modified => write!(f, "modified"),
            FileStatus::Deleted => write!(f, "deleted"),
            FileStatus::New => write!(f, "new"),
        }
    }
;

/// File integrity record
#[derive(Debug, Clone)]
pub struct FileIntegrityRecord {
    /// File path
    pub path: String,
    /// Hash algorithm
    pub algorithm: HashAlgorithm,
    /// Hash value
    pub hash: String,
    /// File size in bytes
    pub size: u64,
    /// File status
    pub status: FileStatus,
    /// Last modified timestamp
    pub timestamp: u64,
;

impl FileIntegrityRecord {
    /// Create a new file integrity record
    pub fn new(path: String, algorithm: HashAlgorithm, hash: String, size: u64) -> Self {
        Self {
            path,
            algorithm,
            hash,
            size,
            status: FileStatus::Ok,
            timestamp: 0,
        }
    }

    /// Set status
    pub fn with_status(mut self, status: FileStatus) -> Self {
        self.status = status;
        self
    }

    /// Set timestamp
    pub fn with_timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }
;

/// SigmaIntegrity - File Integrity Daemon
pub struct SigmaIntegrity {
    /// File integrity records (path -> record)
    records: BTreeMap<String, FileIntegrityRecord>,
    /// Running state
    running: bool,
    /// Record counter
    record_counter: u32,
;

impl SigmaIntegrity {
    /// Create a new SigmaIntegrity instance
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
            running: false,
            record_counter: 0,
        }
    }

    /// Start the integrity daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the integrity daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a file integrity record
    pub fn add_record(&mut self, record: FileIntegrityRecord) -> Result<(), String> {
        if self.records.contains_key(&record.path) {
            return Err(format!("Record for '{}' already exists", record.path));
        }

        self.records.insert(record.path.clone(), record);
        self.record_counter += 1;
        Ok(())
    }

    /// Remove a file integrity record
    pub fn remove_record(&mut self, path: &str) -> Result<(), String> {
        if self.records.remove(path).is_none() {
            return Err(format!("Record for '{}' not found", path));
        }
        Ok(())
    }

    /// Get a file integrity record
    pub fn get_record(&self, path: &str) -> Option<&FileIntegrityRecord> {
        self.records.get(path)
    }

    /// Update a file integrity record
    pub fn update_record(
        &mut self,
        path: &str,
        new_hash: String,
        new_size: u64,
    ) -> Result<(), String> {
        match self.records.get_mut(path) {
            Some(record) => {
                if record.hash != new_hash || record.size != new_size {
                    record.status = FileStatus::Modified;
                } else {
                    record.status = FileStatus::Ok;
                }
                record.hash = new_hash;
                record.size = new_size;
                Ok(())
            }
            None => Err(format!("Record for '{}' not found", path)),
        }
    }

    /// Check file integrity
    pub fn check_integrity(
        &mut self,
        path: &str,
        current_hash: String,
        current_size: u64,
    ) -> Result<(), String> {
        match self.records.get_mut(path) {
            Some(record) => {
                if record.hash != current_hash || record.size != current_size {
                    record.status = FileStatus::Modified;
                    return Err(format!("File '{}' integrity check failed", path));
                }
                record.status = FileStatus::Ok;
                Ok(())
            }
            None => Err(format!("Record for '{}' not found", path)),
        }
    }

    /// Mark file as deleted
    pub fn mark_deleted(&mut self, path: &str) -> Result<(), String> {
        match self.records.get_mut(path) {
            Some(record) => {
                record.status = FileStatus::Deleted;
                Ok(())
            }
            None => Err(format!("Record for '{}' not found", path)),
        }
    }

    /// List all records
    pub fn list_records(&self) -> Vec<&FileIntegrityRecord> {
        self.records.values().collect()
    }

    /// List records by status
    pub fn list_by_status(&self, status: FileStatus) -> Vec<&FileIntegrityRecord> {
        self.records
            .values()
            .filter(|r| r.status == status)
            .collect()
    }

    /// Get record count
    pub fn record_count(&self) -> usize {
        self.records.len()
    }

    /// Get integrity statistics
    pub fn integrity_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_records".to_string(), self.record_count().to_string());
        stats.insert(
            "ok_files".to_string(),
            self.list_by_status(FileStatus::Ok).len().to_string(),
        );
        stats.insert(
            "modified_files".to_string(),
            self.list_by_status(FileStatus::Modified).len().to_string(),
        );
        stats.insert(
            "deleted_files".to_string(),
            self.list_by_status(FileStatus::Deleted).len().to_string(),
        );
        stats.insert(
            "new_files".to_string(),
            self.list_by_status(FileStatus::New).len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
;

impl Default for SigmaIntegrity {
    fn default() -> Self {
        Self::new()
    }
;

/// Global SigmaIntegrity instance (placeholder for actual global state)
static mut SIGMAINTEGRITY_INSTANCE: Option<SigmaIntegrity> = None;

/// Initialize the global SigmaIntegrity instance
pub fn init_sigmaintegrity() {
    unsafe {
        SIGMAINTEGRITY_INSTANCE = Some(SigmaIntegrity::new());
    }
;

/// Get the global SigmaIntegrity instance
pub fn get_sigmaintegrity() -> Option<&'static mut SigmaIntegrity> {
    unsafe { SIGMAINTEGRITY_INSTANCE.as_mut() }
;

/// Convenience function to check file integrity
pub fn check_file_integrity(
    path: &str,
    current_hash: String,
    current_size: u64,
) -> Result<(), String> {
    if let Some(integrity) = get_sigmaintegrity() {
        integrity.check_integrity(path, current_hash, current_size)
    } else {
        Err("SigmaIntegrity not initialized".to_string())
    }
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_algorithm_display() {
        assert_eq!(HashAlgorithm::Sha256.to_string(), "sha256");
        assert_eq!(HashAlgorithm::Sha512.to_string(), "sha512");
    }

    #[test]
    fn test_file_status_display() {
        assert_eq!(FileStatus::Ok.to_string(), "ok");
        assert_eq!(FileStatus::Modified.to_string(), "modified");
    }

    #[test]
    fn test_file_integrity_record() {
        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        )
        .with_status(FileStatus::Ok)
        .with_timestamp(1234567890);

        assert_eq!(record.path, "/etc/passwd");
        assert_eq!(record.status, FileStatus::Ok);
        assert_eq!(record.timestamp, 1234567890);
    }

    #[test]
    fn test_sigmaintegrity() {
        let mut integrity = SigmaIntegrity::new();
        assert!(!integrity.is_running());
        integrity.start();
        assert!(integrity.is_running());
        integrity.stop();
        assert!(!integrity.is_running());
    }

    #[test]
    fn test_sigmaintegrity_add_record() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        );

        assert!(integrity.add_record(record).is_ok());
        assert_eq!(integrity.record_count(), 1);
    }

    #[test]
    fn test_sigmaintegrity_update_record() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        );

        integrity.add_record(record).unwrap();
        assert!(integrity
            .update_record("/etc/passwd", "def456".to_string(), 2048)
            .is_ok());

        let record = integrity.get_record("/etc/passwd").unwrap();
        assert_eq!(record.status, FileStatus::Modified);
    }

    #[test]
    fn test_sigmaintegrity_check_integrity() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        );

        integrity.add_record(record).unwrap();
        assert!(integrity
            .check_integrity("/etc/passwd", "abc123".to_string(), 1024)
            .is_ok());

        let record = integrity.get_record("/etc/passwd").unwrap();
        assert_eq!(record.status, FileStatus::Ok);
    }

    #[test]
    fn test_sigmaintegrity_mark_deleted() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        );

        integrity.add_record(record).unwrap();
        assert!(integrity.mark_deleted("/etc/passwd").is_ok());

        let record = integrity.get_record("/etc/passwd").unwrap();
        assert_eq!(record.status, FileStatus::Deleted);
    }

    #[test]
    fn test_sigmaintegrity_list_by_status() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        )
        .with_status(FileStatus::Modified);

        integrity.add_record(record).unwrap();

        let modified = integrity.list_by_status(FileStatus::Modified);
        assert_eq!(modified.len(), 1);
    }

    #[test]
    fn test_sigmaintegrity_stats() {
        let mut integrity = SigmaIntegrity::new();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        )
        .with_status(FileStatus::Ok);

        integrity.add_record(record).unwrap();

        let stats = integrity.integrity_stats();
        assert_eq!(stats.get("total_records"), Some(&"1".to_string()));
        assert_eq!(stats.get("ok_files"), Some(&"1".to_string()));
    }

    #[test]
    fn test_check_file_integrity_convenience() {
        init_sigmaintegrity();

        let mut integrity = get_sigmaintegrity().unwrap();

        let record = FileIntegrityRecord::new(
            "/etc/passwd".to_string(),
            HashAlgorithm::Sha256,
            "abc123".to_string(),
            1024,
        );

        integrity.add_record(record).unwrap();

        let result = check_file_integrity("/etc/passwd", "abc123".to_string(), 1024);
        assert!(result.is_ok());
    }
;
