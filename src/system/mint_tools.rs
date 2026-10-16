// SPDX-License-Identifier: MIT
// SigmaOS - System Tools Suite
// Inspired by Linux Mint Tools (mintbackup, mintupdate, mintinstall, driver manager, system reports)
// Provides backup, update, software management, driver management, and system diagnostics

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Backup type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupType {
    Full,
    Incremental,
    Differential,
}

/// Backup status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackupStatus {
    Pending,
    InProgress,
    Completed,
    Failed,
}

/// Backup configuration
#[derive(Debug, Clone)]
pub struct BackupConfig {
    pub name: String,
    pub backup_type: BackupType,
    pub source_paths: Vec<String>,
    pub destination: String,
    pub compression: bool,
    pub encryption: bool,
    pub include_hidden: bool,
}

/// Backup job
#[derive(Debug, Clone)]
pub struct BackupJob {
    pub id: u32,
    pub config: BackupConfig,
    pub status: BackupStatus,
    pub start_time: u64,
    pub end_time: Option<u64>,
    pub size_bytes: u64,
    pub error_message: Option<String>,
}

/// Update type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateType {
    Security,
    BugFix,
    Feature,
    Dependency,
}

/// Update severity
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UpdateSeverity {
    Critical,
    High,
    Medium,
    Low,
}

/// Package update
#[derive(Debug, Clone)]
pub struct PackageUpdate {
    pub name: String,
    pub version_old: String,
    pub version_new: String,
    pub update_type: UpdateType,
    pub severity: UpdateSeverity,
    pub size_bytes: u64,
    pub description: String,
}

/// Update manager
#[derive(Debug, Clone)]
pub struct MintUpdateManager {
    pub available_updates: Vec<PackageUpdate>,
    pub auto_update_enabled: bool,
    pub last_check_time: u64,
    pub auto_update_time: String, // cron-style
}

/// Software category
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoftwareCategory {
    AudioVideo,
    Development,
    Education,
    Games,
    Graphics,
    Network,
    Office,
    Science,
    System,
    Utility,
}

/// Software package
#[derive(Debug, Clone)]
pub struct SoftwarePackage {
    pub name: String,
    pub version: String,
    pub category: SoftwareCategory,
    pub description: String,
    pub installed: bool,
    pub size_bytes: u64,
    pub rating: f32,
}

/// Software manager
#[derive(Debug, Clone)]
pub struct SoftwareManager {
    pub packages: BTreeMap<String, SoftwarePackage>,
    pub search_query: String,
    pub categories: Vec<SoftwareCategory>,
}

/// Driver type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
    Graphics,
    Network,
    Audio,
    Printer,
    Scanner,
    Input,
    Storage,
}

/// Driver status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverStatus {
    Installed,
    Available,
    Proprietary,
    OpenSource,
    Missing,
}

/// Driver
#[derive(Debug, Clone)]
pub struct Driver {
    pub name: String,
    pub driver_type: DriverType,
    pub version: String,
    pub status: DriverStatus,
    pub recommended: bool,
    pub proprietary: bool,
}

/// Driver manager
#[derive(Debug, Clone)]
pub struct DriverManager {
    pub drivers: Vec<Driver>,
    pub auto_install: bool,
}

/// Diagnostic test
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DiagnosticTest {
    CPU,
    Memory,
    Disk,
    Network,
    GPU,
    Battery,
    Thermal,
}

/// Test result
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TestResult {
    Pass,
    Fail,
    Warning,
    Skipped,
}

/// Diagnostic entry
#[derive(Debug, Clone)]
pub struct DiagnosticEntry {
    pub test: DiagnosticTest,
    pub result: TestResult,
    pub message: String,
    pub timestamp: u64,
}

/// System report
#[derive(Debug, Clone)]
pub struct SystemReport {
    pub id: u32,
    pub generated_at: u64,
    pub diagnostics: Vec<DiagnosticEntry>,
    pub hardware_info: HardwareInfo,
    pub software_info: SoftwareInfo,
}

/// Hardware info
#[derive(Debug, Clone)]
pub struct HardwareInfo {
    pub cpu_model: String,
    pub cpu_cores: u32,
    pub memory_total: u64,
    pub memory_available: u64,
    pub disk_total: u64,
    pub disk_free: u64,
    pub gpu_model: String,
}

/// Software info
#[derive(Debug, Clone)]
pub struct SoftwareInfo {
    pub os_version: String,
    pub kernel_version: String,
    pub installed_packages: u32,
    pub uptime_seconds: u64,
}

/// System tools engine
pub struct SystemToolsEngine {
    pub backup_jobs: BTreeMap<u32, BackupJob>,
    pub update_manager: MintUpdateManager,
    pub software_manager: SoftwareManager,
    pub driver_manager: DriverManager,
    pub system_reports: BTreeMap<u32, SystemReport>,
    pub next_backup_id: AtomicU32,
    pub next_report_id: AtomicU32,
    pub active: AtomicBool,
}

impl SystemToolsEngine {
    pub fn new() -> Self {
        Self {
            backup_jobs: BTreeMap::new(),
            update_manager: MintUpdateManager {
                available_updates: Vec::new(),
                auto_update_enabled: true,
                last_check_time: 0,
                auto_update_time: "daily".to_string(),
            },
            software_manager: SoftwareManager {
                packages: BTreeMap::new(),
                search_query: String::new(),
                categories: vec![
                    SoftwareCategory::AudioVideo,
                    SoftwareCategory::Development,
                    SoftwareCategory::Education,
                    SoftwareCategory::Games,
                    SoftwareCategory::Graphics,
                    SoftwareCategory::Network,
                    SoftwareCategory::Office,
                    SoftwareCategory::Science,
                    SoftwareCategory::System,
                    SoftwareCategory::Utility,
                ],
            },
            driver_manager: DriverManager {
                drivers: Vec::new(),
                auto_install: false,
            },
            system_reports: BTreeMap::new(),
            next_backup_id: AtomicU32::new(1),
            next_report_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }

    /// Initialize system tools
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }

    /// Create backup job
    pub fn create_backup(&mut self, config: BackupConfig) -> u32 {
        let id = self.next_backup_id.fetch_add(1, Ordering::Relaxed);
        let job = BackupJob {
            id,
            config,
            status: BackupStatus::Pending,
            start_time: 0,
            end_time: None,
            size_bytes: 0,
            error_message: None,
        };
        self.backup_jobs.insert(id, job);
        id
    }

    /// Start backup
    pub fn start_backup(&mut self, backup_id: u32) -> Result<(), &'static str> {
        let job = self
            .backup_jobs
            .get_mut(&backup_id)
            .ok_or("Backup not found")?;
        job.status = BackupStatus::InProgress;
        job.start_time = 0; // Would be actual timestamp
        Ok(())
    }

    /// Complete backup
    pub fn complete_backup(&mut self, backup_id: u32, size_bytes: u64) -> Result<(), &'static str> {
        let job = self
            .backup_jobs
            .get_mut(&backup_id)
            .ok_or("Backup not found")?;
        job.status = BackupStatus::Completed;
        job.end_time = Some(0); // Would be actual timestamp
        job.size_bytes = size_bytes;
        Ok(())
    }

    /// Fail backup
    pub fn fail_backup(&mut self, backup_id: u32, error: String) -> Result<(), &'static str> {
        let job = self
            .backup_jobs
            .get_mut(&backup_id)
            .ok_or("Backup not found")?;
        job.status = BackupStatus::Failed;
        job.end_time = Some(0);
        job.error_message = Some(error);
        Ok(())
    }

    /// Check for updates
    pub fn check_updates(&mut self) {
        // In a real implementation, this would query package repositories
        self.update_manager.last_check_time = 0; // Would be actual timestamp
    }

    /// Install update
    pub fn install_update(&mut self, package_name: String) -> Result<(), &'static str> {
        let index = self
            .update_manager
            .available_updates
            .iter()
            .position(|u| u.name == package_name);
        if let Some(idx) = index {
            self.update_manager.available_updates.remove(idx);
            Ok(())
        } else {
            Err("Update not found")
        }
    }

    /// Install all updates
    pub fn install_all_updates(&mut self) {
        self.update_manager.available_updates.clear();
    }

    /// Toggle auto-update
    pub fn toggle_auto_update(&mut self) {
        self.update_manager.auto_update_enabled = !self.update_manager.auto_update_enabled;
    }

    /// Search software
    pub fn search_software(&mut self, query: String) -> Vec<&SoftwarePackage> {
        self.software_manager.search_query = query.clone();
        let query_lower = query.to_lowercase();
        self.software_manager
            .packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    /// Install software
    pub fn install_software(&mut self, package_name: String) -> Result<(), &'static str> {
        let pkg = self
            .software_manager
            .packages
            .get_mut(&package_name)
            .ok_or("Package not found")?;
        pkg.installed = true;
        Ok(())
    }

    /// Remove software
    pub fn remove_software(&mut self, package_name: String) -> Result<(), &'static str> {
        let pkg = self
            .software_manager
            .packages
            .get_mut(&package_name)
            .ok_or("Package not found")?;
        pkg.installed = false;
        Ok(())
    }

    /// Register driver
    pub fn register_driver(&mut self, driver: Driver) {
        self.driver_manager.drivers.push(driver);
    }

    /// Install driver
    pub fn install_driver(&mut self, driver_name: String) -> Result<(), &'static str> {
        let driver = self
            .driver_manager
            .drivers
            .iter_mut()
            .find(|d| d.name == driver_name)
            .ok_or("Driver not found")?;
        driver.status = DriverStatus::Installed;
        Ok(())
    }

    /// Toggle auto driver install
    pub fn toggle_auto_driver_install(&mut self) {
        self.driver_manager.auto_install = !self.driver_manager.auto_install;
    }

    /// Generate system report
    pub fn generate_report(
        &mut self,
        hardware_info: HardwareInfo,
        software_info: SoftwareInfo,
    ) -> u32 {
        let id = self.next_report_id.fetch_add(1, Ordering::Relaxed);
        let report = SystemReport {
            id,
            generated_at: 0, // Would be actual timestamp
            diagnostics: Vec::new(),
            hardware_info,
            software_info,
        };
        self.system_reports.insert(id, report);
        id
    }

    /// Add diagnostic
    pub fn add_diagnostic(
        &mut self,
        report_id: u32,
        test: DiagnosticTest,
        result: TestResult,
        message: String,
    ) -> Result<(), &'static str> {
        let report = self
            .system_reports
            .get_mut(&report_id)
            .ok_or("Report not found")?;
        report.diagnostics.push(DiagnosticEntry {
            test,
            result,
            message,
            timestamp: 0,
        });
        Ok(())
    }

    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for SystemToolsEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Global system tools
static mut SYSTEM_TOOLS: Option<SystemToolsEngine> = None;

/// Initialize system tools
pub fn init_system_tools() {
    unsafe {
        SYSTEM_TOOLS = Some(SystemToolsEngine::new());
    }
}

/// Get global system tools
pub fn get_system_tools() -> Option<&'static mut SystemToolsEngine> {
    unsafe { SYSTEM_TOOLS.as_mut() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_tools_init() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        assert!(tools.is_active());
    }

    #[test]
    fn test_backup() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        let config = BackupConfig {
            name: "Test Backup".to_string(),
            backup_type: BackupType::Full,
            source_paths: vec!["/home".to_string()],
            destination: "/backup".to_string(),
            compression: true,
            encryption: false,
            include_hidden: false,
        };
        let backup_id = tools.create_backup(config);
        tools.start_backup(backup_id).unwrap();
        assert_eq!(
            tools.backup_jobs[&backup_id].status,
            BackupStatus::InProgress
        );
        tools.complete_backup(backup_id, 1024 * 1024).unwrap();
        assert_eq!(
            tools.backup_jobs[&backup_id].status,
            BackupStatus::Completed
        );
    }

    #[test]
    fn test_updates() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        tools.check_updates();
        tools.toggle_auto_update();
        assert!(!tools.update_manager.auto_update_enabled);
    }

    #[test]
    fn test_software() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        let pkg = SoftwarePackage {
            name: "TestApp".to_string(),
            version: "1.0".to_string(),
            category: SoftwareCategory::Utility,
            description: "Test application".to_string(),
            installed: false,
            size_bytes: 1024,
            rating: 4.5,
        };
        tools
            .software_manager
            .packages
            .insert("TestApp".to_string(), pkg);
        tools.install_software("TestApp".to_string()).unwrap();
        assert!(tools.software_manager.packages["TestApp"].installed);
    }

    #[test]
    fn test_drivers() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        let driver = Driver {
            name: "Test Driver".to_string(),
            driver_type: DriverType::Graphics,
            version: "1.0".to_string(),
            status: DriverStatus::Available,
            recommended: true,
            proprietary: false,
        };
        tools.register_driver(driver);
        tools.install_driver("Test Driver".to_string()).unwrap();
        assert_eq!(
            tools.driver_manager.drivers[0].status,
            DriverStatus::Installed
        );
    }

    #[test]
    fn test_system_report() {
        let mut tools = SystemToolsEngine::new();
        tools.init();
        let hw_info = HardwareInfo {
            cpu_model: "Test CPU".to_string(),
            cpu_cores: 4,
            memory_total: 8 * 1024 * 1024 * 1024,
            memory_available: 4 * 1024 * 1024 * 1024,
            disk_total: 500 * 1024 * 1024 * 1024,
            disk_free: 250 * 1024 * 1024 * 1024,
            gpu_model: "Test GPU".to_string(),
        };
        let sw_info = SoftwareInfo {
            os_version: "1.0".to_string(),
            kernel_version: "5.0".to_string(),
            installed_packages: 100,
            uptime_seconds: 3600,
        };
        let report_id = tools.generate_report(hw_info, sw_info);
        tools
            .add_diagnostic(
                report_id,
                DiagnosticTest::CPU,
                TestResult::Pass,
                "CPU OK".to_string(),
            )
            .unwrap();
        assert_eq!(tools.system_reports[&report_id].diagnostics.len(), 1);
    }
}
