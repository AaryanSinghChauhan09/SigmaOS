// SPDX-License-Identifier: MIT
// SigmaOS - Disk Usage Analyzer
// Inspired by Baobab and QDirStat
// Provides disk usage analysis and visualization

#![no_std]

extern crate alloc;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

/// File system entry
#[derive(Debug, Clone)]
pub struct FileSystemEntry {
    pub path: String,
    pub name: String,
    pub size: u64,
    pub is_directory: bool,
    pub children: Vec<FileSystemEntry>,
    pub percentage: f32,
}

/// Scan result
#[derive(Debug, Clone)]
pub struct ScanResult {
    pub root: FileSystemEntry,
    pub total_size: u64,
    pub free_space: u64,
    pub mount_point: String,
}

/// Disk usage analyzer
pub struct DiskUsageAnalyzer {
    pub scan_results: Vec<ScanResult>,
    pub current_scan: Option<ScanResult>,
    pub show_hidden: bool,
    pub scan_depth: u32,
}

impl DiskUsageAnalyzer {
    pub fn new() -> Self {
        Self {
            scan_results: Vec::new(),
            current_scan: None,
            show_hidden: false,
            scan_depth: 10,
        }
    }

    /// Scan directory
    pub fn scan_directory(&mut self, path: String) -> Result<ScanResult, &'static str> {
        // Simplified - in real implementation would scan actual filesystem
        let entry = FileSystemEntry {
            path: path.clone(),
            name: path.split('/').last().unwrap_or("root").to_string(),
            size: 1024 * 1024 * 1024, // 1GB
            is_directory: true,
            children: Vec::new(),
            percentage: 100.0,
        };

        let result = ScanResult {
            root: entry,
            total_size: 1024 * 1024 * 1024,
            free_space: 512 * 1024 * 1024,
            mount_point: path,
        };

        self.current_scan = Some(result.clone());
        self.scan_results.push(result.clone());
        Ok(result)
    }

    /// Scan entire filesystem
    pub fn scan_filesystem(&mut self) -> Result<ScanResult, &'static str> {
        self.scan_directory("/".to_string())
    }

    /// Get current scan
    pub fn get_current_scan(&self) -> Option<&ScanResult> {
        self.current_scan.as_ref()
    }

    /// Get largest directories
    pub fn get_largest_directories(&self, limit: usize) -> Vec<&FileSystemEntry> {
        let mut entries = Vec::new();

        if let Some(ref scan) = self.current_scan {
            self.collect_directories(&scan.root, &mut entries);
        }

        entries.sort_by(|a, b| b.size.cmp(&a.size));
        entries.into_iter().take(limit).collect()
    }

    /// Collect directories recursively
    fn collect_directories<'a>(
        &'a self,
        entry: &'a FileSystemEntry,
        entries: &mut Vec<&'a FileSystemEntry>,
    ) {
        if entry.is_directory {
            entries.push(entry);
            for child in &entry.children {
                self.collect_directories(child, entries);
            }
        }
    }

    /// Find large files
    pub fn find_large_files(&self, threshold: u64) -> Vec<&FileSystemEntry> {
        let mut files = Vec::new();

        if let Some(ref scan) = self.current_scan {
            self.collect_files(&scan.root, &mut files);
        }

        files.retain(|f| f.size >= threshold);
        files.sort_by(|a, b| b.size.cmp(&a.size));
        files
    }

    /// Collect files recursively
    fn collect_files<'a>(
        &'a self,
        entry: &'a FileSystemEntry,
        files: &mut Vec<&'a FileSystemEntry>,
    ) {
        if entry.is_directory {
            for child in &entry.children {
                self.collect_files(child, files);
            }
        } else {
            files.push(entry);
        }
    }

    /// Format size
    pub fn format_size(size: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if size >= TB {
            format!("{:.2} TB", size as f64 / TB as f64)
        } else if size >= GB {
            format!("{:.2} GB", size as f64 / GB as f64)
        } else if size >= MB {
            format!("{:.2} MB", size as f64 / MB as f64)
        } else if size >= KB {
            format!("{:.2} KB", size as f64 / KB as f64)
        } else {
            format!("{} B", size)
        }
    }

    /// Set show hidden
    pub fn set_show_hidden(&mut self, show: bool) {
        self.show_hidden = show;
    }

    /// Set scan depth
    pub fn set_scan_depth(&mut self, depth: u32) {
        self.scan_depth = depth.max(1);
    }

    /// Clear scan results
    pub fn clear_scans(&mut self) {
        self.scan_results.clear();
        self.current_scan = None;
    }

    /// Scan count
    pub fn scan_count(&self) -> usize {
        self.scan_results.len()
    }
}

/// Global disk usage analyzer
static mut DISK_USAGE_ANALYZER: Option<DiskUsageAnalyzer> = None;

/// Initialize disk usage analyzer
pub fn init_disk_usage_analyzer() {
    unsafe {
        DISK_USAGE_ANALYZER = Some(DiskUsageAnalyzer::new());
    }
}

/// Get disk usage analyzer
pub fn get_disk_usage_analyzer() -> &'static mut DiskUsageAnalyzer {
    unsafe {
        DISK_USAGE_ANALYZER
            .as_mut()
            .expect("Disk usage analyzer not initialized")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage_analyzer_creation() {
        let analyzer = DiskUsageAnalyzer::new();
        assert_eq!(analyzer.scan_count(), 0);
    }

    #[test]
    fn test_scan_directory() {
        let mut analyzer = DiskUsageAnalyzer::new();
        let result = analyzer.scan_directory("/home".to_string()).unwrap();
        assert!(analyzer.get_current_scan().is_some());
    }

    #[test]
    fn test_format_size() {
        assert_eq!(DiskUsageAnalyzer::format_size(1024), "1.00 KB");
        assert_eq!(DiskUsageAnalyzer::format_size(1024 * 1024), "1.00 MB");
        assert_eq!(
            DiskUsageAnalyzer::format_size(1024 * 1024 * 1024),
            "1.00 GB"
        );
    }

    #[test]
    fn test_clear_scans() {
        let mut analyzer = DiskUsageAnalyzer::new();
        analyzer.scan_directory("/home".to_string()).unwrap();
        analyzer.clear_scans();
        assert_eq!(analyzer.scan_count(), 0);
    }
}
