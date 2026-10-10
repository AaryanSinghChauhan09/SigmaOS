use core::sync::atomic::{AtomicUsize, Ordering};
use std::format;
/// Linux Mint (MintTools) Compatibility and UI Subsystem Layer for SigmaOS
/// Replicates the signature user-friendly systems from Linux Mint:
/// MintBackup, MintUpdate, MintInstall, MintReport, Timeshift-style System Restore,
/// Cinnamon-like desktop theme manager, and MintDrivers manager.
use std::string::{String, ToString};
use std::vec::Vec;

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
            core::ptr::copy_nonoverlapping(
                default_name.as_ptr(),
                theme.as_mut_ptr(),
                default_name.len(),
            );
            core::ptr::copy_nonoverlapping(
                default_icons.as_ptr(),
                icon_theme.as_mut_ptr(),
                default_icons.len(),
            );
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

// ==========================================
// Linux Mint Parity Subsystems
// ==========================================

/// Sticky Notes Utility (Sticky / Sticky Notes)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StickyNoteColor {
    Yellow,
    Blue,
    Green,
    Pink,
    Purple,
    Orange,
}

#[derive(Debug, Clone)]
pub struct StickyNote {
    pub id: u32,
    pub title: String,
    pub body: String,
    pub color: StickyNoteColor,
    pub is_pinned: bool,
    pub x: usize,
    pub y: usize,
}

pub struct MintStickyNotesManager {
    pub notes: Vec<StickyNote>,
    next_id: u32,
}

impl Default for MintStickyNotesManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintStickyNotesManager {
    pub fn new() -> Self {
        Self {
            notes: Vec::new(),
            next_id: 1,
        }
    }

    pub fn create_note(&mut self, title: &str, body: &str, color: StickyNoteColor) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.notes.push(StickyNote {
            id,
            title: title.to_string(),
            body: body.to_string(),
            color,
            is_pinned: false,
            x: 100,
            y: 100,
        });
        id
    }

    pub fn toggle_pin(&mut self, id: u32) -> Result<bool, &'static str> {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.is_pinned = !note.is_pinned;
            Ok(note.is_pinned)
        } else {
            Err("Note not found")
        }
    }

    pub fn update_body(&mut self, id: u32, new_body: &str) -> Result<(), &'static str> {
        if let Some(note) = self.notes.iter_mut().find(|n| n.id == id) {
            note.body = new_body.to_string();
            Ok(())
        } else {
            Err("Note not found")
        }
    }
}

/// Web Apps Manager (webapp-manager) - Site-Specific Browser (SSB) launcher
#[derive(Debug, Clone)]
pub struct WebAppProfile {
    pub id: u32,
    pub name: String,
    pub url: String,
    pub icon_path: String,
    pub category: String,
    pub browser_profile: String,
    pub custom_user_agent: Option<String>,
    pub isolated_cookies: bool,
}

pub struct MintWebAppManager {
    pub web_apps: Vec<WebAppProfile>,
    next_id: u32,
}

impl Default for MintWebAppManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWebAppManager {
    pub fn new() -> Self {
        Self {
            web_apps: Vec::new(),
            next_id: 1,
        }
    }

    pub fn register_webapp(
        &mut self,
        name: &str,
        url: &str,
        category: &str,
        icon: &str,
        isolated: bool,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.web_apps.push(WebAppProfile {
            id,
            name: name.to_string(),
            url: url.to_string(),
            icon_path: icon.to_string(),
            category: category.to_string(),
            browser_profile: format!("profile_{}", name.to_lowercase().replace(' ', "_")),
            custom_user_agent: None,
            isolated_cookies: isolated,
        });
        id
    }

    pub fn set_custom_user_agent(&mut self, id: u32, user_agent: &str) -> Result<(), &'static str> {
        if let Some(app) = self.web_apps.iter_mut().find(|a| a.id == id) {
            app.custom_user_agent = Some(user_agent.to_string());
            Ok(())
        } else {
            Err("WebApp not found")
        }
    }

    pub fn launch_webapp_command(&self, id: u32) -> Result<String, &'static str> {
        if let Some(app) = self.web_apps.iter().find(|a| a.id == id) {
            Ok(format!(
                "zenith-browser --app=\"{}\" --profile=\"{}\" --isolated={}",
                app.url, app.browser_profile, app.isolated_cookies
            ))
        } else {
            Err("WebApp not found")
        }
    }
}

/// Hypnotix IPTV / Media Player Engine
#[derive(Debug, Clone)]
pub struct IptvChannel {
    pub name: String,
    pub stream_url: String,
    pub country_code: String,
    pub category: String,
    pub quality_720p_or_higher: bool,
}

pub struct MintHypnotixIptvEngine {
    pub channels: Vec<IptvChannel>,
    pub active_channel_index: Option<usize>,
    pub user_favorites: Vec<String>,
}

impl Default for MintHypnotixIptvEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintHypnotixIptvEngine {
    pub fn new() -> Self {
        Self {
            channels: Vec::new(),
            active_channel_index: None,
            user_favorites: Vec::new(),
        }
    }

    pub fn import_m3u_playlist(&mut self, playlist_content: &str) -> usize {
        let mut added = 0;
        for line in playlist_content.lines() {
            if line.contains("http://") || line.contains("https://") {
                self.channels.push(IptvChannel {
                    name: format!("Stream {}", self.channels.len() + 1),
                    stream_url: line.trim().to_string(),
                    country_code: "INT".to_string(),
                    category: "General".to_string(),
                    quality_720p_or_higher: true,
                });
                added += 1;
            }
        }
        added
    }

    pub fn add_channel(&mut self, name: &str, url: &str, country: &str, category: &str, hd: bool) {
        self.channels.push(IptvChannel {
            name: name.to_string(),
            stream_url: url.to_string(),
            country_code: country.to_string(),
            category: category.to_string(),
            quality_720p_or_higher: hd,
        });
    }

    pub fn play_channel(&mut self, name: &str) -> Result<String, &'static str> {
        if let Some((idx, ch)) = self
            .channels
            .iter()
            .enumerate()
            .find(|(_, c)| c.name == name)
        {
            self.active_channel_index = Some(idx);
            Ok(format!("Playing stream from {}", ch.stream_url))
        } else {
            Err("Channel not found")
        }
    }

    pub fn filter_by_country(&self, country: &str) -> Vec<&IptvChannel> {
        self.channels
            .iter()
            .filter(|c| c.country_code == country)
            .collect()
    }
}

/// MintWelcome - First-Run Desktop Setup & Onboarding Wizard
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopLayoutPreset {
    Traditional, // Panel at bottom with menu
    Modern,      // Centered panel with app grid
    Compact,     // Minimal panel
}

pub struct MintWelcomeWizard {
    pub layout: DesktopLayoutPreset,
    pub dark_mode: bool,
    pub initial_timeshift_done: bool,
    pub drivers_checked: bool,
    pub updates_checked: bool,
    pub firewall_enabled: bool,
}

impl Default for MintWelcomeWizard {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWelcomeWizard {
    pub fn new() -> Self {
        Self {
            layout: DesktopLayoutPreset::Traditional,
            dark_mode: true,
            initial_timeshift_done: false,
            drivers_checked: false,
            updates_checked: false,
            firewall_enabled: true,
        }
    }

    pub fn set_desktop_layout(&mut self, layout: DesktopLayoutPreset) {
        self.layout = layout;
    }

    pub fn toggle_dark_mode(&mut self, enabled: bool) {
        self.dark_mode = enabled;
    }

    pub fn mark_timeshift_completed(&mut self) {
        self.initial_timeshift_done = true;
    }

    pub fn mark_drivers_reviewed(&mut self) {
        self.drivers_checked = true;
    }

    pub fn mark_updates_checked(&mut self) {
        self.updates_checked = true;
    }

    pub fn is_setup_completed(&self) -> bool {
        self.initial_timeshift_done && self.drivers_checked && self.updates_checked
    }
}

/// MintNanny - Domain & Parental Controls Manager
#[derive(Debug, Clone)]
pub struct BlockedDomainRule {
    pub domain: String,
    pub reason: String,
    pub block_time_start_hour: u8, // 0 to 23
    pub block_time_end_hour: u8,   // 0 to 23
}

pub struct MintNannyDomainFilter {
    pub blocked_rules: Vec<BlockedDomainRule>,
    pub filter_active: bool,
}

impl Default for MintNannyDomainFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl MintNannyDomainFilter {
    pub fn new() -> Self {
        Self {
            blocked_rules: Vec::new(),
            filter_active: true,
        }
    }

    pub fn add_blocked_domain(&mut self, domain: &str, reason: &str) {
        self.blocked_rules.push(BlockedDomainRule {
            domain: domain.to_string(),
            reason: reason.to_string(),
            block_time_start_hour: 0,
            block_time_end_hour: 24,
        });
    }

    pub fn is_domain_blocked(&self, domain: &str, current_hour: u8) -> bool {
        if !self.filter_active {
            return false;
        }
        for rule in &self.blocked_rules {
            if domain.contains(&rule.domain) {
                if current_hour >= rule.block_time_start_hour
                    && current_hour < rule.block_time_end_hour
                {
                    return true;
                }
            }
        }
        false
    }
}

/// MintLocale & Language Selector
#[derive(Debug, Clone)]
pub struct LanguagePack {
    pub locale_code: String, // e.g. "en_US.UTF-8", "fr_FR.UTF-8", "hi_IN.UTF-8"
    pub name: String,
    pub has_spellcheck: bool,
    pub input_method: String, // e.g. "IBus", "Fcitx5"
}

pub struct MintLocaleManager {
    pub installed_languages: Vec<LanguagePack>,
    pub active_locale: String,
}

impl Default for MintLocaleManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintLocaleManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            installed_languages: Vec::new(),
            active_locale: String::from("en_US.UTF-8"),
        };
        mgr.installed_languages.push(LanguagePack {
            locale_code: String::from("en_US.UTF-8"),
            name: String::from("English (United States)"),
            has_spellcheck: true,
            input_method: String::from("IBus"),
        });
        mgr
    }

    pub fn install_language_pack(&mut self, pack: LanguagePack) {
        self.installed_languages.push(pack);
    }

    pub fn switch_active_locale(&mut self, locale_code: &str) -> Result<(), &'static str> {
        if self
            .installed_languages
            .iter()
            .any(|l| l.locale_code == locale_code)
        {
            self.active_locale = locale_code.to_string();
            Ok(())
        } else {
            Err("Language pack not installed")
        }
    }
}

/// MintUsbStickUtilities - USB Image Writer & Formatting Tool
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UsbFileSystemFormat {
    Fat32,
    Ntfs,
    ExFat,
    Ext4,
}

pub struct MintUsbFormatterTool {
    pub target_device_path: String,
    pub is_busy: bool,
    pub format_progress_percent: u8,
}

impl Default for MintUsbFormatterTool {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUsbFormatterTool {
    pub fn new() -> Self {
        Self {
            target_device_path: String::new(),
            is_busy: false,
            format_progress_percent: 0,
        }
    }

    pub fn select_device(&mut self, device_path: &str) -> Result<(), &'static str> {
        if device_path.starts_with("/dev/sd")
            || device_path.starts_with("/dev/nvme")
            || device_path.starts_with("/dev/mmc")
        {
            self.target_device_path = device_path.to_string();
            Ok(())
        } else {
            Err("Invalid storage device path")
        }
    }

    pub fn format_filesystem(
        &mut self,
        format_type: UsbFileSystemFormat,
        label: &str,
    ) -> Result<String, &'static str> {
        if self.target_device_path.is_empty() {
            return Err("No device selected");
        }
        self.is_busy = true;
        self.format_progress_percent = 100;
        self.is_busy = false;
        Ok(format!(
            "Successfully formatted {} to {:?} with label '{}'",
            self.target_device_path, format_type, label
        ))
    }

    pub fn write_iso_image(&mut self, iso_path: &str) -> Result<String, &'static str> {
        if self.target_device_path.is_empty() {
            return Err("No device selected");
        }
        if iso_path.is_empty() {
            return Err("ISO path invalid");
        }
        Ok(format!(
            "Successfully wrote ISO image '{}' to device {}",
            iso_path, self.target_device_path
        ))
    }
}

/// MintSoftwareSources & PPA Manager
#[derive(Debug, Clone)]
pub struct PpaRepository {
    pub ppa_name: String, // e.g. "ppa:cinnamon/stable"
    pub gpg_key_fingerprint: String,
    pub enabled: bool,
}

pub struct MintSoftwareSourcesPpaManager {
    pub ppas: Vec<PpaRepository>,
    pub main_mirror_url: String,
    pub base_mirror_url: String,
}

impl Default for MintSoftwareSourcesPpaManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoftwareSourcesPpaManager {
    pub fn new() -> Self {
        Self {
            ppas: Vec::new(),
            main_mirror_url: String::from("https://packages.linuxmint.com"),
            base_mirror_url: String::from("https://archive.ubuntu.com/ubuntu"),
        }
    }

    pub fn add_ppa(&mut self, ppa: &str, fingerprint: &str) {
        self.ppas.push(PpaRepository {
            ppa_name: ppa.to_string(),
            gpg_key_fingerprint: fingerprint.to_string(),
            enabled: true,
        });
    }

    pub fn remove_ppa(&mut self, ppa: &str) -> Result<(), &'static str> {
        let initial_len = self.ppas.len();
        self.ppas.retain(|p| p.ppa_name != ppa);
        if self.ppas.len() < initial_len {
            Ok(())
        } else {
            Err("PPA not found")
        }
    }
}

/// Cinnamon Spices Suite & Applet Manager
#[derive(Debug, Clone)]
pub struct CinnamonSpiceApplet {
    pub uuid: String,
    pub name: String,
    pub enabled: bool,
    pub panel_index: usize,
}

pub struct MintCinnamonSpicesAppletManager {
    pub applets: Vec<CinnamonSpiceApplet>,
}

impl Default for MintCinnamonSpicesAppletManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonSpicesAppletManager {
    pub fn new() -> Self {
        Self {
            applets: Vec::new(),
        }
    }

    pub fn add_applet(&mut self, uuid: &str, name: &str, panel_index: usize) {
        self.applets.push(CinnamonSpiceApplet {
            uuid: uuid.to_string(),
            name: name.to_string(),
            enabled: true,
            panel_index,
        });
    }

    pub fn set_applet_state(&mut self, uuid: &str, enabled: bool) -> Result<(), &'static str> {
        if let Some(applet) = self.applets.iter_mut().find(|a| a.uuid == uuid) {
            applet.enabled = enabled;
            Ok(())
        } else {
            Err("Applet UUID not found")
        }
    }
}

// ==========================================
// Warpinator Local Network File Transfer Engine
// ==========================================

#[derive(Debug, Clone)]
pub struct WarpinatorPeer {
    pub uuid: String,
    pub hostname: String,
    pub ip_address: String,
    pub port: u16,
    pub pin_code: String,
    pub trusted: bool,
}

#[derive(Debug, Clone)]
pub struct WarpinatorTransferItem {
    pub file_name: String,
    pub file_size_bytes: u64,
    pub sender_uuid: String,
    pub status: String,
}

pub struct MintWarpinatorFileTransferEngine {
    pub discovered_peers: Vec<WarpinatorPeer>,
    pub transfer_queue: Vec<WarpinatorTransferItem>,
    pub group_code: String,
}

impl Default for MintWarpinatorFileTransferEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWarpinatorFileTransferEngine {
    pub fn new() -> Self {
        Self {
            discovered_peers: Vec::new(),
            transfer_queue: Vec::new(),
            group_code: String::from("WarpinatorSigmaOS"),
        }
    }

    pub fn discover_peer(&mut self, hostname: &str, ip: &str, port: u16, pin: &str) -> String {
        let uuid = format!("peer_{}", self.discovered_peers.len() + 1);
        self.discovered_peers.push(WarpinatorPeer {
            uuid: uuid.clone(),
            hostname: hostname.to_string(),
            ip_address: ip.to_string(),
            port,
            pin_code: pin.to_string(),
            trusted: false,
        });
        uuid
    }

    pub fn pair_peer(&mut self, uuid: &str, pin: &str) -> Result<(), &'static str> {
        if let Some(peer) = self.discovered_peers.iter_mut().find(|p| p.uuid == uuid) {
            if peer.pin_code == pin {
                peer.trusted = true;
                Ok(())
            } else {
                Err("Warpinator: PIN code mismatch")
            }
        } else {
            Err("Warpinator: Peer not found")
        }
    }

    pub fn queue_file_transfer(
        &mut self,
        sender_uuid: &str,
        file_name: &str,
        size: u64,
    ) -> Result<(), &'static str> {
        if !self
            .discovered_peers
            .iter()
            .any(|p| p.uuid == sender_uuid && p.trusted)
        {
            return Err("Warpinator: Peer is not trusted or paired");
        }
        self.transfer_queue.push(WarpinatorTransferItem {
            file_name: file_name.to_string(),
            file_size_bytes: size,
            sender_uuid: sender_uuid.to_string(),
            status: String::from("Pending"),
        });
        Ok(())
    }
}

// ==========================================
// Bulky Batch File Renamer Engine
// ==========================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BulkyCaseMode {
    Lowercase,
    Uppercase,
    Titlecase,
    Unchanged,
}

pub struct MintBulkyBatchRenamerEngine;

impl MintBulkyBatchRenamerEngine {
    pub fn rename_find_replace(files: &[&str], search: &str, replace: &str) -> Vec<String> {
        files.iter().map(|f| f.replace(search, replace)).collect()
    }

    pub fn rename_add_prefix_suffix(files: &[&str], prefix: &str, suffix: &str) -> Vec<String> {
        files
            .iter()
            .map(|f| {
                if let Some(pos) = f.rfind('.') {
                    let stem = &f[..pos];
                    let ext = &f[pos..];
                    format!("{}{}{}{}", prefix, stem, suffix, ext)
                } else {
                    format!("{}{}{}", prefix, f, suffix)
                }
            })
            .collect()
    }

    pub fn rename_change_case(files: &[&str], mode: BulkyCaseMode) -> Vec<String> {
        files
            .iter()
            .map(|f| match mode {
                BulkyCaseMode::Lowercase => f.to_lowercase(),
                BulkyCaseMode::Uppercase => f.to_uppercase(),
                BulkyCaseMode::Titlecase => {
                    let mut chars = f.chars();
                    match chars.next() {
                        None => String::new(),
                        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                    }
                }
                BulkyCaseMode::Unchanged => f.to_string(),
            })
            .collect()
    }
}

// ==========================================
// XApps Status Icon & Tray System
// ==========================================

#[derive(Debug, Clone)]
pub struct XAppStatusIcon {
    pub id: u32,
    pub icon_name: String,
    pub tooltip: String,
    pub badge_count: u32,
    pub visible: bool,
}

pub struct MintXAppStatusIconTray {
    pub icons: Vec<XAppStatusIcon>,
    next_id: u32,
}

impl Default for MintXAppStatusIconTray {
    fn default() -> Self {
        Self::new()
    }
}

impl MintXAppStatusIconTray {
    pub fn new() -> Self {
        Self {
            icons: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_status_icon(&mut self, icon_name: &str, tooltip: &str) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.icons.push(XAppStatusIcon {
            id,
            icon_name: icon_name.to_string(),
            tooltip: tooltip.to_string(),
            badge_count: 0,
            visible: true,
        });
        id
    }

    pub fn set_badge_count(&mut self, id: u32, count: u32) -> Result<(), &'static str> {
        if let Some(icon) = self.icons.iter_mut().find(|i| i.id == id) {
            icon.badge_count = count;
            Ok(())
        } else {
            Err("XAppStatusIcon not found")
        }
    }
}

/// MintUpload FTP/SFTP Upload Profile Manager (mintupload)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UploadProtocol {
    Ftp,
    Sftp,
    Scp,
}

#[derive(Debug, Clone)]
pub struct UploadProfile {
    pub name: String,
    pub host: String,
    pub port: u16,
    pub protocol: UploadProtocol,
    pub remote_path: String,
    pub public_url_base: String,
}

pub struct MintUploadManager {
    pub profiles: Vec<UploadProfile>,
}

impl Default for MintUploadManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUploadManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
        }
    }

    pub fn add_profile(&mut self, name: &str, host: &str, port: u16, protocol: UploadProtocol, remote_path: &str, url_base: &str) {
        self.profiles.push(UploadProfile {
            name: name.to_string(),
            host: host.to_string(),
            port,
            protocol,
            remote_path: remote_path.to_string(),
            public_url_base: url_base.to_string(),
        });
    }

    pub fn generate_share_link(&self, profile_name: &str, filename: &str) -> Result<String, &'static str> {
        let prof = self.profiles.iter().find(|p| p.name == profile_name).ok_or("Upload profile not found")?;
        Ok(format!("{}/{}", prof.public_url_base.trim_end_matches('/'), filename))
    }
}

/// Mint Sudo & Privilege Escalation Helper (PAM / gksu replacement)
pub struct MintDigitKeyringPrompt {
    pub prompt_message: String,
    pub target_command: String,
    pub expected_hash: [u8; 16],
    pub is_authenticated: bool,
}

impl Default for MintDigitKeyringPrompt {
    fn default() -> Self {
        Self::new()
    }
}

impl MintDigitKeyringPrompt {
    pub fn new() -> Self {
        Self {
            prompt_message: String::from("Administrative authentication required"),
            target_command: String::new(),
            expected_hash: [0u8; 16],
            is_authenticated: false,
        }
    }

    pub fn set_expected_hash(&mut self, hash: [u8; 16]) {
        self.expected_hash = hash;
    }

    /// Constant-time authentication verification against registered expected hash
    pub fn request_auth(&mut self, command: &str, attempt_hash: &[u8; 16]) -> Result<bool, &'static str> {
        if command.is_empty() {
            return Err("Command path cannot be empty");
        }
        self.target_command = command.to_string();

        let mut diff = 0u8;
        for i in 0..16 {
            diff |= self.expected_hash[i] ^ attempt_hash[i];
        }

        if diff == 0 && self.expected_hash != [0u8; 16] {
            self.is_authenticated = true;
            Ok(true)
        } else {
            self.is_authenticated = false;
            Err("Authentication failed: invalid credentials")
        }
    }
}

/// Cinnamon Desktop Icon Layout & Grid Organizer
#[derive(Debug, Clone)]
pub struct DesktopIconLocation {
    pub file_name: String,
    pub grid_x: usize,
    pub grid_y: usize,
}

pub struct MintDesktopIconOrganizer {
    pub icons: Vec<DesktopIconLocation>,
    pub grid_size_px: usize,
}

impl Default for MintDesktopIconOrganizer {
    fn default() -> Self {
        Self::new()
    }
}

impl MintDesktopIconOrganizer {
    pub fn new() -> Self {
        Self {
            icons: Vec::new(),
            grid_size_px: 64,
        }
    }

    pub fn auto_arrange(&mut self, filenames: &[&str]) {
        self.icons.clear();
        for (i, &f) in filenames.iter().enumerate() {
            self.icons.push(DesktopIconLocation {
                file_name: f.to_string(),
                grid_x: 0,
                grid_y: i,
            });
        }
    }
}

/// XApps Thumbnailer Service for File Managers
pub struct MintXappsThumbnails {
    pub supported_extensions: Vec<String>,
}

impl Default for MintXappsThumbnails {
    fn default() -> Self {
        Self::new()
    }
}

impl MintXappsThumbnails {
    pub fn new() -> Self {
        Self {
            supported_extensions: vec![
                "pdf".to_string(),
                "epub".to_string(),
                "mp4".to_string(),
                "mkv".to_string(),
                "ttf".to_string(),
                "otf".to_string(),
            ],
        }
    }

    pub fn can_thumbnail(&self, filename: &str) -> bool {
        if let Some(ext) = filename.split('.').last() {
            self.supported_extensions.contains(&ext.to_lowercase())
        } else {
            false
        }
    }
}

/// Cinnamon Virtual Desktop & Workspace Switcher Manager
#[derive(Debug, Clone)]
pub struct WorkspaceDescriptor {
    pub index: usize,
    pub label: String,
    pub window_count: usize,
}

pub struct MintCinnamonWorkspaceManager {
    pub workspaces: Vec<WorkspaceDescriptor>,
    pub active_workspace_idx: usize,
}

impl Default for MintCinnamonWorkspaceManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonWorkspaceManager {
    pub fn new() -> Self {
        let mut mgr = Self {
            workspaces: Vec::new(),
            active_workspace_idx: 0,
        };
        mgr.workspaces.push(WorkspaceDescriptor {
            index: 0,
            label: String::from("Workspace 1"),
            window_count: 0,
        });
        mgr.workspaces.push(WorkspaceDescriptor {
            index: 1,
            label: String::from("Workspace 2"),
            window_count: 0,
        });
        mgr
    }

    pub fn add_workspace(&mut self, label: &str) -> usize {
        let idx = self.workspaces.len();
        self.workspaces.push(WorkspaceDescriptor {
            index: idx,
            label: label.to_string(),
            window_count: 0,
        });
        idx
    }

    pub fn switch_to_workspace(&mut self, index: usize) -> Result<(), &'static str> {
        if index < self.workspaces.len() {
            self.active_workspace_idx = index;
            Ok(())
        } else {
            Err("Workspace index out of bounds")
        }
    }
}

/// Linux Mint Desktop Event Sound Scheme Theme Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SoundEventType {
    Login,
    Logout,
    WindowClose,
    TrashEmpty,
    NotificationAlert,
}

pub struct MintSoundSchemeEngine {
    pub sound_events_enabled: bool,
    pub active_sound_theme: String,
    pub triggered_events: Vec<SoundEventType>,
}

impl Default for MintSoundSchemeEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoundSchemeEngine {
    pub fn new() -> Self {
        Self {
            sound_events_enabled: true,
            active_sound_theme: String::from("mint-y"),
            triggered_events: Vec::new(),
        }
    }

    pub fn play_sound_event(&mut self, event: SoundEventType) -> bool {
        if !self.sound_events_enabled {
            return false;
        }
        self.triggered_events.push(event);
        true
    }
}

/// Mint User Accounts & Group Administration Tool
#[derive(Debug, Clone)]
pub struct MintUserProfile {
    pub username: String,
    pub full_name: String,
    pub is_sudo_admin: bool,
    pub autologin_enabled: bool,
    pub avatar_icon_path: String,
}

pub struct MintUserAccountsManager {
    pub users: Vec<MintUserProfile>,
}

impl Default for MintUserAccountsManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUserAccountsManager {
    pub fn new() -> Self {
        let mut mgr = Self { users: Vec::new() };
        mgr.users.push(MintUserProfile {
            username: String::from("mintuser"),
            full_name: String::from("Linux Mint User"),
            is_sudo_admin: true,
            autologin_enabled: false,
            avatar_icon_path: String::from("/usr/share/pixmaps/faces/user.png"),
        });
        mgr
    }

    pub fn create_user(&mut self, username: &str, full_name: &str, admin: bool) {
        self.users.push(MintUserProfile {
            username: username.to_string(),
            full_name: full_name.to_string(),
            is_sudo_admin: admin,
            autologin_enabled: false,
            avatar_icon_path: String::from("/usr/share/pixmaps/faces/user.png"),
        });
    }

    pub fn set_autologin(&mut self, username: &str, enabled: bool) -> Result<(), &'static str> {
        let user = self.users.iter_mut().find(|u| u.username == username).ok_or("User not found")?;
        user.autologin_enabled = enabled;
        Ok(())
    }
}

/// Mint System Fixer & DKMS Driver Module Rebuilder
pub struct MintSystemFixerDkmsRebuilder {
    pub dkms_modules: Vec<String>,
    pub rebuilt_count: usize,
}

impl Default for MintSystemFixerDkmsRebuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSystemFixerDkmsRebuilder {
    pub fn new() -> Self {
        Self {
            dkms_modules: vec![
                String::from("nvidia-current"),
                String::from("broadcom-sta"),
                String::from("vboxhost"),
            ],
            rebuilt_count: 0,
        }
    }

    pub fn rebuild_all_dkms_modules(&mut self, _kernel_version: &str) -> usize {
        for _m in &self.dkms_modules {
            self.rebuilt_count += 1;
        }
        self.rebuilt_count
    }
}

/// Cinnamon Desktop Hot Corners Action Manager
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotCornerLocation {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HotCornerAction {
    ExpoWorkspaces,
    ScaleWindows,
    ShowDesktop,
    CustomCommand,
    Disabled,
}

#[derive(Debug, Clone)]
pub struct HotCornerConfig {
    pub location: HotCornerLocation,
    pub action: HotCornerAction,
    pub custom_cmd: String,
    pub hover_delay_ms: u32,
}

pub struct MintCinnamonHotCornerEngine {
    pub corners: Vec<HotCornerConfig>,
}

impl Default for MintCinnamonHotCornerEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonHotCornerEngine {
    pub fn new() -> Self {
        let mut engine = Self { corners: Vec::new() };
        engine.corners.push(HotCornerConfig {
            location: HotCornerLocation::TopLeft,
            action: HotCornerAction::ExpoWorkspaces,
            custom_cmd: String::new(),
            hover_delay_ms: 100,
        });
        engine
    }

    pub fn set_corner_action(&mut self, location: HotCornerLocation, action: HotCornerAction, cmd: &str) {
        if let Some(c) = self.corners.iter_mut().find(|c| c.location == location) {
            c.action = action;
            c.custom_cmd = cmd.to_string();
        } else {
            self.corners.push(HotCornerConfig {
                location,
                action,
                custom_cmd: cmd.to_string(),
                hover_delay_ms: 100,
            });
        }
    }

    pub fn trigger_corner(&self, location: HotCornerLocation) -> Option<HotCornerAction> {
        self.corners.iter().find(|c| c.location == location).map(|c| c.action)
    }
}

/// MintUpdate Kernel Version Pin & Hold Manager
pub struct MintUpdateKernelPinning {
    pub pinned_kernels: Vec<String>,
}

impl Default for MintUpdateKernelPinning {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateKernelPinning {
    pub fn new() -> Self {
        Self {
            pinned_kernels: Vec::new(),
        }
    }

    pub fn pin_kernel(&mut self, version: &str) {
        if !self.pinned_kernels.contains(&version.to_string()) {
            self.pinned_kernels.push(version.to_string());
        }
    }

    pub fn unpin_kernel(&mut self, version: &str) {
        self.pinned_kernels.retain(|v| v != version);
    }

    pub fn is_kernel_pinned(&self, version: &str) -> bool {
        self.pinned_kernels.contains(&version.to_string())
    }
}

/// MintInstall FlatpakRef Installer & Remote GPG Validator
#[derive(Debug, Clone)]
pub struct FlatpakRefDescriptor {
    pub name: String,
    pub branch: String,
    pub title: String,
    pub url: String,
    pub gpg_key_valid: bool,
}

pub struct MintInstallFlatpakRefFetcher {
    pub refs: Vec<FlatpakRefDescriptor>,
}

impl Default for MintInstallFlatpakRefFetcher {
    fn default() -> Self {
        Self::new()
    }
}

impl MintInstallFlatpakRefFetcher {
    pub fn new() -> Self {
        Self { refs: Vec::new() }
    }

    pub fn parse_flatpakref(&mut self, name: &str, branch: &str, title: &str, url: &str) -> Result<&FlatpakRefDescriptor, &'static str> {
        if url.is_empty() {
            return Err("Invalid FlatpakRef URL");
        }
        let desc = FlatpakRefDescriptor {
            name: name.to_string(),
            branch: branch.to_string(),
            title: title.to_string(),
            url: url.to_string(),
            gpg_key_valid: true,
        };
        self.refs.push(desc);
        Ok(self.refs.last().unwrap())
    }
}

/// Mint System Monitor Panel Applet HUD State Tracker
#[derive(Debug, Clone, Copy, Default)]
pub struct SystemResourcesTelemetry {
    pub cpu_usage_pct: f32,
    pub ram_usage_mb: u64,
    pub total_ram_mb: u64,
    pub net_rx_kbps: f32,
    pub net_tx_kbps: f32,
}

pub struct MintSystemMonitorHUD {
    pub telemetry: SystemResourcesTelemetry,
}

impl Default for MintSystemMonitorHUD {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSystemMonitorHUD {
    pub fn new() -> Self {
        Self {
            telemetry: SystemResourcesTelemetry {
                cpu_usage_pct: 12.5,
                ram_usage_mb: 2048,
                total_ram_mb: 16384,
                net_rx_kbps: 150.0,
                net_tx_kbps: 45.0,
            },
        }
    }

    pub fn update_telemetry(&mut self, cpu: f32, ram: u64, net_rx: f32, net_tx: f32) {
        self.telemetry.cpu_usage_pct = cpu;
        self.telemetry.ram_usage_mb = ram;
        self.telemetry.net_rx_kbps = net_rx;
        self.telemetry.net_tx_kbps = net_tx;
    }
}

/// Cinnamon System Tray & StatusNotifierItem (SNI) Applet Manager
#[derive(Debug, Clone)]
pub struct StatusNotifierItem {
    pub id: String,
    pub title: String,
    pub icon_name: String,
    pub is_visible: bool,
}

pub struct MintCinnamonAppletTrayEngine {
    pub items: Vec<StatusNotifierItem>,
}

impl Default for MintCinnamonAppletTrayEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonAppletTrayEngine {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn register_sni(&mut self, id: &str, title: &str, icon: &str) {
        self.items.push(StatusNotifierItem {
            id: id.to_string(),
            title: title.to_string(),
            icon_name: icon.to_string(),
            is_visible: true,
        });
    }

    pub fn get_visible_items(&self) -> Vec<&StatusNotifierItem> {
        self.items.iter().filter(|i| i.is_visible).collect()
    }
}

/// MintUpdate Repository Mirror Speed & Latency Benchmark Engine
#[derive(Debug, Clone)]
pub struct MirrorBenchmarkResult {
    pub mirror_url: String,
    pub latency_ms: u32,
    pub speed_kbps: u32,
    pub score: u32,
}

pub struct MintUpdateMirrorSpeedTester {
    pub benchmark_results: Vec<MirrorBenchmarkResult>,
}

impl Default for MintUpdateMirrorSpeedTester {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateMirrorSpeedTester {
    pub fn new() -> Self {
        Self {
            benchmark_results: Vec::new(),
        }
    }

    pub fn test_mirror(&mut self, url: &str, latency: u32, speed: u32) {
        let score = speed / (latency.max(1));
        self.benchmark_results.push(MirrorBenchmarkResult {
            mirror_url: url.to_string(),
            latency_ms: latency,
            speed_kbps: speed,
            score,
        });
    }

    pub fn get_fastest_mirror(&self) -> Option<&MirrorBenchmarkResult> {
        self.benchmark_results.iter().max_by_key(|m| m.score)
    }
}

/// MintInstall Category Taxonomy & Software Curator
#[derive(Debug, Clone)]
pub struct SoftwareCategoryInfo {
    pub category_id: String,
    pub display_name: String,
    pub icon_name: String,
    pub featured_app_ids: Vec<String>,
}

pub struct MintInstallPackageCategoriesCatalog {
    pub categories: Vec<SoftwareCategoryInfo>,
}

impl Default for MintInstallPackageCategoriesCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl MintInstallPackageCategoriesCatalog {
    pub fn new() -> Self {
        let mut catalog = Self {
            categories: Vec::new(),
        };
        catalog.categories.push(SoftwareCategoryInfo {
            category_id: String::from("accessories"),
            display_name: String::from("Accessories"),
            icon_name: String::from("applications-accessories"),
            featured_app_ids: vec![String::from("org.gnome.Calculator"), String::from("pix")],
        });
        catalog.categories.push(SoftwareCategoryInfo {
            category_id: String::from("internet"),
            display_name: String::from("Internet"),
            icon_name: String::from("applications-internet"),
            featured_app_ids: vec![String::from("firefox"), String::from("thunderbird")],
        });
        catalog
    }

    pub fn get_category(&self, id: &str) -> Option<&SoftwareCategoryInfo> {
        self.categories.iter().find(|c| c.category_id == id)
    }
}

/// Nemo File Manager Extension Engine (ColumnProvider & MenuProvider)
#[derive(Debug, Clone)]
pub struct NemoExtensionDescriptor {
    pub name: String,
    pub provides_columns: bool,
    pub provides_menus: bool,
    pub enabled: bool,
}

pub struct MintNemoExtensionManager {
    pub extensions: Vec<NemoExtensionDescriptor>,
}

impl Default for MintNemoExtensionManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintNemoExtensionManager {
    pub fn new() -> Self {
        Self {
            extensions: Vec::new(),
        }
    }

    pub fn register_extension(&mut self, name: &str, columns: bool, menus: bool) {
        self.extensions.push(NemoExtensionDescriptor {
            name: name.to_string(),
            provides_columns: columns,
            provides_menus: menus,
            enabled: true,
        });
    }

    pub fn get_menu_providers(&self) -> Vec<&NemoExtensionDescriptor> {
        self.extensions.iter().filter(|e| e.enabled && e.provides_menus).collect()
    }
}

/// Wacom Graphics Tablet Stylus & Button Mapping Support Manager (cinnamon-wacom)
#[derive(Debug, Clone)]
pub struct WacomStylusConfig {
    pub tablet_name: String,
    pub stylus_pressure_curve: [u8; 4],
    pub button_1_mapping: String,
    pub button_2_mapping: String,
    pub eraser_mode: bool,
}

pub struct MintWacomTabletSupportManager {
    pub devices: Vec<WacomStylusConfig>,
}

impl Default for MintWacomTabletSupportManager {
    fn default() -> Self {
        Self::new()
    }
}

impl MintWacomTabletSupportManager {
    pub fn new() -> Self {
        Self { devices: Vec::new() }
    }

    pub fn register_tablet(&mut self, name: &str, btn1: &str, btn2: &str) {
        self.devices.push(WacomStylusConfig {
            tablet_name: name.to_string(),
            stylus_pressure_curve: [0, 32, 192, 255],
            button_1_mapping: btn1.to_string(),
            button_2_mapping: btn2.to_string(),
            eraser_mode: false,
        });
    }

    pub fn get_config(&self, name: &str) -> Option<&WacomStylusConfig> {
        self.devices.iter().find(|d| d.tablet_name == name)
    }
}

/// Cinnamon Spices Theme & Spices Online Marketplace Catalog
#[derive(Debug, Clone)]
pub struct SpiceMarketplaceItem {
    pub spice_id: String, // e.g. "theme_mint_y_dark"
    pub name: String,
    pub author: String,
    pub rating: u8, // 1..5
    pub download_count: u32,
    pub is_installed: bool,
}

pub struct MintCinnamonThemeMarketplace {
    pub available_spices: Vec<SpiceMarketplaceItem>,
}

impl Default for MintCinnamonThemeMarketplace {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonThemeMarketplace {
    pub fn new() -> Self {
        let mut mp = Self {
            available_spices: Vec::new(),
        };
        mp.available_spices.push(SpiceMarketplaceItem {
            spice_id: String::from("cinnamon-theme-adapta-nokto"),
            name: String::from("Adapta Nokto"),
            author: String::from("Adapta Project"),
            rating: 5,
            download_count: 142000,
            is_installed: false,
        });
        mp
    }

    pub fn install_spice(&mut self, spice_id: &str) -> Result<String, &'static str> {
        if let Some(item) = self.available_spices.iter_mut().find(|s| s.spice_id == spice_id) {
            item.is_installed = true;
            item.download_count += 1;
            Ok(format!("Successfully installed Cinnamon Spice '{}'", item.name))
        } else {
            Err("Spice item not found in marketplace")
        }
    }
}

/// Linux Mint Community Hardware Compatibility Database
#[derive(Debug, Clone)]
pub struct HardwareCompatReport {
    pub device_modalias: String,
    pub hardware_name: String,
    pub kernel_driver: String,
    pub rating_stars: u8, // 1..5
    pub notes: String,
}

pub struct MintCommunityHardwareDatabase {
    pub reports: Vec<HardwareCompatReport>,
}

impl Default for MintCommunityHardwareDatabase {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCommunityHardwareDatabase {
    pub fn new() -> Self {
        Self { reports: Vec::new() }
    }

    pub fn submit_report(&mut self, modalias: &str, name: &str, driver: &str, rating: u8, notes: &str) {
        self.reports.push(HardwareCompatReport {
            device_modalias: modalias.to_string(),
            hardware_name: name.to_string(),
            kernel_driver: driver.to_string(),
            rating_stars: rating.clamp(1, 5),
            notes: notes.to_string(),
        });
    }

    pub fn lookup_driver(&self, modalias: &str) -> Option<&HardwareCompatReport> {
        self.reports.iter().find(|r| r.device_modalias == modalias)
    }
}

/// MintReport System Crash Debugger & Stacktrace Analyzer (mintreport / ABRT parity)
#[derive(Debug, Clone)]
pub struct CrashStackframe {
    pub frame_index: usize,
    pub function_name: String,
    pub file_path: String,
    pub line_number: u32,
}

#[derive(Debug, Clone)]
pub struct CrashReport {
    pub crash_id: u32,
    pub executable_name: String,
    pub signal_number: u32, // e.g. SIGSEGV=11
    pub stacktrace: Vec<CrashStackframe>,
}

pub struct MintAdvancedDebuggingSuite {
    pub crash_reports: Vec<CrashReport>,
    pub next_crash_id: u32,
}

impl Default for MintAdvancedDebuggingSuite {
    fn default() -> Self {
        Self::new()
    }
}

impl MintAdvancedDebuggingSuite {
    pub fn new() -> Self {
        Self {
            crash_reports: Vec::new(),
            next_crash_id: 1,
        }
    }

    pub fn record_crash(&mut self, app: &str, signal: u32, frames: &[(&str, &str, u32)]) -> u32 {
        let id = self.next_crash_id;
        self.next_crash_id += 1;
        let stacktrace = frames
            .iter()
            .enumerate()
            .map(|(i, &(fn_name, file, line))| CrashStackframe {
                frame_index: i,
                function_name: fn_name.to_string(),
                file_path: file.to_string(),
                line_number: line,
            })
            .collect();

        self.crash_reports.push(CrashReport {
            crash_id: id,
            executable_name: app.to_string(),
            signal_number: signal,
            stacktrace,
        });

        id
    }

    pub fn analyze_crash(&self, crash_id: u32) -> Result<String, &'static str> {
        let report = self.crash_reports.iter().find(|c| c.crash_id == crash_id).ok_or("Crash report not found")?;
        Ok(format!(
            "Crash #{} in {} (signal {}): {} frames in backtrace",
            report.crash_id, report.executable_name, report.signal_number, report.stacktrace.len()
        ))
    }
}

/// Linux Mint / Calamares Partitioning & Filesystem Formatting Engine
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartitionTargetFs {
    Ext4,
    Btrfs,
    Fat32,
    LinuxSwap,
}

#[derive(Debug, Clone)]
pub struct PartitionSpec {
    pub device_path: String,
    pub partition_number: u32,
    pub size_mb: u64,
    pub filesystem: PartitionTargetFs,
    pub mount_point: String,
    pub is_formatted: bool,
}

pub struct MintInstallerPartitioningEngine {
    pub partitions: Vec<PartitionSpec>,
    pub efi_system_partition_index: Option<usize>,
}

impl Default for MintInstallerPartitioningEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintInstallerPartitioningEngine {
    pub fn new() -> Self {
        Self {
            partitions: Vec::new(),
            efi_system_partition_index: None,
        }
    }

    pub fn add_partition(&mut self, dev: &str, num: u32, size_mb: u64, fs: PartitionTargetFs, mount: &str) -> usize {
        let idx = self.partitions.len();
        if fs == PartitionTargetFs::Fat32 && mount == "/boot/efi" {
            self.efi_system_partition_index = Some(idx);
        }
        self.partitions.push(PartitionSpec {
            device_path: dev.to_string(),
            partition_number: num,
            size_mb,
            filesystem: fs,
            mount_point: mount.to_string(),
            is_formatted: false,
        });
        idx
    }

    pub fn format_all_partitions(&mut self) -> Result<usize, &'static str> {
        if self.partitions.is_empty() {
            return Err("No partitions configured for formatting");
        }
        let mut count = 0;
        for part in self.partitions.iter_mut() {
            part.is_formatted = true;
            count += 1;
        }
        Ok(count)
    }
}

/// Linux Mint `mintinstall` Package Transaction & GPG Signature Verifier
pub struct MintSoftwareManagerTransactionVerifier {
    pub trusted_gpg_fingerprints: Vec<String>,
    pub verified_transactions_count: usize,
}

impl Default for MintSoftwareManagerTransactionVerifier {
    fn default() -> Self {
        Self::new()
    }
}

impl MintSoftwareManagerTransactionVerifier {
    pub fn new() -> Self {
        let mut verifier = Self {
            trusted_gpg_fingerprints: Vec::new(),
            verified_transactions_count: 0,
        };
        verifier.trusted_gpg_fingerprints.push(String::from("A4B2C3D4E5F678901234567890ABCDEF12345678")); // Linux Mint Release Key
        verifier
    }

    pub fn add_trusted_key(&mut self, fingerprint: &str) {
        if !self.trusted_gpg_fingerprints.contains(&fingerprint.to_string()) {
            self.trusted_gpg_fingerprints.push(fingerprint.to_string());
        }
    }

    pub fn verify_and_commit_package(&mut self, pkg_name: &str, signing_key: &str) -> Result<String, &'static str> {
        if !self.trusted_gpg_fingerprints.contains(&signing_key.to_string()) {
            return Err("GPG verification failed: Untrusted signature key");
        }
        self.verified_transactions_count += 1;
        Ok(format!("Successfully verified and installed signed package '{}'", pkg_name))
    }
}

/// Timeshift Snapshot Reboot & Power-Loss Recovery Verification Engine
pub struct MintTimeshiftSnapshotRestoreVerification {
    pub active_state_hash: u64,
    pub snapshot_state_hashes: Vec<(u32, u64)>, // (snap_id, hash)
}

impl Default for MintTimeshiftSnapshotRestoreVerification {
    fn default() -> Self {
        Self::new()
    }
}

impl MintTimeshiftSnapshotRestoreVerification {
    pub fn new() -> Self {
        Self {
            active_state_hash: 0xA5A5_1234_5678_9ABC,
            snapshot_state_hashes: Vec::new(),
        }
    }

    pub fn register_snapshot_hash(&mut self, snap_id: u32, hash: u64) {
        self.snapshot_state_hashes.push((snap_id, hash));
    }

    pub fn perform_atomic_rollback(&mut self, snap_id: u32) -> Result<u64, &'static str> {
        if let Some(&(_, hash)) = self.snapshot_state_hashes.iter().find(|(id, _)| *id == snap_id) {
            self.active_state_hash = hash;
            Ok(hash)
        } else {
            Err("Target restore checkpoint hash not found")
        }
    }
}

/// Mint Cinnamon / Zenith Compositor Wayland & Framebuffer Bridge
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompositorRenderBackend {
    WaylandNative,
    SoftwareFramebuffer,
    HardwareDrmKms,
}

pub struct MintZenithCompositorBackendBridge {
    pub active_backend: CompositorRenderBackend,
    pub is_initialized: bool,
    pub screen_width: u32,
    pub screen_height: u32,
}

impl Default for MintZenithCompositorBackendBridge {
    fn default() -> Self {
        Self::new()
    }
}

impl MintZenithCompositorBackendBridge {
    pub fn new() -> Self {
        Self {
            active_backend: CompositorRenderBackend::SoftwareFramebuffer,
            is_initialized: false,
            screen_width: 1920,
            screen_height: 1080,
        }
    }

    pub fn initialize_backend(&mut self, backend: CompositorRenderBackend) -> Result<(), &'static str> {
        self.active_backend = backend;
        self.is_initialized = true;
        Ok(())
    }

    pub fn render_composite_frame(&self) -> Result<usize, &'static str> {
        if !self.is_initialized {
            return Err("Compositor bridge not initialized");
        }
        let frame_bytes = (self.screen_width * self.screen_height * 4) as usize;
        Ok(frame_bytes)
    }
}

/// Cinnamon Spices Applet & Extension Manifest Validator (metadata.json parser)
#[derive(Debug, Clone)]
pub struct SpiceManifest {
    pub uuid: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub max_cinnamon_version: String,
    pub required_dependencies: Vec<String>,
}

pub struct MintCinnamonSpicesRegistry {
    pub manifests: Vec<SpiceManifest>,
}

impl Default for MintCinnamonSpicesRegistry {
    fn default() -> Self {
        Self::new()
    }
}

impl MintCinnamonSpicesRegistry {
    pub fn new() -> Self {
        Self { manifests: Vec::new() }
    }

    pub fn register_manifest(&mut self, uuid: &str, name: &str, desc: &str, ver: &str, max_cin: &str, deps: &[&str]) {
        self.manifests.push(SpiceManifest {
            uuid: uuid.to_string(),
            name: name.to_string(),
            description: desc.to_string(),
            version: ver.to_string(),
            max_cinnamon_version: max_cin.to_string(),
            required_dependencies: deps.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn validate_compatibility(&self, uuid: &str, current_cinnamon_ver: &str) -> Result<bool, &'static str> {
        let m = self.manifests.iter().find(|m| m.uuid == uuid).ok_or("Spice manifest not found")?;
        Ok(current_cinnamon_ver <= m.max_cinnamon_version.as_str())
    }
}

/// MintUpdate Automated Kernel & Security Patch Staging Pipeline
pub struct MintUpdateAutomatedStagingEngine {
    pub minimum_safety_score: usize,
    pub staged_packages: Vec<MintUpdatePackage>,
}

impl Default for MintUpdateAutomatedStagingEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl MintUpdateAutomatedStagingEngine {
    pub fn new() -> Self {
        Self {
            minimum_safety_score: 80,
            staged_packages: Vec::new(),
        }
    }

    pub fn stage_safe_updates(&mut self, pending: &[MintUpdatePackage]) -> usize {
        let mut count = 0;
        for pkg in pending {
            if pkg.safety_score >= self.minimum_safety_score {
                self.staged_packages.push(pkg.clone());
                count += 1;
            }
        }
        count
    }
}

/// MintInstall Fast Inverted Index Search Engine for AppStream Catalog
#[derive(Debug, Clone)]
pub struct AppStreamSearchDocument {
    pub app_id: String,
    pub title: String,
    pub keywords: Vec<String>,
}

pub struct MintInstallAppStreamSearchIndex {
    pub documents: Vec<AppStreamSearchDocument>,
}

impl Default for MintInstallAppStreamSearchIndex {
    fn default() -> Self {
        Self::new()
    }
}

impl MintInstallAppStreamSearchIndex {
    pub fn new() -> Self {
        Self { documents: Vec::new() }
    }

    pub fn index_app(&mut self, app_id: &str, title: &str, keywords: &[&str]) {
        self.documents.push(AppStreamSearchDocument {
            app_id: app_id.to_string(),
            title: title.to_string(),
            keywords: keywords.iter().map(|s| s.to_string()).collect(),
        });
    }

    pub fn search(&self, query: &str) -> Vec<&AppStreamSearchDocument> {
        let q = query.to_lowercase();
        self.documents
            .iter()
            .filter(|doc| {
                doc.title.to_lowercase().contains(&q)
                    || doc.app_id.to_lowercase().contains(&q)
                    || doc.keywords.iter().any(|k| k.to_lowercase().contains(&q))
            })
            .collect()
    }
}

/// Nemo File Manager Spatial File Previewer Plugin
#[derive(Debug, Clone)]
pub struct FilePreviewMetadata {
    pub filename: String,
    pub mime_type: String,
    pub preview_summary: String,
    pub size_bytes: u64,
}

pub struct MintNemoPreviewPlugin {
    pub previews: Vec<FilePreviewMetadata>,
}

impl Default for MintNemoPreviewPlugin {
    fn default() -> Self {
        Self::new()
    }
}

impl MintNemoPreviewPlugin {
    pub fn new() -> Self {
        Self { previews: Vec::new() }
    }

    pub fn generate_preview(&mut self, filename: &str, mime: &str, size: u64) -> &FilePreviewMetadata {
        let summary = format!("Spatial preview for {} [{}] - {} bytes", filename, mime, size);
        self.previews.push(FilePreviewMetadata {
            filename: filename.to_string(),
            mime_type: mime.to_string(),
            preview_summary: summary,
            size_bytes: size,
        });
        self.previews.last().unwrap()
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mint_sticky_notes_manager() {
        let mut notes = MintStickyNotesManager::new();
        let id1 = notes.create_note("Shopping", "Buy apples & milk", StickyNoteColor::Yellow);
        assert_eq!(id1, 1);
        assert_eq!(notes.notes.len(), 1);

        let pinned = notes.toggle_pin(id1).unwrap();
        assert!(pinned);

        notes.update_body(id1, "Buy apples, milk, & bread").unwrap();
        assert_eq!(notes.notes[0].body, "Buy apples, milk, & bread");
    }

    #[test]
    fn test_mint_cinnamon_spices_registry() {
        let mut reg = MintCinnamonSpicesRegistry::new();
        reg.register_manifest("weather@cinnamon.org", "Weather Applet", "Displays weather info", "1.2", "6.2", &["python3-requests"]);
        assert_eq!(reg.manifests.len(), 1);

        assert!(reg.validate_compatibility("weather@cinnamon.org", "6.0").unwrap());
        assert!(!reg.validate_compatibility("weather@cinnamon.org", "6.5").unwrap());
    }

    #[test]
    fn test_mint_update_automated_staging_engine() {
        let mut staging = MintUpdateAutomatedStagingEngine::new();
        let pending = vec![
            MintUpdatePackage::new(b"zenith", b"1.0.0", b"1.1.0", MintUpdateLevel::Level1Safe),
            MintUpdatePackage::new(b"kernel-core", b"6.5.0", b"6.6.0", MintUpdateLevel::Level5Critical),
        ];

        let staged = staging.stage_safe_updates(&pending);
        assert_eq!(staged, 1); // Only Level1Safe (score 99 >= 80) staged
        assert_eq!(staging.staged_packages.len(), 1);
    }

    #[test]
    fn test_mint_install_appstream_search_index() {
        let mut index = MintInstallAppStreamSearchIndex::new();
        index.index_app("org.videolan.VLC", "VLC Media Player", &["video", "movie", "audio", "player"]);

        let results = index.search("movie");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "VLC Media Player");
    }

    #[test]
    fn test_mint_nemo_preview_plugin() {
        let mut preview = MintNemoPreviewPlugin::new();
        let meta = preview.generate_preview("photo.png", "image/png", 102400);
        assert_eq!(meta.filename, "photo.png");
        assert!(meta.preview_summary.contains("102400 bytes"));
    }

    #[test]
    fn test_mint_installer_partitioning_engine() {
        let mut installer_part = MintInstallerPartitioningEngine::new();
        installer_part.add_partition("/dev/sda", 1, 512, PartitionTargetFs::Fat32, "/boot/efi");
        installer_part.add_partition("/dev/sda", 2, 60000, PartitionTargetFs::Ext4, "/");
        assert_eq!(installer_part.partitions.len(), 2);
        assert_eq!(installer_part.efi_system_partition_index, Some(0));

        let formatted_count = installer_part.format_all_partitions().unwrap();
        assert_eq!(formatted_count, 2);
        assert!(installer_part.partitions[0].is_formatted);
    }

    #[test]
    fn test_mint_software_manager_transaction_verifier() {
        let mut verifier = MintSoftwareManagerTransactionVerifier::new();
        let release_key = "A4B2C3D4E5F678901234567890ABCDEF12345678";

        let res = verifier.verify_and_commit_package("cinnamon-desktop", release_key).unwrap();
        assert!(res.contains("Successfully verified and installed"));
        assert_eq!(verifier.verified_transactions_count, 1);

        assert!(verifier.verify_and_commit_package("untrusted-pkg", "INVALID_KEY").is_err());
    }

    #[test]
    fn test_mint_timeshift_snapshot_restore_verification() {
        let mut timeshift_verif = MintTimeshiftSnapshotRestoreVerification::new();
        timeshift_verif.register_snapshot_hash(1, 0x1122_3344_5566_7788);

        let restored_hash = timeshift_verif.perform_atomic_rollback(1).unwrap();
        assert_eq!(restored_hash, 0x1122_3344_5566_7788);
        assert_eq!(timeshift_verif.active_state_hash, 0x1122_3344_5566_7788);
    }

    #[test]
    fn test_mint_zenith_compositor_backend_bridge() {
        let mut bridge = MintZenithCompositorBackendBridge::new();
        assert!(bridge.render_composite_frame().is_err());

        bridge.initialize_backend(CompositorRenderBackend::WaylandNative).unwrap();
        assert!(bridge.is_initialized);

        let bytes = bridge.render_composite_frame().unwrap();
        assert_eq!(bytes, 1920 * 1080 * 4);
    }

    #[test]
    fn test_mint_wacom_tablet_support_manager() {
        let mut wacom = MintWacomTabletSupportManager::new();
        wacom.register_tablet("Wacom Intuos Pro", "Undo", "Redo");
        let cfg = wacom.get_config("Wacom Intuos Pro").unwrap();
        assert_eq!(cfg.button_1_mapping, "Undo");
    }

    #[test]
    fn test_mint_cinnamon_theme_marketplace() {
        let mut marketplace = MintCinnamonThemeMarketplace::new();
        assert_eq!(marketplace.available_spices.len(), 1);

        let res = marketplace.install_spice("cinnamon-theme-adapta-nokto").unwrap();
        assert!(res.contains("Successfully installed"));
        assert!(marketplace.available_spices[0].is_installed);
    }

    #[test]
    fn test_mint_community_hardware_database() {
        let mut hw_db = MintCommunityHardwareDatabase::new();
        hw_db.submit_report("pci:v0002d0003", "Intel Wi-Fi 6", "iwlwifi", 5, "Works out of the box");

        let r = hw_db.lookup_driver("pci:v0002d0003").unwrap();
        assert_eq!(r.hardware_name, "Intel Wi-Fi 6");
        assert_eq!(r.rating_stars, 5);
    }

    #[test]
    fn test_mint_advanced_debugging_suite() {
        let mut dbg = MintAdvancedDebuggingSuite::new();
        let id = dbg.record_crash("zenith-wm", 11, &[("main", "main.rs", 42), ("render", "render.rs", 100)]);
        assert_eq!(id, 1);

        let analysis = dbg.analyze_crash(id).unwrap();
        assert!(analysis.contains("2 frames in backtrace"));
    }

    #[test]
    fn test_mint_cinnamon_applet_tray_engine() {
        let mut tray = MintCinnamonAppletTrayEngine::new();
        tray.register_sni("org.gnome.Volume", "Volume Control", "audio-volume-high");
        assert_eq!(tray.get_visible_items().len(), 1);
        assert_eq!(tray.get_visible_items()[0].title, "Volume Control");
    }

    #[test]
    fn test_mint_update_mirror_speed_tester() {
        let mut tester = MintUpdateMirrorSpeedTester::new();
        tester.test_mirror("https://mirror.us.mint.org", 20, 50000);
        tester.test_mirror("https://mirror.eu.mint.org", 100, 20000);

        let fastest = tester.get_fastest_mirror().unwrap();
        assert_eq!(fastest.mirror_url, "https://mirror.us.mint.org");
    }

    #[test]
    fn test_mint_install_package_categories_catalog() {
        let catalog = MintInstallPackageCategoriesCatalog::new();
        let internet = catalog.get_category("internet").unwrap();
        assert_eq!(internet.display_name, "Internet");
        assert!(internet.featured_app_ids.contains(&"firefox".to_string()));
    }

    #[test]
    fn test_mint_nemo_extension_manager() {
        let mut nemo_ext = MintNemoExtensionManager::new();
        nemo_ext.register_extension("nemo-share", false, true);

        let providers = nemo_ext.get_menu_providers();
        assert_eq!(providers.len(), 1);
        assert_eq!(providers[0].name, "nemo-share");
    }

    #[test]
    fn test_mint_cinnamon_hot_corner_engine() {
        let mut corners = MintCinnamonHotCornerEngine::new();
        assert_eq!(corners.trigger_corner(HotCornerLocation::TopLeft), Some(HotCornerAction::ExpoWorkspaces));

        corners.set_corner_action(HotCornerLocation::BottomRight, HotCornerAction::ShowDesktop, "");
        assert_eq!(corners.trigger_corner(HotCornerLocation::BottomRight), Some(HotCornerAction::ShowDesktop));
    }

    #[test]
    fn test_mint_update_kernel_pinning() {
        let mut kernel_pin = MintUpdateKernelPinning::new();
        kernel_pin.pin_kernel("6.5.6-sigma");
        assert!(kernel_pin.is_kernel_pinned("6.5.6-sigma"));

        kernel_pin.unpin_kernel("6.5.6-sigma");
        assert!(!kernel_pin.is_kernel_pinned("6.5.6-sigma"));
    }

    #[test]
    fn test_mint_install_flatpak_ref_fetcher() {
        let mut flatpak_ref = MintInstallFlatpakRefFetcher::new();
        let desc = flatpak_ref.parse_flatpakref("vlc", "stable", "VLC Media Player", "https://dl.flathub.org/repo/appstream/vlc.flatpakref").unwrap();
        assert_eq!(desc.name, "vlc");
        assert!(desc.gpg_key_valid);
    }

    #[test]
    fn test_mint_system_monitor_hud() {
        let mut hud = MintSystemMonitorHUD::new();
        hud.update_telemetry(25.0, 4096, 500.0, 100.0);
        assert_eq!(hud.telemetry.cpu_usage_pct, 25.0);
        assert_eq!(hud.telemetry.ram_usage_mb, 4096);
    }

    #[test]
    fn test_mint_cinnamon_workspace_manager() {
        let mut ws_mgr = MintCinnamonWorkspaceManager::new();
        assert_eq!(ws_mgr.workspaces.len(), 2);

        let new_idx = ws_mgr.add_workspace("Development");
        assert_eq!(new_idx, 2);

        assert!(ws_mgr.switch_to_workspace(2).is_ok());
        assert_eq!(ws_mgr.active_workspace_idx, 2);
    }

    #[test]
    fn test_mint_sound_scheme_engine() {
        let mut sounds = MintSoundSchemeEngine::new();
        assert!(sounds.play_sound_event(SoundEventType::Login));
        assert_eq!(sounds.triggered_events.len(), 1);

        sounds.sound_events_enabled = false;
        assert!(!sounds.play_sound_event(SoundEventType::TrashEmpty));
    }

    #[test]
    fn test_mint_user_accounts_manager() {
        let mut users_mgr = MintUserAccountsManager::new();
        assert_eq!(users_mgr.users.len(), 1);

        users_mgr.create_user("alice", "Alice Smith", false);
        assert_eq!(users_mgr.users.len(), 2);

        assert!(users_mgr.set_autologin("alice", true).is_ok());
        assert!(users_mgr.users[1].autologin_enabled);
    }

    #[test]
    fn test_mint_system_fixer_dkms_rebuilder() {
        let mut fixer = MintSystemFixerDkmsRebuilder::new();
        let rebuilt = fixer.rebuild_all_dkms_modules("6.8.0-sigma");
        assert_eq!(rebuilt, 3);
    }

    #[test]
    fn test_mint_upload_manager() {
        let mut upload = MintUploadManager::new();
        upload.add_profile("Community FTP", "ftp.example.com", 21, UploadProtocol::Ftp, "/uploads", "https://example.com/files");
        assert_eq!(upload.profiles.len(), 1);

        let link = upload.generate_share_link("Community FTP", "image.png").unwrap();
        assert_eq!(link, "https://example.com/files/image.png");
    }

    #[test]
    fn test_mint_digit_keyring_prompt() {
        let mut prompt = MintDigitKeyringPrompt::new();
        let valid_hash = [0x77u8; 16];
        let invalid_hash = [0x00u8; 16];
        prompt.set_expected_hash(valid_hash);

        assert!(prompt.request_auth("apt update", &invalid_hash).is_err());
        assert!(!prompt.is_authenticated);

        assert!(prompt.request_auth("apt update", &valid_hash).is_ok());
        assert!(prompt.is_authenticated);
    }

    #[test]
    fn test_mint_desktop_icon_organizer() {
        let mut organizer = MintDesktopIconOrganizer::new();
        organizer.auto_arrange(&["Home", "Trash", "Documents"]);
        assert_eq!(organizer.icons.len(), 3);
        assert_eq!(organizer.icons[0].file_name, "Home");
        assert_eq!(organizer.icons[0].grid_y, 0);
        assert_eq!(organizer.icons[2].grid_y, 2);
    }

    #[test]
    fn test_mint_xapps_thumbnails() {
        let xapps_thumb = MintXappsThumbnails::new();
        assert!(xapps_thumb.can_thumbnail("document.pdf"));
        assert!(xapps_thumb.can_thumbnail("movie.mp4"));
        assert!(!xapps_thumb.can_thumbnail("archive.iso"));
    }

    #[test]
    fn test_mint_webapp_manager() {
        let mut webapps = MintWebAppManager::new();
        let id = webapps.register_webapp(
            "GitHub",
            "https://github.com",
            "Development",
            "github.png",
            true,
        );
        assert_eq!(id, 1);

        webapps
            .set_custom_user_agent(id, "Mozilla/5.0 Custom")
            .unwrap();
        assert_eq!(
            webapps.web_apps[0].custom_user_agent,
            Some("Mozilla/5.0 Custom".to_string())
        );

        let cmd = webapps.launch_webapp_command(id).unwrap();
        assert!(cmd.contains("zenith-browser --app=\"https://github.com\""));
    }

    #[test]
    fn test_mint_hypnotix_iptv_engine() {
        let mut hypnotix = MintHypnotixIptvEngine::new();
        hypnotix.add_channel(
            "News 24",
            "https://stream.news24.com/live.m3u8",
            "US",
            "News",
            true,
        );
        assert_eq!(hypnotix.channels.len(), 1);

        let m3u_added =
            hypnotix.import_m3u_playlist("#EXTM3U\nhttps://stream.sports.com/live.m3u8");
        assert_eq!(m3u_added, 1);
        assert_eq!(hypnotix.channels.len(), 2);

        let status = hypnotix.play_channel("News 24").unwrap();
        assert!(status.contains("Playing stream from https://stream.news24.com/live.m3u8"));

        let us_channels = hypnotix.filter_by_country("US");
        assert_eq!(us_channels.len(), 1);
    }

    #[test]
    fn test_mint_welcome_wizard() {
        let mut wizard = MintWelcomeWizard::new();
        assert_eq!(wizard.layout, DesktopLayoutPreset::Traditional);
        assert!(!wizard.is_setup_completed());

        wizard.set_desktop_layout(DesktopLayoutPreset::Modern);
        wizard.mark_timeshift_completed();
        wizard.mark_drivers_reviewed();
        wizard.mark_updates_checked();

        assert_eq!(wizard.layout, DesktopLayoutPreset::Modern);
        assert!(wizard.is_setup_completed());
    }

    #[test]
    fn test_mint_nanny_domain_filter() {
        let mut nanny = MintNannyDomainFilter::new();
        nanny.add_blocked_domain("gambling.com", "Parental Control");

        assert!(nanny.is_domain_blocked("https://gambling.com/poker", 14));
        assert!(!nanny.is_domain_blocked("https://wikipedia.org", 14));

        nanny.filter_active = false;
        assert!(!nanny.is_domain_blocked("https://gambling.com/poker", 14));
    }

    #[test]
    fn test_mint_locale_manager() {
        let mut locale_mgr = MintLocaleManager::new();
        assert_eq!(locale_mgr.active_locale, "en_US.UTF-8");

        locale_mgr.install_language_pack(LanguagePack {
            locale_code: "fr_FR.UTF-8".to_string(),
            name: "French".to_string(),
            has_spellcheck: true,
            input_method: "IBus".to_string(),
        });

        assert!(locale_mgr.switch_active_locale("fr_FR.UTF-8").is_ok());
        assert_eq!(locale_mgr.active_locale, "fr_FR.UTF-8");
    }

    #[test]
    fn test_mint_usb_formatter_tool() {
        let mut usb_tool = MintUsbFormatterTool::new();
        assert!(usb_tool.select_device("/dev/sdb").is_ok());

        let fmt_result = usb_tool
            .format_filesystem(UsbFileSystemFormat::Fat32, "MINT_USB")
            .unwrap();
        assert!(fmt_result.contains("Successfully formatted /dev/sdb"));

        let iso_result = usb_tool.write_iso_image("/home/user/sigmaos.iso").unwrap();
        assert!(iso_result.contains("Successfully wrote ISO image"));
    }

    #[test]
    fn test_mint_software_sources_ppa_manager() {
        let mut ppa_mgr = MintSoftwareSourcesPpaManager::new();
        ppa_mgr.add_ppa("ppa:cinnamon/stable", "1234567890ABCDEF");
        assert_eq!(ppa_mgr.ppas.len(), 1);

        assert!(ppa_mgr.remove_ppa("ppa:cinnamon/stable").is_ok());
        assert_eq!(ppa_mgr.ppas.len(), 0);
    }

    #[test]
    fn test_mint_cinnamon_spices_applet_manager() {
        let mut applet_mgr = MintCinnamonSpicesAppletManager::new();
        applet_mgr.add_applet("workspace-switcher@cinnamon.org", "Workspace Switcher", 0);
        assert_eq!(applet_mgr.applets.len(), 1);

        assert!(applet_mgr
            .set_applet_state("workspace-switcher@cinnamon.org", false)
            .is_ok());
        assert!(!applet_mgr.applets[0].enabled);
    }

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
    fn test_mint_warpinator_file_transfer() {
        let mut warpinator = MintWarpinatorFileTransferEngine::new();
        let peer_uuid = warpinator.discover_peer("mint-laptop", "192.168.1.100", 42000, "123456");

        // Unpaired queue attempt should fail
        assert!(warpinator
            .queue_file_transfer(&peer_uuid, "document.pdf", 1024)
            .is_err());

        // Pair with matching PIN
        assert!(warpinator.pair_peer(&peer_uuid, "123456").is_ok());

        // Paired queue attempt should succeed
        assert!(warpinator
            .queue_file_transfer(&peer_uuid, "document.pdf", 1024)
            .is_ok());
        assert_eq!(warpinator.transfer_queue.len(), 1);
    }

    #[test]
    fn test_mint_bulky_batch_renamer() {
        let files = vec!["photo_1.png", "photo_2.png"];

        let replaced =
            MintBulkyBatchRenamerEngine::rename_find_replace(&files, "photo_", "vacation_");
        assert_eq!(replaced, vec!["vacation_1.png", "vacation_2.png"]);

        let prefixed =
            MintBulkyBatchRenamerEngine::rename_add_prefix_suffix(&files, "2026_", "_backup");
        assert_eq!(
            prefixed,
            vec!["2026_photo_1_backup.png", "2026_photo_2_backup.png"]
        );

        let cased =
            MintBulkyBatchRenamerEngine::rename_change_case(&files, BulkyCaseMode::Uppercase);
        assert_eq!(cased, vec!["PHOTO_1.PNG", "PHOTO_2.PNG"]);
    }

    #[test]
    fn test_mint_xapp_status_icon_tray() {
        let mut tray = MintXAppStatusIconTray::new();
        let id = tray.add_status_icon("update-notifier", "Updates Available");
        assert_eq!(id, 1);

        assert!(tray.set_badge_count(id, 5).is_ok());
        assert_eq!(tray.icons[0].badge_count, 5);
    }

    #[test]
    fn test_mint4win_installer_flow() {
        let config = Mint4WinInstallationConfig::default_windows_c('C', "mintuser");
        let mut installer = Mint4WinInstallerEngine::new(config);

        let alloc_res = installer.allocate_loopback_disks().unwrap();
        assert!(alloc_res.contains("Successfully allocated 32768 MB"));

        let bcd_res = installer.configure_windows_bcd_boot_entry().unwrap();
        assert!(bcd_res.contains("Added Windows BCD boot entry"));

        let script = installer.generate_unattended_install_script();
        assert!(script.contains("unattended_user=\"mintuser\""));

        let uninst_res = installer.uninstall_mint4win().unwrap();
        assert!(uninst_res.contains("Successfully removed mint4win"));
    }
}
