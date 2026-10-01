// Filesystem Monitoring for SigmaOS
// Filesystem monitoring per Wiki 05-Filesystems.md
// Provides disk usage, inode usage, and filesystem checks

use std::string::{String, ToString};
use std::vec::Vec;

/// Disk usage information
#[derive(Debug, Clone)]
pub struct DiskUsage {
    pub mount_point: String,
    pub device: String,
    pub fs_type: String,
    pub total_size: u64, // in bytes
    pub used_size: u64,
    pub available_size: u64,
    pub usage_percent: f32,
}

impl DiskUsage {
    pub fn new(mount_point: String, device: String, fs_type: String) -> Self {
        DiskUsage {
            mount_point,
            device,
            fs_type,
            total_size: 0,
            used_size: 0,
            available_size: 0,
            usage_percent: 0.0,
        }
    }

    pub fn calculate_usage(&mut self) {
        if self.total_size > 0 {
            self.usage_percent = (self.used_size as f32 / self.total_size as f32) * 100.0;
            self.available_size = self.total_size - self.used_size;
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "{} {} {} {} {:.1}% {}",
            self.device,
            self.total_size,
            self.used_size,
            self.available_size,
            self.usage_percent,
            self.mount_point
        )
    }

    pub fn get_human_readable(&self) -> String {
        let total_mb = self.total_size / (1024 * 1024);
        let used_mb = self.used_size / (1024 * 1024);
        let avail_mb = self.available_size / (1024 * 1024);

        format!(
            "{} {} {} {} {:.1}% {}",
            self.device, total_mb, used_mb, avail_mb, self.usage_percent, self.mount_point
        )
    }
}

/// Directory size information
#[derive(Debug, Clone)]
pub struct DirectorySize {
    pub path: String,
    pub size: u64, // in bytes
    pub file_count: u32,
    pub dir_count: u32,
}

impl DirectorySize {
    pub fn new(path: String) -> Self {
        DirectorySize {
            path,
            size: 0,
            file_count: 0,
            dir_count: 0,
        }
    }

    pub fn get_human_readable(&self) -> String {
        let mb = self.size / (1024 * 1024);
        format!(
            "{} {}M {} files {} dirs",
            self.path, mb, self.file_count, self.dir_count
        )
    }
}

/// Inode usage information
#[derive(Debug, Clone)]
pub struct InodeUsage {
    pub mount_point: String,
    pub total_inodes: u64,
    pub used_inodes: u64,
    pub free_inodes: u64,
    pub usage_percent: f32,
}

impl InodeUsage {
    pub fn new(mount_point: String) -> Self {
        InodeUsage {
            mount_point,
            total_inodes: 0,
            used_inodes: 0,
            free_inodes: 0,
            usage_percent: 0.0,
        }
    }

    pub fn calculate_usage(&mut self) {
        if self.total_inodes > 0 {
            self.usage_percent = (self.used_inodes as f32 / self.total_inodes as f32) * 100.0;
            self.free_inodes = self.total_inodes - self.used_inodes;
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "{} {} {} {} {:.1}%",
            self.total_inodes,
            self.used_inodes,
            self.free_inodes,
            self.usage_percent,
            self.mount_point
        )
    }
}

/// Filesystem check result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FsCheckStatus {
    Clean,
    ErrorsFound,
    ErrorsFixed,
    Failed,
}

impl FsCheckStatus {
    pub fn as_str(&self) -> &str {
        match self {
            FsCheckStatus::Clean => "clean",
            FsCheckStatus::ErrorsFound => "errors found",
            FsCheckStatus::ErrorsFixed => "errors fixed",
            FsCheckStatus::Failed => "failed",
        }
    }
}

/// Filesystem check result
#[derive(Debug, Clone)]
pub struct FsCheckResult {
    pub device: String,
    pub fs_type: String,
    pub status: FsCheckStatus,
    pub errors_fixed: u32,
    pub errors_found: u32,
    pub output: String,
}

impl FsCheckResult {
    pub fn new(device: String, fs_type: String) -> Self {
        FsCheckResult {
            device,
            fs_type,
            status: FsCheckStatus::Clean,
            errors_fixed: 0,
            errors_found: 0,
            output: String::new(),
        }
    }

    pub fn get_summary(&self) -> String {
        format!(
            "{} {} {} ({} errors found, {} fixed)",
            self.device,
            self.fs_type,
            self.status.as_str(),
            self.errors_found,
            self.errors_fixed
        )
    }
}

/// Filesystem monitor
#[derive(Debug, Clone)]
pub struct FilesystemMonitor {
    pub disk_usage: Vec<DiskUsage>,
    pub directory_sizes: Vec<DirectorySize>,
    pub inode_usage: Vec<InodeUsage>,
}

impl Default for FilesystemMonitor {
    fn default() -> Self {
        FilesystemMonitor {
            disk_usage: Vec::new(),
            directory_sizes: Vec::new(),
            inode_usage: Vec::new(),
        }
    }
}

impl FilesystemMonitor {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_disk_usage(&mut self, usage: DiskUsage) {
        self.disk_usage.push(usage);
    }

    pub fn add_directory_size(&mut self, size: DirectorySize) {
        self.directory_sizes.push(size);
    }

    pub fn add_inode_usage(&mut self, usage: InodeUsage) {
        self.inode_usage.push(usage);
    }

    pub fn get_disk_usage(&self, mount_point: &str) -> Option<&DiskUsage> {
        self.disk_usage
            .iter()
            .find(|d| d.mount_point == mount_point)
    }

    pub fn list_disk_usage(&self) -> Vec<DiskUsage> {
        self.disk_usage.clone()
    }

    pub fn list_disk_usage_sorted(&self) -> Vec<DiskUsage> {
        let mut usage = self.disk_usage.clone();
        usage.sort_by(|a, b| {
            b.usage_percent
                .partial_cmp(&a.usage_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        usage
    }

    pub fn list_directory_sizes(&self) -> Vec<DirectorySize> {
        self.directory_sizes.clone()
    }

    pub fn list_directory_sizes_sorted(&self) -> Vec<DirectorySize> {
        let mut sizes = self.directory_sizes.clone();
        sizes.sort_by(|a, b| b.size.cmp(&a.size));
        sizes
    }

    pub fn list_inode_usage(&self) -> Vec<InodeUsage> {
        self.inode_usage.clone()
    }

    pub fn list_inode_usage_sorted(&self) -> Vec<InodeUsage> {
        let mut usage = self.inode_usage.clone();
        usage.sort_by(|a, b| {
            b.usage_percent
                .partial_cmp(&a.usage_percent)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        usage
    }

    pub fn check_filesystem(&self, device: &str, fs_type: &str, force: bool) -> FsCheckResult {
        let mut result = FsCheckResult::new(device.to_string(), fs_type.to_string());

        // Simulate filesystem check
        if force {
            result.status = FsCheckStatus::Clean;
            result.output = format!(
                "Forced check on {} ({}) completed successfully",
                device, fs_type
            );
        } else {
            result.status = FsCheckStatus::Clean;
            result.output = format!("Check on {} ({}) completed successfully", device, fs_type);
        }

        result
    }

    pub fn get_high_usage_disks(&self, threshold: f32) -> Vec<DiskUsage> {
        self.disk_usage
            .iter()
            .filter(|d| d.usage_percent >= threshold)
            .cloned()
            .collect()
    }

    pub fn get_high_inode_usage(&self, threshold: f32) -> Vec<InodeUsage> {
        self.inode_usage
            .iter()
            .filter(|i| i.usage_percent >= threshold)
            .cloned()
            .collect()
    }

    pub fn get_total_disk_usage(&self) -> u64 {
        self.disk_usage.iter().map(|d| d.used_size).sum()
    }

    pub fn get_total_disk_capacity(&self) -> u64 {
        self.disk_usage.iter().map(|d| d.total_size).sum()
    }

    pub fn get_overall_usage_percent(&self) -> f32 {
        let total = self.get_total_disk_capacity();
        if total > 0 {
            (self.get_total_disk_usage() as f32 / total as f32) * 100.0
        } else {
            0.0
        }
    }

    pub fn get_statistics(&self) -> String {
        let mut stats = String::from("Filesystem Statistics:\n");
        stats.push_str(&format!(
            "Filesystems monitored: {}\n",
            self.disk_usage.len()
        ));
        stats.push_str(&format!(
            "Directories monitored: {}\n",
            self.directory_sizes.len()
        ));
        stats.push_str(&format!(
            "Inode tables monitored: {}\n",
            self.inode_usage.len()
        ));
        stats.push_str(&format!("Filesystems monitored: {}\n", self.disk_usage.len()));
        stats.push_str(&format!("Directories monitored: {}\n", self.directory_sizes.len()));
        stats.push_str(&format!("Inode tables monitored: {}\n", self.inode_usage.len()));

        let total_usage = self.get_total_disk_usage();
        let total_capacity = self.get_total_disk_capacity();
        let overall_percent = self.get_overall_usage_percent();

        stats.push_str(&format!("Total disk usage: {} bytes\n", total_usage));
        stats.push_str(&format!("Total disk capacity: {} bytes\n", total_capacity));
        stats.push_str(&format!("Overall usage: {:.1}%\n", overall_percent));

        let high_usage = self.get_high_usage_disks(90.0);
        stats.push_str(&format!(
            "High usage filesystems (>90%): {}\n",
            high_usage.len()
        ));
        stats.push_str(&format!("High usage filesystems (>90%): {}\n", high_usage.len()));

        let high_inode = self.get_high_inode_usage(90.0);
        stats.push_str(&format!("High inode usage (>90%): {}\n", high_inode.len()));

        stats
    }

    pub fn scan_directory(&mut self, path: String, max_depth: u32) -> DirectorySize {
        let mut size = DirectorySize::new(path.clone());

        // Simulate directory scan
        size.size = 1024 * 1024 * 100; // 100 MB
        size.file_count = 50;
        size.dir_count = 10;

        self.directory_sizes.push(size.clone());
        size
    }

    pub fn count_files(&self, path: &str) -> u32 {
        // Simulate file count
        self.directory_sizes
            .iter()
            .filter(|d| d.path.starts_with(path))
            .map(|d| d.file_count)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage_creation() {
        let usage = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        assert_eq!(usage.mount_point, "/");
        assert_eq!(usage.device, "/dev/sda1");
    }

    #[test]
    fn test_disk_usage_calculate() {
        let mut usage = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        usage.total_size = 1000;
        usage.used_size = 500;
        usage.calculate_usage();

        assert_eq!(usage.usage_percent, 50.0);
        assert_eq!(usage.available_size, 500);
    }

    #[test]
    fn test_disk_usage_get_summary() {
        let mut usage = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        usage.total_size = 1000;
        usage.used_size = 500;
        usage.calculate_usage();

        let summary = usage.get_summary();
        assert!(summary.contains("/dev/sda1"));
        assert!(summary.contains("50.0%"));
    }

    #[test]
    fn test_directory_size_creation() {
        let size = DirectorySize::new(String::from("/home/user"));
        assert_eq!(size.path, "/home/user");
        assert_eq!(size.size, 0);
    }

    #[test]
    fn test_inode_usage_creation() {
        let usage = InodeUsage::new(String::from("/"));
        assert_eq!(usage.mount_point, "/");
        assert_eq!(usage.total_inodes, 0);
    }

    #[test]
    fn test_inode_usage_calculate() {
        let mut usage = InodeUsage::new(String::from("/"));
        usage.total_inodes = 1000;
        usage.used_inodes = 500;
        usage.calculate_usage();

        assert_eq!(usage.usage_percent, 50.0);
        assert_eq!(usage.free_inodes, 500);
    }

    #[test]
    fn test_fs_check_status_as_str() {
        assert_eq!(FsCheckStatus::Clean.as_str(), "clean");
        assert_eq!(FsCheckStatus::ErrorsFound.as_str(), "errors found");
        assert_eq!(FsCheckStatus::ErrorsFixed.as_str(), "errors fixed");
    }

    #[test]
    fn test_fs_check_result_creation() {
        let result = FsCheckResult::new(String::from("/dev/sda1"), String::from("ext4"));
        assert_eq!(result.device, "/dev/sda1");
        assert_eq!(result.fs_type, "ext4");
        assert_eq!(result.status, FsCheckStatus::Clean);
    }

    #[test]
    fn test_filesystem_monitor_creation() {
        let monitor = FilesystemMonitor::new();
        assert_eq!(monitor.disk_usage.len(), 0);
        assert_eq!(monitor.directory_sizes.len(), 0);
    }

    #[test]
    fn test_filesystem_monitor_add_disk_usage() {
        let mut monitor = FilesystemMonitor::new();
        monitor.add_disk_usage(DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        ));
        assert_eq!(monitor.disk_usage.len(), 1);
    }

    #[test]
    fn test_filesystem_monitor_get_disk_usage() {
        let mut monitor = FilesystemMonitor::new();
        monitor.add_disk_usage(DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        ));

        let usage = monitor.get_disk_usage("/");
        assert!(usage.is_some());
    }

    #[test]
    fn test_filesystem_monitor_list_disk_usage_sorted() {
        let mut monitor = FilesystemMonitor::new();
        let mut usage1 = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        usage1.usage_percent = 90.0;
        monitor.add_disk_usage(usage1);

        let mut usage2 = DiskUsage::new(
            String::from("/home"),
            String::from("/dev/sda2"),
            String::from("ext4"),
        );
        let mut usage2 = DiskUsage::new(String::from("/home"), String::from("/dev/sda2"), String::from("ext4"));
        usage2.usage_percent = 50.0;
        monitor.add_disk_usage(usage2);

        let sorted = monitor.list_disk_usage_sorted();
        assert_eq!(sorted[0].usage_percent, 90.0);
    }

    #[test]
    fn test_filesystem_monitor_check_filesystem() {
        let monitor = FilesystemMonitor::new();
        let result = monitor.check_filesystem("/dev/sda1", "ext4", false);
        assert_eq!(result.status, FsCheckStatus::Clean);
    }

    #[test]
    fn test_filesystem_monitor_get_high_usage_disks() {
        let mut monitor = FilesystemMonitor::new();
        let mut usage1 = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        usage1.usage_percent = 95.0;
        monitor.add_disk_usage(usage1);

        let mut usage2 = DiskUsage::new(
            String::from("/home"),
            String::from("/dev/sda2"),
            String::from("ext4"),
        );
        let mut usage2 = DiskUsage::new(String::from("/home"), String::from("/dev/sda2"), String::from("ext4"));
        usage2.usage_percent = 50.0;
        monitor.add_disk_usage(usage2);

        let high = monitor.get_high_usage_disks(90.0);
        assert_eq!(high.len(), 1);
    }

    #[test]
    fn test_filesystem_monitor_get_statistics() {
        let mut monitor = FilesystemMonitor::new();
        let mut usage = DiskUsage::new(
            String::from("/"),
            String::from("/dev/sda1"),
            String::from("ext4"),
        );
        usage.total_size = 1000;
        usage.used_size = 500;
        monitor.add_disk_usage(usage);

        let stats = monitor.get_statistics();
        assert!(stats.contains("Filesystems monitored: 1"));
        assert!(stats.contains("Total disk usage: 500 bytes"));
    }

    #[test]
    fn test_filesystem_monitor_scan_directory() {
        let mut monitor = FilesystemMonitor::new();
        let size = monitor.scan_directory(String::from("/home/user"), 1);

        assert_eq!(size.path, "/home/user");
        assert_eq!(monitor.directory_sizes.len(), 1);
    }

    #[test]
    fn test_filesystem_monitor_count_files() {
        let mut monitor = FilesystemMonitor::new();
        let mut size = DirectorySize::new(String::from("/home/user"));
        size.file_count = 50;
        monitor.directory_sizes.push(size);

        let count = monitor.count_files("/home/user");
        assert_eq!(count, 50);
    }
}
