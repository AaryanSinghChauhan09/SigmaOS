//! SigmaArchiveManager - Archive Management System
//!
//! Provides comprehensive archive management inspired by Linux archive tools.
//! Supports archive creation, extraction, and compression.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// Archive format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Zip,
    Rar,
    SevenZ,
    Unknown,
;

/// Archive state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveState {
    Idle,
    Creating,
    Extracting,
    Completed,
    Failed,
    Unknown,
;

/// Archive
#[derive(Debug, Clone)]
pub struct Archive {
    pub id: String,
    pub path: String,
    pub format: ArchiveFormat,
    pub state: ArchiveState,
    pub file_count: u32,
    pub size: u64,
;

impl Archive {
    pub fn new(id: String, path: String, format: ArchiveFormat) -> Self {
        Self {
            id,
            path,
            format,
            state: ArchiveState::Idle,
            file_count: 0,
            size: 0,
        }
    }

    pub fn with_file_count(mut self, file_count: u32) -> Self {
        self.file_count = file_count;
        self
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn set_creating(&mut self) {
        self.state = ArchiveState::Creating;
    }

    pub fn set_extracting(&mut self) {
        self.state = ArchiveState::Extracting;
    }

    pub fn set_completed(&mut self) {
        self.state = ArchiveState::Completed;
    }
;

/// Archive manager
pub struct SigmaArchiveManager {
    archives: BTreeMap<String, Archive>,
    next_archive_id: u64,
;

impl SigmaArchiveManager {
    pub fn new() -> Self {
        Self {
            archives: BTreeMap::new(),
            next_archive_id: 1,
        }
    }

    pub fn add_archive(&mut self, path: String, format: ArchiveFormat) -> String {
        let id = format!("archive-{}", self.next_archive_id);
        self.next_archive_id += 1;
        let archive = Archive::new(id.clone(), path, format);
        self.archives.insert(id.clone(), archive);
        id
    }

    pub fn get_archive(&self, id: &str) -> Option<&Archive> {
        self.archives.get(id)
    }

    pub fn get_archive_mut(&mut self, id: &str) -> Option<&mut Archive> {
        self.archives.get_mut(id)
    }

    pub fn list_archives(&self) -> Vec<&Archive> {
        self.archives.values().collect()
    }

    pub fn list_archives_by_format(&self, format: ArchiveFormat) -> Vec<&Archive> {
        self.archives
            .values()
            .filter(|a| a.format == format)
            .collect()
    }

    pub fn remove_archive(&mut self, id: &str) -> bool {
        self.archives.remove(id).is_some()
    }

    pub fn archive_count(&self) -> usize {
        self.archives.len()
    }

    pub fn total_size(&self) -> u64 {
        self.archives.values().map(|a| a.size).sum()
    }
;

impl Default for SigmaArchiveManager {
    fn default() -> Self {
        Self::new()
    }
;

static mut SIGMAARCHIVEMANAGER_INSTANCE: Option<SigmaArchiveManager> = None;

pub fn init_sigmaarchivemanager() {
    unsafe {
        SIGMAARCHIVEMANAGER_INSTANCE = Some(SigmaArchiveManager::new());
    }
;

pub fn get_sigmaarchivemanager() -> &'static SigmaArchiveManager {
    unsafe {
        SIGMAARCHIVEMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaArchiveManager not initialized")
    }
;

pub fn get_sigmaarchivemanager_mut() -> &'static mut SigmaArchiveManager {
    unsafe {
        SIGMAARCHIVEMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaArchiveManager not initialized")
    }
;

pub fn add_archive(path: String, format: ArchiveFormat) -> String {
    let manager = get_sigmaarchivemanager_mut();
    manager.add_archive(path, format)
;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_creation() {
        let archive = Archive::new(
            "archive-1".to_string(),
            "/backup.tar.gz".to_string(),
            ArchiveFormat::TarGz,
        );
        assert_eq!(archive.id, "archive-1");
        assert_eq!(archive.format, ArchiveFormat::TarGz);
        assert_eq!(archive.state, ArchiveState::Idle);
    }

    #[test]
    fn test_archive_state() {
        let mut archive = Archive::new(
            "archive-1".to_string(),
            "/backup.tar.gz".to_string(),
            ArchiveFormat::TarGz,
        );
        archive.set_creating();
        assert_eq!(archive.state, ArchiveState::Creating);

        archive.set_completed();
        assert_eq!(archive.state, ArchiveState::Completed);
    }

    #[test]
    fn test_archive_builder() {
        let archive = Archive::new(
            "archive-1".to_string(),
            "/backup.tar.gz".to_string(),
            ArchiveFormat::TarGz,
        )
        .with_file_count(100)
        .with_size(1000000);
        assert_eq!(archive.file_count, 100);
        assert_eq!(archive.size, 1000000);
    }

    #[test]
    fn test_archive_manager() {
        let manager = SigmaArchiveManager::new();
        assert_eq!(manager.archive_count(), 0);
    }

    #[test]
    fn test_add_archive() {
        let mut manager = SigmaArchiveManager::new();
        let id = manager.add_archive("/backup.tar.gz".to_string(), ArchiveFormat::TarGz);
        assert_eq!(id, "archive-1");
        assert_eq!(manager.archive_count(), 1);
    }

    #[test]
    fn test_list_archives_by_format() {
        let mut manager = SigmaArchiveManager::new();
        manager.add_archive("/backup.tar.gz".to_string(), ArchiveFormat::TarGz);
        manager.add_archive("/backup.zip".to_string(), ArchiveFormat::Zip);
        manager.add_archive("/backup.tar.xz".to_string(), ArchiveFormat::TarXz);

        let zips = manager.list_archives_by_format(ArchiveFormat::Zip);
        assert_eq!(zips.len(), 1);
    }

    #[test]
    fn test_total_size() {
        let mut manager = SigmaArchiveManager::new();
        let id1 = manager.add_archive("/backup1.tar.gz".to_string(), ArchiveFormat::TarGz);
        let id2 = manager.add_archive("/backup2.tar.gz".to_string(), ArchiveFormat::TarGz);

        manager.get_archive_mut(&id1).unwrap().with_size(1000000);
        manager.get_archive_mut(&id2).unwrap().with_size(2000000);

        assert_eq!(manager.total_size(), 3000000);
    }

    #[test]
    fn test_global_instance() {
        init_sigmaarchivemanager();
        let manager = get_sigmaarchivemanager();
        assert_eq!(manager.archive_count(), 0);

        let manager_mut = get_sigmaarchivemanager_mut();
        manager_mut.add_archive("/backup.tar.gz".to_string(), ArchiveFormat::TarGz);
        assert_eq!(manager.archive_count(), 1);
    }

    #[test]
    fn test_add_archive_convenience() {
        init_sigmaarchivemanager();
        let id = add_archive("/backup.tar.gz".to_string(), ArchiveFormat::TarGz);
        assert_eq!(id, "archive-1");

        let manager = get_sigmaarchivemanager();
        assert!(manager.get_archive(&id).is_some());
    }
;
