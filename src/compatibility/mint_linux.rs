<<<<<<< HEAD
/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, and MintReport.

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
}
||||||| 43be3a7e8
=======
#![no_std]
#![no_main]

use core::mem;
/// Linux Mint-inspired User Experience Compatibility Suite for SigmaOS
/// Provides MintUpdate-style package ranking, MintBackup-style directory archiving,
/// MintSoftware-style community ratings, and Zenith dynamic compositor window arrangements.
use core::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MintError {
    Success = 0,
    BackupFailed = 1,
    UpdateError = 2,
    LayoutFailed = 3,
}

/// MintUpdate-style Package Update Classifications
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum MintUpdateLevel {
    Safe = 1,         // Tested, fully certified
    Recommended = 2,  // Safe, updates core submodules
    Normal = 3,       // Minor features, typical security
    Experimental = 4, // Needs user evaluation
    Dangerous = 5,    // Hard system kernel updates
}

#[derive(Debug, Clone, Copy)]
pub struct MintUpdateItem {
    pub package_name: [u8; 32],
    pub version: [u8; 16],
    pub level: MintUpdateLevel,
}

impl MintUpdateItem {
    pub fn new(package_name: &[u8], version: &[u8], level: MintUpdateLevel) -> Self {
        let mut pkg_array = [0u8; 32];
        let mut ver_array = [0u8; 16];
        let pkg_len = package_name.len().min(31);
        let ver_len = version.len().min(15);

        unsafe {
            core::ptr::copy_nonoverlapping(package_name.as_ptr(), pkg_array.as_mut_ptr(), pkg_len);
            core::ptr::copy_nonoverlapping(version.as_ptr(), ver_array.as_mut_ptr(), ver_len);
        }

        MintUpdateItem {
            package_name: pkg_array,
            version: ver_array,
            level,
        }
    }
}

/// MintUpdate-inspired Latency and Classification Manager
pub struct MintUpdateManager {
    pub updates: Vec<Option<MintUpdateItem>>,
    pub fastest_mirror_id: AtomicUsize,
}

impl MintUpdateManager {
    pub fn new() -> Self {
        MintUpdateManager {
            updates: Vec::new(),
            fastest_mirror_id: AtomicUsize::new(0),
        }
    }

    pub fn register_update(&mut self, item: MintUpdateItem) {
        self.updates.push(Some(item));
    }

    /// Simulate mirror ping latency sweep to select the fastest mirror
    pub fn select_fastest_mirror(&self, latencies_ms: &[usize]) -> usize {
        let mut min_latency = usize::MAX;
        let mut best_id = 0;

        for i in 0..latencies_ms.len() {
            if latencies_ms[i] < min_latency {
                min_latency = latencies_ms[i];
                best_id = i;
            }
        }

        self.fastest_mirror_id.store(best_id, Ordering::SeqCst);
        best_id
    }
}

/// MintBackup-inspired User Directory Archiver
pub struct MintBackupTool {
    pub backup_count: AtomicUsize,
}

impl MintBackupTool {
    pub fn new() -> Self {
        MintBackupTool {
            backup_count: AtomicUsize::new(0),
        }
    }

    /// Create archive snapshot of user directory bytes with basic CRC-32 checksum integrity
    pub fn archive_directory(
        &self,
        src_data: &[u8],
        archive: &mut [u8],
    ) -> Result<(usize, u32), MintError> {
        if src_data.len() > archive.len() {
            return Err(MintError::BackupFailed);
        }

        // Compute a mock checksum of directory bytes
        let mut crc = 0xFFFFFFFFu32;
        for i in 0..src_data.len() {
            crc ^= src_data[i] as u32;
            for _ in 0..8 {
                if (crc & 1) != 0 {
                    crc = (crc >> 1) ^ 0xEDB88320;
                } else {
                    crc >>= 1;
                }
            }
            archive[i] = src_data[i];
        }

        self.backup_count.fetch_add(1, Ordering::SeqCst);
        Ok((src_data.len(), !crc))
    }
}

/// MintSoftware-inspired package catalog rating reviews database
pub struct SoftwareMeta {
    pub name: [u8; 32],
    pub rating_stars: u32,
    pub reviews_count: u32,
}

pub struct MintSoftwareManager {
    pub software_list: Vec<Option<SoftwareMeta>>,
}

impl MintSoftwareManager {
    pub fn new() -> Self {
        MintSoftwareManager {
            software_list: Vec::new(),
        }
    }

    pub fn register_software(&mut self, name: &[u8], stars: u32, reviews: u32) {
        let mut name_array = [0u8; 32];
        let len = name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(name.as_ptr(), name_array.as_mut_ptr(), len);
        }
        self.software_list.push(Some(SoftwareMeta {
            name: name_array,
            rating_stars: stars,
            reviews_count: reviews,
        }));
    }
}

/// Zenith Dynamic Display Coordinator Layout representation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WindowCoordinates {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

pub struct ZenithDisplayCompositor;

impl ZenithDisplayCompositor {
    /// Arrange windows using a Master-and-Stack Binary Tiling Layout algorithm (Linux Mint Window Manager equivalent)
    pub fn arrange_tiling(
        screen_w: usize,
        screen_h: usize,
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), MintError> {
        if num_windows == 0 {
            return Ok(());
        }

        if coords.len() < num_windows {
            return Err(MintError::LayoutFailed);
        }

        if num_windows == 1 {
            coords[0] = WindowCoordinates {
                x: 0,
                y: 0,
                width: screen_w,
                height: screen_h,
            };
            return Ok(());
        }

        // Split screen: Master window on left (50%), Stack windows stacked on right (50%)
        let master_w = screen_w / 2;
        coords[0] = WindowCoordinates {
            x: 0,
            y: 0,
            width: master_w,
            height: screen_h,
        };

        let stack_count = num_windows - 1;
        let stack_h = screen_h / stack_count;

        for i in 1..num_windows {
            coords[i] = WindowCoordinates {
                x: master_w,
                y: (i - 1) * stack_h,
                width: screen_w - master_w,
                height: stack_h,
            };
        }

        Ok(())
    }

    /// Arrange windows using Stacking layout (Cascaded coordinations)
    pub fn arrange_stacking(
        num_windows: usize,
        coords: &mut [WindowCoordinates],
    ) -> Result<(), MintError> {
        for i in 0..num_windows {
            if i >= coords.len() {
                return Err(MintError::LayoutFailed);
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

struct Vec<T> {
    pub data: *mut T,
    pub len: usize,
    pub capacity: usize,
}

impl<T> Vec<T> {
    fn new() -> Self {
        Vec {
            data: core::ptr::null_mut(),
            len: 0,
            capacity: 0,
        }
    }
    fn push(&mut self, item: T) {
        unsafe {
            if self.len >= self.capacity {
                self.grow();
            }
            if self.capacity > self.len {
                core::ptr::write(self.data.add(self.len), item);
                self.len += 1;
            }
        }
    }
    unsafe fn grow(&mut self) {
        let new_capacity = if self.capacity == 0 {
            4
        } else {
            self.capacity * 2
        };
        let new_data = alloc(new_capacity * mem::size_of::<T>()) as *mut T;
        if !new_data.is_null() {
            for i in 0..self.len {
                core::ptr::copy_nonoverlapping(self.data.add(i), new_data.add(i), 1);
            }
            if self.capacity > 0 {
                free(self.data as *mut u8);
            }
            self.data = new_data;
            self.capacity = new_capacity;
        }
    }
}

impl<T> core::ops::Index<usize> for Vec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &*self.data.add(index) }
    }
}

impl<T> core::ops::IndexMut<usize> for Vec<T> {
    fn index_mut(&mut self, index: usize) -> &mut T {
        if index >= self.len {
            panic!("index out of bounds");
        }
        unsafe { &mut *self.data.add(index) }
    }
}

impl<T> Drop for Vec<T> {
    fn drop(&mut self) {
        if self.capacity > 0 {
            unsafe {
                for i in 0..self.len {
                    core::ptr::drop_in_place(self.data.add(i));
                }
                free(self.data as *mut u8);
            }
        }
    }
}

#[cfg(not(target_os = "none"))]
unsafe fn alloc(size: usize) -> *mut u8 {
    use std::alloc::{alloc as std_alloc, Layout};
    let layout = Layout::from_size_align(size, 8).unwrap();
    std_alloc(layout)
}

#[cfg(not(target_os = "none"))]
unsafe fn free(ptr: *mut u8) {
    let _ = ptr;
}

#[cfg(target_os = "none")]
extern "C" {
    fn alloc(size: usize) -> *mut u8;
    fn free(ptr: *mut u8);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_update_manager() {
        let mut manager = MintUpdateManager::new();
        manager.register_update(MintUpdateItem::new(
            b"kernel",
            b"1.0.1",
            MintUpdateLevel::Dangerous,
        ));
        manager.register_update(MintUpdateItem::new(
            b"firefox",
            b"120.0",
            MintUpdateLevel::Normal,
        ));

        assert_eq!(manager.updates.len, 2);

        let mut update_name = [0u8; 7];
        for i in 0..7 {
            update_name[i] = manager.updates[1].unwrap().package_name[i];
        }
        assert_eq!(&update_name, b"firefox");

        let latencies = [45, 12, 98, 150];
        let best_mirror = manager.select_fastest_mirror(&latencies);
        assert_eq!(best_mirror, 1);
    }

    #[test]
    fn test_mint_backup_checksum() {
        let src = b"Important User Profile Data";
        let mut archive = [0u8; 64];
        let tool = MintBackupTool::new();

        let (bytes, checksum) = tool.archive_directory(src, &mut archive).unwrap();
        assert_eq!(bytes, src.len());
        assert!(checksum > 0);
        assert_eq!(&archive[..bytes], src);
    }

    #[test]
    fn test_zenith_layouts() {
        let mut coords = [WindowCoordinates {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }; 3];
        assert!(ZenithDisplayCompositor::arrange_tiling(1920, 1080, 3, &mut coords).is_ok());

        // Master window takes left 50%
        assert_eq!(
            coords[0],
            WindowCoordinates {
                x: 0,
                y: 0,
                width: 960,
                height: 1080
            }
        );

        // Stack windows split right 50% horizontally
        assert_eq!(
            coords[1],
            WindowCoordinates {
                x: 960,
                y: 0,
                width: 960,
                height: 540
            }
        );
        assert_eq!(
            coords[2],
            WindowCoordinates {
                x: 960,
                y: 540,
                width: 960,
                height: 540
            }
        );

        // Test cascade stacking
        assert!(ZenithDisplayCompositor::arrange_stacking(3, &mut coords).is_ok());
        assert_eq!(
            coords[1],
            WindowCoordinates {
                x: 30,
                y: 30,
                width: 800,
                height: 600
            }
        );
    }
}
>>>>>>> origin/fix/mem-leak-custom-vec-drop-7188808108065826003
