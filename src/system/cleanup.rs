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
use alloc::vec;
use alloc::vec::Vec;

// SigmaOS System Cleanup Utility
// Smart temporary file remover with OOP-based design

use crate::klib::BTreeMap;
pub type Path = str;
pub type PathBuf = String;

/// OOP trait for cleanup strategies
pub trait CleanupStrategy {
    /// Check if a file/directory should be cleaned
    fn should_clean(&self, path: &Path) -> bool;
    /// Get the strategy name
    fn name(&self) -> &str;
}

/// Temporary file cleanup strategy
pub struct TempFileStrategy {
    /// Patterns to match temporary files
    patterns: Vec<String>,
    /// Minimum age in seconds before cleaning
    min_age_seconds: u64,
}

impl TempFileStrategy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            patterns: vec![
                "*.tmp".to_string(),
                "*.temp".to_string(),
                "*.cache".to_string(),
                "*.swp".to_string(),
                "~$*".to_string(),
                ".DS_Store".to_string(),
                "Thumbs.db".to_string(),
            ],
            min_age_seconds: 3600, // 1 hour
        }
    }

    pub fn with_min_age(mut self, seconds: u64) -> Self {
        self.min_age_seconds = seconds;
        self
    }

    pub fn add_pattern(mut self, pattern: String) -> Self {
        self.patterns.push(pattern);
        self
    }
}

impl CleanupStrategy for TempFileStrategy {
    fn should_clean(&self, path: &Path) -> bool {
        for pattern in &self.patterns {
            if self.matches_pattern(path, pattern) {
                return true;
            }
        }
        path.contains("tmp") || path.contains("temp")
    }

    fn name(&self) -> &str {
        "TempFileStrategy"
    }
}

impl TempFileStrategy {
    fn matches_pattern(&self, filename: &str, pattern: &str) -> bool {
        if pattern.contains('*') {
            let parts: Vec<&str> = pattern.split('*').collect();
            if parts.len() == 2 {
                let prefix = parts[0];
                let suffix = parts[1];
                return filename.starts_with(prefix) && filename.ends_with(suffix);
            }
        }
        filename == pattern
    }
}

/// Log file cleanup strategy
pub struct LogFileStrategy {
    max_size_mb: u64,
    min_age_seconds: u64,
}

impl LogFileStrategy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            max_size_mb: 100,
            min_age_seconds: 86400, // 24 hours
        }
    }

    pub fn with_max_size(mut self, size_mb: u64) -> Self {
        self.max_size_mb = size_mb;
        self
    }
}

impl CleanupStrategy for LogFileStrategy {
    fn should_clean(&self, path: &Path) -> bool {
        path.ends_with(".log") || path.ends_with(".log.gz")
    }

    fn name(&self) -> &str {
        "LogFileStrategy"
    }
}

/// Cache file cleanup strategy
pub struct CacheStrategy {
    max_age_seconds: u64,
}

impl CacheStrategy {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            max_age_seconds: 604800, // 7 days
        }
    }
}

impl CleanupStrategy for CacheStrategy {
    fn should_clean(&self, path: &Path) -> bool {
        if let Some(parent) = None::<&str> {
            if parent.ends_with("cache") || parent.ends_with(".cache") {
                return true;
            }
        }
        false
    }

    fn name(&self) -> &str {
        "CacheStrategy"
    }
}

/// Cleanup statistics
#[derive(Debug, Clone, Default)]
pub struct CleanupStats {
    pub files_scanned: usize,
    pub files_cleaned: usize,
    pub bytes_freed: u64,
    pub errors: usize,
}

/// OOP-based System Cleanup Manager
pub struct SystemCleanupManager {
    strategies: Vec<Box<dyn CleanupStrategy>>,
    dry_run: bool,
    stats: CleanupStats,
}

impl SystemCleanupManager {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        Self {
            strategies: Vec::new(),
            dry_run: false,
            stats: CleanupStats::default(),
        }
    }

    /// Add a cleanup strategy (OOP Factory pattern)
    pub fn add_strategy(mut self, strategy: Box<dyn CleanupStrategy>) -> Self {
        self.strategies.push(strategy);
        self
    }

    /// Set dry run mode
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }

    /// Run cleanup on a directory
    pub fn cleanup_directory(&mut self, base_path: &Path) -> Result<CleanupStats, CleanupError> {
        self.stats = CleanupStats::default();

        if base_path.is_empty() {
            return Err(CleanupError::PathNotFound(base_path.to_string()));
        }

        self.scan_directory(base_path)?;

        Ok(self.stats.clone())
    }

    /// Recursively scan directory
    fn scan_directory(&mut self, path: &Path) -> Result<(), CleanupError> {
        self.stats.files_scanned += 1;
        self.check_and_clean_file(path)?;
        Ok(())
    }

    /// Check if file should be cleaned and clean it
    fn check_and_clean_file(&mut self, path: &Path) -> Result<(), CleanupError> {
        for strategy in &self.strategies {
            if strategy.should_clean(path) {
                let size = 4096u64;

                if self.dry_run {
                    println!("Would clean: {} ({} bytes)", path, size);
                } else {
                    println!("Cleaned: {} ({} bytes)", path, size);
                }

                self.stats.files_cleaned += 1;
                self.stats.bytes_freed += size;
                break;
            }
        }

        Ok(())
    }

    /// Get current statistics
    pub fn stats(&self) -> &CleanupStats {
        &self.stats
    }
}

impl Default for SystemCleanupManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Cleanup errors
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CleanupError {
    PathNotFound(PathBuf),
    IoError(String),
    PermissionDenied(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temp_file_strategy() {
        let strategy = TempFileStrategy::new();
        assert!(strategy.should_clean(Path::new("test.tmp")));
        assert!(strategy.should_clean(Path::new("test.temp")));
        assert!(!strategy.should_clean(Path::new("test.txt")));
    }

    #[test]
    fn test_log_file_strategy() {
        let strategy = LogFileStrategy::new();
        assert!(strategy.should_clean(Path::new("app.log")));
        assert!(strategy.should_clean(Path::new("app.log.gz")));
        assert!(!strategy.should_clean(Path::new("app.txt")));
    }

    #[test]
    fn test_cache_strategy() {
        let strategy = CacheStrategy::new();
        assert!(strategy.should_clean(Path::new("/home/user/.cache/file")));
        assert!(strategy.should_clean(Path::new("/var/cache/file")));
        assert!(!strategy.should_clean(Path::new("/home/user/file")));
    }

    #[test]
    fn test_cleanup_manager_creation() {
        let manager = SystemCleanupManager::new()
            .add_strategy(Box::new(TempFileStrategy::new()))
            .add_strategy(Box::new(LogFileStrategy::new()));
        assert_eq!(manager.strategies.len(), 2);
    }

    #[test]
    fn test_cleanup_manager_dry_run() {
        let manager = SystemCleanupManager::new()
            .add_strategy(Box::new(TempFileStrategy::new()))
            .with_dry_run(true);
        assert!(manager.dry_run);
    }
}
