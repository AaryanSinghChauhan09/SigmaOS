/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, MintReport, Timeshift-style System Restore,
/// Cinnamon-like desktop theme manager, and MintDrivers manager.

use core::sync::atomic::{AtomicUsize, Ordering};
use crate::klib::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintUpdateLevel {
    Level1Safe,      // Certified safe updates (no system files)
    Level2Tested,    // Thoroughly tested system package upgrades
    Level3Normal,    // Normal upstream upgrades
    Level4Sensitive, // Potentially sensitive, requiring reboot
    Level5Critical,  // Core kernel/VMM critical upgrades (advise care)
}

#[derive(Debug, Clone)]
pub struct MintUpdatePackage {
    pub name: [u8; 32],
    pub version_old: [u8; 16],
    pub version_new: [u8; 16],
    pub level: MintUpdateLevel,
    pub safety_score: usize,
}

impl MintUpdatePackage {
    pub fn new(name: &[u8], old: &[u8], new: &[u8], level: MintUpdateLevel) -> Self {
        let mut name_arr = [0u8; 32];
        let mut old_arr = [0u8; 16];
        let mut new_arr = [0u8; 16];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        old_arr[..old.len().min(15)].copy_from_slice(&old[..old.len().min(15)]);
        new_arr[..new.len().min(15)].copy_from_slice(&new[..new.len().min(15)]);

        let safety_score = match level {
            MintUpdateLevel::Level1Safe => 99,
            MintUpdateLevel::Level2Tested => 95,
            MintUpdateLevel::Level3Normal => 85,
            MintUpdateLevel::Level4Sensitive => 65,
            MintUpdateLevel::Level5Critical => 30,
        };

        MintUpdatePackage {
            name: name_arr,
            version_old: old_arr,
            version_new: new_arr,
            level,
            safety_score,
        }
    }
}

/// MintUpdate: Safe update managers, mirror selection, and kernel swapping
pub struct MintUpdateManager {
    pub pending_updates: Vec<MintUpdatePackage>,
    pub selected_mirror_speed_ms: usize,
    pub current_kernel_ver: [u8; 16],
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateManager {
    pub fn new() -> Self {
        let mut current_kernel_ver = [0u8; 16];
        let version = b"6.5.6-sigma";
        current_kernel_ver[..version.len()].copy_from_slice(version);

        MintUpdateManager {
            pending_updates: Vec::new(),
            selected_mirror_speed_ms: 9999,
            current_kernel_ver,
        }
    }

    pub fn add_update(&mut self, update: MintUpdatePackage) {
        self.pending_updates.push(update);
    }

    pub fn auto_select_fastest_mirror(&mut self, mirrors: &[(&[u8], usize)]) {
        let mut best_speed = 9999;
        for &(_, speed) in mirrors {
            if speed < best_speed {
                best_speed = speed;
            }
        }
        self.selected_mirror_speed_ms = best_speed;
    }

    pub fn hot_swap_active_kernel(&mut self, new_version: &[u8]) -> Result<(), &'static str> {
        if new_version.is_empty() {
            return Err("Invalid kernel version");
        }
        let len = new_version.len().min(15);
        self.current_kernel_ver = [0u8; 16];
        self.current_kernel_ver[..len].copy_from_slice(&new_version[..len]);
        Ok(())
    }
}

/// MintBackup: Incremental user-data backup and profile state archiver
pub struct MintBackupTool {
    pub user_backups_count: AtomicUsize,
    pub active_backup_path: [u8; 64],
}

impl Default for MintBackupTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MintBackupTool {
    pub fn new() -> Self {
        MintBackupTool {
            user_backups_count: AtomicUsize::new(0),
            active_backup_path: [0u8; 64],
        }
    }

    pub fn perform_user_backup(&mut self, backup_dir: &[u8]) -> Result<usize, &'static str> {
        if backup_dir.is_empty() {
            return Err("Backup target directory is invalid");
        }
        let len = backup_dir.len().min(63);
        self.active_backup_path[..len].copy_from_slice(&backup_dir[..len]);
        let id = self.user_backups_count.fetch_add(1, Ordering::SeqCst);
        Ok(id)
    }
}

/// MintInstall: High-level application software ratings and metadata
#[derive(Debug, Clone)]
pub struct MintAppMetadata {
    pub name: [u8; 32],
    pub rating_stars: usize, // 1 to 5
    pub reviews_count: usize,
    pub is_flatpak: bool,
}

impl MintAppMetadata {
    pub fn new(name: &[u8], rating_stars: usize, reviews_count: usize, is_flatpak: bool) -> Self {
        let mut name_arr = [0u8; 32];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        MintAppMetadata {
            name: name_arr,
            rating_stars: rating_stars.clamp(1, 5),
            reviews_count,
            is_flatpak,
        }
    }
}

pub struct MintSoftwareManager {
    pub apps_catalog: Vec<MintAppMetadata>,
}

impl Default for MintSoftwareManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoftwareManager {
    pub fn new() -> Self {
        MintSoftwareManager {
            apps_catalog: Vec::new(),
        }
    }

    pub fn add_app_to_catalog(&mut self, app: MintAppMetadata) {
        self.apps_catalog.push(app);
    }
}

/// MintReport: Detects system crashes, memory warnings, and provides direct advice remedies
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintReportAlertSeverity {
    Info,
    Warning,
    Critical,
}

#[derive(Debug, Clone)]
pub struct MintReportAlert {
    pub name: [u8; 32],
    pub severity: MintReportAlertSeverity,
    pub remedy_advice: [u8; 64],
}

impl MintReportAlert {
    pub fn new(name: &[u8], severity: MintReportAlertSeverity, advice: &[u8]) -> Self {
        let mut name_arr = [0u8; 32];
        let mut advice_arr = [0u8; 64];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        advice_arr[..advice.len().min(63)].copy_from_slice(&advice[..advice.len().min(63)]);

        MintReportAlert {
            name: name_arr,
            severity,
            remedy_advice: advice_arr,
        }
    }
}

pub struct MintReportSystem {
    pub active_alerts: Vec<MintReportAlert>,
}

impl Default for MintReportSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl MintReportSystem {
    pub fn new() -> Self {
        MintReportSystem {
            active_alerts: Vec::new(),
        }
    }

    pub fn register_crash_alert(&mut self, app_name: &[u8]) {
        let mut alert_name = [0u8; 32];
        let len = app_name.len().min(15);
        alert_name[..len].copy_from_slice(&app_name[..len]);
        let suffix = b" crashed";
        alert_name[len..len + suffix.len()].copy_from_slice(suffix);

        let alert = MintReportAlert::new(
            &alert_name,
            MintReportAlertSeverity::Critical,
            b"Please restart the service or run sigpkg update",
        );
        self.active_alerts.push(alert);
    }
}

/// Timeshift-style System Restore checkpoint
#[derive(Debug, Clone)]
pub struct TimeshiftSnapshot {
    pub id: usize,
    pub timestamp_epoch: u64,
    pub description: [u8; 64],
    pub system_state_hash: u64, // Simulated Merkle root hash of systems
}

impl TimeshiftSnapshot {
    pub fn new(id: usize, timestamp_epoch: u64, desc: &[u8], hash: u64) -> Self {
        let mut desc_arr = [0u8; 64];
        let len = desc.len().min(63);
        desc_arr[..len].copy_from_slice(&desc[..len]);
        TimeshiftSnapshot {
            id,
            timestamp_epoch,
            description: desc_arr,
            system_state_hash: hash,
        }
    }
}

/// Timeshift-inspired System Restore point manager
pub struct MintTimeshiftEngine {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub next_snapshot_id: AtomicUsize,
}

impl Default for MintTimeshiftEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintTimeshiftEngine {
    pub fn new() -> Self {
        MintTimeshiftEngine {
            snapshots: Vec::new(),
            next_snapshot_id: AtomicUsize::new(1),
        }
    }

    pub fn create_checkpoint(&mut self, timestamp: u64, desc: &[u8], state_hash: u64) -> usize {
        let id = self.next_snapshot_id.fetch_add(1, Ordering::SeqCst);
        let checkpoint = TimeshiftSnapshot::new(id, timestamp, desc, state_hash);
        self.snapshots.push(checkpoint);
        id
    }

    pub fn restore_checkpoint(&self, snapshot_id: usize) -> Result<u64, &'static str> {
        for snap in self.snapshots.iter() {
            if snap.id == snapshot_id {
                return Ok(snap.system_state_hash);
            }
        }
        Err("Timeshift: Target system restore point not found.")
    }
}

/// Cinnamon-inspired desktop styling configuration
#[derive(Debug, Clone, Copy)]
pub struct MintCinnamonStyling {
    pub panel_height: u32,
    pub menu_layout_compact: bool,
    pub opacity_percent: u32,
    pub window_effects_enabled: bool,
}

impl MintCinnamonStyling {
    pub fn default() -> Self {
        MintCinnamonStyling {
            panel_height: 40,
            menu_layout_compact: false,
            opacity_percent: 100,
            window_effects_enabled: true,
        }
    }

    pub fn configure_workspace(&mut self, height: u32, compact: bool, opacity: u32, effects: bool) {
        self.panel_height = height;
        self.menu_layout_compact = compact;
        self.opacity_percent = opacity.min(100);
        self.window_effects_enabled = effects;
    }
}

/// Hardware Driver metadata managed by the MintDrivers-equivalent system
#[derive(Debug, Clone)]
pub struct MintDriverInfo {
    pub name: [u8; 48],
    pub hardware_class: [u8; 32],
    pub proprietary: bool,
    pub active: bool,
}

impl MintDriverInfo {
    pub fn new(name: &[u8], class: &[u8], proprietary: bool) -> Self {
        let mut name_arr = [0u8; 48];
        let mut class_arr = [0u8; 32];
        name_arr[..name.len().min(47)].copy_from_slice(&name[..name.len().min(47)]);
        class_arr[..class.len().min(31)].copy_from_slice(&class[..class.len().min(31)]);
        MintDriverInfo {
            name: name_arr,
            hardware_class: class_arr,
            proprietary,
            active: false,
        }
    }
}

/// MintDrivers-inspired Hardware Driver Manager
pub struct MintDriverManager {
    pub available_drivers: Vec<MintDriverInfo>,
}

impl Default for MintDriverManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintDriverManager {
    pub fn new() -> Self {
        MintDriverManager {
            available_drivers: Vec::new(),
        }
    }

    pub fn register_driver(&mut self, driver: MintDriverInfo) {
        self.available_drivers.push(driver);
    }

    pub fn toggle_driver(&mut self, name: &[u8], active: bool) -> Result<(), &'static str> {
        for driver in self.available_drivers.iter_mut() {
            let mut matches = true;
            for i in 0..name.len().min(47) {
                if driver.name[i] != name[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                driver.active = active;
                return Ok(());
            }
        }
        Err("MintDrivers: Specified driver not found.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();
        let pkg =
            MintUpdatePackage::new(b"zenith", b"1.0.0", b"1.1.0", MintUpdateLevel::Level1Safe);
        manager.add_update(pkg);

        assert_eq!(manager.pending_updates.len(), 1);
        assert_eq!(manager.pending_updates[0].safety_score, 99);

        // Fast mirror selection
        manager.auto_select_fastest_mirror(&[(b"us-mirror", 45), (b"eu-mirror", 120)]);
        assert_eq!(manager.selected_mirror_speed_ms, 45);

        // Hot swap active kernel version
        manager.hot_swap_active_kernel(b"6.6.0").unwrap();
        assert!(manager.current_kernel_ver.starts_with(b"6.6.0"));
    }

    #[test]
    fn test_mint_backup_tool() {
        let mut backup = MintBackupTool::new();
        let backup_id = backup.perform_user_backup(b"/backup/user_state").unwrap();
        assert_eq!(backup_id, 0);
    }

    #[test]
    fn test_mint_software_manager() {
        let mut software = MintSoftwareManager::new();
        let app = MintAppMetadata::new(b"alacritty", 5, 230, true);
        software.add_app_to_catalog(app);
        assert_eq!(software.apps_catalog.len(), 1);
        assert_eq!(software.apps_catalog[0].rating_stars, 5);
    }

    #[test]
    fn test_mint_report_system() {
        let mut report = MintReportSystem::new();
        report.register_crash_alert(b"launcher");
        assert_eq!(report.active_alerts.len(), 1);
        assert_eq!(
            report.active_alerts[0].severity,
            MintReportAlertSeverity::Critical
        );
    }

    #[test]
    fn test_mint_timeshift_restore_points() {
        let mut timeshift = MintTimeshiftEngine::new();
        let snap_id = timeshift.create_checkpoint(1690000000, b"Fresh boot restore point", 0xDEADBEEF);
        assert_eq!(snap_id, 1);

        let hash = timeshift.restore_checkpoint(1).unwrap();
        assert_eq!(hash, 0xDEADBEEF);

        assert!(timeshift.restore_checkpoint(99).is_err());
    }

    #[test]
    fn test_mint_cinnamon_styling_options() {
        let mut style = MintCinnamonStyling::default();
        assert_eq!(style.panel_height, 40);
        assert!(style.window_effects_enabled);

        style.configure_workspace(36, true, 85, false);
        assert_eq!(style.panel_height, 36);
        assert!(style.menu_layout_compact);
        assert_eq!(style.opacity_percent, 85);
        assert!(!style.window_effects_enabled);
    }

    #[test]
    fn test_mint_driver_manager_flows() {
        let mut drivers = MintDriverManager::new();
        let wifi_drv = MintDriverInfo::new(b"Broadcom BCM4360 WiFi", b"Wireless Controller", true);
        drivers.register_driver(wifi_drv);

        assert_eq!(drivers.available_drivers.len(), 1);
        assert!(!drivers.available_drivers[0].active);

        drivers.toggle_driver(b"Broadcom BCM4360 WiFi", true).unwrap();
        assert!(drivers.available_drivers[0].active);
    }
}
