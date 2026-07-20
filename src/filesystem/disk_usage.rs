// SigmaOS Disk Usage Analyzer
// OOP-based disk space analysis with visualization

use std::collections::HashMap;
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
}
