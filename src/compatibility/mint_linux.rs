/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, MintReport, Timeshift-style System Restore,
/// Cinnamon-like desktop theme manager, and MintDrivers manager.

use core::sync::atomic::{AtomicUsize, Ordering};
#[cfg(not(feature = "standalone_test"))]
use crate::klib::Vec;

#[cfg(feature = "standalone_test")]
extern crate alloc;
#[cfg(feature = "standalone_test")]
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintError {
    LayoutFailed,
    UpdateError,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCoordinates {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct SoftwareMeta {
    pub name: [u8; 32],
    pub rating: usize,
}

#[derive(Debug, Clone)]
pub struct MintUpdateItem {
    pub name: [u8; 32],
    pub package_name: [u8; 32],
    pub version: [u8; 16],
    pub level: MintUpdateLevel,
}

#[derive(Debug, Clone)]
pub struct ZenithDisplayCompositor {
    pub active_layout: [u8; 32],
}

impl ZenithDisplayCompositor {
    pub fn new() -> Self {
        Self { active_layout: [0u8; 32] }
    }
}

impl Default for ZenithDisplayCompositor {
    fn default() -> Self {
        Self::new()
    }
}

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
        let mut n = [0u8; 32];
        let mut o = [0u8; 16];
        let mut v = [0u8; 16];
        n[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        o[..old.len().min(15)].copy_from_slice(&old[..old.len().min(15)]);
        v[..new.len().min(15)].copy_from_slice(&new[..new.len().min(15)]);

        let score = match level {
            MintUpdateLevel::Level1Safe => 100,
            MintUpdateLevel::Level2Tested => 90,
            MintUpdateLevel::Level3Normal => 75,
            MintUpdateLevel::Level4Sensitive => 50,
            MintUpdateLevel::Level5Critical => 25,
        };

        MintUpdatePackage {
            name: n,
            version_old: o,
            version_new: v,
            level,
            safety_score: score,
        }
    }
}

pub struct MintUpdateManager {
    pub available_updates: Vec<MintUpdatePackage>,
    pub selected_blacklists: Vec<[u8; 32]>,
}

impl Default for MintUpdateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateManager {
    pub fn new() -> Self {
        MintUpdateManager {
            available_updates: Vec::new(),
            selected_blacklists: Vec::new(),
        }
    }

    pub fn add_update(&mut self, pkg: MintUpdatePackage) {
        self.available_updates.push(pkg);
    }

    pub fn blacklist_package(&mut self, name: &[u8]) {
        let mut arr = [0u8; 32];
        arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        self.selected_blacklists.push(arr);
    }

    pub fn get_installable_updates(&self) -> Vec<MintUpdatePackage> {
        let mut list = Vec::new();
        for update in self.available_updates.iter() {
            let mut is_blacklisted = false;
            for bl in self.selected_blacklists.iter() {
                if update.name == *bl {
                    is_blacklisted = true;
                    break;
                }
            }
            if !is_blacklisted {
                list.push(update.clone());
            }
        }
        list
    }
}

#[derive(Debug, Clone)]
pub struct MintDriverInfo {
    pub name: [u8; 32],
    pub category: [u8; 32],
    pub is_proprietary: bool,
    pub active: bool,
}

impl MintDriverInfo {
    pub fn new(name: &[u8], category: &[u8], proprietary: bool) -> Self {
        let mut n = [0u8; 32];
        let mut c = [0u8; 32];
        n[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        c[..category.len().min(31)].copy_from_slice(&category[..category.len().min(31)]);

        MintDriverInfo {
            name: n,
            category: c,
            is_proprietary: proprietary,
            active: false,
        }
    }
}

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

    pub fn toggle_driver(&mut self, name: &[u8], enable: bool) -> Result<(), &'static str> {
        for drv in self.available_drivers.iter_mut() {
            if drv.name.starts_with(name) {
                drv.active = enable;
                return Ok(());
            }
        }
        Err("Driver not found")
    }
}

#[derive(Debug, Clone)]
pub struct AppReview {
    pub reviewer: [u8; 32],
    pub stars: usize, // 1 to 5
}

#[derive(Debug, Clone)]
pub struct MintAppMetadata {
    pub name: [u8; 32],
    pub category: [u8; 16],
    pub rating_stars: usize,
    pub reviews_count: usize,
    pub reviews: Vec<AppReview>,
}

impl MintAppMetadata {
    pub fn new(name: &[u8], category: &[u8]) -> Self {
        let mut n = [0u8; 32];
        let mut c = [0u8; 16];
        n[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        c[..category.len().min(15)].copy_from_slice(&category[..category.len().min(15)]);

        MintAppMetadata {
            name: n,
            category: c,
            rating_stars: 0,
            reviews_count: 0,
            reviews: Vec::new(),
        }
    }

    pub fn add_review(&mut self, reviewer: &[u8], stars: usize) {
        let mut r = [0u8; 32];
        r[..reviewer.len().min(31)].copy_from_slice(&reviewer[..reviewer.len().min(31)]);

        self.reviews.push(AppReview {
            reviewer: r,
            stars: stars.clamp(1, 5),
        });
        self.reviews_count += 1;

        let mut sum = 0;
        for rev in self.reviews.iter() {
            sum += rev.stars;
        }
        self.rating_stars = sum / self.reviews_count;
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

    pub fn search_by_category(&self, category: &[u8]) -> Vec<MintAppMetadata> {
        let mut filtered = Vec::new();
        for app in self.apps_catalog.iter() {
            let mut matches = true;
            for i in 0..category.len().min(15) {
                if app.category[i] != category[i] {
                    matches = false;
                    break;
                }
            }
            if matches {
                filtered.push(app.clone());
            }
        }
        filtered
    }

    pub fn get_featured_apps(&self) -> Vec<MintAppMetadata> {
        let mut sorted = self.apps_catalog.clone();
        let len = sorted.len();
        for i in 0..len {
            for j in 0..len.saturating_sub(i).saturating_sub(1) {
                if sorted[j].rating_stars < sorted[j + 1].rating_stars {
                    sorted.swap(j, j + 1);
                }
            }
        }
        sorted
    }

    pub fn arrange_stacking(
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), &'static str> {
        for i in 0..num_windows {
            if i >= coords.len() {
                return Err("Layout failed");
            }
            coords[i] = WindowCoordinates {
                x: i * 30,
                y: i * 30,
                width: 800,
                height: 600,
            };
        }
        Ok(())
    }
}

// ==========================================
// Cinnamon Desktop Theme Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CinnamonDesklet {
    pub id: u32,
    pub x: usize,
    pub y: usize,
}

pub struct CinnamonThemeEngine {
    pub active_gtk_theme: [u8; 32],
    pub desklets: Vec<Option<CinnamonDesklet>>,
    pub is_panel_enabled: bool,
}

impl CinnamonThemeEngine {
    pub fn new() -> Self {
        let mut theme = [0u8; 32];
        let default_name = b"Mint-Y-Dark";
        unsafe {
            core::ptr::copy_nonoverlapping(default_name.as_ptr(), theme.as_mut_ptr(), default_name.len());
        }
        Self {
            active_gtk_theme: theme,
            desklets: Vec::new(),
            is_panel_enabled: true,
        }
    }

    pub fn set_gtk_theme(&mut self, theme_name: &[u8]) {
        let mut theme = [0u8; 32];
        let len = theme_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(theme_name.as_ptr(), theme.as_mut_ptr(), len);
        }
        self.active_gtk_theme = theme;
    }

    pub fn add_desklet(&mut self, id: u32, x: usize, y: usize) {
        self.desklets.push(Some(CinnamonDesklet { id, x, y }));
    }
}

impl Default for CinnamonThemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ==========================================
// Timeshift-style System Restorer
// ==========================================

#[derive(Debug, Clone, Copy)]
pub struct SystemRestorePoint {
    pub id: u32,
    pub is_rsync: bool,
    pub timestamp_ms: u64,
}

pub struct TimeshiftSystemRestorer {
    pub restore_points: Vec<Option<SystemRestorePoint>>,
    pub active_restore_point_id: u32,
}

impl TimeshiftSystemRestorer {
    pub fn new() -> Self {
        Self {
            restore_points: Vec::new(),
            active_restore_point_id: 0,
        }
    }

    pub fn create_restore_point(&mut self, id: u32, is_rsync: bool) {
        self.restore_points.push(Some(SystemRestorePoint {
            id,
            is_rsync,
            timestamp_ms: 0,
        }));
    }

    pub fn rollback_system(&mut self, id: u32) -> Result<(), &'static str> {
        let mut found = false;
        for i in 0..self.restore_points.len() {
            if let Some(ref rp) = self.restore_points[i] {
                if rp.id == id {
                    found = true;
                    break;
                }
            }
        }
        if found {
            self.active_restore_point_id = id;
            Ok(())
        } else {
            Err("Update error")
        }
    }
}

impl Default for TimeshiftSystemRestorer {
    fn default() -> Self {
        Self::new()
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
    pub system_state_hash: u64,
}

impl TimeshiftSnapshot {
    pub fn new(id: usize, timestamp_epoch: u64, desc: &[u8], hash: u64) -> Self {
        let mut d = [0u8; 64];
        d[..desc.len().min(63)].copy_from_slice(&desc[..desc.len().min(63)]);
        TimeshiftSnapshot {
            id,
            timestamp_epoch,
            description: d,
            system_state_hash: hash,
        }
    }
}

pub struct MintTimeshiftEngine {
    pub snapshots: Vec<TimeshiftSnapshot>,
    pub next_id: AtomicUsize,
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
            next_id: AtomicUsize::new(1),
        }
    }

    pub fn create_checkpoint(&mut self, timestamp: u64, desc: &[u8], state_hash: u64) -> usize {
        let id = self.next_id.fetch_add(1, Ordering::SeqCst);
        let snap = TimeshiftSnapshot::new(id, timestamp, desc, state_hash);
        self.snapshots.push(snap);
        id
    }

    pub fn restore_checkpoint(&self, snapshot_id: usize) -> Option<u64> {
        for snap in self.snapshots.iter() {
            if snap.id == snapshot_id {
                return Some(snap.system_state_hash);
            }
        }
        None
    }
}

pub struct MintCinnamonStyling {
    pub panel_height: usize,
    pub window_effects_enabled: bool,
}

impl Default for MintCinnamonStyling {
    fn default() -> Self {
        Self {
            panel_height: 40,
            window_effects_enabled: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager_levels_and_blacklist() {
        let mut mgr = MintUpdateManager::new();

        let pkg1 = MintUpdatePackage::new(b"kernel", b"5.15", b"6.12", MintUpdateLevel::Level5Critical);
        let pkg2 = MintUpdatePackage::new(b"nano", b"5.0", b"6.0", MintUpdateLevel::Level1Safe);

        mgr.add_update(pkg1);
        mgr.add_update(pkg2);

        assert_eq!(mgr.get_installable_updates().len(), 2);

        mgr.blacklist_package(b"kernel");
        let filtered = mgr.get_installable_updates();
        assert_eq!(filtered.len(), 1);
        assert_eq!(&filtered[0].name[..4], b"nano");
    }

    #[test]
    fn test_mint_software_store_reviews_and_featured() {
        let mut store = MintSoftwareManager::new();

        let mut app1 = MintAppMetadata::new(b"GIMP", b"Graphics");
        app1.add_review(b"user1", 5);
        app1.add_review(b"user2", 4);

        let mut app2 = MintAppMetadata::new(b"VLC", b"Multimedia");
        app2.add_review(b"user3", 3);

        store.add_app_to_catalog(app1);
        store.add_app_to_catalog(app2);

        assert_eq!(store.search_by_category(b"Graphics").len(), 1);

        let featured = store.get_featured_apps();
        assert_eq!(&featured[0].name[..4], b"GIMP");
    }

    #[test]
    fn test_mint_report_crashes() {
        let mut report = MintReportSystem::new();
        report.register_crash_alert(b"Firefox");

        assert_eq!(report.active_alerts.len(), 1);
        assert_eq!(report.active_alerts[0].severity, MintReportAlertSeverity::Critical);
    }

    #[test]
    fn test_mint_timeshift_restore_points() {
        let mut timeshift = MintTimeshiftEngine::new();
        let snap_id = timeshift.create_checkpoint(1690000000, b"Fresh boot restore point", 0xDEADBEEF);
        assert_eq!(snap_id, 1);

        let hash = timeshift.restore_checkpoint(1).unwrap();
        assert_eq!(hash, 0xDEADBEEF);
    }

    #[test]
    fn test_mint_cinnamon_styling_options() {
        let style = MintCinnamonStyling::default();
        assert_eq!(style.panel_height, 40);
        assert!(style.window_effects_enabled);
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
