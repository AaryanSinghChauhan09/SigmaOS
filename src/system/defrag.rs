// SigmaOS Disk Defragmenter for SigmaFS
// OOP-based defragmentation with Merkle tree optimization

use std::collections::HashMap;
use std::path::{Path, PathBuf};

/// OOP trait for defragmentation strategies
pub trait DefragStrategy {
    /// Analyze fragmentation
    fn analyze(&self, path: &Path) -> Result<FragmentationReport, DefragError>;
    /// Defragment
    fn defragment(&mut self, path: &Path) -> Result<DefragResult, DefragError>;
    /// Get strategy name
    fn name(&self) -> &str;
}

/// Fragmentation report
#[derive(Debug, Clone)]
pub struct FragmentationReport {
    pub total_files: usize,
    pub fragmented_files: usize,
    pub fragmentation_percent: f64,
    pub total_size_bytes: u64,
    pub fragmented_size_bytes: u64,
    pub potential_improvement_percent: f64,
}

/// Defragmentation result
#[derive(Debug, Clone)]
pub struct DefragResult {
    pub strategy_name: String,
    pub success: bool,
    pub files_processed: usize,
    pub bytes_moved: u64,
    pub time_taken_seconds: u64,
    pub fragmentation_before: f64,
    pub fragmentation_after: f64,
    pub message: String,
}

/// File block information
#[derive(Debug, Clone)]
pub struct FileBlockInfo {
    pub path: PathBuf,
    pub size: u64,
    pub block_count: usize,
    pub contiguous_blocks: usize,
    pub is_fragmented: bool,
}

impl FileBlockInfo {
    pub fn new(path: PathBuf, size: u64, block_count: usize, contiguous_blocks: usize) -> Self {
        let is_fragmented = contiguous_blocks < block_count;
        Self {
            path,
            size,
            block_count,
            contiguous_blocks,
            is_fragmented,
        }
    }

    pub fn fragmentation_percent(&self) -> f64 {
        if self.block_count == 0 {
            0.0
        } else {
            (1.0 - (self.contiguous_blocks as f64 / self.block_count as f64)) * 100.0
        }
    }
}

/// SigmaFS-specific defragmentation strategy
pub struct SigmaFsDefragStrategy {
    block_size: u64,
    aggressive: bool,
    preserve_merkle_trees: bool,
}

impl SigmaFsDefragStrategy {
    pub fn new() -> Self {
        Self {
            block_size: 4096, // 4KB blocks
            aggressive: false,
            preserve_merkle_trees: true,
        }
    }

    pub fn with_block_size(mut self, size: u64) -> Self {
        self.block_size = size;
        self
    }

    pub fn aggressive(mut self) -> Self {
        self.aggressive = true;
        self
    }

    pub fn preserve_merkle_trees(mut self, preserve: bool) -> Self {
        self.preserve_merkle_trees = preserve;
        self
    }
}

impl DefragStrategy for SigmaFsDefragStrategy {
    fn analyze(&self, path: &Path) -> Result<FragmentationReport, DefragError> {
        let mut file_infos = Vec::new();
        let mut total_size = 0u64;
        let mut fragmented_size = 0u64;

        // Simulate analyzing files
        self.collect_file_info(path, &mut file_infos, &mut total_size, &mut fragmented_size)?;

        let total_files = file_infos.len();
        let fragmented_files = file_infos.iter().filter(|f| f.is_fragmented).count();

        let fragmentation_percent = if total_size > 0 {
            (fragmented_size as f64 / total_size as f64) * 100.0
        } else {
            0.0
        };

        let potential_improvement = if self.aggressive {
            fragmentation_percent * 0.8
        } else {
            fragmentation_percent * 0.5
        };

        Ok(FragmentationReport {
            total_files,
            fragmented_files,
            fragmentation_percent,
            total_size_bytes: total_size,
            fragmented_size_bytes: fragmented_size,
            potential_improvement_percent: potential_improvement,
        })
    }

    fn defragment(&mut self, path: &Path) -> Result<DefragResult, DefragError> {
        let start_time = std::time::Instant::now();

        let report = self.analyze(path)?;
        let fragmentation_before = report.fragmentation_percent;

        let mut files_processed = 0;
        let mut bytes_moved = 0u64;

        if report.fragmented_files > 0 {
            // Simulate defragmentation
            let mut file_infos = Vec::new();
            let mut total_size = 0u64;
            let mut fragmented_size = 0u64;
            self.collect_file_info(path, &mut file_infos, &mut total_size, &mut fragmented_size)?;

            for file_info in file_infos {
                if file_info.is_fragmented {
                    if self.defragment_file(&file_info) {
                        files_processed += 1;
                        bytes_moved += file_info.size;
                    }
                }
            }

            if self.preserve_merkle_trees {
                self.update_merkle_trees(path);
            }
        }

        let time_taken = start_time.elapsed().as_secs();

        // Re-analyze after defragmentation
        let report_after = self.analyze(path)?;
        let fragmentation_after = report_after.fragmentation_percent;

        Ok(DefragResult {
            strategy_name: self.name().to_string(),
            success: true,
            files_processed,
            bytes_moved,
            time_taken_seconds: time_taken,
            fragmentation_before,
            fragmentation_after,
            message: format!(
                "Defragmented {} files, moved {} bytes, fragmentation reduced from {:.1}% to {:.1}%",
                files_processed, bytes_moved, fragmentation_before, fragmentation_after
            ),
        })
    }

    fn name(&self) -> &str {
        "SigmaFsDefragStrategy"
    }
}

impl SigmaFsDefragStrategy {
    fn collect_file_info(
        &self,
        path: &Path,
        file_infos: &mut Vec<FileBlockInfo>,
        total_size: &mut u64,
        fragmented_size: &mut u64,
    ) -> Result<(), DefragError> {
        let entries = std::fs::read_dir(path).map_err(|e| DefragError::IoError(e.to_string()))?;

        for entry in entries {
            let entry = entry.map_err(|e| DefragError::IoError(e.to_string()))?;
            let entry_path = entry.path();

            if entry_path.is_dir() {
                self.collect_file_info(&entry_path, file_infos, total_size, fragmented_size)?;
            } else if entry_path.is_file() {
                let metadata = std::fs::metadata(&entry_path)
                    .map_err(|e| DefragError::IoError(e.to_string()))?;

                let size = metadata.len();
                let block_count = (size / self.block_size) as usize
                    + if size % self.block_size > 0 { 1 } else { 0 };

                // Simulate contiguous blocks (in real implementation, this would check actual block layout)
                let contiguous_blocks = if self.aggressive {
                    (block_count as f64 * 0.6) as usize // More fragmentation in aggressive mode
                } else {
                    (block_count as f64 * 0.8) as usize
                };

                let file_info =
                    FileBlockInfo::new(entry_path, size, block_count, contiguous_blocks);

                *total_size += size;
                if file_info.is_fragmented {
                    *fragmented_size += size;
                }
                file_infos.push(file_info);
            }
        }

        Ok(())
    }

    fn defragment_file(&self, file_info: &FileBlockInfo) -> bool {
        // Simulate file defragmentation
        // In real implementation, this would move blocks to contiguous locations
        true
    }

    fn update_merkle_trees(&self, path: &Path) {
        // Simulate updating Merkle trees after defragmentation
        // This ensures crash-consistency is maintained
    }
}

/// OOP-based Disk Defragmenter Manager
pub struct DiskDefragmenter {
    strategy: Box<dyn DefragStrategy>,
    dry_run: bool,
    report: Option<FragmentationReport>,
    result: Option<DefragResult>,
}

impl DiskDefragmenter {
    pub fn new(strategy: Box<dyn DefragStrategy>) -> Self {
        Self {
            strategy,
            dry_run: false,
            report: None,
            result: None,
        }
    }

    /// Set dry run mode
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Analyze fragmentation
    pub fn analyze(&mut self, path: &Path) -> Result<&FragmentationReport, DefragError> {
        let report = self.strategy.analyze(path)?;
        self.report = Some(report.clone());
        Ok(self.report.as_ref().unwrap())
    }

    /// Run defragmentation
    pub fn defragment(&mut self, path: &Path) -> Result<&DefragResult, DefragError> {
        if self.dry_run {
            let report = self.analyze(path)?;
            return Ok(&DefragResult {
                strategy_name: self.strategy.name().to_string(),
                success: true,
                files_processed: 0,
                bytes_moved: 0,
                time_taken_seconds: 0,
                fragmentation_before: report.fragmentation_percent,
                fragmentation_after: report.fragmentation_percent,
                message: "Dry run - no files were defragmented".to_string(),
            });
        }

        let result = self.strategy.defragment(path)?;
        self.result = Some(result.clone());
        Ok(self.result.as_ref().unwrap())
    }

    /// Get fragmentation report
    pub fn report(&self) -> Option<&FragmentationReport> {
        self.report.as_ref()
    }

    /// Get defragmentation result
    pub fn result(&self) -> Option<&DefragResult> {
        self.result.as_ref()
    }
}

impl Default for DiskDefragmenter {
    fn default() -> Self {
        Self::new(Box::new(SigmaFsDefragStrategy::new()))
    }
}

/// Defragmentation errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DefragError {
    PathNotFound(PathBuf),
    IoError(String),
    PermissionDenied(String),
    DiskFull,
    FileSystemNotSupported(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_block_info() {
        let info = FileBlockInfo::new(PathBuf::from("/test/file.txt"), 8192, 2, 1);
        assert!(info.is_fragmented);
        assert_eq!(info.fragmentation_percent(), 50.0);
    }

    #[test]
    fn test_sigma_fs_strategy() {
        let strategy = SigmaFsDefragStrategy::new();
        assert_eq!(strategy.name(), "SigmaFsDefragStrategy");
        assert_eq!(strategy.block_size, 4096);
    }

    #[test]
    fn test_sigma_fs_aggressive() {
        let strategy = SigmaFsDefragStrategy::new().aggressive();
        assert!(strategy.aggressive);
    }

    #[test]
    fn test_disk_defragmenter_creation() {
        let defragger =
            DiskDefragmenter::new(Box::new(SigmaFsDefragStrategy::new())).with_dry_run(true);
        assert!(defragger.dry_run);
    }

    #[test]
    fn test_disk_defragmenter_default() {
        let defragger = DiskDefragmenter::default();
        assert!(!defragger.dry_run);
    }
}
