//! SigmaFileManager - File Management System
//!
//! Provides comprehensive file management operations inspired by Linux file utilities.
//! Supports file operations, directory management, and file permissions.

#![no_std]

extern crate alloc;

use alloc::{
    collections::BTreeMap,
    string::{String, ToString},
    vec::Vec,
};

/// File type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Regular,
    Directory,
    Symlink,
    CharacterDevice,
    BlockDevice,
    Socket,
    FIFO,
    Unknown,
}

/// File permissions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilePermissions {
    pub owner_read: bool,
    pub owner_write: bool,
    pub owner_execute: bool,
    pub group_read: bool,
    pub group_write: bool,
    pub group_execute: bool,
    pub other_read: bool,
    pub other_write: bool,
    pub other_execute: bool,
}

impl FilePermissions {
    pub fn new() -> Self {
        Self {
            owner_read: true,
            owner_write: true,
            owner_execute: false,
            group_read: true,
            group_write: false,
            group_execute: false,
            other_read: true,
            other_write: false,
            other_execute: false,
        }
    }

    pub fn mode(&self) -> u32 {
        let mut mode = 0u32;
        if self.owner_read {
            mode |= 0o400;
        }
        if self.owner_write {
            mode |= 0o200;
        }
        if self.owner_execute {
            mode |= 0o100;
        }
        if self.group_read {
            mode |= 0o040;
        }
        if self.group_write {
            mode |= 0o020;
        }
        if self.group_execute {
            mode |= 0o010;
        }
        if self.other_read {
            mode |= 0o004;
        }
        if self.other_write {
            mode |= 0o002;
        }
        if self.other_execute {
            mode |= 0o001;
        }
        mode
    }
}

impl Default for FilePermissions {
    fn default() -> Self {
        Self::new()
    }
}

/// File entry
#[derive(Debug, Clone)]
pub struct FileEntry {
    pub path: String,
    pub file_type: FileType,
    pub size: u64,
    pub permissions: FilePermissions,
    pub owner: String,
    pub group: String,
}

impl FileEntry {
    pub fn new(path: String, file_type: FileType) -> Self {
        Self {
            path,
            file_type,
            size: 0,
            permissions: FilePermissions::new(),
            owner: String::new(),
            group: String::new(),
        }
    }

    pub fn with_size(mut self, size: u64) -> Self {
        self.size = size;
        self
    }

    pub fn with_permissions(mut self, permissions: FilePermissions) -> Self {
        self.permissions = permissions;
        self
    }

    pub fn with_owner(mut self, owner: String) -> Self {
        self.owner = owner;
        self
    }

    pub fn with_group(mut self, group: String) -> Self {
        self.group = group;
        self
    }
}

/// File manager
pub struct SigmaFileManager {
    files: BTreeMap<String, FileEntry>,
}

impl SigmaFileManager {
    pub fn new() -> Self {
        Self {
            files: BTreeMap::new(),
        }
    }

    pub fn add_file(&mut self, entry: FileEntry) {
        self.files.insert(entry.path.clone(), entry);
    }

    pub fn get_file(&self, path: &str) -> Option<&FileEntry> {
        self.files.get(path)
    }

    pub fn get_file_mut(&mut self, path: &str) -> Option<&mut FileEntry> {
        self.files.get_mut(path)
    }

    pub fn list_files(&self) -> Vec<&FileEntry> {
        self.files.values().collect()
    }

    pub fn list_files_by_type(&self, file_type: FileType) -> Vec<&FileEntry> {
        self.files
            .values()
            .filter(|f| f.file_type == file_type)
            .collect()
    }

    pub fn remove_file(&mut self, path: &str) -> bool {
        self.files.remove(path).is_some()
    }

    pub fn file_count(&self) -> usize {
        self.files.len()
    }

    pub fn total_size(&self) -> u64 {
        self.files.values().map(|f| f.size).sum()
    }
}

impl Default for SigmaFileManager {
    fn default() -> Self {
        Self::new()
    }
}

static mut SIGMAFILEMANAGER_INSTANCE: Option<SigmaFileManager> = None;

pub fn init_sigmafilemanager() {
    unsafe {
        SIGMAFILEMANAGER_INSTANCE = Some(SigmaFileManager::new());
    }
}

pub fn get_sigmafilemanager() -> &'static SigmaFileManager {
    unsafe {
        SIGMAFILEMANAGER_INSTANCE
            .as_ref()
            .expect("SigmaFileManager not initialized")
    }
}

pub fn get_sigmafilemanager_mut() -> &'static mut SigmaFileManager {
    unsafe {
        SIGMAFILEMANAGER_INSTANCE
            .as_mut()
            .expect("SigmaFileManager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_permissions() {
        let perms = FilePermissions::new();
        assert!(perms.owner_read);
        assert!(perms.owner_write);
        assert!(!perms.owner_execute);
        assert_eq!(perms.mode(), 0o644);
    }

    #[test]
    fn test_file_entry_creation() {
        let entry = FileEntry::new("/home/user/file.txt".to_string(), FileType::Regular);
        assert_eq!(entry.path, "/home/user/file.txt");
        assert_eq!(entry.file_type, FileType::Regular);
    }

    #[test]
    fn test_file_entry_builder() {
        let entry = FileEntry::new("/home/user/file.txt".to_string(), FileType::Regular)
            .with_size(1024)
            .with_owner("user".to_string())
            .with_group("users".to_string());
        assert_eq!(entry.size, 1024);
        assert_eq!(entry.owner, "user");
        assert_eq!(entry.group, "users");
    }

    #[test]
    fn test_file_manager() {
        let manager = SigmaFileManager::new();
        assert_eq!(manager.file_count(), 0);
    }

    #[test]
    fn test_add_file() {
        let mut manager = SigmaFileManager::new();
        let entry = FileEntry::new("/home/user/file.txt".to_string(), FileType::Regular);
        manager.add_file(entry);
        assert_eq!(manager.file_count(), 1);
    }

    #[test]
    fn test_list_files_by_type() {
        let mut manager = SigmaFileManager::new();
        manager.add_file(FileEntry::new(
            "/home/user/file.txt".to_string(),
            FileType::Regular,
        ));
        manager.add_file(FileEntry::new(
            "/home/user".to_string(),
            FileType::Directory,
        ));
        manager.add_file(FileEntry::new(
            "/home/user/link".to_string(),
            FileType::Symlink,
        ));

        let dirs = manager.list_files_by_type(FileType::Directory);
        assert_eq!(dirs.len(), 1);
    }

    #[test]
    fn test_total_size() {
        let mut manager = SigmaFileManager::new();
        manager.add_file(
            FileEntry::new("/home/user/file1.txt".to_string(), FileType::Regular).with_size(1024),
        );
        manager.add_file(
            FileEntry::new("/home/user/file2.txt".to_string(), FileType::Regular).with_size(2048),
        );

        assert_eq!(manager.total_size(), 3072);
    }

    #[test]
    fn test_global_instance() {
        init_sigmafilemanager();
        let manager = get_sigmafilemanager();
        assert_eq!(manager.file_count(), 0);

        let manager_mut = get_sigmafilemanager_mut();
        manager_mut.add_file(FileEntry::new(
            "/home/user/file.txt".to_string(),
            FileType::Regular,
        ));
        assert_eq!(manager.file_count(), 1);
    }
}
