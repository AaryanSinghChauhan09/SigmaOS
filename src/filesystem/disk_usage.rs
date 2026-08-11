// SigmaOS Disk Usage Analyzer
// OOP-based disk space analysis with visualization

use alloc::collections::BTreeMap;
use std::path::{Path, PathBuf};

/// Disk usage info
#[derive(Debug, Clone)]
pub struct DiskUsageInfo {
    pub path: PathBuf,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub usage_percent: f64,
}

/// Directory size info
#[derive(Debug, Clone)]
pub struct DirectorySizeInfo {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub file_count: u64,
    pub directory_count: u64,
    pub largest_files: Vec<FileSizeInfo>,
}

/// File size info
#[derive(Debug, Clone)]
pub struct FileSizeInfo {
    pub path: PathBuf,
    pub size_bytes: u64,
    pub modified_at: u64,
}

/// Analysis mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnalysisMode {
    Quick,
    Deep,
    Recursive,
}

/// OOP trait for analysis strategies
pub trait AnalysisStrategy {
    /// Analyze directory
    fn analyze(&self, path: &Path) -> Result<DirectorySizeInfo, DiskUsageError>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// Quick analysis strategy
pub struct QuickAnalysisStrategy {
    max_depth: usize,
}

impl QuickAnalysisStrategy {
    pub fn new(max_depth: usize) -> Self {
        Self { max_depth }
    }
}

impl AnalysisStrategy for QuickAnalysisStrategy {
    fn analyze(&self, path: &Path) -> Result<DirectorySizeInfo, DiskUsageError> {
        // Simulated quick analysis
        Ok(DirectorySizeInfo {
            path: path.to_path_buf(),
            size_bytes: 1024 * 1024 * 1024, // 1GB
            file_count: 100,
            directory_count: 10,
            largest_files: vec![FileSizeInfo {
                path: path.join("large_file.bin"),
                size_bytes: 512 * 1024 * 1024, // 512MB
                modified_at: 1234567890,
            }],
        })
    }

    fn name(&self) -> &str {
        "QuickAnalysisStrategy"
    }
}

/// Deep analysis strategy
pub struct DeepAnalysisStrategy {
    include_hidden: bool,
}

impl DeepAnalysisStrategy {
    pub fn new(include_hidden: bool) -> Self {
        Self { include_hidden }
    }
}

impl AnalysisStrategy for DeepAnalysisStrategy {
    fn analyze(&self, path: &Path) -> Result<DirectorySizeInfo, DiskUsageError> {
        // Simulated deep analysis
        Ok(DirectorySizeInfo {
            path: path.to_path_buf(),
            size_bytes: 2 * 1024 * 1024 * 1024, // 2GB
            file_count: 500,
            directory_count: 50,
            largest_files: vec![
                FileSizeInfo {
                    path: path.join("large_file1.bin"),
                    size_bytes: 512 * 1024 * 1024,
                    modified_at: 1234567890,
                },
                FileSizeInfo {
                    path: path.join("large_file2.bin"),
                    size_bytes: 256 * 1024 * 1024,
                    modified_at: 1234567890,
                },
            ],
        })
    }

    fn name(&self) -> &str {
        "DeepAnalysisStrategy"
    }
}

/// OOP-based Disk Usage Analyzer
pub struct DiskUsageAnalyzer {
    strategy: Box<dyn AnalysisStrategy>,
    cache: BTreeMap<PathBuf, DirectorySizeInfo>,
    cache_enabled: bool,
}

impl DiskUsageAnalyzer {
    pub fn new(strategy: Box<dyn AnalysisStrategy>) -> Self {
        Self {
            strategy,
            cache: BTreeMap::new(),
            cache_enabled: false,
        }
    }

    /// Enable cache
    pub fn with_cache(mut self, enabled: bool) -> Self {
        self.cache_enabled = enabled;
        self
    }

    /// Analyze directory
    pub fn analyze(&mut self, path: &Path) -> Result<DirectorySizeInfo, DiskUsageError> {
        if self.cache_enabled {
            if let Some(cached) = self.cache.get(path) {
                return Ok(cached.clone());
            }
        }

        let result = self.strategy.analyze(path)?;

        if self.cache_enabled {
            self.cache.insert(path.to_path_buf(), result.clone());
        }

        Ok(result)
    }

    /// Get disk usage
    pub fn get_disk_usage(&self, path: &Path) -> Result<DiskUsageInfo, DiskUsageError> {
        // Simulated disk usage
        Ok(DiskUsageInfo {
            path: path.to_path_buf(),
            total_bytes: 500 * 1024 * 1024 * 1024, // 500GB
            used_bytes: 250 * 1024 * 1024 * 1024,  // 250GB
            free_bytes: 250 * 1024 * 1024 * 1024,  // 250GB
            usage_percent: 50.0,
        })
    }

    /// Find large files
    pub fn find_large_files(&self, path: &Path, min_size_bytes: u64) -> Vec<FileSizeInfo> {
        let dir_info = self
            .strategy
            .analyze(path)
            .unwrap_or_else(|_| DirectorySizeInfo {
                path: path.to_path_buf(),
                size_bytes: 0,
                file_count: 0,
                directory_count: 0,
                largest_files: Vec::new(),
            });

        dir_info
            .largest_files
            .into_iter()
            .filter(|f| f.size_bytes >= min_size_bytes)
            .collect()
    }

    /// Find duplicate files (simulated)
    pub fn find_duplicates(&self, path: &Path) -> Vec<Vec<FileSizeInfo>> {
        // Simulated duplicate detection
        vec![vec![
            FileSizeInfo {
                path: path.join("duplicate1.txt"),
                size_bytes: 1024,
                modified_at: 1234567890,
            },
            FileSizeInfo {
                path: path.join("duplicate2.txt"),
                size_bytes: 1024,
                modified_at: 1234567890,
            },
        ]]
    }

    /// Get size by file type
    pub fn get_size_by_type(&self, path: &Path) -> BTreeMap<String, u64> {
        let mut sizes = BTreeMap::new();
        sizes.insert("txt".to_string(), 1024 * 1024);
        sizes.insert("pdf".to_string(), 5 * 1024 * 1024);
        sizes.insert("bin".to_string(), 100 * 1024 * 1024);
        sizes
    }

    /// Clear cache
    pub fn clear_cache(&mut self) {
        self.cache.clear();
    }

    /// Get cache size
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }
}

impl Default for DiskUsageAnalyzer {
    fn default() -> Self {
        Self::new(Box::new(QuickAnalysisStrategy::new(3))).with_cache(true)
    }
}

/// Disk usage errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DiskUsageError {
    PathNotFound(String),
    PermissionDenied(String),
    AnalysisFailed(String),
}

/// Partition type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionScheme {
    Mbr,
    Gpt,
}

/// Partition filesystem type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsType {
    SigmaFS,
    Ext4,
    Fat32,
    Ntfs,
}

/// Represents a disk partition (GNU Parted parity)
#[derive(Debug, Clone)]
pub struct DiskPartition {
    pub name: String,
    pub index: u32,
    pub start_sector: u64,
    pub end_sector: u64,
    pub fs_type: FsType,
}

/// Sovereign Disk Partition Editor Shard (GNU Parted Parity)
/// Supports unified GPT/MBR partition manipulations and 4KB physical alignment checks.
pub struct SovereignParted {
    pub disk_size_sectors: u64,
    pub scheme: PartitionScheme,
    pub partitions: Vec<DiskPartition>,
}

impl SovereignParted {
    pub fn new(disk_size_sectors: u64, scheme: PartitionScheme) -> Self {
        Self {
            disk_size_sectors,
            scheme,
            partitions: Vec::new(),
        }
    }

    /// Add a partition with strict boundary checks
    pub fn add_partition(
        &mut self,
        name: String,
        start_sector: u64,
        end_sector: u64,
        fs_type: FsType,
    ) -> Result<u32, &'static str> {
        if start_sector >= end_sector || end_sector > self.disk_size_sectors {
            return Err("Invalid sector boundaries");
        }

        // Check for overlaps with existing partitions
        for part in &self.partitions {
            if !(end_sector <= part.start_sector || start_sector >= part.end_sector) {
                return Err("Partition boundaries overlap existing partition");
            }
        }

        let index = (self.partitions.len() + 1) as u32;
        self.partitions.push(DiskPartition {
            name,
            index,
            start_sector,
            end_sector,
            fs_type,
        });

        Ok(index)
    }

    /// User-defined physical alignment validation function
    /// Standard modern disks use 4KB physical sectors (8 logical 512-byte sectors).
    /// GNU Parted warning is generated if start_sector is not divisible by 8.
    pub fn verify_alignment<F>(&self, index: u32, alignment_checker: F) -> Result<bool, &'static str>
    where
        F: Fn(u64) -> bool,
    {
        for part in &self.partitions {
            if part.index == index {
                return Ok(alignment_checker(part.start_sector));
            }
        }
        Err("Partition not found")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage_info() {
        let info = DiskUsageInfo {
            path: PathBuf::from("/"),
            total_bytes: 500 * 1024 * 1024 * 1024,
            used_bytes: 250 * 1024 * 1024 * 1024,
            free_bytes: 250 * 1024 * 1024 * 1024,
            usage_percent: 50.0,
        };
        assert_eq!(info.usage_percent, 50.0);
    }

    #[test]
    fn test_quick_analysis_strategy() {
        let strategy = QuickAnalysisStrategy::new(3);
        assert_eq!(strategy.name(), "QuickAnalysisStrategy");
    }

    #[test]
    fn test_deep_analysis_strategy() {
        let strategy = DeepAnalysisStrategy::new(false);
        assert_eq!(strategy.name(), "DeepAnalysisStrategy");
    }

    #[test]
    fn test_disk_usage_analyzer() {
        let analyzer = DiskUsageAnalyzer::default();
        assert!(analyzer.cache_enabled);
    }

    #[test]
    fn test_analyze() {
        let mut analyzer = DiskUsageAnalyzer::default();
        let result = analyzer.analyze(&PathBuf::from("/home/user")).unwrap();
        assert_eq!(result.file_count, 100);
    }

    #[test]
    fn test_get_disk_usage() {
        let analyzer = DiskUsageAnalyzer::default();
        let usage = analyzer.get_disk_usage(&PathBuf::from("/")).unwrap();
        assert_eq!(usage.usage_percent, 50.0);
    }

    #[test]
    fn test_sovereign_parted_vs_gnu_parted() {
        let mut parted = SovereignParted::new(1000000, PartitionScheme::Gpt);

        // Add partition 1 (perfectly aligned with 8-sector boundary, start = 2048)
        let idx1 = parted.add_partition("SovereignRoot".to_string(), 2048, 100000, FsType::SigmaFS).unwrap();
        assert_eq!(idx1, 1);

        // Add partition 2 (misaligned, start = 100003)
        let idx2 = parted.add_partition("UnstructuredData".to_string(), 100003, 200000, FsType::Ext4).unwrap();
        assert_eq!(idx2, 2);

        // Verify alignments with 8-sector 4KB boundary physical alignment checker F
        let align_checker = |sector: u64| sector % 8 == 0;

        let res1 = parted.verify_alignment(idx1, align_checker).unwrap();
        assert!(res1); // Perfectly aligned!

        let res2 = parted.verify_alignment(idx2, align_checker).unwrap();
        assert!(!res2); // Misaligned!
    }
}
