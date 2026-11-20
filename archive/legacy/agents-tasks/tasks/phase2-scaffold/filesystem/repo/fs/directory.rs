// Directory Operations and Directory Inode Entry Management
// Part of SigmaOS File Management Subsystem

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

use super::fd::FsError;
use super::inode::InodeType;

pub const MAX_DIR_ENTRIES: usize = 4096;
pub const MAX_FILENAME_LEN: usize = 255;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    pub name: String,
    pub inode_id: u64,
    pub inode_type: InodeType,
}

impl DirectoryEntry {
    pub fn new(name: impl Into<String>, inode_id: u64, inode_type: InodeType) -> Self {
        Self {
            name: name.into(),
            inode_id,
            inode_type,
        }
    }
}

#[derive(Debug)]
pub struct Directory {
    pub dir_inode_id: u64,
    pub entries: Vec<DirectoryEntry>,
    pub max_capacity: usize,
}

impl Directory {
    pub fn new(dir_inode_id: u64, parent_inode_id: u64) -> Self {
        let mut entries = Vec::new();
        entries.push(DirectoryEntry::new(".", dir_inode_id, InodeType::Directory));
        entries.push(DirectoryEntry::new("..", parent_inode_id, InodeType::Directory));
        Self {
            dir_inode_id,
            entries,
            max_capacity: MAX_DIR_ENTRIES,
        }
    }

    pub fn add_entry(
        &mut self,
        name: &str,
        inode_id: u64,
        inode_type: InodeType,
    ) -> Result<(), FsError> {
        if name.is_empty() || name.len() > MAX_FILENAME_LEN || name.contains('/') {
            return Err(FsError::InvalidOperation);
        }
        if self.entries.len() >= self.max_capacity {
            return Err(FsError::StorageFull);
        }
        if self.lookup(name).is_ok() {
            return Err(FsError::FileAlreadyExists);
        }
        self.entries.push(DirectoryEntry::new(name, inode_id, inode_type));
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Result<DirectoryEntry, FsError> {
        for entry in &self.entries {
            if entry.name == name {
                return Ok(entry.clone());
            }
        }
        Err(FsError::NotFound)
    }

    pub fn remove_entry(&mut self, name: &str) -> Result<DirectoryEntry, FsError> {
        if name == "." || name == ".." {
            return Err(FsError::InvalidOperation);
        }
        let pos = self
            .entries
            .iter()
            .position(|e| e.name == name)
            .ok_or(FsError::NotFound)?;
        Ok(self.entries.remove(pos))
    }

    pub fn is_empty(&self) -> bool {
        self.entries.iter().all(|e| e.name == "." || e.name == "..")
    }

    pub fn list_entries(&self) -> Vec<DirectoryEntry> {
        self.entries.clone()
    }
}
