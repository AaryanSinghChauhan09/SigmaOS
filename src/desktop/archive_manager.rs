// SPDX-License-Identifier: MIT
// SigmaOS - Archive Manager
// Inspired by File Roller and Ark
// Provides archive creation and extraction

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// Archive format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveFormat {
    Zip,
    Tar,
    TarGz,
    TarBz2,
    TarXz,
    Rar,
    SevenZ,
}

/// Compression level
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompressionLevel {
    None,
    Fast,
    Normal,
    Maximum,
}

/// Archive
#[derive(Debug, Clone)]
pub struct Archive {
    pub path: String,
    pub format: ArchiveFormat,
    pub size: u64,
    pub file_count: u32,
    pub compressed_size: u64,
}

/// Archive manager
pub struct ArchiveManager {
    pub archives: Vec<Archive>,
    pub compression_level: CompressionLevel,
}

impl ArchiveManager {
    pub fn new() -> Self {
        Self {
            archives: Vec::new(),
            compression_level: CompressionLevel::Normal,
        }
    }

    /// Create archive
    pub fn create_archive(
        &mut self,
        path: String,
        format: ArchiveFormat,
        files: Vec<String>,
    ) -> Result<(), &'static str> {
        let archive = Archive {
            path: path.clone(),
            format,
            size: 0,
            file_count: files.len() as u32,
            compressed_size: 0,
        };
        self.archives.push(archive);
        Ok(())
    }

    /// Extract archive
    pub fn extract_archive(
        &mut self,
        path: String,
        destination: String,
    ) -> Result<(), &'static str> {
        // Simplified - in real implementation would extract actual files
        Ok(())
    }

    /// List archive contents
    pub fn list_contents(&self, path: String) -> Result<Vec<String>, &'static str> {
        // Simplified - in real implementation would list actual contents
        Ok(Vec::new())
    }

    /// Set compression level
    pub fn set_compression_level(&mut self, level: CompressionLevel) {
        self.compression_level = level;
    }

    /// Get all archives
    pub fn get_all_archives(&self) -> &[Archive] {
        &self.archives
    }

    /// Remove archive
    pub fn remove_archive(&mut self, index: usize) -> Result<(), &'static str> {
        if index >= self.archives.len() {
            return Err("Archive index out of bounds");
        }
        self.archives.remove(index);
        Ok(())
    }

    /// Archive count
    pub fn archive_count(&self) -> usize {
        self.archives.len()
    }
}

/// Global archive manager
static mut ARCHIVE_MANAGER: Option<ArchiveManager> = None;

/// Initialize archive manager
pub fn init_archive_manager() {
    unsafe {
        ARCHIVE_MANAGER = Some(ArchiveManager::new());
    }
}

/// Get archive manager
pub fn get_archive_manager() -> &'static mut ArchiveManager {
    unsafe {
        ARCHIVE_MANAGER
            .as_mut()
            .expect("Archive manager not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_archive_manager_creation() {
        let am = ArchiveManager::new();
        assert_eq!(am.archive_count(), 0);
    }

    #[test]
    fn test_create_archive() {
        let mut am = ArchiveManager::new();
        am.create_archive("test.zip".to_string(), ArchiveFormat::Zip, Vec::new())
            .unwrap();
        assert_eq!(am.archive_count(), 1);
    }

    #[test]
    fn test_remove_archive() {
        let mut am = ArchiveManager::new();
        am.create_archive("test.zip".to_string(), ArchiveFormat::Zip, Vec::new())
            .unwrap();
        am.remove_archive(0).unwrap();
        assert_eq!(am.archive_count(), 0);
    }
}
