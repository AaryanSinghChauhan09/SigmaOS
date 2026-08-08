/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, and MintReport.

extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;
use alloc::string::ToString;
use alloc::format;
use core::sync::atomic::{AtomicUsize, Ordering};

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

    pub fn auto_select_fastest_mirror(&mut self, mirrors: &[( &[u8], usize )]) {
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

// =========================================================================
// 1. MINT DESKTOP CUSTOMIZER (MINTDESK CINNAMON LAYOUT)
// =========================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonLayoutMode {
    Traditional, // Windows-like bottom panel with taskbar
    Modern,      // macOS-like top bar and bottom dock
    Compact,     // Netbook/tablet compact sidebar layout
}

pub struct MintDeskCinnamonLayout {
    pub layout_mode: CinnamonLayoutMode,
    pub panel_widgets: Vec<String>,
    pub desklet_items: Vec<String>,
}

impl MintDeskCinnamonLayout {
    pub fn new() -> Self {
        Self {
            layout_mode: CinnamonLayoutMode::Traditional,
            panel_widgets: vec!["menu".to_string(), "window-list".to_string(), "systray".to_string()],
            desklet_items: Vec::new(),
        }
    }

    pub fn set_layout_mode(&mut self, mode: CinnamonLayoutMode) {
        self.layout_mode = mode;
        match mode {
            CinnamonLayoutMode::Traditional => {
                self.panel_widgets = vec!["menu".to_string(), "window-list".to_string(), "systray".to_string()];
            }
            CinnamonLayoutMode::Modern => {
                self.panel_widgets = vec!["global-menu".to_string(), "dock".to_string(), "indicator-applet".to_string()];
            }
            CinnamonLayoutMode::Compact => {
                self.panel_widgets = vec!["vertical-launcher".to_string(), "systray-compact".to_string()];
            }
        }
    }

    pub fn add_panel_widget(&mut self, widget: &str) {
        self.panel_widgets.push(widget.to_string());
    }

    pub fn add_desklet_item(&mut self, desklet: &str) {
        self.desklet_items.push(desklet.to_string());
    }
}

impl Default for MintDeskCinnamonLayout {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 2. MINT TIMESHIFT CHECKPOINT MANAGER
// =========================================================================

pub struct MintTimeshiftCheckpointManager {
    pub last_snapshot_timestamp: u64,
    pub snapshots_taken: Vec<String>,
}

impl MintTimeshiftCheckpointManager {
    pub fn new() -> Self {
        Self {
            last_snapshot_timestamp: 0,
            snapshots_taken: Vec::new(),
        }
    }

    /// Evaluates upgrade safety level. Insists on taking a Timeshift snapshot
    /// if update is Level4Sensitive or Level5Critical and none was taken recently.
    pub fn verify_and_schedule_pre_upgrade_snapshot(
        &mut self,
        package: &str,
        level: MintUpdateLevel,
        current_timestamp: u64,
    ) -> Result<bool, &'static str> {
        let is_sensitive = level == MintUpdateLevel::Level4Sensitive || level == MintUpdateLevel::Level5Critical;

        if is_sensitive {
            // Snapshot required if older than 24 hours (86400 seconds)
            if self.last_snapshot_timestamp == 0 || (current_timestamp - self.last_snapshot_timestamp >= 86400) {
                let name = format!("timeshift-pre-upgrade-{}-{}", package, current_timestamp);
                self.snapshots_taken.push(name);
                self.last_snapshot_timestamp = current_timestamp;
                return Ok(true); // Snapshot successfully scheduled & triggered
            }
        }
        Ok(false) // No snapshot required for safe/standard updates or recent snap exists
    }
}

impl Default for MintTimeshiftCheckpointManager {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// 3. MINT SYSTEM AUTO-FIXER (MINT SYSTEM FIXER)
// =========================================================================

pub struct MintSystemFixer {
    pub applied_remedies: Vec<String>,
}

impl MintSystemFixer {
    pub fn new() -> Self {
        Self {
            applied_remedies: Vec::new(),
        }
    }

    /// Scans a crash report, identifies critical system failures (Xorg, graphics drivers,
    /// dynamic kernel modules), and applies automated remedies.
    pub fn diagnose_and_apply_remedy(&mut self, app_name: &str) -> Option<&'static str> {
        if app_name.contains("nvidia") || app_name.contains("amdgpu") || app_name.contains("dkms") {
            let action = "trigger-dkms-rebuild";
            self.applied_remedies.push(format!("{}:{}", app_name, action));
            Some("Hardware kernel driver crash detected. Initiated automatic DKMS rebuild.")
        } else if app_name.contains("xorg") || app_name.contains("cinnamon") {
            let action = "restart-display-manager";
            self.applied_remedies.push(format!("{}:{}", app_name, action));
            Some("X11 Server / display environment crashed. Re-started session safely.")
        } else {
            let action = "reinstall-package";
            self.applied_remedies.push(format!("{}:{}", app_name, action));
            Some("Application package corrupted. Scheduling automatic sigpkg reinstall.")
        }
    }
}

impl Default for MintSystemFixer {
    fn default() -> Self {
        Self::new()
    }
}

// =========================================================================
// UNIT TESTS MODULE
// =========================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();
        let pkg = MintUpdatePackage::new(b"zenith", b"1.0.0", b"1.1.0", MintUpdateLevel::Level1Safe);
        manager.add_update(pkg);

        assert_eq!(manager.pending_updates.len(), 1);
        assert_eq!(manager.pending_updates[0].safety_score, 99);

        // Fast mirror selection
        manager.auto_select_fastest_mirror(&[
            (b"us-mirror", 45),
            (b"eu-mirror", 120),
        ]);
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
        assert_eq!(report.active_alerts[0].severity, MintReportAlertSeverity::Critical);
    }

    #[test]
    fn test_mint_desktop_cinnamon_layout() {
        let mut desktop = MintDeskCinnamonLayout::new();
        assert_eq!(desktop.layout_mode, CinnamonLayoutMode::Traditional);
        assert_eq!(desktop.panel_widgets.len(), 3);

        desktop.set_layout_mode(CinnamonLayoutMode::Modern);
        assert_eq!(desktop.layout_mode, CinnamonLayoutMode::Modern);
        assert_eq!(desktop.panel_widgets[0], "global-menu");

        desktop.add_panel_widget("weather-applet");
        assert_eq!(desktop.panel_widgets[3], "weather-applet");

        desktop.add_desklet_item("cpu-monitor");
        assert_eq!(desktop.desklet_items[0], "cpu-monitor");
    }

    #[test]
    fn test_mint_timeshift_checkpoint_manager() {
        let mut timeshift = MintTimeshiftCheckpointManager::new();

        // Level 1 Safe update does not trigger snapshot
        let res1 = timeshift.verify_and_schedule_pre_upgrade_snapshot("libc6", MintUpdateLevel::Level1Safe, 1700000000).unwrap();
        assert!(!res1);
        assert_eq!(timeshift.snapshots_taken.len(), 0);

        // Level 5 Critical update triggers snapshot
        let res2 = timeshift.verify_and_schedule_pre_upgrade_snapshot("linux-kernel", MintUpdateLevel::Level5Critical, 1700000000).unwrap();
        assert!(res2);
        assert_eq!(timeshift.snapshots_taken.len(), 1);
        assert_eq!(timeshift.last_snapshot_timestamp, 1700000000);

        // Successive Critical update within 24 hours does NOT trigger snapshot (recent one exists)
        let res3 = timeshift.verify_and_schedule_pre_upgrade_snapshot("virtualbox-guest", MintUpdateLevel::Level5Critical, 1700010000).unwrap();
        assert!(!res3);
        assert_eq!(timeshift.snapshots_taken.len(), 1);

        // Critical update after 24 hours triggers snapshot
        let res4 = timeshift.verify_and_schedule_pre_upgrade_snapshot("systemd-sys", MintUpdateLevel::Level5Critical, 1700090000).unwrap();
        assert!(res4);
        assert_eq!(timeshift.snapshots_taken.len(), 2);
    }

    #[test]
    fn test_mint_system_fixer() {
        let mut fixer = MintSystemFixer::new();

        // Diagnose kernel driver crash
        let rem1 = fixer.diagnose_and_apply_remedy("nvidia-dkms").unwrap();
        assert!(rem1.contains("DKMS rebuild"));
        assert_eq!(fixer.applied_remedies[0], "nvidia-dkms:trigger-dkms-rebuild");

        // Diagnose xserver crash
        let rem2 = fixer.diagnose_and_apply_remedy("xorg-server").unwrap();
        assert!(rem2.contains("display environment"));
        assert_eq!(fixer.applied_remedies[1], "xorg-server:restart-display-manager");

        // Diagnose application corruption
        let rem3 = fixer.diagnose_and_apply_remedy("libreoffice-calc").unwrap();
        assert!(rem3.contains("reinstall"));
        assert_eq!(fixer.applied_remedies[2], "libreoffice-calc:reinstall-package");
    }
}
