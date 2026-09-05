use std::format;
/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, MintReport, Timeshift-style System Restore,
/// Cinnamon-like desktop theme manager, and MintDrivers manager.
use std::string::{String, ToString};
use std::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

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
}

#[derive(Debug, Clone)]
pub struct MintUpdateItem {
    pub name: [u8; 32],
}

pub struct ZenithDisplayCompositor;

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

/// App review structure representing user feedback (GNOME Software / Google Play inspired)
#[derive(Debug, Clone)]
pub struct AppReview {
    pub reviewer: [u8; 32],
    pub stars: usize, // 1 to 5
    pub comment: [u8; 64],
}

impl AppReview {
    pub fn new(reviewer: &[u8], stars: usize, comment: &[u8]) -> Self {
        let mut reviewer_arr = [0u8; 32];
        let mut comment_arr = [0u8; 64];
        reviewer_arr[..reviewer.len().min(31)].copy_from_slice(&reviewer[..reviewer.len().min(31)]);
        comment_arr[..comment.len().min(63)].copy_from_slice(&comment[..comment.len().min(63)]);

        AppReview {
            reviewer: reviewer_arr,
            stars: stars.clamp(1, 5),
            comment: comment_arr,
        }
    }
}

/// MintInstall: High-level application software ratings, reviews, and categories metadata
#[derive(Debug, Clone)]
pub struct MintAppMetadata {
    pub name: [u8; 32],
    pub rating_stars: usize, // 1 to 5 (calculated as average of reviews)
    pub reviews_count: usize,
    pub is_flatpak: bool,
    pub category: [u8; 16], // e.g. "System", "Games", "Office"
    pub license: [u8; 16],  // e.g. "GPL-3.0", "MIT"
    pub size_bytes: u64,
    pub reviews: Vec<AppReview>,
}

impl MintAppMetadata {
    pub fn new(
        name: &[u8],
        category: &[u8],
        license: &[u8],
        size_bytes: u64,
        is_flatpak: bool,
    ) -> Self {
        let mut name_arr = [0u8; 32];
        let mut category_arr = [0u8; 16];
        let mut license_arr = [0u8; 16];
        name_arr[..name.len().min(31)].copy_from_slice(&name[..name.len().min(31)]);
        category_arr[..category.len().min(15)].copy_from_slice(&category[..category.len().min(15)]);
        license_arr[..license.len().min(15)].copy_from_slice(&license[..license.len().min(15)]);

        MintAppMetadata {
            name: name_arr,
            rating_stars: 5, // Default perfect score prior to reviews
            reviews_count: 0,
            is_flatpak,
            category: category_arr,
            license: license_arr,
            size_bytes,
            reviews: Vec::new(),
        }
    }

    /// Appends a new user rating/review dynamically and recalculates the average stars rating.
    pub fn add_review(&mut self, review: AppReview) {
        self.reviews.push(review);
        self.reviews_count = self.reviews.len();

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

    /// Arrange windows using Stacking layout (Cascaded coordinations)
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

    pub fn search_by_category(&self, category: &[u8]) -> Vec<MintAppMetadata> {
        let mut filtered = Vec::new();
        let cat_len = category.len();
        for app in self.apps_catalog.iter() {
            if app.category.len() < cat_len {
                continue;
            }
            let mut matches = true;
            for i in 0..category.len() {
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

    /// Returns apps ranked by user ratings (Featured Apps).
    pub fn get_featured_apps(&self) -> Vec<MintAppMetadata> {
        let mut sorted = self.apps_catalog.clone();
        // Simple bubble sort over vector to rank featured apps without external traits
        for i in 0..sorted.len() {
            for j in 0..sorted.len().saturating_sub(i).saturating_sub(1) {
                if sorted[j].rating_stars < sorted[j + 1].rating_stars {
                    sorted.swap(j, j + 1);
                }
            }
        }
        sorted
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

/// Cinnamon Theme Presets (Linux Mint Mint-Y, Mint-X, Yaru, Adapta)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CinnamonPreset {
    MintYDark,
    MintYLight,
    MintYAqua,
    MintYPurple,
    MintYTeal,
    MintXDefault,
    CinnamonAdwaita,
    CustomCinnamon,
}

/// Cinnamon Spices & Theme Preset Manager
pub struct CinnamonThemeEngine {
    pub active_gtk_theme: [u8; 32],
    pub current_preset: CinnamonPreset,
    pub desklets: Vec<Option<CinnamonDesklet>>,
    pub is_panel_enabled: bool,
    pub panel_transparency_alpha: u8, // 0 to 255
    pub applet_icon_theme: [u8; 32],
    pub sound_scheme_enabled: bool,
}

impl CinnamonThemeEngine {
    pub fn new() -> Self {
        let mut theme = [0u8; 32];
        let mut icon_theme = [0u8; 32];
        let default_name = b"Mint-Y-Dark";
        let default_icons = b"Mint-Y";
        unsafe {
            core::ptr::copy_nonoverlapping(default_name.as_ptr(), theme.as_mut_ptr(), default_name.len());
            core::ptr::copy_nonoverlapping(default_icons.as_ptr(), icon_theme.as_mut_ptr(), default_icons.len());
        }
        Self {
            active_gtk_theme: theme,
            current_preset: CinnamonPreset::MintYDark,
            desklets: Vec::new(),
            is_panel_enabled: true,
            panel_transparency_alpha: 230,
            applet_icon_theme: icon_theme,
            sound_scheme_enabled: true,
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

    pub fn apply_preset(&mut self, preset: CinnamonPreset) {
        self.current_preset = preset;
        let (gtk_name, icon_name): (&[u8], &[u8]) = match preset {
            CinnamonPreset::MintYDark => (b"Mint-Y-Dark", b"Mint-Y"),
            CinnamonPreset::MintYLight => (b"Mint-Y", b"Mint-Y"),
            CinnamonPreset::MintYAqua => (b"Mint-Y-Dark-Aqua", b"Mint-Y-Aqua"),
            CinnamonPreset::MintYPurple => (b"Mint-Y-Dark-Purple", b"Mint-Y-Purple"),
            CinnamonPreset::MintYTeal => (b"Mint-Y-Dark-Teal", b"Mint-Y-Teal"),
            CinnamonPreset::MintXDefault => (b"Mint-X", b"Mint-X"),
            CinnamonPreset::CinnamonAdwaita => (b"Adwaita-dark", b"Adwaita"),
            CinnamonPreset::CustomCinnamon => (b"Custom-Cinnamon", b"Custom-Icons"),
        };

        self.set_gtk_theme(gtk_name);
        let mut icon_arr = [0u8; 32];
        let len = icon_name.len().min(31);
        unsafe {
            core::ptr::copy_nonoverlapping(icon_name.as_ptr(), icon_arr.as_mut_ptr(), len);
        }
        self.applet_icon_theme = icon_arr;
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
// ==========================================
// Linux Mint mint4win & Wubi Loopback Windows Installer Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopbackDiskFormat {
    VhdFixed,
    VhdDynamic,
    RawNtfsImage,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowsBootloaderType {
    Ntldr,        // Windows XP / Server 2003
    BcdBootmgr,   // Windows Vista / 7 / 8 / 10 / 11
    BcdUefi,      // Windows UEFI BCD Bootloader
    Grub4Dos,     // Legacy MBR chainloader
    UefiEfiEntry, // UEFI NVRAM Boot Entry
}

#[derive(Debug, Clone)]
pub struct Mint4WinInstallationConfig {
    pub target_drive_letter: char, // e.g. 'C'
    pub target_folder: String,     // e.g. "C:\mint4win"
    pub disk_format: LoopbackDiskFormat,
    pub bootloader_type: WindowsBootloaderType,
    pub root_disk_size_mb: u64, // e.g. 32768 MB (32 GB)
    pub swap_file_size_mb: u64, // e.g. 4096 MB (4 GB)
    pub default_username: String,
    pub host_os_version: String,
}

impl Mint4WinInstallationConfig {
    pub fn default_windows_c(drive_letter: char, username: &str) -> Self {
        Mint4WinInstallationConfig {
            target_drive_letter: drive_letter,
            target_folder: format!("{}:\\mint4win", drive_letter),
            disk_format: LoopbackDiskFormat::RawNtfsImage,
            bootloader_type: WindowsBootloaderType::BcdBootmgr,
            root_disk_size_mb: 32768,
            swap_file_size_mb: 4096,
            default_username: username.to_string(),
            host_os_version: "Windows 11".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtfsFastStartupState {
    Clean,
    DirtyHibernated,
}

#[derive(Debug, Clone)]
pub struct Mint4WinConfig {
    pub target_drive_letter: char,
    pub install_folder: String,
    pub root_disk_size_mb: u64,
    pub swap_disk_size_mb: u64,
    pub username: String,
    pub language: String,
    pub bootloader_type: WindowsBootloaderType,
}

#[derive(Debug, Clone)]
pub struct VirtualDiskImage {
    pub windows_path: String,
    pub size_mb: u64,
}

#[derive(Debug, Clone)]
pub struct WindowsUninstallerEntry {
    pub key_path: String,
    pub uninstall_string: String,
}

pub struct Mint4WinInstaller {
    pub config: Mint4WinConfig,
    pub fast_startup_state: NtfsFastStartupState,
    pub root_disk: Option<VirtualDiskImage>,
    pub bcd_entry_guid: Option<String>,
}

impl Mint4WinInstaller {
    pub fn new(config: Mint4WinConfig) -> Self {
        Self {
            config,
            fast_startup_state: NtfsFastStartupState::Clean,
            root_disk: None,
            bcd_entry_guid: None,
        }
    }

    pub fn detect_ntfs_fast_startup(&mut self, is_hibernated: bool, _is_dirty: bool) -> NtfsFastStartupState {
        if is_hibernated {
            self.fast_startup_state = NtfsFastStartupState::DirtyHibernated;
        } else {
            self.fast_startup_state = NtfsFastStartupState::Clean;
        }
        self.fast_startup_state
    }

    pub fn create_loopback_disks(&mut self) -> Result<(), &'static str> {
        if self.fast_startup_state == NtfsFastStartupState::DirtyHibernated {
            return Err("Cannot install on hibernated NTFS partition");
        }
        self.root_disk = Some(VirtualDiskImage {
            windows_path: format!("C:\\{}\\disks\\root.disk", self.config.install_folder),
            size_mb: self.config.root_disk_size_mb,
        });
        Ok(())
    }

    pub fn register_windows_boot_entry(&mut self) -> String {
        self.bcd_entry_guid = Some("{a1b2c3d4-e5f6-7890-abcd-1234567890ab}".to_string());
        "bcdedit /create {guid} /d \"SigmaOS Linux Mint\"".to_string()
    }

    pub fn register_windows_uninstaller(&self) -> WindowsUninstallerEntry {
        WindowsUninstallerEntry {
            key_path: "HKLM\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\SigmaOS_mint4win".to_string(),
            uninstall_string: format!("C:\\{}\\uninstall.exe", self.config.install_folder),
        }
    }

    pub fn expand_root_disk(&mut self, additional_mb: u64) -> Result<u64, &'static str> {
        if let Some(ref mut disk) = self.root_disk {
            disk.size_mb += additional_mb;
            Ok(disk.size_mb)
        } else {
            Err("Root disk not created")
        }
    }

    pub fn execute_uninstallation(&mut self) -> Result<u64, &'static str> {
        let root_size = self.root_disk.as_ref().map(|d| d.size_mb).unwrap_or(0);
        let reclaimed = root_size + self.config.swap_disk_size_mb;
        self.root_disk = None;
        self.bcd_entry_guid = None;
        Ok(reclaimed)
    }
}

/// Linux Mint `mint4win` & Ubuntu `Wubi` inspired Windows Loopback Installer Engine
pub struct Mint4WinInstallerEngine {
    pub config: Mint4WinInstallationConfig,
    pub loopback_root_vhd_created: bool,
    pub bcd_entry_added: bool,
    pub installed: bool,
    pub bcd_guid: String,
}

impl Mint4WinInstallerEngine {
    pub fn new(config: Mint4WinInstallationConfig) -> Self {
        Mint4WinInstallerEngine {
            config,
            loopback_root_vhd_created: false,
            bcd_entry_added: false,
            installed: false,
            bcd_guid: String::from("{a1b2c3d4-e5f6-7890-abcd-1234567890ab}"),
        }
    }

    pub fn allocate_loopback_disks(&mut self) -> Result<String, &'static str> {
        if self.config.root_disk_size_mb < 8192 {
            return Err("mint4win: Minimum root disk size is 8192 MB (8 GB)");
        }

        self.loopback_root_vhd_created = true;
        Ok(format!(
            "Successfully allocated {} MB loopback root disk and {} MB swap file at {}",
            self.config.root_disk_size_mb, self.config.swap_file_size_mb, self.config.target_folder
        ))
    }

    pub fn configure_windows_bcd_boot_entry(&mut self) -> Result<String, &'static str> {
        if !self.loopback_root_vhd_created {
            return Err("mint4win: Must allocate loopback disk before configuring Windows BCD");
        }

        self.bcd_entry_added = true;
        self.installed = true;
        Ok(format!(
            "Added Windows BCD boot entry [{}] 'SigmaOS (Linux Mint Dual-Boot)'",
            self.bcd_guid
        ))
    }

    pub fn generate_unattended_install_script(&self) -> String {
        format!(
            "unattended_user=\"{}\"\nloopback_root=\"{}\\disks\\root.disk\"\nloopback_swap=\"{}\\disks\\swap.disk\"\nbootloader=\"{:?}\"\n",
            self.config.default_username,
            self.config.target_folder,
            self.config.target_folder,
            self.config.bootloader_type
        )
    }

    pub fn uninstall_mint4win(&mut self) -> Result<String, &'static str> {
        if !self.installed {
            return Err("mint4win is not installed");
        }

        self.bcd_entry_added = false;
        self.loopback_root_vhd_created = false;
        self.installed = false;

        Ok("Successfully removed mint4win Windows boot entry and loopback disk files".to_string())
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
    fn test_mint_software_manager_with_reviews() {
        let mut software = MintSoftwareManager::new();

        // 1. Create app metadata and add reviews
        let mut app1 = MintAppMetadata::new(b"alacritty", b"System", b"Apache-2.0", 4500000, true);
        app1.add_review(AppReview::new(b"gamer1", 5, b"Fast terminal!"));
        app1.add_review(AppReview::new(b"dev1", 3, b"Nice, but lacks tabs."));

        // Average should be (5 + 3) / 2 = 4 stars
        assert_eq!(app1.rating_stars, 4);
        assert_eq!(app1.reviews_count, 2);

        let mut app2 = MintAppMetadata::new(b"flipper", b"Games", b"GPL-3.0", 12000000, false);
        app2.add_review(AppReview::new(b"gamer2", 5, b"Pristine retro gameplay!"));

        software.add_app_to_catalog(app1);
        software.add_app_to_catalog(app2);

        // 2. Test Category Search
        let system_apps = software.search_by_category(b"System");
        assert_eq!(system_apps.len(), 1);
        assert!(system_apps[0].name.starts_with(b"alacritty"));

        // 3. Test Featured (Ranked) Apps
        let featured = software.get_featured_apps();
        assert_eq!(featured.len(), 2);
        assert!(featured[0].name.starts_with(b"flipper")); // 5 stars > 4 stars
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
        let snap_id =
            timeshift.create_checkpoint(1690000000, b"Fresh boot restore point", 0xDEADBEEF);
        assert_eq!(snap_id, 1);

        let hash = timeshift.restore_checkpoint(1).unwrap();
        assert_eq!(hash, 0xDEADBEEF);

        let mut engine = CinnamonThemeEngine::new();
        engine.add_desklet(101, 200, 200);
        assert_eq!(engine.desklets.len(), 1);
        assert_eq!(engine.desklets[0].unwrap().id, 101);
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

        // Test Cinnamon Theme Presets
        let mut cinnamon = CinnamonThemeEngine::new();
        assert_eq!(cinnamon.current_preset, CinnamonPreset::MintYDark);
        assert!(cinnamon.active_gtk_theme.starts_with(b"Mint-Y-Dark"));

        cinnamon.apply_preset(CinnamonPreset::MintYAqua);
        assert_eq!(cinnamon.current_preset, CinnamonPreset::MintYAqua);
        assert!(cinnamon.active_gtk_theme.starts_with(b"Mint-Y-Dark-Aqua"));
        assert!(cinnamon.applet_icon_theme.starts_with(b"Mint-Y-Aqua"));
    }

    #[test]
    fn test_mint_driver_manager_flows() {
        let mut drivers = MintDriverManager::new();
        let wifi_drv = MintDriverInfo::new(b"Broadcom BCM4360 WiFi", b"Wireless Controller", true);
        drivers.register_driver(wifi_drv);

        assert_eq!(drivers.available_drivers.len(), 1);
        assert!(!drivers.available_drivers[0].active);

        drivers
            .toggle_driver(b"Broadcom BCM4360 WiFi", true)
            .unwrap();
        assert!(drivers.available_drivers[0].active);
    }

    #[test]
    fn test_mint4win_installer_flow() {
        let config = Mint4WinConfig {
            target_drive_letter: 'C',
            install_folder: "sigmaos".to_string(),
            root_disk_size_mb: 32768,
            swap_disk_size_mb: 4096,
            username: "mintuser".to_string(),
            language: "en_US".to_string(),
            bootloader_type: WindowsBootloaderType::BcdUefi,
        };

        let mut installer = Mint4WinInstaller::new(config);

        // 1. Detect Fast Startup / Hibernation safety check
        assert_eq!(
            installer.detect_ntfs_fast_startup(true, false),
            NtfsFastStartupState::DirtyHibernated
        );

        // Attempting to create loopback disk on hibernated NTFS fails for safety
        assert!(installer.create_loopback_disks().is_err());

        // Clear fast startup hibernation block
        installer.detect_ntfs_fast_startup(false, false);
        assert_eq!(installer.fast_startup_state, NtfsFastStartupState::Clean);

        // 2. Allocate sparse loopback virtual disk images
        assert!(installer.create_loopback_disks().is_ok());
        let root_disk = installer.root_disk.as_ref().unwrap();
        assert_eq!(root_disk.windows_path, "C:\\sigmaos\\disks\\root.disk");
        assert_eq!(root_disk.size_mb, 32768);

        // 3. Register BCD boot entry
        let bcd_cmd = installer.register_windows_boot_entry();
        assert!(bcd_cmd.contains("bcdedit /create"));
        assert!(installer.bcd_entry_guid.is_some());

        // 4. Register Windows Control Panel Uninstaller entry
        let uninst = installer.register_windows_uninstaller();
        assert!(uninst.key_path.contains("SigmaOS_mint4win"));
        assert!(uninst
            .uninstall_string
            .contains("C:\\sigmaos\\uninstall.exe"));

        // 5. Expand root loopback disk capacity dynamically
        let new_size = installer.expand_root_disk(16384).unwrap();
        assert_eq!(new_size, 49152);

        // 6. Execute uninstallation and reclaim disk space
        let reclaimed = installer.execute_uninstallation().unwrap();
        assert_eq!(reclaimed, 53248); // 49152 + 4096 swap
        assert!(installer.root_disk.is_none());
    }
}
