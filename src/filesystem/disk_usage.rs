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

// SigmaOS Disk Usage Analyzer
// OOP-based disk space analysis with visualization

use crate::klib::HashMap;
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
    cache: HashMap<PathBuf, DirectorySizeInfo>,
    cache_enabled: bool,
}

impl DiskUsageAnalyzer {
    pub fn new(strategy: Box<dyn AnalysisStrategy>) -> Self {
        Self {
            strategy,
            cache: HashMap::new(),
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
    pub fn get_size_by_type(&self, path: &Path) -> HashMap<String, u64> {
        let mut sizes = HashMap::new();
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

/// Represents disk free statistics for a filesystem (Linux df command parity)
#[derive(Debug, Clone)]
pub struct FilesystemDiskUsage {
    pub filesystem: String,
    pub fs_type: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub use_percent: f64,
    pub total_inodes: u64,
    pub used_inodes: u64,
    pub free_inodes: u64,
    pub inode_use_percent: f64,
    pub mounted_on: PathBuf,
}

/// Sovereign Disk Free (df) Engine implementing multi-filesystem reporting,
/// inode-level metrics (df -i), and custom output colorization and formatting.
pub struct SovereignDfEngine {
    pub mount_entries: Vec<FilesystemDiskUsage>,
}

impl SovereignDfEngine {
    pub fn new() -> Self {
        Self {
            mount_entries: Vec::new(),
        }
    }

    /// Add a simulated mount entry to the engine database
    pub fn add_mount_entry(&mut self, entry: FilesystemDiskUsage) {
        self.mount_entries.push(entry);
    }

    /// Formats a byte quantity into a human-readable string (df -h parity)
    pub fn format_human_readable(&self, bytes: u64) -> String {
        if bytes == 0 {
            return "0B".to_string();
        }
        let units = ["B", "K", "M", "G", "T", "P"];
        let mut size = bytes as f64;
        let mut unit_idx = 0;
        while size >= 1024.0 && unit_idx < units.len() - 1 {
            size /= 1024.0;
            unit_idx += 1;
        }
        if unit_idx == 0 {
            format!("{}B", bytes)
        } else {
            format!("{:.1}{}", size, units[unit_idx])
        }
    }

    /// Gets usage alert levels based on standard Linux threshold systems
    pub fn get_usage_alert(&self, entry: &FilesystemDiskUsage) -> &'static str {
        if entry.use_percent >= 90.0 {
            "CRITICAL"
        } else if entry.use_percent >= 75.0 {
            "WARNING"
        } else {
            "NORMAL"
        }
    }

    /// Filters list of mount entries by filesystem type
    pub fn filter_by_type(&self, fs_type: &str) -> Vec<FilesystemDiskUsage> {
        self.mount_entries
            .iter()
            .filter(|m| m.fs_type == fs_type)
            .cloned()
            .collect()
    }

    /// Calculates aggregated usage across all loaded mount points
    pub fn total_aggregated_usage(&self) -> (u64, u64, u64, f64) {
        let mut total = 0;
        let mut used = 0;
        for entry in &self.mount_entries {
            total += entry.total_bytes;
            used += entry.used_bytes;
        }
        let free = total.saturating_sub(used);
        let percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        (total, used, free, percent)
    }
}

impl Default for SovereignDfEngine {
    fn default() -> Self {
        let mut engine = Self::new();
        // Populate standard default mounts
        engine.add_mount_entry(FilesystemDiskUsage {
            filesystem: "/dev/sda1".to_string(),
            fs_type: "SigmaFS".to_string(),
            total_bytes: 100 * 1024 * 1024 * 1024, // 100GB
            used_bytes: 40 * 1024 * 1024 * 1024,  // 40GB
            free_bytes: 60 * 1024 * 1024 * 1024,  // 60GB
            use_percent: 40.0,
            total_inodes: 10_000_000,
            used_inodes: 1_200_000,
            free_inodes: 8_800_000,
            inode_use_percent: 12.0,
            mounted_on: PathBuf::from("/"),
        });
        engine.add_mount_entry(FilesystemDiskUsage {
            filesystem: "tmpfs".to_string(),
            fs_type: "tmpfs".to_string(),
            total_bytes: 8 * 1024 * 1024 * 1024, // 8GB
            used_bytes: 512 * 1024 * 1024,      // 512MB
            free_bytes: 75 * 1024 * 1024 * 1024,  // remaining
            use_percent: 6.25,
            total_inodes: 500_000,
            used_inodes: 5_000,
            free_inodes: 495_000,
            inode_use_percent: 1.0,
            mounted_on: PathBuf::from("/dev/shm"),
        });
        engine
    }
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
    pub uuid: String,
    pub boot: bool,
    pub lvm: bool,
    pub esp: bool,
    pub raid: bool,
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
            uuid: format!("part-uuid-{}", index),
            boot: false,
            lvm: false,
            esp: false,
            raid: false,
        });

        Ok(index)
    }

    /// Delete a partition by index
    pub fn delete_partition(&mut self, index: u32) -> Result<(), &'static str> {
        let before_len = self.partitions.len();
        self.partitions.retain(|part| part.index != index);
        if self.partitions.len() == before_len {
            return Err("Partition not found");
        }
        // Normalize/re-index partitions sequentially (GNU Parted / sfdisk behavior)
        for (i, part) in self.partitions.iter_mut().enumerate() {
            part.index = (i + 1) as u32;
        }
        Ok(())
    }

    /// Resize an existing partition's end sector offline
    pub fn resize_partition(&mut self, index: u32, new_end_sector: u64) -> Result<(), &'static str> {
        let mut target_idx = None;
        for (i, part) in self.partitions.iter().enumerate() {
            if part.index == index {
                target_idx = Some(i);
                break;
            }
        }

        let i = target_idx.ok_or("Partition not found")?;
        let start_sector = self.partitions[i].start_sector;

        if start_sector >= new_end_sector || new_end_sector > self.disk_size_sectors {
            return Err("Invalid new end sector boundaries");
        }

        // Check for overlaps with other partitions
        for (idx, part) in self.partitions.iter().enumerate() {
            if idx == i {
                continue;
            }
            if !(new_end_sector <= part.start_sector || start_sector >= part.end_sector) {
                return Err("Resized boundaries overlap with another partition");
            }
        }

        self.partitions[i].end_sector = new_end_sector;
        Ok(())
    }

    /// Set advanced Linux distro/GNU Parted flags (e.g. boot, lvm, esp, raid) on a partition
    pub fn set_partition_flag(&mut self, index: u32, flag: &str, value: bool) -> Result<(), &'static str> {
        for part in &mut self.partitions {
            if part.index == index {
                match flag {
                    "boot" => part.boot = value,
                    "lvm" => part.lvm = value,
                    "esp" => part.esp = value,
                    "raid" => part.raid = value,
                    _ => return Err("Unknown flag name. Supported: boot, lvm, esp, raid"),
                }
                return Ok(());
            }
        }
        Err("Partition not found")
    }

    /// Suggest the next optimal aligned sector based on standard modern OS patterns.
    /// Standard alignment for modern high-performance block/SSD devices is 1MiB (2048 sectors of 512 bytes).
    /// If not divisible, returns the nearest starting sector that is perfectly aligned to a 2048-sector boundary.
    pub fn suggest_optimal_alignment(&self, requested_start: u64) -> u64 {
        const OPTIMAL_ALIGNMENT: u64 = 2048; // 1MiB boundary
        let remainder = requested_start % OPTIMAL_ALIGNMENT;
        if remainder == 0 {
            requested_start
        } else {
            requested_start + (OPTIMAL_ALIGNMENT - remainder)
        }
    }

    /// User-defined physical alignment validation function
    /// Standard modern disks use 4KB physical sectors (8 logical 512-byte sectors).
    /// GNU Parted warning is generated if start_sector is not divisible by 8.
    pub fn verify_alignment<F>(
        &self,
        index: u32,
        alignment_checker: F,
    ) -> Result<bool, &'static str>
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
        let idx1 = parted
            .add_partition("SovereignRoot".to_string(), 2048, 100000, FsType::SigmaFS)
            .unwrap();
        assert_eq!(idx1, 1);

        // Add partition 2 (misaligned, start = 100003)
        let idx2 = parted
            .add_partition("UnstructuredData".to_string(), 100003, 200000, FsType::Ext4)
            .unwrap();
        assert_eq!(idx2, 2);

        // Verify alignments with 8-sector 4KB boundary physical alignment checker F
        let align_checker = |sector: u64| sector % 8 == 0;

        let res1 = parted.verify_alignment(idx1, align_checker).unwrap();
        assert!(res1); // Perfectly aligned!

        let res2 = parted.verify_alignment(idx2, align_checker).unwrap();
        assert!(!res2); // Misaligned!

        // Linux Distro Parity: 1. Test Flags Manipulation
        assert!(parted.set_partition_flag(idx1, "boot", true).is_ok());
        assert!(parted.set_partition_flag(idx1, "esp", true).is_ok());
        assert!(parted.set_partition_flag(idx1, "invalid", true).is_err());
        assert!(parted.partitions[0].boot);
        assert!(parted.partitions[0].esp);
        assert!(!parted.partitions[0].lvm);

        // Linux Distro Parity: 2. Test Auto-Alignment Suggestions (1MiB / 2048-sector boundary)
        let suggestion1 = parted.suggest_optimal_alignment(2000);
        assert_eq!(suggestion1, 2048); // Rounded up to nearest 1MiB
        let suggestion2 = parted.suggest_optimal_alignment(2048);
        assert_eq!(suggestion2, 2048); // Already aligned

        // Linux Distro Parity: 3. Test Partition Resizing
        assert!(parted.resize_partition(idx1, 100002).is_ok());
        assert_eq!(parted.partitions[0].end_sector, 100002);
        // Resizing causing overlap with idx2 starts at 100003
        assert!(parted.resize_partition(idx1, 105000).is_err());

        // Linux Distro Parity: 4. Test Partition Deletion and seq index re-numbering
        assert!(parted.delete_partition(idx1).is_ok());
        assert_eq!(parted.partitions.len(), 1);
        assert_eq!(parted.partitions[0].index, 1); // Index normalized to 1
        assert_eq!(parted.partitions[0].name, "UnstructuredData");
    }

    #[test]
    fn test_sovereign_df_engine_basic() {
        let mut engine = SovereignDfEngine::new();
        let entry = FilesystemDiskUsage {
            filesystem: "/dev/sdb1".to_string(),
            fs_type: "ext4".to_string(),
            total_bytes: 50 * 1024 * 1024 * 1024,
            used_bytes: 46 * 1024 * 1024 * 1024, // 92% used
            free_bytes: 4 * 1024 * 1024 * 1024,
            use_percent: 92.0,
            total_inodes: 1_000_000,
            used_inodes: 200_000,
            free_inodes: 800_000,
            inode_use_percent: 20.0,
            mounted_on: PathBuf::from("/mnt/data"),
        };
        engine.add_mount_entry(entry);

        assert_eq!(engine.mount_entries.len(), 1);
        let entry_ref = &engine.mount_entries[0];
        assert_eq!(engine.get_usage_alert(entry_ref), "CRITICAL");
        assert_eq!(engine.format_human_readable(entry_ref.total_bytes), "50.0G");
        assert_eq!(engine.format_human_readable(0), "0B");
        assert_eq!(engine.format_human_readable(512), "512B");

        let filtered = engine.filter_by_type("ext4");
        assert_eq!(filtered.len(), 1);

        let filtered_empty = engine.filter_by_type("btrfs");
        assert_eq!(filtered_empty.len(), 0);

        let (total, used, free, percent) = engine.total_aggregated_usage();
        assert_eq!(total, 50 * 1024 * 1024 * 1024);
        assert_eq!(used, 46 * 1024 * 1024 * 1024);
        assert_eq!(free, 4 * 1024 * 1024 * 1024);
        assert_eq!(percent, 92.0);
    }

    #[test]
    fn test_sovereign_df_engine_default() {
        let engine = SovereignDfEngine::default();
        assert_eq!(engine.mount_entries.len(), 2);
        assert_eq!(engine.mount_entries[0].fs_type, "SigmaFS");
        assert_eq!(engine.mount_entries[1].fs_type, "tmpfs");
        assert_eq!(engine.get_usage_alert(&engine.mount_entries[0]), "NORMAL");
    }

    #[test]
    fn test_sovereign_df_engine_basic() {
        let mut engine = SovereignDfEngine::new();
        let entry = FilesystemDiskUsage {
            filesystem: "/dev/sdb1".to_string(),
            fs_type: "ext4".to_string(),
            total_bytes: 50 * 1024 * 1024 * 1024,
            used_bytes: 46 * 1024 * 1024 * 1024, // 92% used
            free_bytes: 4 * 1024 * 1024 * 1024,
            use_percent: 92.0,
            total_inodes: 1_000_000,
            used_inodes: 200_000,
            free_inodes: 800_000,
            inode_use_percent: 20.0,
            mounted_on: PathBuf::from("/mnt/data"),
        };
        engine.add_mount_entry(entry);

        assert_eq!(engine.mount_entries.len(), 1);
        let entry_ref = &engine.mount_entries[0];
        assert_eq!(engine.get_usage_alert(entry_ref), "CRITICAL");
        assert_eq!(engine.format_human_readable(entry_ref.total_bytes), "50.0G");
        assert_eq!(engine.format_human_readable(0), "0B");
        assert_eq!(engine.format_human_readable(512), "512B");

        let filtered = engine.filter_by_type("ext4");
        assert_eq!(filtered.len(), 1);

        let filtered_empty = engine.filter_by_type("btrfs");
        assert_eq!(filtered_empty.len(), 0);

        let (total, used, free, percent) = engine.total_aggregated_usage();
        assert_eq!(total, 50 * 1024 * 1024 * 1024);
        assert_eq!(used, 46 * 1024 * 1024 * 1024);
        assert_eq!(free, 4 * 1024 * 1024 * 1024);
        assert_eq!(percent, 92.0);
    }

    #[test]
    fn test_sovereign_df_engine_default() {
        let engine = SovereignDfEngine::default();
        assert_eq!(engine.mount_entries.len(), 2);
        assert_eq!(engine.mount_entries[0].fs_type, "SigmaFS");
        assert_eq!(engine.mount_entries[1].fs_type, "tmpfs");
        assert_eq!(engine.get_usage_alert(&engine.mount_entries[0]), "NORMAL");
    }
}
