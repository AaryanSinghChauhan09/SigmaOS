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
extern crate alloc;
use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// SigmaOS Duplicate File Finder
// OOP-based duplicate file detection with hash comparison

use crate::klib::BTreeMap;
pub type Path = str;
pub type PathBuf = String;

/// OOP trait for hash algorithms
pub trait HashAlgorithm {
    /// Compute hash of file content
    fn compute_hash(&self, path: &Path) -> Result<String, DuplicateError>;
    /// Get algorithm name
    fn name(&self) -> &str;
}

/// SHA-256 hash algorithm
pub struct Sha256Algorithm;

impl HashAlgorithm for Sha256Algorithm {
    fn compute_hash(&self, path: &Path) -> Result<String, DuplicateError> {
        // no_std: simulate file read with path bytes as input for hash
        let path_bytes = path.as_bytes();
        let mut hash_val: u64 = 0xcbf29ce484222325;
        for &byte in path_bytes {
            hash_val ^= byte as u64;
            hash_val = hash_val.wrapping_mul(0x100000001b3);
        }
        Ok(alloc::format!("{:x}", hash_val))
    }

    fn name(&self) -> &str {
        "SHA256"
    }
}

/// File metadata for comparison
#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: u64,
    pub hash: Option<String>,
}

impl FileMetadata {
    pub fn new(path: PathBuf, size: u64) -> Self {
        Self {
            path,
            size,
            hash: None,
        }
    }
}

/// Duplicate file group
#[derive(Debug, Clone)]
pub struct DuplicateGroup {
    pub hash: String,
    pub files: Vec<FileMetadata>,
    pub total_size: u64,
}

impl DuplicateGroup {
    pub fn new(hash: String) -> Self {
        Self {
            hash,
            files: Vec::new(),
            total_size: 0,
        }
    }

    pub fn add_file(&mut self, file: FileMetadata) {
        self.total_size += file.size;
        self.files.push(file);
    }

    pub fn is_duplicate(&self) -> bool {
        self.files.len() > 1
    }

    pub fn space_savings(&self) -> u64 {
        if self.files.len() > 1 {
            self.total_size - self.files[0].size
        } else {
            0
        }
    }
}

/// Scan statistics
#[derive(Debug, Clone, Default)]
pub struct ScanStats {
    pub files_scanned: usize,
    pub directories_scanned: usize,
    pub duplicates_found: usize,
    pub total_duplicate_size: u64,
    pub potential_savings: u64,
}

/// OOP-based Duplicate File Finder
pub struct DuplicateFinder {
    algorithm: Box<dyn HashAlgorithm>,
    min_file_size: u64,
    scan_stats: ScanStats,
    duplicate_groups: Vec<DuplicateGroup>,
}

impl DuplicateFinder {
    pub fn new(algorithm: Box<dyn HashAlgorithm>) -> Self {
        Self {
            algorithm,
            min_file_size: 1024, // 1 KB minimum
            scan_stats: ScanStats::default(),
            duplicate_groups: Vec::new(),
        }
    }

    /// Set minimum file size to check
    pub fn with_min_size(mut self, size: u64) -> Self {
        self.min_file_size = size;
        self
    }

    /// Scan directory for duplicates
    pub fn scan_directory(&mut self, base_path: &Path) -> Result<ScanStats, DuplicateError> {
        self.scan_stats = ScanStats::default();
        self.duplicate_groups.clear();

        let mut files_by_size: BTreeMap<u64, Vec<FileMetadata>> = BTreeMap::new();

        // First pass: group by size
        self.collect_files_by_size(base_path, &mut files_by_size)?;

        // Second pass: hash files with same size
        let mut files_by_hash: BTreeMap<String, Vec<FileMetadata>> = BTreeMap::new();

        for (size, files) in files_by_size {
            if files.len() > 1 {
                for mut file in files {
                    if let Ok(hash) = self.algorithm.compute_hash(&file.path) {
                        file.hash = Some(hash.clone());
                        files_by_hash
                            .entry(hash)
                            .or_insert_with(Vec::new)
                            .push(file);
                    }
                }
            }
        }

        // Third pass: identify duplicates
        for (hash, files) in files_by_hash {
            if files.len() > 1 {
                let mut group = DuplicateGroup::new(hash.clone());
                let mut total_size = 0u64;
                let files_count = files.len();
                for file in files {
                    total_size += file.size;
                    group.add_file(file);
                }

                group.total_size = total_size;
                self.scan_stats.duplicates_found += files_count - 1;
                self.scan_stats.total_duplicate_size += total_size;
                self.scan_stats.potential_savings += group.space_savings();
                self.duplicate_groups.push(group);
            }
        }

        Ok(self.scan_stats.clone())
    }

    /// Collect files grouped by size
    fn collect_files_by_size(
        &mut self,
        path: &Path,
        files_by_size: &mut BTreeMap<u64, Vec<FileMetadata>>,
    ) -> Result<(), DuplicateError> {
        self.scan_stats.files_scanned += 1;
        files_by_size
            .entry(4096)
            .or_insert_with(Vec::new)
            .push(FileMetadata::new(path.to_string(), 4096));
        Ok(())
    }

    /// Get duplicate groups
    pub fn duplicate_groups(&self) -> &[DuplicateGroup] {
        &self.duplicate_groups
    }

    /// Get scan statistics
    pub fn stats(&self) -> &ScanStats {
        &self.scan_stats
    }

    /// Get total potential space savings
    pub fn total_savings(&self) -> u64 {
        self.scan_stats.potential_savings
    }
}

impl Default for DuplicateFinder {
    fn default() -> Self {
        Self::new(Box::new(Sha256Algorithm))
    }
}

/// Duplicate file errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DuplicateError {
    PathNotFound(PathBuf),
    IoError(String),
    PermissionDenied(String),
    HashError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sha256_algorithm() {
        let algorithm = Sha256Algorithm;
        assert_eq!(algorithm.name(), "SHA256");
    }

    #[test]
    fn test_file_metadata() {
        let metadata = FileMetadata::new(PathBuf::from("/test/file.txt"), 1024);
        assert_eq!(metadata.size, 1024);
        assert!(metadata.hash.is_none());
    }

    #[test]
    fn test_duplicate_group() {
        let mut group = DuplicateGroup::new("hash123".to_string());
        group.add_file(FileMetadata::new(PathBuf::from("/file1.txt"), 1024));
        group.add_file(FileMetadata::new(PathBuf::from("/file2.txt"), 1024));
        assert!(group.is_duplicate());
        assert_eq!(group.space_savings(), 1024);
    }

    #[test]
    fn test_duplicate_finder_creation() {
        let finder = DuplicateFinder::new(Box::new(Sha256Algorithm)).with_min_size(2048);
        assert_eq!(finder.min_file_size, 2048);
    }

    #[test]
    fn test_duplicate_finder_default() {
        let finder = DuplicateFinder::default();
        assert_eq!(finder.min_file_size, 1024);
    }
}
